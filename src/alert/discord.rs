//! Discord webhook formatter.

use super::formatter::{truncate, AlertPayload, WebhookFormatter, WebhookService};
use serde_json::{json, Value};

/// Discord webhook formatter with rich embeds.
pub struct DiscordFormatter;

impl WebhookFormatter for DiscordFormatter {
    fn format(&self, payload: &AlertPayload, _service: &WebhookService) -> Value {
        // Build embed fields from payload fields
        let fields: Vec<Value> = payload
            .fields
            .iter()
            .take(25) // Discord limit
            .map(|(k, v)| {
                json!({
                    "name": truncate(k, 256),
                    "value": truncate(v, 1024),
                    "inline": true
                })
            })
            .collect();

        // Build the embed
        let mut embed = json!({
            "title": format!("{} {}", payload.severity.tag(), truncate(&payload.title, 250)),
            "description": format!("```\n{}\n```", truncate(&payload.message, 4000)),
            "color": payload.severity.discord_color(),
            "timestamp": payload.timestamp,
        });

        if !fields.is_empty() {
            embed["fields"] = json!(fields);
        }

        if let Some(ref program) = payload.program {
            embed["footer"] = json!({ "text": format!("Source: {}", program) });
        }

        json!({
            "embeds": [embed]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alert::condition::AlertSeverity;

    #[test]
    fn test_discord_format_basic() {
        let formatter = DiscordFormatter;
        let payload = AlertPayload::new("Connection Error", "Failed to connect to peer")
            .with_severity(AlertSeverity::Error)
            .with_program("lodestar");

        let json = formatter.format(&payload, &WebhookService::Discord);

        // Verify structure
        assert!(json["embeds"].is_array());
        assert_eq!(json["embeds"].as_array().unwrap().len(), 1);

        let embed = &json["embeds"][0];
        assert!(embed["title"].as_str().unwrap().contains("[ERR]"));
        assert!(embed["description"]
            .as_str()
            .unwrap()
            .contains("Failed to connect"));
        assert_eq!(embed["color"], 0xFF5500);
        assert!(embed["footer"]["text"]
            .as_str()
            .unwrap()
            .contains("lodestar"));
    }

    #[test]
    fn test_discord_format_with_fields() {
        let formatter = DiscordFormatter;
        let payload = AlertPayload::new("Peer Drop", "Peer count dropped")
            .with_severity(AlertSeverity::Warning)
            .with_field("previous", "50")
            .with_field("current", "3");

        let json = formatter.format(&payload, &WebhookService::Discord);

        let embed = &json["embeds"][0];
        let fields = embed["fields"].as_array().unwrap();
        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn test_discord_format_severity_colors() {
        let formatter = DiscordFormatter;

        let critical = AlertPayload::new("Critical", "msg").with_severity(AlertSeverity::Critical);
        let json = formatter.format(&critical, &WebhookService::Discord);
        assert_eq!(json["embeds"][0]["color"], 0xFF0000);

        let error = AlertPayload::new("Error", "msg").with_severity(AlertSeverity::Error);
        let json = formatter.format(&error, &WebhookService::Discord);
        assert_eq!(json["embeds"][0]["color"], 0xFF5500);

        let warning = AlertPayload::new("Warning", "msg").with_severity(AlertSeverity::Warning);
        let json = formatter.format(&warning, &WebhookService::Discord);
        assert_eq!(json["embeds"][0]["color"], 0xFFAA00);

        let info = AlertPayload::new("Info", "msg").with_severity(AlertSeverity::Info);
        let json = formatter.format(&info, &WebhookService::Discord);
        assert_eq!(json["embeds"][0]["color"], 0x55AAFF);
    }

    #[test]
    fn test_discord_truncation() {
        let formatter = DiscordFormatter;
        let long_message = "x".repeat(5000);
        let payload = AlertPayload::new("Test", &long_message);

        let json = formatter.format(&payload, &WebhookService::Discord);

        // Description should be truncated
        let desc = json["embeds"][0]["description"].as_str().unwrap();
        assert!(desc.len() < 4100); // 4000 + code block markers
    }
}
