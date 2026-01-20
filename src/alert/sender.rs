//! Async HTTP webhook sender.

use super::formatter::{AlertPayload, WebhookFormatter, WebhookService};
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::mpsc;

/// Error sending a webhook.
#[derive(Debug, Error)]
pub enum SendError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("webhook returned error status: {0}")]
    Status(reqwest::StatusCode),

    #[error("channel send error")]
    ChannelClosed,
}

/// Async webhook sender that processes alerts in the background.
pub struct WebhookSender {
    client: Client,
    url: String,
    service: WebhookService,
    formatter: Arc<dyn WebhookFormatter>,
    timeout: Duration,
}

/// Build an HTTP client with the given timeout, logging errors.
fn build_client(timeout: Duration) -> Client {
    Client::builder()
        .timeout(timeout)
        .build()
        .unwrap_or_else(|e| {
            eprintln!("phos: warning: failed to build HTTP client: {e}, using default");
            Client::new()
        })
}

impl WebhookSender {
    /// Create a new webhook sender.
    pub fn new(
        url: impl Into<String>,
        service: WebhookService,
        formatter: Arc<dyn WebhookFormatter>,
    ) -> Self {
        let timeout = Duration::from_secs(5);
        Self {
            client: build_client(timeout),
            url: url.into(),
            service,
            formatter,
            timeout,
        }
    }

    /// Set the request timeout.
    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self.client = build_client(timeout);
        self
    }

    /// Send an alert synchronously (blocking).
    pub async fn send(&self, payload: &AlertPayload) -> Result<(), SendError> {
        let body = self.formatter.format(payload, &self.service);

        let response = self
            .client
            .post(&self.url)
            .header("Content-Type", self.formatter.content_type())
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(SendError::Status(response.status()))
        }
    }

    /// Get the webhook URL.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Get the webhook service type.
    #[must_use]
    pub fn service(&self) -> &WebhookService {
        &self.service
    }
}

/// Message sent to the background sender task.
#[derive(Debug)]
pub struct AlertMessage {
    pub payload: AlertPayload,
    pub url: String,
}

/// Background alert sender that processes alerts asynchronously.
pub struct BackgroundSender {
    tx: mpsc::UnboundedSender<AlertMessage>,
}

impl BackgroundSender {
    /// Create a new background sender and spawn the processing task.
    ///
    /// Returns the sender handle. The background task will run until all
    /// sender handles are dropped.
    pub fn spawn(service: WebhookService, formatter: Arc<dyn WebhookFormatter>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            process_alerts(rx, service, formatter).await;
        });

        Self { tx }
    }

    /// Queue an alert for sending.
    pub fn send(&self, url: String, payload: AlertPayload) -> Result<(), SendError> {
        self.tx
            .send(AlertMessage { payload, url })
            .map_err(|_| SendError::ChannelClosed)
    }
}

async fn process_alerts(
    mut rx: mpsc::UnboundedReceiver<AlertMessage>,
    service: WebhookService,
    formatter: Arc<dyn WebhookFormatter>,
) {
    let client = build_client(Duration::from_secs(5));

    while let Some(msg) = rx.recv().await {
        let body = formatter.format(&msg.payload, &service);

        let result = client
            .post(&msg.url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await;

        match result {
            Ok(response) => {
                if !response.status().is_success() {
                    eprintln!(
                        "phos: webhook returned status {}: {}",
                        response.status(),
                        msg.url
                    );
                }
            }
            Err(e) => {
                eprintln!("phos: webhook error: {e}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alert::discord::DiscordFormatter;

    #[test]
    fn test_webhook_sender_creation() {
        let formatter = Arc::new(DiscordFormatter);
        let sender = WebhookSender::new(
            "https://discord.com/api/webhooks/123/abc",
            WebhookService::Discord,
            formatter,
        );

        assert_eq!(sender.url(), "https://discord.com/api/webhooks/123/abc");
        assert_eq!(sender.service(), &WebhookService::Discord);
    }

    #[test]
    fn test_webhook_sender_timeout() {
        let formatter = Arc::new(DiscordFormatter);
        let sender = WebhookSender::new("https://example.com", WebhookService::Generic, formatter)
            .with_timeout(Duration::from_secs(10));

        assert_eq!(sender.timeout, Duration::from_secs(10));
    }
}
