//! Program system for extensible log colorization.
//!
//! This module provides the infrastructure for defining and managing log colorization
//! programs. Each program defines rules for colorizing a specific log format.
//!
//! # Key Types
//!
//! - [`Program`]: Trait for defining colorization programs
//! - [`SimpleProgram`]: Convenience implementation for most use cases
//! - [`ProgramRegistry`]: Registry for managing and discovering programs
//! - [`ProgramInfo`]: Metadata about a program (name, category, etc.)
//!
//! # Built-in Programs
//!
//! phos includes 99+ built-in programs. Use [`crate::programs::default_registry`]
//! to get a registry with all built-in programs.
//!
//! # Examples
//!
//! ```rust
//! use phos::programs;
//!
//! // Get the default registry with all built-in programs
//! let registry = programs::default_registry();
//!
//! // Find a program by name
//! let docker = registry.get("docker").unwrap();
//! println!("Program: {}", docker.info().name);
//!
//! // Auto-detect program from a command
//! if let Some(program) = registry.detect("docker logs -f mycontainer") {
//!     println!("Detected: {}", program.info().name);
//! }
//!
//! // List programs by category
//! use phos::Category;
//! for info in registry.list_by_category(Category::Ethereum) {
//!     println!("  {}: {}", info.name, info.description);
//! }
//! ```
//!
//! # Custom Programs
//!
//! ```rust
//! use std::sync::Arc;
//! use phos::{Rule, SemanticColor, Category};
//! use phos::program::{SimpleProgram, ProgramRegistry};
//!
//! // Create a custom program
//! let my_program = SimpleProgram::new(
//!     "custom.myapp",
//!     "MyApp",
//!     "My application logs",
//!     Category::Dev,
//!     vec![
//!         Rule::new(r"\bERROR\b").unwrap()
//!             .semantic(SemanticColor::Error)
//!             .build(),
//!     ],
//! ).with_detect_patterns(vec!["myapp", "my-app"]);
//!
//! // Register it
//! let mut registry = ProgramRegistry::new();
//! registry.register(Arc::new(my_program));
//! ```

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

use regex::Regex;

use crate::category::Category;
use crate::colors::Color;
use crate::rule::Rule;

pub mod config;
pub mod loader;

/// Information about a program.
///
/// Uses `Cow<'static, str>` for zero-copy static strings in built-in programs,
/// while still supporting owned strings for user-defined programs.
#[derive(Debug, Clone)]
pub struct ProgramInfo {
    /// Unique identifier (e.g., "ethereum.lodestar", "devops.docker")
    pub id: Cow<'static, str>,
    /// Display name (e.g., "Lodestar", "Docker")
    pub name: Cow<'static, str>,
    /// Description of what this program colorizes
    pub description: Cow<'static, str>,
    /// Category for grouping programs
    pub category: Category,
}

impl ProgramInfo {
    /// Create a new ProgramInfo with owned strings.
    ///
    /// Use this for user-defined programs loaded from configuration files.
    pub fn new(id: &str, name: &str, description: &str, category: Category) -> Self {
        Self {
            id: Cow::Owned(id.to_string()),
            name: Cow::Owned(name.to_string()),
            description: Cow::Owned(description.to_string()),
            category,
        }
    }

    /// Create a new ProgramInfo with static strings (zero allocation).
    ///
    /// Use this for built-in programs where all strings are compile-time constants.
    pub fn new_static(
        id: &'static str,
        name: &'static str,
        description: &'static str,
        category: Category,
    ) -> Self {
        Self {
            id: Cow::Borrowed(id),
            name: Cow::Borrowed(name),
            description: Cow::Borrowed(description),
            category,
        }
    }
}

/// A program that provides colorization rules for a specific log format.
///
/// Implement this trait to define custom log colorization programs.
/// For most cases, [`SimpleProgram`] provides a convenient implementation.
///
/// # Required Methods
///
/// - [`info`](Self::info): Returns program metadata
/// - [`rules`](Self::rules): Returns the colorization rules
///
/// # Optional Methods
///
/// - [`domain_colors`](Self::domain_colors): Domain-specific colors
/// - [`detect_patterns`](Self::detect_patterns): Patterns for auto-detection
pub trait Program: Send + Sync {
    /// Get program information.
    fn info(&self) -> &ProgramInfo;

    /// Get the colorization rules for this program.
    /// Returns an Arc to avoid cloning compiled regexes on every access.
    fn rules(&self) -> Arc<[Rule]>;

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
/// This is the recommended way to define custom programs. It implements
/// the [`Program`] trait and provides a builder-style API.
///
/// # Examples
///
/// ```rust
/// use std::sync::Arc;
/// use phos::{Rule, SemanticColor, Category};
/// use phos::program::SimpleProgram;
///
/// let program = SimpleProgram::new(
///     "custom.myapp",
///     "MyApp",
///     "My application logs",
///     Category::Dev,
///     vec![
///         Rule::new(r"\bERROR\b").unwrap()
///             .semantic(SemanticColor::Error)
///             .build(),
///     ],
/// )
/// .with_detect_patterns(vec!["myapp"]);
/// ```
pub struct SimpleProgram {
    info: ProgramInfo,
    rules: Arc<[Rule]>,
    detect_patterns: Vec<&'static str>,
    domain_colors: HashMap<String, Color>,
}

impl SimpleProgram {
    /// Create a new simple program with static strings (zero allocation).
    ///
    /// Use this for built-in programs where all strings are compile-time constants.
    pub fn new(
        id: &'static str,
        name: &'static str,
        description: &'static str,
        category: Category,
        rules: Vec<Rule>,
    ) -> Self {
        Self {
            info: ProgramInfo::new_static(id, name, description, category),
            rules: rules.into(),
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

    fn rules(&self) -> Arc<[Rule]> {
        Arc::clone(&self.rules)
    }

    fn detect_patterns(&self) -> Vec<&'static str> {
        self.detect_patterns.clone()
    }

    fn domain_colors(&self) -> HashMap<String, Color> {
        self.domain_colors.clone()
    }
}

/// Registry for managing programs.
///
/// The registry stores programs and provides lookup by ID, name, or command.
/// Use [`crate::programs::default_registry`] to get a registry with all
/// built-in programs.
///
/// # Examples
///
/// ```rust
/// use phos::programs;
///
/// let registry = programs::default_registry();
///
/// // Lookup by ID or name
/// let docker = registry.get("docker").unwrap();
/// let docker = registry.get("devops.docker").unwrap(); // Full ID also works
///
/// // Auto-detect from command
/// let detected = registry.detect("docker logs -f container");
/// assert!(detected.is_some());
/// ```
pub struct ProgramRegistry {
    programs: HashMap<String, Arc<dyn Program>>,
    /// Cached compiled regexes for detection patterns.
    /// Maps pattern string to compiled Regex.
    detection_cache: HashMap<&'static str, Regex>,
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
            detection_cache: HashMap::new(),
        }
    }

    /// Register a program and cache its detection patterns.
    pub fn register(&mut self, program: Arc<dyn Program>) {
        // Cache detection regexes for this program
        for pattern in program.detect_patterns() {
            if !self.detection_cache.contains_key(pattern) {
                let regex_pattern = format!(r"(?i)\b{}\b", regex::escape(pattern));
                if let Ok(re) = Regex::new(&regex_pattern) {
                    self.detection_cache.insert(pattern, re);
                }
            }
        }

        let id = program.info().id.to_string();
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
                // Look up cached regex
                if let Some(re) = self.detection_cache.get(pattern) {
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
    pub fn list_by_category(&self, category: Category) -> Vec<&ProgramInfo> {
        self.programs
            .values()
            .filter(|p| p.info().category == category)
            .map(|p| p.info())
            .collect()
    }

    /// Get all unique categories that have programs.
    pub fn categories(&self) -> Vec<Category> {
        let mut categories: Vec<Category> = self
            .programs
            .values()
            .map(|p| p.info().category)
            .collect();
        categories.sort_by_key(|c| c.as_str());
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
        detect: Vec<&'static str>,
    }

    impl TestProgram {
        fn new(id: &str, name: &str, category: Category) -> Self {
            Self {
                info: ProgramInfo::new(id, name, "Test program", category),
                detect: vec!["test"],
            }
        }

        fn with_detect(mut self, patterns: Vec<&'static str>) -> Self {
            self.detect = patterns;
            self
        }
    }

    impl Program for TestProgram {
        fn info(&self) -> &ProgramInfo {
            &self.info
        }

        fn rules(&self) -> Arc<[Rule]> {
            Arc::from([])
        }

        fn detect_patterns(&self) -> Vec<&'static str> {
            self.detect.clone()
        }
    }

    #[test]
    fn test_registry_register_and_get() {
        let mut registry = ProgramRegistry::new();
        let program = Arc::new(TestProgram::new("dev.program", "TestProgram", Category::Dev));
        registry.register(program);

        assert!(registry.get("dev.program").is_some());
        assert!(registry.get("program").is_some()); // Short name lookup
        assert!(registry.get("testprogram").is_some()); // Name lookup (case insensitive)
    }

    #[test]
    fn test_registry_detect() {
        let mut registry = ProgramRegistry::new();
        let program = Arc::new(TestProgram::new("dev.program", "TestProgram", Category::Dev));
        registry.register(program);

        assert!(registry.detect("run test command").is_some());
        assert!(registry.detect("other command").is_none());
    }

    #[test]
    fn test_registry_list_by_category() {
        let mut registry = ProgramRegistry::new();
        registry.register(Arc::new(
            TestProgram::new("dev.prog1", "Prog1", Category::Dev).with_detect(vec!["prog1"]),
        ));
        registry.register(Arc::new(
            TestProgram::new("dev.prog2", "Prog2", Category::Dev).with_detect(vec!["prog2"]),
        ));
        registry.register(Arc::new(
            TestProgram::new("system.prog3", "Prog3", Category::System).with_detect(vec!["prog3"]),
        ));

        assert_eq!(registry.list_by_category(Category::Dev).len(), 2);
        assert_eq!(registry.list_by_category(Category::System).len(), 1);
    }
}
