//! YAML/JSON configuration loading for user-defined programs.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;

use crate::colors::{Color, ColorSpec};
use crate::rule::Rule;

use super::{Program, ProgramInfo};

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

/// User-defined program configuration from YAML/JSON.
#[derive(Debug, Deserialize)]
pub struct ProgramConfig {
    /// Program name (used as ID if id not specified)
    pub name: String,

    /// Optional explicit ID (defaults to name)
    #[serde(default)]
    pub id: Option<String>,

    /// Description
    #[serde(default)]
    pub description: String,

    /// Category for grouping (defaults to "custom")
    #[serde(default = "default_category")]
    pub category: String,

    /// Patterns for auto-detection
    #[serde(default)]
    pub detect: Vec<String>,

    /// Domain-specific semantic colors
    #[serde(default)]
    pub semantic_colors: HashMap<String, String>,

    /// Colorization rules
    #[serde(default)]
    pub rules: Vec<RuleConfig>,
}

fn default_category() -> String {
    "custom".to_string()
}

/// Rule configuration from file.
#[derive(Debug, Deserialize)]
pub struct RuleConfig {
    /// Regex pattern
    pub regex: String,

    /// Colors to apply (can be semantic, domain, named, or hex)
    #[serde(default)]
    pub colors: Vec<String>,

    /// Whether to apply bold
    #[serde(default)]
    pub bold: bool,
}

impl ProgramConfig {
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

    /// Get the program ID.
    pub fn program_id(&self) -> String {
        self.id.clone().unwrap_or_else(|| {
            format!("{}.{}", self.category, self.name.to_lowercase().replace(' ', "_"))
        })
    }

    /// Convert to a Program implementation.
    pub fn to_program(self) -> Result<Arc<dyn Program>, ConfigError> {
        let info = ProgramInfo::new(
            &self.program_id(),
            &self.name,
            &self.description,
            &self.category,
        );

        // Parse domain colors
        let domain_colors: HashMap<String, Color> = self
            .semantic_colors
            .iter()
            .map(|(name, value)| {
                let color = if value.starts_with('#') {
                    Color::hex(value)
                } else {
                    Color::named(value)
                };
                (name.clone(), color)
            })
            .collect();

        // Parse rules
        let mut rules = Vec::new();
        for rule_config in &self.rules {
            let mut builder = Rule::new(&rule_config.regex)?;

            for color_name in &rule_config.colors {
                let spec = ColorSpec::from_name(color_name);
                match spec {
                    ColorSpec::Semantic(s) => {
                        builder = builder.semantic(s);
                    }
                    ColorSpec::Domain(name) => {
                        // Look up in domain colors, fallback to named
                        if let Some(color) = domain_colors.get(&name) {
                            builder = builder.color(color.clone());
                        } else {
                            builder = builder.named(&name);
                        }
                    }
                    ColorSpec::Named(name) => {
                        builder = builder.named(&name);
                    }
                    ColorSpec::Hex(hex) => {
                        builder = builder.hex(&hex);
                    }
                }
            }

            if rule_config.bold || rule_config.colors.contains(&"bold".to_string()) {
                builder = builder.bold();
            }

            rules.push(builder.build());
        }

        // Convert detect patterns to owned strings
        let detect_patterns: Vec<String> = self.detect;

        Ok(Arc::new(ConfigProgram {
            info,
            rules,
            domain_colors,
            detect_patterns,
        }))
    }
}

/// A program loaded from configuration.
struct ConfigProgram {
    info: ProgramInfo,
    rules: Vec<Rule>,
    domain_colors: HashMap<String, Color>,
    detect_patterns: Vec<String>,
}

impl Program for ConfigProgram {
    fn info(&self) -> &ProgramInfo {
        &self.info
    }

    fn rules(&self) -> Vec<Rule> {
        self.rules.clone()
    }

    fn domain_colors(&self) -> HashMap<String, Color> {
        self.domain_colors.clone()
    }

    fn detect_patterns(&self) -> Vec<&'static str> {
        // We need to return static strings, so we leak the strings
        // This is acceptable since programs are typically loaded once
        self.detect_patterns
            .iter()
            .map(|s| {
                let leaked: &'static str = Box::leak(s.clone().into_boxed_str());
                leaked
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_yaml_config() {
        let yaml = "
name: myapp
description: My custom application
category: custom
detect:
  - myapp
  - docker.*myapp
semantic_colors:
  request_id: '#88AAFF'
  user_id: '#FFAA88'
rules:
  - regex: '\\[ERROR\\]'
    colors: [error]
    bold: true
  - regex: 'request_id=([a-f0-9-]+)'
    colors: [request_id]
";
        let config: ProgramConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.name, "myapp");
        assert_eq!(config.category, "custom");
        assert_eq!(config.detect.len(), 2);
        assert_eq!(config.semantic_colors.len(), 2);
        assert_eq!(config.rules.len(), 2);
    }

    #[test]
    fn test_program_id_generation() {
        let yaml = "
name: My App
category: custom
";
        let config: ProgramConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.program_id(), "custom.my_app");
    }
}
