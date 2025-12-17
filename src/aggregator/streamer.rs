//! Log streaming and colorization.

use bollard::container::LogsOptions;
use bollard::Docker;
use futures::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use tokio::task::JoinHandle;

use crate::aggregator::html::ansi_to_html;
use crate::{programs, Colorizer, Theme};

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

/// Manages log streaming from multiple containers.
pub struct LogStreamer {
    docker: Docker,
    theme: Theme,
    colorizers: Arc<Mutex<HashMap<String, Colorizer>>>,
    tx: broadcast::Sender<ColorizedLogEntry>,
}

impl LogStreamer {
    /// Create a new log streamer.
    pub fn new(docker: Docker, theme: Theme) -> Self {
        let (tx, _) = broadcast::channel(1000);
        Self {
            docker,
            theme,
            colorizers: Arc::new(Mutex::new(HashMap::new())),
            tx,
        }
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
        let docker = self.docker.clone();
        let tx = self.tx.clone();
        let theme = self.theme.clone();
        let colorizers = self.colorizers.clone();
        let program_id = program.unwrap_or_else(|| "unknown".to_string());

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

            let options = LogsOptions::<String> {
                follow: true,
                stdout: true,
                stderr: true,
                timestamps: true,
                tail: "100".to_string(),
                ..Default::default()
            };

            let mut stream = docker.logs(&container_id, Some(options));

            while let Some(result) = stream.next().await {
                match result {
                    Ok(log_output) => {
                        let line = log_output.to_string();
                        let line = line.trim();
                        if line.is_empty() {
                            continue;
                        }

                        let colorized = colorizer.colorize(line);
                        let html = ansi_to_html(&colorized);

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
                        eprintln!(
                            "Log stream error for {}: {}",
                            container_name, e
                        );
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
    format!("{}.{:09}Z", secs, nanos)
}
