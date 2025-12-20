//! Log streaming and colorization.

use futures::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};
use tokio::task::JoinHandle;

use crate::aggregator::html::ansi_to_html;
use crate::aggregator::provider::ContainerProvider;
use crate::alert::AlertCondition;
use crate::{Colorizer, Theme, programs};

/// A colorized log entry ready for display.
#[derive(Debug, Clone)]
pub struct ColorizedLogEntry {
    /// Container ID.
    pub container_id: String,
    /// Container name.
    pub container_name: String,
    /// Detected program.
    pub program: String,
    /// Timestamp (ISO 8601).
    pub timestamp: String,
    /// Original log line.
    pub raw: String,
    /// ANSI-colorized line (for terminal).
    pub colorized: String,
    /// HTML-colorized line (for web).
    pub html: String,
}

impl ColorizedLogEntry {
    /// Serialize to JSON.
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "container_id": self.container_id,
            "container_name": self.container_name,
            "program": self.program,
            "timestamp": self.timestamp,
            "html": self.html
        })
    }
}

/// Alert configuration for the log streamer.
#[derive(Clone)]
pub struct AlertConfig {
    /// Webhook URL for alerts.
    pub webhook_url: String,
    /// Alert conditions to evaluate.
    pub conditions: Vec<AlertCondition>,
}

/// Manages log streaming from multiple containers.
pub struct LogStreamer {
    provider: Arc<dyn ContainerProvider>,
    theme: Theme,
    colorizers: Arc<Mutex<HashMap<String, Colorizer>>>,
    tx: broadcast::Sender<ColorizedLogEntry>,
    max_lines: usize,
    alert_config: Option<AlertConfig>,
}

impl LogStreamer {
    /// Create a new log streamer with default settings.
    pub fn new(provider: Arc<dyn ContainerProvider>, theme: Theme) -> Self {
        Self::with_config(provider, theme, 10000, None)
    }

    /// Create a log streamer with custom max_lines setting.
    pub fn with_max_lines(provider: Arc<dyn ContainerProvider>, theme: Theme, max_lines: usize) -> Self {
        Self::with_config(provider, theme, max_lines, None)
    }

    /// Create a log streamer with full configuration.
    pub fn with_config(
        provider: Arc<dyn ContainerProvider>,
        theme: Theme,
        max_lines: usize,
        alert_config: Option<AlertConfig>,
    ) -> Self {
        // Cap channel size at 10000 to prevent memory issues
        let channel_size = max_lines.min(10000).max(100);
        let (tx, _) = broadcast::channel(channel_size);
        Self {
            provider,
            theme,
            colorizers: Arc::new(Mutex::new(HashMap::new())),
            tx,
            max_lines,
            alert_config,
        }
    }

    /// Get the max lines setting.
    pub fn max_lines(&self) -> usize {
        self.max_lines
    }

    /// Get a broadcast receiver for log entries.
    pub fn subscribe(&self) -> broadcast::Receiver<ColorizedLogEntry> {
        self.tx.subscribe()
    }

    /// Get the broadcast sender (for cloning to handlers).
    pub fn sender(&self) -> broadcast::Sender<ColorizedLogEntry> {
        self.tx.clone()
    }

    /// Spawn a log streaming task for a container.
    pub fn spawn_container_stream(
        &self,
        container_id: String,
        container_name: String,
        program: Option<String>,
    ) -> JoinHandle<()> {
        let provider = self.provider.clone();
        let tx = self.tx.clone();
        let theme = self.theme.clone();
        let colorizers = self.colorizers.clone();
        let program_id = program.clone().unwrap_or_else(|| "unknown".to_string());
        let alert_config = self.alert_config.clone();

        tokio::spawn(async move {
            // Get or create colorizer for this program
            let mut colorizer = {
                let mut colorizers = colorizers.lock().await;
                if !colorizers.contains_key(&program_id) {
                    let registry = programs::default_registry();
                    let rules = registry
                        .get(&program_id)
                        .map(|p| p.rules())
                        .unwrap_or_default();
                    let c = Colorizer::new(rules).with_theme(theme.clone());
                    colorizers.insert(program_id.clone(), c);
                }
                colorizers.get(&program_id).unwrap().clone()
            };

            // Create alert manager if configured (wrapped in Arc for thread-safe sharing)
            use crate::alert::AlertManager;
            let alert_manager: Option<Arc<std::sync::Mutex<AlertManager>>> = alert_config.map(|config| {
                let manager = AlertManager::new(config.webhook_url)
                    .with_conditions(config.conditions)
                    .with_program(&program_id);
                Arc::new(std::sync::Mutex::new(manager))
            });

            // Get log stream from provider
            let mut stream = match provider.get_logs(&container_id, 100, true).await {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to get logs for {container_name}: {e}");
                    return;
                }
            };

            let mut error_count = 0usize;

            while let Some(result) = stream.next().await {
                match result {
                    Ok(log_line) => {
                        let line = log_line.content.trim();
                        if line.is_empty() {
                            continue;
                        }

                        let colorized = colorizer.colorize(line);
                        let html = ansi_to_html(&colorized);

                        // Check for error patterns and update count
                        let line_lower = line.to_lowercase();
                        if line_lower.contains("error") || line_lower.contains("err=") {
                            error_count = error_count.saturating_add(1);
                        }

                        // Check alerts in a blocking context (AlertManager uses block_on internally)
                        if let Some(ref mgr) = alert_manager {
                            let mgr = Arc::clone(mgr);
                            let line_owned = line.to_string();
                            let count = error_count;
                            // Spawn blocking to avoid blocking the async runtime
                            let _ = tokio::task::spawn_blocking(move || {
                                if let Ok(mut manager) = mgr.lock() {
                                    manager.check_line(&line_owned, count, None, None);
                                }
                            });
                        }

                        let entry = ColorizedLogEntry {
                            container_id: container_id.clone(),
                            container_name: container_name.clone(),
                            program: program_id.clone(),
                            timestamp: chrono_now(),
                            raw: line.to_string(),
                            colorized: colorized.to_string(),
                            html,
                        };

                        // Ignore send errors (no subscribers)
                        let _ = tx.send(entry);
                    }
                    Err(e) => {
                        eprintln!("Log stream error for {container_name}: {e}");
                        break;
                    }
                }
            }
        })
    }
}

/// Get current timestamp in ISO 8601 format.
fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    let nanos = duration.subsec_nanos();
    format!("{secs}.{nanos:09}Z")
}
