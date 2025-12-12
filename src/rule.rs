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
}

/// Builder for creating rules.
pub struct RuleBuilder {
    regex: Regex,
    colors: Vec<Color>,
    count_mode: CountMode,
    bold: bool,
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

    /// Build the rule.
    pub fn build(self) -> Rule {
        Rule {
            regex: self.regex,
            colors: self.colors,
            count_mode: self.count_mode,
            bold: self.bold,
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
}
