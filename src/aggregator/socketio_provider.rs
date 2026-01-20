//! DAppNode provider using Socket.IO to communicate with DAPPMANAGER.
//!
//! This provider connects to DAppNode's dappmanager via Socket.IO and uses
//! the RPC protocol to list packages and fetch logs.

use async_trait::async_trait;
use futures::stream;
use rust_socketio::{
    asynchronous::{Client, ClientBuilder},
    Payload,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot, Mutex};
use tokio::time::interval;

use super::{
    discovery::matches_glob,
    provider::{ContainerProvider, LogLine, LogStream, ProviderError},
    ContainerInfo,
};
use crate::programs;

/// Default dappmanager URL for DAppNode.
const DEFAULT_DAPPNODE_URL: &str = "http://my.dappnode:80";

/// Polling interval for log updates.
const LOG_POLL_INTERVAL: Duration = Duration::from_secs(2);

/// Package info returned by dappmanager's packagesGet.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct DappnodePackage {
    /// Package DNP name (e.g., "geth.dnp.dappnode.eth")
    dnp_name: Option<String>,
    /// Package version
    version: Option<String>,
    /// Package state
    state: Option<String>,
    /// Whether it's a core package
    is_core: Option<bool>,
    /// Containers in the package
    containers: Option<Vec<DappnodeContainer>>,
}

/// Container info within a package.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct DappnodeContainer {
    container_name: Option<String>,
    container_id: Option<String>,
    service_name: Option<String>,
    state: Option<String>,
    image: Option<String>,
}

/// DAppNode provider using Socket.IO.
pub struct SocketIOProvider {
    url: String,
    filter_pattern: Option<String>,
    /// Shared Socket.IO client.
    client: Arc<Mutex<Option<Client>>>,
}

impl SocketIOProvider {
    /// Create a new Socket.IO provider with default dappnode URL.
    pub fn new() -> Self {
        Self {
            url: DEFAULT_DAPPNODE_URL.to_string(),
            filter_pattern: None,
            client: Arc::new(Mutex::new(None)),
        }
    }

    /// Create with a custom URL.
    pub fn with_url(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            filter_pattern: None,
            client: Arc::new(Mutex::new(None)),
        }
    }

    /// Set a name filter pattern.
    pub fn with_filter(mut self, pattern: impl Into<String>) -> Self {
        self.filter_pattern = Some(pattern.into());
        self
    }

    /// Establish Socket.IO connection if not already connected.
    async fn ensure_connected(&self) -> Result<(), ProviderError> {
        let mut client_guard = self.client.lock().await;
        if client_guard.is_some() {
            return Ok(());
        }

        let client = ClientBuilder::new(&self.url)
            .on("error", |err, _| {
                Box::pin(async move {
                    eprintln!("Socket.IO error: {:?}", err);
                })
            })
            .connect()
            .await
            .map_err(|e| ProviderError::Connection(format!("Socket.IO connect failed: {}", e)))?;

        *client_guard = Some(client);
        Ok(())
    }

    /// Make an RPC call via Socket.IO.
    async fn call(&self, method: &str, args: Vec<Value>) -> Result<Value, ProviderError> {
        self.ensure_connected().await?;

        let client_guard = self.client.lock().await;
        let client = client_guard
            .as_ref()
            .ok_or_else(|| ProviderError::Connection("Not connected".into()))?;

        // Create the RPC payload
        let rpc_payload = json!({
            "method": method,
            "args": args
        });

        // Use a channel to receive the response
        let (tx, rx) = oneshot::channel::<Result<Value, String>>();
        let tx = Arc::new(Mutex::new(Some(tx)));

        // Set up callback handler
        let tx_clone = tx.clone();
        let callback = move |payload: Payload, _: Client| {
            let tx = tx_clone.clone();
            Box::pin(async move {
                if let Some(sender) = tx.lock().await.take() {
                    match payload {
                        Payload::Text(values) => {
                            if let Some(first) = values.first() {
                                let _ = sender.send(Ok(first.clone()));
                            } else {
                                let _ = sender.send(Err("Empty response".into()));
                            }
                        }
                        Payload::Binary(data) => {
                            let _ = sender
                                .send(Err(format!("Unexpected binary: {} bytes", data.len())));
                        }
                        _ => {
                            let _ = sender.send(Err("Unknown payload type".into()));
                        }
                    }
                }
            }) as std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
        };

        // Emit RPC call with callback
        client
            .emit_with_ack("rpc", rpc_payload, Duration::from_secs(30), callback)
            .await
            .map_err(|e| ProviderError::Rpc(format!("Failed to emit RPC: {}", e)))?;

        // Wait for response
        let result = tokio::time::timeout(Duration::from_secs(30), rx)
            .await
            .map_err(|_| ProviderError::Rpc("RPC timeout".into()))?
            .map_err(|_| ProviderError::Rpc("Response channel closed".into()))?
            .map_err(ProviderError::Rpc)?;

        Ok(result)
    }
}

#[async_trait]
impl ContainerProvider for SocketIOProvider {
    async fn list_containers(&self) -> Result<Vec<ContainerInfo>, ProviderError> {
        // Call packagesGet RPC
        let result = self.call("packagesGet", vec![]).await?;

        let packages: Vec<DappnodePackage> = serde_json::from_value(result)
            .map_err(|e| ProviderError::Rpc(format!("Failed to parse packages: {e}")))?;

        let registry = programs::default_registry();
        let pattern = self.filter_pattern.as_deref().unwrap_or("");

        // Flatten packages into containers using functional iterators
        let containers = packages
            .into_iter()
            .flat_map(|pkg| {
                pkg.containers
                    .map(|containers| {
                        // Multi-container package: map each container
                        containers
                            .into_iter()
                            .filter_map(|c| {
                                let name = c.container_name.unwrap_or_default();
                                matches_glob(&name, pattern).then(|| {
                                    ContainerInfo::new(
                                        &registry,
                                        c.container_id.unwrap_or_else(|| name.clone()),
                                        name,
                                        c.image.unwrap_or_default(),
                                        c.state.unwrap_or_else(|| "unknown".to_string()),
                                    )
                                })
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_else(|| {
                        // Single-container package
                        let name = pkg.dnp_name.unwrap_or_default();
                        matches_glob(&name, pattern)
                            .then(|| {
                                ContainerInfo::new(
                                    &registry,
                                    name.clone(),
                                    name,
                                    "",
                                    pkg.state.unwrap_or_else(|| "unknown".to_string()),
                                )
                            })
                            .into_iter()
                            .collect()
                    })
            })
            .collect();

        Ok(containers)
    }

    async fn get_logs(
        &self,
        container_id: &str,
        tail: usize,
        follow: bool,
    ) -> Result<LogStream, ProviderError> {
        let container_name = container_id.to_string();

        // Build options for packageLog
        let options = json!({
            "tail": if tail > 0 { tail } else { 100 },
            "timestamps": true
        });

        // Fetch initial logs
        let result = self
            .call(
                "packageLog",
                vec![json!({ "containerName": container_name, "options": options })],
            )
            .await?;

        // Parse the log result - it returns a string of logs
        let logs_str = result.as_str().unwrap_or("");

        let initial_lines: Vec<LogLine> = logs_str
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| LogLine {
                content: line.to_string(),
                is_stderr: false,
                timestamp: None,
            })
            .collect();

        if !follow {
            // Just return the initial logs
            let stream = stream::iter(initial_lines.into_iter().map(Ok));
            return Ok(Box::pin(stream));
        }

        // For following, we need to poll since dappmanager doesn't support streaming
        let (tx, rx) = mpsc::channel::<Result<LogLine, ProviderError>>(100);

        // Send initial logs
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            for line in initial_lines {
                if tx_clone.send(Ok(line)).await.is_err() {
                    break;
                }
            }
        });

        // Spawn polling task
        let url = self.url.clone();
        let filter = self.filter_pattern.clone();
        let container_name_clone = container_name.clone();

        tokio::spawn(async move {
            let provider = SocketIOProvider {
                url,
                filter_pattern: filter,
                client: Arc::new(Mutex::new(None)),
            };

            let mut poll_interval = interval(LOG_POLL_INTERVAL);
            let mut last_line_count = 0usize;

            loop {
                poll_interval.tick().await;

                let options = json!({
                    "tail": 50,
                    "timestamps": true
                });

                match provider
                    .call(
                        "packageLog",
                        vec![json!({ "containerName": &container_name_clone, "options": options })],
                    )
                    .await
                {
                    Ok(result) => {
                        let logs_str = result.as_str().unwrap_or("");
                        let lines: Vec<&str> = logs_str.lines().collect();
                        let new_count = lines.len();

                        // Only send lines we haven't seen
                        if new_count > last_line_count {
                            for line in lines.iter().skip(last_line_count) {
                                if line.is_empty() {
                                    continue;
                                }
                                let log_line = LogLine {
                                    content: line.to_string(),
                                    is_stderr: false,
                                    timestamp: None,
                                };
                                if tx.send(Ok(log_line)).await.is_err() {
                                    return; // Receiver dropped
                                }
                            }
                        }
                        last_line_count = new_count;
                    }
                    Err(e) => {
                        eprintln!("Log polling error: {}", e);
                        // Continue polling despite errors
                    }
                }
            }
        });

        // Convert receiver to stream
        let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        Ok(Box::pin(stream))
    }

    async fn verify_connection(&self) -> Result<(), ProviderError> {
        self.ensure_connected().await?;
        // Try listing packages as a ping
        let _ = self.call("packagesGet", vec![]).await?;
        Ok(())
    }

    fn name(&self) -> &'static str {
        "socketio"
    }
}

impl Default for SocketIOProvider {
    fn default() -> Self {
        Self::new()
    }
}
