//! Configuration file loading and parsing.

use std::fs;
use std::path::Path;

use serde::Deserialize;
use thiserror::Error;

use crate::colors::SemanticColor;
use crate::rule::Rule;

/// Configuration loading errors.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("Failed to parse YAML: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Invalid regex pattern: {0}")]
    RegexError(#[from] regex::Error),

    #[error("Unknown file format: {0}")]
    UnknownFormat(String),
}

/// Configuration file format.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Configuration name
    pub name: String,

    /// Description
    #[serde(default)]
    pub description: String,

    /// Rules to apply
    #[serde(default)]
    pub rules: Vec<RuleConfig>,
}

/// Rule configuration from file.
/// Shared between CLI config files and user-defined program configs.
#[derive(Debug, Clone, Deserialize)]
pub struct RuleConfig {
    /// Regex pattern
    pub regex: String,

    /// Colors to apply
    #[serde(default)]
    pub colors: Vec<String>,

    /// Whether to apply bold
    #[serde(default)]
    pub bold: bool,

    /// Skip the entire line if this rule matches
    #[serde(default)]
    pub skip: bool,

    /// Replacement pattern (uses ${1}, ${2} for backreferences)
    #[serde(default)]
    pub replace: Option<String>,
}

impl Config {
    /// Load configuration from a file path.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;

        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        match extension.to_lowercase().as_str() {
            "yaml" | "yml" => Ok(serde_yaml::from_str(&content)?),
            "json" => Ok(serde_json::from_str(&content)?),
            ext => Err(ConfigError::UnknownFormat(ext.to_string())),
        }
    }

    /// Convert configuration to rules.
    pub fn to_rules(&self) -> Result<Vec<Rule>, ConfigError> {
        self.rules.iter().map(rule_from_config).collect()
    }
}

/// Convert a single rule config to a Rule.
fn rule_from_config(config: &RuleConfig) -> Result<Rule, ConfigError> {
    let mut builder = config
        .colors
        .iter()
        .filter(|c| *c != "bold")
        .fold(Rule::new(&config.regex)?, |b, color| {
            match parse_semantic_color(color) {
                Some(s) => b.semantic(s),
                None => b.named(color),
            }
        });

    // Apply skip if set
    if config.skip {
        builder = builder.skip();
    }

    // Apply replace if set
    if let Some(ref replacement) = config.replace {
        builder = builder.replace(replacement);
    }

    let should_bold = config.bold || config.colors.iter().any(|c| c == "bold");

    Ok(if should_bold {
        builder.bold().build()
    } else {
        builder.build()
    })
}

/// Parse a semantic color name.
/// Non-semantic colors (domain colors, hex, ANSI) return None and are handled elsewhere.
fn parse_semantic_color(name: &str) -> Option<SemanticColor> {
    SemanticColor::from_name(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_yaml() {
        let yaml = r#"
name: test
description: Test config
rules:
  - regex: '\berror\b'
    colors: [error, bold]
  - regex: '\d+'
    colors: [number]
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.name, "test");
        assert_eq!(config.rules.len(), 2);
    }

    #[test]
    fn test_semantic_color_parsing() {
        assert_eq!(parse_semantic_color("error"), Some(SemanticColor::Error));
        assert_eq!(parse_semantic_color("WARN"), Some(SemanticColor::Warn));
        assert_eq!(parse_semantic_color("unknown"), None);
    }
}
