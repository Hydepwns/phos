//! DAppNode provider using WAMP RPC to communicate with DAPPMANAGER.
//!
//! This provider connects to DAppNode's WAMP router and calls DAPPMANAGER
//! procedures to list packages and fetch logs.

use async_trait::async_trait;
use futures::{SinkExt, StreamExt, stream};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, mpsc};
use tokio::time::interval;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::programs;
use super::{ContainerInfo, provider::{ContainerProvider, LogLine, LogStream, ProviderError}};

/// Default WAMP router URL for DAppNode.
const DEFAULT_WAMP_URL: &str = "ws://my.wamp.dnp.dappnode.eth:8080/ws";

/// WAMP realm for DAppNode admin operations.
const WAMP_REALM: &str = "dappnode_admin";

/// DAPPMANAGER procedure domain suffix.
const DAPPMANAGER_DOMAIN: &str = ".dappmanager.dnp.dappnode.eth";

/// Polling interval for log updates (since DAPPMANAGER doesn't support streaming).
const LOG_POLL_INTERVAL: Duration = Duration::from_secs(2);

/// WAMP message types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum WampMessageType {
    Hello = 1,
    Welcome = 2,
    Call = 48,
    Result = 50,
    Error = 8,
}

/// Package info returned by DAPPMANAGER's listPackages.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DappnodePackage {
    /// Package DNP name (e.g., "geth.dnp.dappnode.eth")
    dnp_name: Option<String>,
    /// Container name
    container_name: Option<String>,
    /// Package name
    name: Option<String>,
    /// Package state
    state: Option<String>,
    /// Container ID
    container_id: Option<String>,
    /// Image name
    image: Option<String>,
    /// Services in the package
    containers: Option<Vec<DappnodeContainer>>,
}

/// Container/service info within a package.
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

/// DAppNode provider using WAMP.
pub struct DappnodeProvider {
    wamp_url: String,
    filter_pattern: Option<String>,
    /// Shared WAMP connection state.
    connection: Arc<Mutex<Option<WampConnection>>>,
}

/// WAMP connection state.
#[allow(dead_code)]
struct WampConnection {
    sender: futures::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        Message,
    >,
    receiver: futures::stream::SplitStream<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    >,
    session_id: u64,
    request_id: u64,
}

impl DappnodeProvider {
    /// Create a new DAppNode provider with default WAMP URL.
    pub fn new() -> Self {
        Self {
            wamp_url: DEFAULT_WAMP_URL.to_string(),
            filter_pattern: None,
            connection: Arc::new(Mutex::new(None)),
        }
    }

    /// Create with a custom WAMP URL.
    pub fn with_url(url: impl Into<String>) -> Self {
        Self {
            wamp_url: url.into(),
            filter_pattern: None,
            connection: Arc::new(Mutex::new(None)),
        }
    }

    /// Set a name filter pattern.
    pub fn with_filter(mut self, pattern: impl Into<String>) -> Self {
        self.filter_pattern = Some(pattern.into());
        self
    }

    /// Establish WAMP connection if not already connected.
    async fn ensure_connected(&self) -> Result<(), ProviderError> {
        let mut conn_guard = self.connection.lock().await;
        if conn_guard.is_some() {
            return Ok(());
        }

        // Connect to WAMP router
        let (ws_stream, _) = connect_async(&self.wamp_url)
            .await
            .map_err(|e| ProviderError::Connection(format!("WebSocket connect failed: {}", e)))?;

        let (mut sender, mut receiver) = ws_stream.split();

        // Send HELLO message
        let hello = json!([
            WampMessageType::Hello as u8,
            WAMP_REALM,
            {
                "roles": {
                    "caller": {}
                }
            }
        ]);
        sender
            .send(Message::Text(hello.to_string().into()))
            .await
            .map_err(|e| ProviderError::Connection(format!("Failed to send HELLO: {}", e)))?;

        // Wait for WELCOME
        let welcome = receiver
            .next()
            .await
            .ok_or_else(|| ProviderError::Connection("Connection closed before WELCOME".into()))?
            .map_err(|e| ProviderError::Connection(format!("WebSocket error: {}", e)))?;

        let session_id = match welcome {
            Message::Text(text) => {
                let msg: Value = serde_json::from_str(&text)
                    .map_err(|e| ProviderError::Connection(format!("Invalid WELCOME: {}", e)))?;

                let msg_type = msg.get(0).and_then(|v| v.as_u64()).unwrap_or(0);
                if msg_type != WampMessageType::Welcome as u64 {
                    return Err(ProviderError::Connection(format!(
                        "Expected WELCOME, got message type {}",
                        msg_type
                    )));
                }

                msg.get(1).and_then(|v| v.as_u64()).unwrap_or(0)
            }
            _ => return Err(ProviderError::Connection("Invalid WELCOME message".into())),
        };

        *conn_guard = Some(WampConnection {
            sender,
            receiver,
            session_id,
            request_id: 0,
        });

        Ok(())
    }

    /// Make a WAMP RPC call.
    async fn call(&self, procedure: &str, args: Vec<Value>) -> Result<Value, ProviderError> {
        self.ensure_connected().await?;

        let mut conn_guard = self.connection.lock().await;
        let conn = conn_guard
            .as_mut()
            .ok_or_else(|| ProviderError::Connection("Not connected".into()))?;

        conn.request_id += 1;
        let request_id = conn.request_id;

        // Build full procedure name
        let full_procedure = format!("{}{}", procedure, DAPPMANAGER_DOMAIN);

        // Send CALL message: [CALL, Request|id, Options|dict, Procedure|uri, Arguments|list]
        let call_msg = json!([
            WampMessageType::Call as u8,
            request_id,
            {},
            full_procedure,
            args
        ]);

        conn.sender
            .send(Message::Text(call_msg.to_string().into()))
            .await
            .map_err(|e| ProviderError::Rpc(format!("Failed to send CALL: {}", e)))?;

        // Wait for RESULT or ERROR
        loop {
            let msg = conn
                .receiver
                .next()
                .await
                .ok_or_else(|| ProviderError::Rpc("Connection closed".into()))?
                .map_err(|e| ProviderError::Rpc(format!("WebSocket error: {}", e)))?;

            match msg {
                Message::Text(text) => {
                    let response: Value = serde_json::from_str(&text)
                        .map_err(|e| ProviderError::Rpc(format!("Invalid response: {}", e)))?;

                    let msg_type = response.get(0).and_then(|v| v.as_u64()).unwrap_or(0);
                    let resp_request_id = response.get(1).and_then(|v| v.as_u64()).unwrap_or(0);

                    // Skip messages for other requests
                    if resp_request_id != request_id {
                        continue;
                    }

                    if msg_type == WampMessageType::Result as u64 {
                        // [RESULT, CALL.Request|id, Details|dict, YIELD.Arguments|list]
                        let result = response.get(3).cloned().unwrap_or(Value::Null);
                        return Ok(result);
                    } else if msg_type == WampMessageType::Error as u64 {
                        // [ERROR, CALL, Request|id, Details|dict, Error|uri, Arguments|list]
                        let error_uri = response
                            .get(4)
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown");
                        return Err(ProviderError::Rpc(format!("RPC error: {}", error_uri)));
                    }
                }
                Message::Ping(data) => {
                    conn.sender
                        .send(Message::Pong(data))
                        .await
                        .ok();
                }
                _ => {}
            }
        }
    }

    /// Check if a container name matches a glob-like pattern.
    fn matches_pattern(name: &str, pattern: &str) -> bool {
        if pattern.is_empty() {
            return true;
        }

        if let Some(suffix) = pattern.strip_prefix('*') {
            name.ends_with(suffix)
        } else if let Some(prefix) = pattern.strip_suffix('*') {
            name.starts_with(prefix)
        } else {
            name.contains(pattern)
        }
    }

    /// Detect phos program from container name or image.
    fn detect_program(
        registry: &crate::ProgramRegistry,
        name: &str,
        image: &str,
    ) -> Option<String> {
        if let Some(program) = registry.detect(name) {
            return Some(program.info().id.to_string());
        }

        if let Some(program) = registry.detect(image) {
            return Some(program.info().id.to_string());
        }

        let base_name = name
            .split('.')
            .next()
            .unwrap_or(name)
            .replace("-beacon", "")
            .replace("-validator", "")
            .replace("-bn", "")
            .replace("-vc", "");

        registry.detect(&base_name).map(|p| p.info().id.to_string())
    }
}

#[async_trait]
impl ContainerProvider for DappnodeProvider {
    async fn list_containers(&self) -> Result<Vec<ContainerInfo>, ProviderError> {
        // Call listPackages RPC
        let result = self.call("listPackages", vec![]).await?;

        let packages: Vec<DappnodePackage> = serde_json::from_value(result)
            .map_err(|e| ProviderError::Rpc(format!("Failed to parse packages: {}", e)))?;

        let registry = programs::default_registry();
        let mut containers = Vec::new();

        for pkg in packages {
            // Handle packages with multiple containers
            if let Some(pkg_containers) = pkg.containers {
                for c in pkg_containers {
                    let name = c.container_name.unwrap_or_default();

                    if let Some(ref pattern) = self.filter_pattern {
                        if !Self::matches_pattern(&name, pattern) {
                            continue;
                        }
                    }

                    let image = c.image.unwrap_or_default();
                    let program = Self::detect_program(&registry, &name, &image);

                    containers.push(ContainerInfo {
                        id: c.container_id.unwrap_or_else(|| name.clone()),
                        name,
                        image,
                        status: c.state.unwrap_or_else(|| "unknown".to_string()),
                        program,
                    });
                }
            } else {
                // Single-container package
                let name = pkg.container_name
                    .or(pkg.dnp_name.clone())
                    .or(pkg.name.clone())
                    .unwrap_or_default();

                if let Some(ref pattern) = self.filter_pattern {
                    if !Self::matches_pattern(&name, pattern) {
                        continue;
                    }
                }

                let image = pkg.image.unwrap_or_default();
                let program = Self::detect_program(&registry, &name, &image);

                containers.push(ContainerInfo {
                    id: pkg.container_id.unwrap_or_else(|| name.clone()),
                    name,
                    image,
                    status: pkg.state.unwrap_or_else(|| "unknown".to_string()),
                    program,
                });
            }
        }

        Ok(containers)
    }

    async fn get_logs(
        &self,
        container_id: &str,
        tail: usize,
        follow: bool,
    ) -> Result<LogStream, ProviderError> {
        let container_id = container_id.to_string();
        let provider = DappnodeProvider {
            wamp_url: self.wamp_url.clone(),
            filter_pattern: self.filter_pattern.clone(),
            connection: Arc::new(Mutex::new(None)),
        };

        // Fetch initial logs
        let options = json!({
            "tail": if tail > 0 { tail } else { 100 },
            "timestamps": true
        });

        let result = self.call("logPackage", vec![json!(container_id), options.clone()]).await?;

        // Parse the log result - it returns {id, logs} where logs is a string
        let logs_str = result
            .get("logs")
            .and_then(|v| v.as_str())
            .or_else(|| result.as_str())
            .unwrap_or("");

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

        // For following, we need to poll since DAPPMANAGER doesn't support streaming
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
        let container_id_clone = container_id.clone();
        tokio::spawn(async move {
            let mut poll_interval = interval(LOG_POLL_INTERVAL);
            let mut last_line_count = 0usize;

            loop {
                poll_interval.tick().await;

                let options = json!({
                    "tail": 50,
                    "timestamps": true
                });

                match provider.call("logPackage", vec![json!(&container_id_clone), options]).await {
                    Ok(result) => {
                        let logs_str = result
                            .get("logs")
                            .and_then(|v| v.as_str())
                            .or_else(|| result.as_str())
                            .unwrap_or("");

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
        // Try a ping call
        let _ = self.call("ping", vec![]).await?;
        Ok(())
    }

    fn name(&self) -> &'static str {
        "dappnode"
    }
}

impl Default for DappnodeProvider {
    fn default() -> Self {
        Self::new()
    }
}
