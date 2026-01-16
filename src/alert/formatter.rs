//! Webhook formatter trait and common utilities.

use super::condition::AlertSeverity;
use serde_json::Value;
use std::collections::HashMap;

/// Alert payload data to be formatted for webhooks.
#[derive(Debug, Clone)]
pub struct AlertPayload {
    /// Alert title/summary.
    pub title: String,
    /// Full log line or detailed message.
    pub message: String,
    /// Severity level.
    pub severity: AlertSeverity,
    /// Source program (e.g., "lodestar", "docker").
    pub program: Option<String>,
    /// ISO 8601 timestamp.
    pub timestamp: String,
    /// Additional context fields.
    pub fields: HashMap<String, String>,
}

impl AlertPayload {
    /// Create a new alert payload.
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            severity: AlertSeverity::default(),
            program: None,
            timestamp: current_timestamp(),
            fields: HashMap::new(),
        }
    }

    /// Set the severity level.
    #[must_use]
    pub fn with_severity(mut self, severity: AlertSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Set the source program.
    #[must_use]
    pub fn with_program(mut self, program: impl Into<String>) -> Self {
        self.program = Some(program.into());
        self
    }

    /// Add a context field.
    #[must_use]
    pub fn with_field(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields.insert(key.into(), value.into());
        self
    }
}

/// Webhook service types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebhookService {
    /// Discord webhook.
    Discord,
    /// Telegram Bot API.
    Telegram { chat_id: String },
    /// Generic JSON POST.
    Generic,
}

impl WebhookService {
    /// Auto-detect service type from URL.
    #[must_use]
    pub fn detect(url: &str) -> Self {
        if url.contains("discord.com/api/webhooks") || url.contains("discordapp.com/api/webhooks") {
            Self::Discord
        } else if url.contains("api.telegram.org/bot") {
            Self::Telegram {
                chat_id: String::new(),
            }
        } else {
            Self::Generic
        }
    }

    /// Set Telegram chat ID.
    #[must_use]
    pub fn with_chat_id(self, chat_id: impl Into<String>) -> Self {
        match self {
            Self::Telegram { .. } => Self::Telegram {
                chat_id: chat_id.into(),
            },
            other => other,
        }
    }
}

/// Trait for formatting alert payloads for specific webhook services.
pub trait WebhookFormatter: Send + Sync {
    /// Format the alert payload as JSON for the target service.
    fn format(&self, payload: &AlertPayload, service: &WebhookService) -> Value;

    /// Get the Content-Type header for this formatter.
    fn content_type(&self) -> &'static str {
        "application/json"
    }
}

/// Get the current timestamp in ISO 8601 format.
#[must_use]
pub fn current_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();

    // Simple ISO 8601 format without external dependencies
    let days_since_epoch = secs / 86400;
    let time_of_day = secs % 86400;

    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    let seconds = time_of_day % 60;

    // Calculate year, month, day from days since epoch (1970-01-01)
    let (year, month, day) = days_to_ymd(days_since_epoch);

    format!("{year:04}-{month:02}-{day:02}T{hours:02}:{minutes:02}:{seconds:02}Z")
}

/// Convert days since Unix epoch to (year, month, day).
fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    // Simplified calculation - good enough for alerting timestamps
    let mut remaining = days as i64;
    let mut year = 1970i64;

    // Cap at year 9999 to prevent overflow on extreme inputs
    const MAX_YEAR: i64 = 9999;

    loop {
        if year >= MAX_YEAR {
            break;
        }
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        year = year.saturating_add(1);
    }

    let leap = is_leap_year(year);
    let days_in_months = if leap {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1;
    for days_in_month in days_in_months {
        if remaining < days_in_month {
            break;
        }
        remaining -= days_in_month;
        month += 1;
    }

    (year as u64, month, remaining as u64 + 1)
}

fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Truncate a string to a maximum length, respecting UTF-8 boundaries.
#[must_use]
pub fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        s
    } else {
        let mut end = max;
        while !s.is_char_boundary(end) && end > 0 {
            end -= 1;
        }
        &s[..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_service_detect_discord() {
        let service =
            WebhookService::detect("https://discord.com/api/webhooks/123456789/abcdefghijklmnop");
        assert_eq!(service, WebhookService::Discord);

        let service = WebhookService::detect(
            "https://discordapp.com/api/webhooks/123456789/abcdefghijklmnop",
        );
        assert_eq!(service, WebhookService::Discord);
    }

    #[test]
    fn test_webhook_service_detect_telegram() {
        let service = WebhookService::detect("https://api.telegram.org/bot123:ABC/sendMessage");
        assert!(matches!(service, WebhookService::Telegram { .. }));
    }

    #[test]
    fn test_webhook_service_detect_generic() {
        let service = WebhookService::detect("https://hooks.example.com/webhook");
        assert_eq!(service, WebhookService::Generic);
    }

    #[test]
    fn test_alert_payload_builder() {
        let payload = AlertPayload::new("Test Alert", "Something happened")
            .with_severity(AlertSeverity::Error)
            .with_program("lodestar")
            .with_field("peers", "42");

        assert_eq!(payload.title, "Test Alert");
        assert_eq!(payload.message, "Something happened");
        assert_eq!(payload.severity, AlertSeverity::Error);
        assert_eq!(payload.program, Some("lodestar".to_string()));
        assert_eq!(payload.fields.get("peers"), Some(&"42".to_string()));
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello world", 5), "hello");
        // Test UTF-8 boundary handling
        assert_eq!(truncate("hello", 5), "hello");
    }

    #[test]
    fn test_current_timestamp_format() {
        let ts = current_timestamp();
        // Should match ISO 8601 format
        assert!(ts.contains('T'));
        assert!(ts.ends_with('Z'));
        assert_eq!(ts.len(), 20);
    }
}
