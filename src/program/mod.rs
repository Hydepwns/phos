//! Program system for extensible log colorization.
//!
//! This module provides the `Program` trait for defining colorization rules,
//! and the `ProgramRegistry` for managing and discovering programs.

use std::collections::HashMap;
use std::sync::Arc;

use regex::Regex;

use crate::colors::Color;
use crate::rule::Rule;

pub mod config;
pub mod loader;

/// Information about a program.
#[derive(Debug, Clone)]
pub struct ProgramInfo {
    /// Unique identifier (e.g., "ethereum.lodestar", "devops.docker")
    pub id: String,
    /// Display name (e.g., "Lodestar", "Docker")
    pub name: String,
    /// Description of what this program colorizes
    pub description: String,
    /// Category for grouping (e.g., "ethereum", "devops", "system")
    pub category: String,
}

impl ProgramInfo {
    /// Create a new ProgramInfo.
    pub fn new(id: &str, name: &str, description: &str, category: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            category: category.to_string(),
        }
    }
}

/// A program that provides colorization rules for a specific log format.
pub trait Program: Send + Sync {
    /// Get program information.
    fn info(&self) -> &ProgramInfo;

    /// Get the colorization rules for this program.
    fn rules(&self) -> Vec<Rule>;

    /// Get domain-specific color definitions.
    /// These are colors specific to this program's domain that are not universal semantic colors.
    fn domain_colors(&self) -> HashMap<String, Color> {
        HashMap::new()
    }

    /// Get patterns for auto-detecting this program from command lines.
    /// Each pattern is matched against the command being colorized.
    fn detect_patterns(&self) -> Vec<&'static str> {
        Vec::new()
    }
}

/// A simple program implementation that can be constructed from data.
///
/// Use this to define programs without boilerplate struct/impl blocks.
pub struct SimpleProgram {
    info: ProgramInfo,
    rules: Vec<Rule>,
    detect_patterns: Vec<&'static str>,
    domain_colors: HashMap<String, Color>,
}

impl SimpleProgram {
    /// Create a new simple program with the given info and rules.
    pub fn new(
        id: &str,
        name: &str,
        description: &str,
        category: &str,
        rules: Vec<Rule>,
    ) -> Self {
        Self {
            info: ProgramInfo::new(id, name, description, category),
            rules,
            detect_patterns: Vec::new(),
            domain_colors: HashMap::new(),
        }
    }

    /// Builder: set detection patterns for auto-detection from command lines.
    pub fn with_detect_patterns(mut self, patterns: Vec<&'static str>) -> Self {
        self.detect_patterns = patterns;
        self
    }

    /// Builder: set domain-specific colors.
    pub fn with_domain_colors(mut self, colors: HashMap<String, Color>) -> Self {
        self.domain_colors = colors;
        self
    }
}

impl Program for SimpleProgram {
    fn info(&self) -> &ProgramInfo {
        &self.info
    }

    fn rules(&self) -> Vec<Rule> {
        self.rules.clone()
    }

    fn detect_patterns(&self) -> Vec<&'static str> {
        self.detect_patterns.clone()
    }

    fn domain_colors(&self) -> HashMap<String, Color> {
        self.domain_colors.clone()
    }
}

/// Registry for managing programs.
pub struct ProgramRegistry {
    programs: HashMap<String, Arc<dyn Program>>,
}

impl Default for ProgramRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgramRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            programs: HashMap::new(),
        }
    }

    /// Register a program.
    pub fn register(&mut self, program: Arc<dyn Program>) {
        let id = program.info().id.clone();
        self.programs.insert(id, program);
    }

    /// Get a program by ID.
    pub fn get(&self, id: &str) -> Option<Arc<dyn Program>> {
        // Try exact match first
        if let Some(program) = self.programs.get(id) {
            return Some(Arc::clone(program));
        }

        // Try matching just the name part (e.g., "lodestar" matches "ethereum.lodestar")
        for (program_id, program) in &self.programs {
            if program_id.ends_with(&format!(".{id}")) || program.info().name.eq_ignore_ascii_case(id) {
                return Some(Arc::clone(program));
            }
        }

        None
    }

    /// Detect a program from a command string.
    ///
    /// Uses word-boundary matching to avoid false positives where a pattern
    /// might match a substring (e.g., "git" matching "digit").
    /// Returns the most specific match (longest pattern wins).
    pub fn detect(&self, cmd: &str) -> Option<Arc<dyn Program>> {
        let cmd_lower = cmd.to_lowercase();

        // Collect all matches with their pattern length for specificity
        let mut matches: Vec<(usize, Arc<dyn Program>)> = Vec::new();

        for program in self.programs.values() {
            for pattern in program.detect_patterns() {
                // Use word boundary matching to avoid substring false positives
                let regex_pattern = format!(r"(?i)\b{}\b", regex::escape(pattern));
                if let Ok(re) = Regex::new(&regex_pattern) {
                    if re.is_match(&cmd_lower) {
                        matches.push((pattern.len(), Arc::clone(program)));
                        break; // One match per program is enough
                    }
                }
            }
        }

        // Return most specific match (longest pattern wins)
        matches.sort_by(|a, b| b.0.cmp(&a.0));
        matches.into_iter().next().map(|(_, p)| p)
    }

    /// List all registered programs.
    pub fn list(&self) -> Vec<&ProgramInfo> {
        self.programs.values().map(|p| p.info()).collect()
    }

    /// List programs by category.
    pub fn list_by_category(&self, category: &str) -> Vec<&ProgramInfo> {
        self.programs
            .values()
            .filter(|p| p.info().category.eq_ignore_ascii_case(category))
            .map(|p| p.info())
            .collect()
    }

    /// Get all unique categories.
    pub fn categories(&self) -> Vec<String> {
        let mut categories: Vec<String> = self
            .programs
            .values()
            .map(|p| p.info().category.clone())
            .collect();
        categories.sort();
        categories.dedup();
        categories
    }

    /// Get the number of registered programs.
    pub fn len(&self) -> usize {
        self.programs.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.programs.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestProgram {
        info: ProgramInfo,
    }

    impl TestProgram {
        fn new(id: &str, name: &str, category: &str) -> Self {
            Self {
                info: ProgramInfo::new(id, name, "Test program", category),
            }
        }
    }

    impl Program for TestProgram {
        fn info(&self) -> &ProgramInfo {
            &self.info
        }

        fn rules(&self) -> Vec<Rule> {
            Vec::new()
        }

        fn detect_patterns(&self) -> Vec<&'static str> {
            vec!["test"]
        }
    }

    #[test]
    fn test_registry_register_and_get() {
        let mut registry = ProgramRegistry::new();
        let program = Arc::new(TestProgram::new("test.program", "TestProgram", "test"));
        registry.register(program);

        assert!(registry.get("test.program").is_some());
        assert!(registry.get("program").is_some()); // Short name lookup
        assert!(registry.get("testprogram").is_some()); // Name lookup (case insensitive)
    }

    #[test]
    fn test_registry_detect() {
        let mut registry = ProgramRegistry::new();
        let program = Arc::new(TestProgram::new("test.program", "TestProgram", "test"));
        registry.register(program);

        assert!(registry.detect("run test command").is_some());
        assert!(registry.detect("other command").is_none());
    }

    #[test]
    fn test_registry_list_by_category() {
        let mut registry = ProgramRegistry::new();
        registry.register(Arc::new(TestProgram::new("cat1.prog1", "Prog1", "cat1")));
        registry.register(Arc::new(TestProgram::new("cat1.prog2", "Prog2", "cat1")));
        registry.register(Arc::new(TestProgram::new("cat2.prog3", "Prog3", "cat2")));

        assert_eq!(registry.list_by_category("cat1").len(), 2);
        assert_eq!(registry.list_by_category("cat2").len(), 1);
    }
}
