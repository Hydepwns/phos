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
    let builder = if should_bold { builder.bold() } else { builder };

    Ok(builder.build())
}

/// Parse a semantic color name.
/// Non-semantic colors (domain colors, hex, ANSI) return None and are handled elsewhere.
fn parse_semantic_color(name: &str) -> Option<SemanticColor> {
    SemanticColor::from_name(name)
}

// ---------------------------------------------------------------------------
// Global Configuration
// ---------------------------------------------------------------------------

/// Global phos configuration from ~/.config/phos/config.yaml.
///
/// Settings in this file provide defaults that can be overridden by CLI flags.
/// Resolution order: CLI > global config > built-in defaults.
#[derive(Debug, Default, Deserialize)]
pub struct GlobalConfig {
    /// Default theme name
    #[serde(default)]
    pub theme: Option<String>,

    /// Enable stats by default
    #[serde(default)]
    pub stats: bool,

    /// Default stats export format (human, json, prometheus)
    #[serde(default)]
    pub stats_export: Option<String>,

    /// Stats interval in seconds (0 = end only)
    #[serde(default)]
    pub stats_interval: u64,

    /// Force color output
    #[serde(default)]
    pub color: bool,

    /// Default alerting configuration
    #[serde(default)]
    pub alerts: AlertsConfig,
}

/// Alerting configuration section.
#[derive(Debug, Default, Deserialize)]
pub struct AlertsConfig {
    /// Webhook URL
    #[serde(default)]
    pub url: Option<String>,

    /// Telegram chat ID
    #[serde(default)]
    pub telegram_chat_id: Option<String>,

    /// Cooldown between alerts in seconds
    #[serde(default = "default_cooldown")]
    pub cooldown: u64,

    /// Alert conditions
    #[serde(default)]
    pub conditions: Vec<String>,
}

fn default_cooldown() -> u64 {
    60
}

impl GlobalConfig {
    /// Load global configuration from the default path.
    /// Returns None if the config file doesn't exist.
    /// Returns Err if the file exists but is invalid.
    pub fn load() -> Result<Option<Self>, ConfigError> {
        let config_path = match crate::program::loader::global_config_path() {
            Some(p) => p,
            None => return Ok(None),
        };

        if !config_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&config_path)?;
        let extension = config_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("yaml");

        let config = match extension.to_lowercase().as_str() {
            "yaml" | "yml" => serde_yaml::from_str(&content)?,
            "json" => serde_json::from_str(&content)?,
            _ => serde_yaml::from_str(&content)?, // Default to YAML
        };

        Ok(Some(config))
    }

    /// Load global configuration from a specific path.
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;

        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("yaml");

        match extension.to_lowercase().as_str() {
            "yaml" | "yml" => Ok(serde_yaml::from_str(&content)?),
            "json" => Ok(serde_json::from_str(&content)?),
            _ => Ok(serde_yaml::from_str(&content)?),
        }
    }
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
