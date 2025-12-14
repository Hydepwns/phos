//! Regex-based pattern matching rules.

use regex::Regex;

use crate::colors::{Color, SemanticColor};

/// How a rule should be applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountMode {
    /// Apply once per line (first match only)
    Once,
    /// Apply to all matches
    More,
    /// Stop processing after this rule matches
    Stop,
    /// Start block coloring until Unblock
    Block,
    /// End block coloring
    Unblock,
}

impl Default for CountMode {
    fn default() -> Self {
        Self::More
    }
}

/// A colorization rule with pattern and styling.
#[derive(Debug, Clone)]
pub struct Rule {
    /// Compiled regex pattern
    pub regex: Regex,
    /// Colors to apply (first is foreground)
    pub colors: Vec<Color>,
    /// How to apply the rule
    pub count_mode: CountMode,
    /// Whether to apply bold
    pub bold: bool,
    /// Skip the entire line if this rule matches
    pub skip: bool,
    /// Replacement pattern (uses $1, $2 for backreferences)
    pub replace: Option<String>,
}

/// Builder for creating rules.
pub struct RuleBuilder {
    regex: Regex,
    colors: Vec<Color>,
    count_mode: CountMode,
    bold: bool,
    skip: bool,
    replace: Option<String>,
}

impl Rule {
    /// Create a new rule from a regex pattern.
    /// Returns a RuleBuilder for fluent configuration.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(pattern: &str) -> Result<RuleBuilder, regex::Error> {
        let regex = Regex::new(pattern)?;
        Ok(RuleBuilder {
            regex,
            colors: Vec::new(),
            count_mode: CountMode::default(),
            bold: false,
            skip: false,
            replace: None,
        })
    }

    /// Check if the rule matches the given text.
    pub fn is_match(&self, text: &str) -> bool {
        self.regex.is_match(text)
    }

    /// Find all matches in the text.
    pub fn find_iter<'a>(&'a self, text: &'a str) -> impl Iterator<Item = regex::Match<'a>> {
        self.regex.find_iter(text)
    }
}

impl RuleBuilder {
    /// Set a semantic color.
    pub fn semantic(mut self, color: SemanticColor) -> Self {
        self.colors.push(Color::Semantic(color));
        self
    }

    /// Set a named color.
    pub fn named(mut self, name: &str) -> Self {
        self.colors.push(Color::Named(name.to_string()));
        self
    }

    /// Set a hex color.
    pub fn hex(mut self, hex: &str) -> Self {
        self.colors.push(Color::Hex(hex.to_string()));
        self
    }

    /// Add a color directly.
    pub fn color(mut self, color: Color) -> Self {
        self.colors.push(color);
        self
    }

    /// Set bold styling.
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set the count mode.
    pub fn count(mut self, mode: CountMode) -> Self {
        self.count_mode = mode;
        self
    }

    /// Set skip mode - skip the entire line if this rule matches.
    pub fn skip(mut self) -> Self {
        self.skip = true;
        self
    }

    /// Set a replacement pattern.
    /// Uses `${1}`, `${2}`, etc. for backreferences to capture groups.
    /// Named groups use `${name}` syntax.
    pub fn replace(mut self, pattern: &str) -> Self {
        self.replace = Some(pattern.to_string());
        self
    }

    /// Build the rule.
    pub fn build(self) -> Rule {
        Rule {
            regex: self.regex,
            colors: self.colors,
            count_mode: self.count_mode,
            bold: self.bold,
            skip: self.skip,
            replace: self.replace,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_creation() {
        let rule = Rule::new(r"\berror\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build();

        assert!(rule.is_match("an error occurred"));
        assert!(!rule.is_match("no errors here"));
        assert!(rule.bold);
    }

    #[test]
    fn test_count_modes() {
        let rule = Rule::new(r"\d+")
            .unwrap()
            .count(CountMode::Once)
            .build();

        assert_eq!(rule.count_mode, CountMode::Once);
    }

    #[test]
    fn test_skip_rule() {
        let rule = Rule::new(r"DEBUG")
            .unwrap()
            .skip()
            .build();

        assert!(rule.skip);
        assert!(rule.is_match("DEBUG: some message"));
        assert!(!rule.is_match("INFO: some message"));
    }

    #[test]
    fn test_replace_rule() {
        let rule = Rule::new(r"(\d{2}):(\d{2}):(\d{2})")
            .unwrap()
            .replace("${1}h${2}m${3}s")
            .build();

        assert!(rule.replace.is_some());
        assert_eq!(rule.replace.as_deref(), Some("${1}h${2}m${3}s"));

        // Test the replacement using replace_all which is what colorizer uses
        let input = "12:34:56";
        let replacement = rule.replace.as_deref().unwrap();
        let result = rule.regex.replace_all(input, replacement);
        assert_eq!(result.as_ref(), "12h34m56s");
    }
}
