//! Configuration file loading and parsing.

use std::fs;
use std::path::Path;

use serde::de::DeserializeOwned;
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

    #[error("{path}: {source}")]
    PathContext {
        path: String,
        #[source]
        source: Box<ConfigError>,
    },
}

impl ConfigError {
    /// Wrap this error with file path context.
    #[must_use]
    pub fn with_path(self, path: impl AsRef<Path>) -> Self {
        Self::PathContext {
            path: path.as_ref().display().to_string(),
            source: Box::new(self),
        }
    }
}

// ---------------------------------------------------------------------------
// File Format Detection and Loading
// ---------------------------------------------------------------------------

/// File format detected from extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    Yaml,
    Json,
}

impl FileFormat {
    /// Detect format from file path extension.
    ///
    /// Returns `None` for unknown extensions.
    pub fn from_path(path: &Path) -> Option<Self> {
        path.extension()
            .and_then(|e| e.to_str())
            .and_then(Self::from_extension)
    }

    /// Detect format from extension string.
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "yaml" | "yml" => Some(Self::Yaml),
            "json" => Some(Self::Json),
            _ => None,
        }
    }

    /// Parse content in this format.
    pub fn parse<T: DeserializeOwned>(self, content: &str) -> Result<T, ConfigError> {
        match self {
            Self::Yaml => Ok(serde_yaml::from_str(content)?),
            Self::Json => Ok(serde_json::from_str(content)?),
        }
    }
}

/// Load and parse a config file, auto-detecting format from extension.
///
/// If the extension is unrecognized, `default_format` is used.
/// Errors include file path context for easier debugging.
pub fn load_config_file<T: DeserializeOwned>(
    path: &Path,
    default_format: Option<FileFormat>,
) -> Result<T, ConfigError> {
    let content =
        fs::read_to_string(path).map_err(|e| ConfigError::ReadError(e).with_path(path))?;
    let format = FileFormat::from_path(path)
        .or(default_format)
        .ok_or_else(|| {
            ConfigError::UnknownFormat(
                path.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("unknown")
                    .to_string(),
            )
            .with_path(path)
        })?;
    format.parse(&content).map_err(|e| e.with_path(path))
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
        load_config_file(path.as_ref(), None)
    }

    /// Convert configuration to rules.
    pub fn to_rules(&self) -> Result<Vec<Rule>, ConfigError> {
        self.rules.iter().map(rule_from_config).collect()
    }
}

/// Convert a single rule config to a Rule.
fn rule_from_config(config: &RuleConfig) -> Result<Rule, ConfigError> {
    let mut builder = config.colors.iter().filter(|c| *c != "bold").fold(
        Rule::new(&config.regex)?,
        |b, color| match parse_semantic_color(color) {
            Some(s) => b.semantic(s),
            None => b.named(color),
        },
    );

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

    /// PTY configuration
    #[serde(default)]
    pub pty: PtyConfig,
}

/// PTY (pseudo-terminal) configuration.
#[derive(Debug, Deserialize)]
pub struct PtyConfig {
    /// Drain timeout in milliseconds after child exits.
    /// Higher values ensure all output is captured for fast-exiting processes.
    #[serde(default = "default_drain_timeout")]
    pub drain_timeout_ms: i32,

    /// Maximum drain retries before giving up.
    #[serde(default = "default_drain_retries")]
    pub drain_max_retries: u32,

    /// Additional commands to treat as interactive (requiring PTY).
    /// These are added to the built-in list (vim, less, htop, etc.).
    #[serde(default)]
    pub interactive_commands: Vec<String>,
}

impl Default for PtyConfig {
    fn default() -> Self {
        Self {
            drain_timeout_ms: default_drain_timeout(),
            drain_max_retries: default_drain_retries(),
            interactive_commands: Vec::new(),
        }
    }
}

fn default_drain_timeout() -> i32 {
    100
}

fn default_drain_retries() -> u32 {
    3
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
        let Some(config_path) = crate::program::loader::global_config_path() else {
            return Ok(None);
        };

        if !config_path.exists() {
            return Ok(None);
        }

        Ok(Some(load_config_file(
            &config_path,
            Some(FileFormat::Yaml),
        )?))
    }

    /// Load global configuration from a specific path.
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        load_config_file(path.as_ref(), Some(FileFormat::Yaml))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_yaml() {
        let yaml = r"
name: test
description: Test config
rules:
  - regex: '\berror\b'
    colors: [error, bold]
  - regex: '\d+'
    colors: [number]
";
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

    // -------------------------------------------------------------------------
    // PtyConfig Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_pty_config_default() {
        let config = PtyConfig::default();
        assert_eq!(config.drain_timeout_ms, 100);
        assert_eq!(config.drain_max_retries, 3);
        assert!(config.interactive_commands.is_empty());
    }

    #[test]
    fn test_pty_config_parse_yaml() {
        let yaml = r"
drain_timeout_ms: 200
drain_max_retries: 5
interactive_commands:
  - mycli
  - mytool
";
        let config: PtyConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.drain_timeout_ms, 200);
        assert_eq!(config.drain_max_retries, 5);
        assert_eq!(config.interactive_commands, vec!["mycli", "mytool"]);
    }

    #[test]
    fn test_pty_config_parse_partial() {
        // Only specify some fields, others should use defaults
        let yaml = r"
drain_timeout_ms: 150
";
        let config: PtyConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.drain_timeout_ms, 150);
        assert_eq!(config.drain_max_retries, 3); // default
        assert!(config.interactive_commands.is_empty()); // default
    }

    #[test]
    fn test_global_config_with_pty() {
        let yaml = r"
theme: dracula
pty:
  drain_timeout_ms: 250
  interactive_commands:
    - fzf
    - bat
";
        let config: GlobalConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.theme, Some("dracula".to_string()));
        assert_eq!(config.pty.drain_timeout_ms, 250);
        assert_eq!(config.pty.interactive_commands, vec!["fzf", "bat"]);
    }

    #[test]
    fn test_global_config_without_pty_uses_defaults() {
        let yaml = r"
theme: nord
stats: true
";
        let config: GlobalConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.theme, Some("nord".to_string()));
        assert!(config.stats);
        // PTY config should use defaults
        assert_eq!(config.pty.drain_timeout_ms, 100);
        assert_eq!(config.pty.drain_max_retries, 3);
        assert!(config.pty.interactive_commands.is_empty());
    }
}
