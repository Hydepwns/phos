//! Alert configuration file parsing.

use super::condition::AlertCondition;
use super::formatter::WebhookService;
use serde::Deserialize;
use std::path::Path;
use thiserror::Error;

/// Error loading alert configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("failed to parse YAML: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("invalid condition: {0}")]
    InvalidCondition(String),

    #[error("webhook not found: {0}")]
    WebhookNotFound(String),

    #[error("telegram webhook missing chat_id")]
    MissingChatId,
}

/// Alert configuration file structure.
#[derive(Debug, Deserialize, Default)]
pub struct AlertConfig {
    /// Webhook definitions.
    #[serde(default)]
    pub webhooks: Vec<WebhookConfig>,

    /// Alert conditions.
    #[serde(default)]
    pub conditions: Vec<ConditionConfig>,

    /// Rate limiting settings.
    #[serde(default)]
    pub rate_limiting: RateLimitingConfig,
}

impl AlertConfig {
    /// Load configuration from a YAML file.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;
        let config: AlertConfig = serde_yaml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    /// Validate the configuration.
    fn validate(&self) -> Result<(), ConfigError> {
        // Check that all condition webhook references exist
        for cond in &self.conditions {
            for webhook_name in &cond.webhooks {
                if !self.webhooks.iter().any(|w| &w.name == webhook_name) {
                    return Err(ConfigError::WebhookNotFound(webhook_name.clone()));
                }
            }
        }

        // Check telegram webhooks have chat_id
        for webhook in &self.webhooks {
            if webhook.webhook_type == Some("telegram".to_string()) && webhook.chat_id.is_none() {
                return Err(ConfigError::MissingChatId);
            }
        }

        Ok(())
    }

    /// Get a webhook by name.
    #[must_use]
    pub fn get_webhook(&self, name: &str) -> Option<&WebhookConfig> {
        self.webhooks.iter().find(|w| w.name == name)
    }
}

/// Webhook configuration.
#[derive(Debug, Deserialize)]
pub struct WebhookConfig {
    /// Webhook name for referencing.
    pub name: String,

    /// Webhook URL.
    pub url: String,

    /// Webhook type (discord, telegram, generic). Auto-detected if not specified.
    #[serde(rename = "type")]
    pub webhook_type: Option<String>,

    /// Telegram chat ID (required for telegram type).
    pub chat_id: Option<String>,
}

impl WebhookConfig {
    /// Get the webhook service type.
    #[must_use]
    pub fn service(&self) -> WebhookService {
        match self.webhook_type.as_deref() {
            Some("discord") => WebhookService::Discord,
            Some("telegram") => WebhookService::Telegram {
                chat_id: self.chat_id.clone().unwrap_or_default(),
            },
            Some("generic") | None => {
                let detected = WebhookService::detect(&self.url);
                match detected {
                    WebhookService::Telegram { .. } => WebhookService::Telegram {
                        chat_id: self.chat_id.clone().unwrap_or_default(),
                    },
                    other => other,
                }
            }
            _ => WebhookService::Generic,
        }
    }
}

/// Condition configuration.
#[derive(Debug, Deserialize)]
pub struct ConditionConfig {
    /// Condition type.
    #[serde(rename = "type")]
    pub condition_type: String,

    /// Threshold count (for `error_threshold`).
    pub count: Option<usize>,

    /// Threshold value (for `peer_drop`).
    pub threshold: Option<usize>,

    /// Pattern (for pattern type).
    pub pattern: Option<String>,

    /// Webhooks to notify.
    #[serde(default)]
    pub webhooks: Vec<String>,
}

impl ConditionConfig {
    /// Parse into an `AlertCondition`.
    pub fn to_condition(&self) -> Result<AlertCondition, ConfigError> {
        match self.condition_type.as_str() {
            "error" => Ok(AlertCondition::Error),
            "error_threshold" => {
                let count = self.count.ok_or_else(|| {
                    ConfigError::InvalidCondition("error_threshold requires count".to_string())
                })?;
                Ok(AlertCondition::ErrorThreshold { count })
            }
            "peer_drop" => {
                let threshold = self.threshold.ok_or_else(|| {
                    ConfigError::InvalidCondition("peer_drop requires threshold".to_string())
                })?;
                Ok(AlertCondition::PeerDrop { threshold })
            }
            "sync_stall" => Ok(AlertCondition::SyncStall),
            "pattern" => {
                let pattern = self.pattern.as_ref().ok_or_else(|| {
                    ConfigError::InvalidCondition("pattern requires pattern field".to_string())
                })?;
                let regex = regex::Regex::new(pattern)
                    .map_err(|e| ConfigError::InvalidCondition(format!("invalid regex: {e}")))?;
                Ok(AlertCondition::Pattern { regex })
            }
            other => Err(ConfigError::InvalidCondition(format!(
                "unknown type: {other}"
            ))),
        }
    }
}

/// Rate limiting configuration.
#[derive(Debug, Deserialize)]
pub struct RateLimitingConfig {
    /// Global cooldown between alerts.
    #[serde(default = "default_global_cooldown")]
    pub global_cooldown: String,

    /// Per-condition cooldown.
    #[serde(default = "default_per_condition")]
    pub per_condition: String,

    /// Maximum alerts per hour.
    #[serde(default = "default_max_per_hour")]
    pub max_per_hour: usize,
}

impl Default for RateLimitingConfig {
    fn default() -> Self {
        Self {
            global_cooldown: "30s".to_string(),
            per_condition: "60s".to_string(),
            max_per_hour: 50,
        }
    }
}

fn default_global_cooldown() -> String {
    "30s".to_string()
}

fn default_per_condition() -> String {
    "60s".to_string()
}

fn default_max_per_hour() -> usize {
    50
}

/// Parse a duration string like "30s", "5m", "1h".
#[must_use]
pub fn parse_duration(s: &str) -> Option<std::time::Duration> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let (num_str, unit) = if let Some(stripped) = s.strip_suffix('s') {
        (stripped, 1u64)
    } else if let Some(stripped) = s.strip_suffix('m') {
        (stripped, 60u64)
    } else if let Some(stripped) = s.strip_suffix('h') {
        (stripped, 3600u64)
    } else {
        // Assume seconds if no unit
        (s, 1u64)
    };

    num_str
        .parse::<u64>()
        .ok()
        .and_then(|n| n.checked_mul(unit))
        .map(std::time::Duration::from_secs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(
            parse_duration("30s"),
            Some(std::time::Duration::from_secs(30))
        );
        assert_eq!(
            parse_duration("5m"),
            Some(std::time::Duration::from_secs(300))
        );
        assert_eq!(
            parse_duration("1h"),
            Some(std::time::Duration::from_secs(3600))
        );
        assert_eq!(
            parse_duration("60"),
            Some(std::time::Duration::from_secs(60))
        );
        assert_eq!(parse_duration(""), None);
        assert_eq!(parse_duration("abc"), None);
    }

    #[test]
    fn test_webhook_service_detection() {
        let webhook = WebhookConfig {
            name: "discord".to_string(),
            url: "https://discord.com/api/webhooks/123/abc".to_string(),
            webhook_type: None,
            chat_id: None,
        };
        assert_eq!(webhook.service(), WebhookService::Discord);

        let webhook = WebhookConfig {
            name: "telegram".to_string(),
            url: "https://api.telegram.org/bot123:ABC/sendMessage".to_string(),
            webhook_type: None,
            chat_id: Some("-123456".to_string()),
        };
        match webhook.service() {
            WebhookService::Telegram { chat_id } => assert_eq!(chat_id, "-123456"),
            _ => panic!("expected Telegram"),
        }
    }

    #[test]
    fn test_condition_config_parse() {
        let config = ConditionConfig {
            condition_type: "error".to_string(),
            count: None,
            threshold: None,
            pattern: None,
            webhooks: vec![],
        };
        assert!(matches!(
            config.to_condition().unwrap(),
            AlertCondition::Error
        ));

        let config = ConditionConfig {
            condition_type: "error_threshold".to_string(),
            count: Some(10),
            threshold: None,
            pattern: None,
            webhooks: vec![],
        };
        match config.to_condition().unwrap() {
            AlertCondition::ErrorThreshold { count } => assert_eq!(count, 10),
            _ => panic!("expected ErrorThreshold"),
        }

        let config = ConditionConfig {
            condition_type: "peer_drop".to_string(),
            count: None,
            threshold: Some(5),
            pattern: None,
            webhooks: vec![],
        };
        match config.to_condition().unwrap() {
            AlertCondition::PeerDrop { threshold } => assert_eq!(threshold, 5),
            _ => panic!("expected PeerDrop"),
        }
    }

    #[test]
    fn test_default_rate_limiting() {
        let config = RateLimitingConfig::default();
        assert_eq!(config.global_cooldown, "30s");
        assert_eq!(config.per_condition, "60s");
        assert_eq!(config.max_per_hour, 50);
    }
}
