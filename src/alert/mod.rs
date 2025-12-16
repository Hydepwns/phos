//! Alert webhook system for phos.
//!
//! Provides webhook notifications for log events like errors, peer drops, and sync stalls.
//!
//! # Supported Services
//!
//! - **Discord**: Rich embeds with color-coded severity
//! - **Telegram**: `MarkdownV2` formatted messages
//! - **Generic**: Simple JSON POST
//!
//! # Usage
//!
//! ```rust,ignore
//! use phos::alert::{AlertManager, AlertCondition};
//!
//! let manager = AlertManager::new("https://discord.com/api/webhooks/xxx/yyy")
//!     .with_condition(AlertCondition::Error)
//!     .with_condition(AlertCondition::PeerDrop { threshold: 5 });
//!
//! // In processing loop:
//! manager.check_line(line, error_count, peer_count, slot, program);
//! ```

pub mod condition;
pub mod config;
pub mod discord;
pub mod evaluator;
pub mod formatter;
pub mod rate_limit;
pub mod sender;
pub mod telegram;

pub use condition::{AlertCondition, AlertSeverity, ParseConditionError};
pub use config::{AlertConfig, ConfigError};
pub use evaluator::ConditionEvaluator;
pub use formatter::{AlertPayload, WebhookFormatter, WebhookService};
pub use rate_limit::{RateLimitResult, RateLimiter};
pub use sender::{SendError, WebhookSender};

use discord::DiscordFormatter;
use std::sync::Arc;
use telegram::TelegramFormatter;
use tokio::runtime::Runtime;

/// Alert manager that coordinates condition evaluation, rate limiting, and webhook sending.
pub struct AlertManager {
    /// Webhook URL.
    url: String,
    /// Webhook service type.
    service: WebhookService,
    /// Alert conditions to evaluate.
    conditions: Vec<AlertCondition>,
    /// Condition evaluator.
    evaluator: ConditionEvaluator,
    /// Rate limiter.
    rate_limiter: RateLimiter,
    /// Webhook sender.
    sender: WebhookSender,
    /// Tokio runtime for async operations.
    runtime: Runtime,
    /// Program name for alerts.
    program: Option<String>,
}

impl AlertManager {
    /// Create a new alert manager for the given webhook URL.
    pub fn new(url: impl Into<String>) -> Self {
        let url = url.into();
        let service = WebhookService::detect(&url);
        let formatter: Arc<dyn WebhookFormatter> = match service {
            WebhookService::Telegram { .. } => Arc::new(TelegramFormatter),
            WebhookService::Discord | WebhookService::Generic => Arc::new(DiscordFormatter),
        };

        let sender = WebhookSender::new(&url, service.clone(), formatter);

        let runtime = Runtime::new().expect("failed to create tokio runtime");

        Self {
            url,
            service,
            conditions: Vec::new(),
            evaluator: ConditionEvaluator::new(),
            rate_limiter: RateLimiter::new(),
            sender,
            runtime,
            program: None,
        }
    }

    /// Set the Telegram chat ID (required for Telegram webhooks).
    #[must_use]
    pub fn with_chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.service = self.service.with_chat_id(chat_id);

        // Recreate sender with updated service
        let formatter: Arc<dyn WebhookFormatter> = Arc::new(TelegramFormatter);
        self.sender = WebhookSender::new(&self.url, self.service.clone(), formatter);

        self
    }

    /// Add an alert condition.
    #[must_use]
    pub fn with_condition(mut self, condition: AlertCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// Add multiple alert conditions.
    #[must_use]
    pub fn with_conditions(mut self, conditions: impl IntoIterator<Item = AlertCondition>) -> Self {
        self.conditions.extend(conditions);
        self
    }

    /// Set the program name for alerts.
    #[must_use]
    pub fn with_program(mut self, program: impl Into<String>) -> Self {
        self.program = Some(program.into());
        self
    }

    /// Set the alert cooldown.
    #[must_use]
    pub fn with_cooldown(mut self, cooldown: std::time::Duration) -> Self {
        self.rate_limiter = self
            .rate_limiter
            .with_global_cooldown(cooldown)
            .with_per_condition_cooldown(cooldown);
        self
    }

    /// Check a log line against all conditions and send alerts if triggered.
    ///
    /// This is the main entry point called from the processing loop.
    pub fn check_line(
        &mut self,
        line: &str,
        error_count: usize,
        peer_count: Option<usize>,
        slot: Option<u64>,
    ) {
        // Update evaluator state
        self.evaluator.update_state(peer_count, slot);

        // Collect alerts to send (to avoid borrow conflicts)
        let mut alerts_to_send: Vec<(AlertPayload, String)> = Vec::new();

        // Check each condition
        for condition in &self.conditions {
            // Check rate limit first
            let cond_type = condition.condition_type();
            if !self.rate_limiter.can_alert(cond_type).is_allowed() {
                continue;
            }

            // Evaluate condition
            if let Some(payload) = self.evaluator.evaluate(
                condition,
                line,
                error_count,
                peer_count,
                slot,
                self.program.as_deref(),
            ) {
                alerts_to_send.push((payload, cond_type.to_string()));
            }
        }

        // Send collected alerts
        for (payload, cond_type) in alerts_to_send {
            self.send_alert(&payload, &cond_type);
        }
    }

    /// Send an alert asynchronously.
    fn send_alert(&mut self, payload: &AlertPayload, condition_type: &str) {
        // Record in rate limiter
        self.rate_limiter.record_alert(condition_type);

        // Send webhook asynchronously
        let sender = &self.sender;
        let result = self.runtime.block_on(sender.send(payload));

        if let Err(e) = result {
            eprintln!("phos: alert failed: {e}");
        }
    }

    /// Get the webhook URL.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Get the number of configured conditions.
    pub fn condition_count(&self) -> usize {
        self.conditions.len()
    }

    /// Reset the alert manager state.
    pub fn reset(&mut self) {
        self.evaluator.reset();
        self.rate_limiter.reset();
    }
}

/// Builder for creating an `AlertManager` from CLI arguments.
pub struct AlertManagerBuilder {
    url: Option<String>,
    chat_id: Option<String>,
    conditions: Vec<AlertCondition>,
    program: Option<String>,
    cooldown: Option<std::time::Duration>,
}

impl AlertManagerBuilder {
    /// Create a new builder.
    #[must_use] pub fn new() -> Self {
        Self {
            url: None,
            chat_id: None,
            conditions: Vec::new(),
            program: None,
            cooldown: None,
        }
    }

    /// Set the webhook URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the Telegram chat ID.
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.chat_id = Some(chat_id.into());
        self
    }

    /// Add a condition from a CLI string.
    pub fn condition(mut self, condition_str: &str) -> Result<Self, ParseConditionError> {
        let condition: AlertCondition = condition_str.parse()?;
        self.conditions.push(condition);
        Ok(self)
    }

    /// Add multiple conditions from CLI strings.
    pub fn conditions(mut self, condition_strs: &[String]) -> Result<Self, ParseConditionError> {
        let parsed: Result<Vec<AlertCondition>, _> = condition_strs
            .iter()
            .map(|s| s.parse())
            .collect();
        self.conditions.extend(parsed?);
        Ok(self)
    }

    /// Set the program name.
    #[must_use]
    pub fn program(mut self, program: impl Into<String>) -> Self {
        self.program = Some(program.into());
        self
    }

    /// Set the cooldown in seconds.
    #[must_use] pub fn cooldown_secs(mut self, secs: u64) -> Self {
        self.cooldown = Some(std::time::Duration::from_secs(secs));
        self
    }

    /// Build the `AlertManager`.
    #[must_use] pub fn build(self) -> Option<AlertManager> {
        let url = self.url?;

        let mut manager = AlertManager::new(url);

        if let Some(chat_id) = self.chat_id {
            manager = manager.with_chat_id(chat_id);
        }

        if self.conditions.is_empty() {
            // Default to error condition if none specified
            manager = manager.with_condition(AlertCondition::Error);
        } else {
            manager = manager.with_conditions(self.conditions);
        }

        if let Some(program) = self.program {
            manager = manager.with_program(program);
        }

        if let Some(cooldown) = self.cooldown {
            manager = manager.with_cooldown(cooldown);
        }

        Some(manager)
    }
}

impl Default for AlertManagerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_manager_builder() {
        let manager = AlertManagerBuilder::new()
            .url("https://discord.com/api/webhooks/123/abc")
            .condition("error")
            .unwrap()
            .condition("peer-drop:5")
            .unwrap()
            .program("lodestar")
            .cooldown_secs(60)
            .build();

        assert!(manager.is_some());
        let manager = manager.unwrap();
        assert_eq!(manager.condition_count(), 2);
        assert_eq!(manager.url(), "https://discord.com/api/webhooks/123/abc");
    }

    #[test]
    fn test_alert_manager_builder_no_url() {
        let manager = AlertManagerBuilder::new().condition("error").unwrap().build();
        assert!(manager.is_none());
    }

    #[test]
    fn test_alert_manager_builder_telegram() {
        let manager = AlertManagerBuilder::new()
            .url("https://api.telegram.org/bot123:ABC/sendMessage")
            .chat_id("-1001234567890")
            .condition("error")
            .unwrap()
            .build();

        assert!(manager.is_some());
    }

    #[test]
    fn test_alert_manager_default_condition() {
        let manager = AlertManagerBuilder::new()
            .url("https://example.com/webhook")
            .build();

        assert!(manager.is_some());
        let manager = manager.unwrap();
        // Should default to error condition
        assert_eq!(manager.condition_count(), 1);
    }
}
