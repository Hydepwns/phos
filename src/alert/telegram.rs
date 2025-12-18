//! Telegram Bot API webhook formatter.

#![allow(clippy::format_push_string)]

use super::formatter::{AlertPayload, WebhookFormatter, WebhookService, truncate};
use serde_json::{Value, json};

/// Telegram Bot API formatter with `MarkdownV2`.
pub struct TelegramFormatter;

impl WebhookFormatter for TelegramFormatter {
    fn format(&self, payload: &AlertPayload, service: &WebhookService) -> Value {
        let chat_id = match service {
            WebhookService::Telegram { chat_id } => chat_id.clone(),
            _ => String::new(),
        };

        // Build message text with markdown formatting
        let mut text = format!(
            "{} *{}*\n\n```\n{}\n```",
            payload.severity.tag(),
            escape_markdown(&payload.title),
            truncate(&payload.message, 3800)
        );

        // Add fields
        if !payload.fields.is_empty() {
            text.push_str("\n\n");
            for (key, value) in &payload.fields {
                text.push_str(&format!(
                    "*{}*: {}\n",
                    escape_markdown(key),
                    escape_markdown(value)
                ));
            }
        }

        // Add source
        if let Some(ref program) = payload.program {
            text.push_str(&format!("\n_Source: {}_", escape_markdown(program)));
        }

        json!({
            "chat_id": chat_id,
            "text": text,
            "parse_mode": "MarkdownV2"
        })
    }
}

/// Escape special characters for Telegram `MarkdownV2`.
fn escape_markdown(s: &str) -> String {
    let special_chars = [
        '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!',
    ];

    let mut result = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        if special_chars.contains(&c) {
            result.push('\\');
        }
        result.push(c);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alert::condition::AlertSeverity;

    #[test]
    fn test_telegram_format_basic() {
        let formatter = TelegramFormatter;
        let payload = AlertPayload::new("Connection Error", "Failed to connect to peer")
            .with_severity(AlertSeverity::Error)
            .with_program("lodestar");

        let service = WebhookService::Telegram {
            chat_id: "-1001234567890".to_string(),
        };
        let json = formatter.format(&payload, &service);

        assert_eq!(json["chat_id"], "-1001234567890");
        assert_eq!(json["parse_mode"], "MarkdownV2");

        let text = json["text"].as_str().unwrap();
        assert!(text.contains("[ERR]"));
        assert!(text.contains("Connection Error"));
        assert!(text.contains("lodestar"));
    }

    #[test]
    fn test_telegram_format_with_fields() {
        let formatter = TelegramFormatter;
        let payload = AlertPayload::new("Peer Drop", "Peer count dropped")
            .with_field("previous", "50")
            .with_field("current", "3");

        let service = WebhookService::Telegram {
            chat_id: "123".to_string(),
        };
        let json = formatter.format(&payload, &service);

        let text = json["text"].as_str().unwrap();
        assert!(text.contains("previous"));
        assert!(text.contains("current"));
    }

    #[test]
    fn test_escape_markdown() {
        assert_eq!(escape_markdown("hello"), "hello");
        assert_eq!(escape_markdown("hello_world"), "hello\\_world");
        assert_eq!(escape_markdown("*bold*"), "\\*bold\\*");
        assert_eq!(escape_markdown("test.com"), "test\\.com");
    }

    #[test]
    fn test_telegram_severity_tags() {
        let formatter = TelegramFormatter;
        let service = WebhookService::Telegram {
            chat_id: "123".to_string(),
        };

        let critical = AlertPayload::new("Test", "msg").with_severity(AlertSeverity::Critical);
        let json = formatter.format(&critical, &service);
        assert!(json["text"].as_str().unwrap().contains("[!!!]"));

        let error = AlertPayload::new("Test", "msg").with_severity(AlertSeverity::Error);
        let json = formatter.format(&error, &service);
        assert!(json["text"].as_str().unwrap().contains("[ERR]"));

        let warning = AlertPayload::new("Test", "msg").with_severity(AlertSeverity::Warning);
        let json = formatter.format(&warning, &service);
        assert!(json["text"].as_str().unwrap().contains("[WRN]"));

        let info = AlertPayload::new("Test", "msg").with_severity(AlertSeverity::Info);
        let json = formatter.format(&info, &service);
        assert!(json["text"].as_str().unwrap().contains("[INF]"));
    }
}
