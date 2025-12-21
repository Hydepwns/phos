//! Web server for the log aggregator UI.

use axum::{
    Json, Router,
    extract::{
        Path, Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

use crate::Theme;
use crate::alert::AlertCondition;
use crate::aggregator::{AlertConfig, ContainerProvider, LogStreamer};

/// Configuration for the aggregator.
#[derive(Clone)]
pub struct AggregatorConfig {
    /// Color theme for log colorization.
    pub theme: Theme,
    /// Optional container name filter.
    pub filter: Option<String>,
    /// Maximum log lines to buffer.
    pub max_lines: usize,
    /// Optional webhook URL for alerts.
    pub alert_webhook: Option<String>,
    /// Alert conditions to evaluate.
    pub alert_conditions: Vec<AlertCondition>,
}

impl Default for AggregatorConfig {
    fn default() -> Self {
        Self {
            theme: Theme::default_dark(),
            filter: None,
            max_lines: 10000,
            alert_webhook: None,
            alert_conditions: Vec::new(),
        }
    }
}

/// Application state shared across handlers.
#[derive(Clone)]
pub struct AppState {
    pub provider: Arc<dyn ContainerProvider>,
    pub streamer: Arc<LogStreamer>,
    pub active_streams: Arc<Mutex<HashMap<String, tokio::task::JoinHandle<()>>>>,
}

impl AppState {
    /// Create new application state with a provider.
    pub fn new(provider: Arc<dyn ContainerProvider>, theme: Theme) -> Self {
        let streamer = Arc::new(LogStreamer::new(provider.clone(), theme));
        Self {
            provider,
            streamer,
            active_streams: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create from configuration with a provider.
    pub fn from_config(provider: Arc<dyn ContainerProvider>, config: AggregatorConfig) -> Self {
        let alert_config = config.alert_webhook.map(|url| AlertConfig {
            webhook_url: url,
            conditions: config.alert_conditions,
        });

        let streamer = Arc::new(LogStreamer::with_config(
            provider.clone(),
            config.theme,
            config.max_lines,
            alert_config,
        ));

        Self {
            provider,
            streamer,
            active_streams: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

/// Create the axum router with all routes.
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Static UI
        .route("/", get(index_html))
        .route("/styles.css", get(styles_css))
        // API endpoints
        .route("/api/containers", get(list_containers))
        .route("/api/themes", get(list_themes))
        // WebSocket for log streaming
        .route("/ws/logs/{container_id}", get(container_logs_ws))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Serve the main HTML page.
async fn index_html() -> Html<&'static str> {
    Html(INDEX_HTML)
}

/// Serve the CSS stylesheet.
async fn styles_css() -> impl IntoResponse {
    (StatusCode::OK, [("content-type", "text/css")], STYLES_CSS)
}

/// List all containers.
async fn list_containers(State(state): State<AppState>) -> Response {
    match state.provider.list_containers().await {
        Ok(containers) => {
            let json: Vec<_> = containers.iter().map(|c| c.to_json()).collect();
            Json(json).into_response()
        }
        Err(e) => {
            let error = serde_json::json!({
                "error": format!("Failed to list containers: {}", e)
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response()
        }
    }
}

/// List available themes.
async fn list_themes() -> Json<Vec<&'static str>> {
    Json(vec![
        "default-dark",
        "dracula",
        "nord",
        "catppuccin",
        "gruvbox",
        "monokai",
        "solarized",
        "synthwave84",
        "tokyo-night",
        "horizon",
        "matrix",
        "phosphor",
        "high-contrast",
    ])
}

#[derive(Debug, Deserialize)]
struct LogsQuery {
    program: Option<String>,
}

/// WebSocket handler for container logs.
async fn container_logs_ws(
    ws: WebSocketUpgrade,
    Path(container_id): Path<String>,
    Query(query): Query<LogsQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_logs_ws(socket, container_id, query.program, state))
}

/// Handle WebSocket connection for log streaming.
async fn handle_logs_ws(
    socket: WebSocket,
    container_id: String,
    program_override: Option<String>,
    state: AppState,
) {
    let (mut sender, mut receiver) = socket.split();

    // Find container info
    let containers = match state.provider.list_containers().await {
        Ok(c) => c,
        Err(e) => {
            let _ = sender
                .send(Message::Text(
                    format!("{{\"error\": \"Failed to list containers: {e}\"}}").into(),
                ))
                .await;
            return;
        }
    };

    let container = match containers
        .iter()
        .find(|c| c.id == container_id || c.name == container_id)
    {
        Some(c) => c.clone(),
        None => {
            let _ = sender
                .send(Message::Text(
                    format!("{{\"error\": \"Container not found: {container_id}\"}}").into(),
                ))
                .await;
            return;
        }
    };

    // Use program override or detected program
    let program = program_override.or(container.program.clone());

    // Subscribe to log entries
    let mut rx = state.streamer.subscribe();

    // Start streaming for this container if not already
    {
        let mut active = state.active_streams.lock().await;
        if !active.contains_key(&container.id) {
            let handle = state.streamer.spawn_container_stream(
                container.id.clone(),
                container.name.clone(),
                program,
            );
            active.insert(container.id.clone(), handle);
        }
    }

    let target_id = container.id.clone();

    // Spawn task to forward log entries to WebSocket
    let send_task = tokio::spawn(async move {
        while let Ok(entry) = rx.recv().await {
            // Only send entries for this container
            if entry.container_id == target_id {
                let json = entry.to_json().to_string();
                if sender.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    // Wait for client disconnect
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Close(_)) => break,
            Err(_) => break,
            _ => {}
        }
    }

    send_task.abort();
}

/// Embedded HTML for the web UI.
const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>phos - Log Aggregator</title>
  <link rel="stylesheet" href="/styles.css">
</head>
<body>
  <header>
    <h1>phos</h1>
    <div class="controls">
      <select id="container">
        <option value="">Select container...</option>
      </select>
      <button id="clear">Clear</button>
      <label>
        <input type="checkbox" id="follow" checked>
        Follow
      </label>
    </div>
  </header>
  <main id="logs"></main>
  <script>
    const logs = document.getElementById('logs');
    const containerSelect = document.getElementById('container');
    const followCheckbox = document.getElementById('follow');
    const clearBtn = document.getElementById('clear');
    let ws = null;
    const MAX_LINES = 10000;

    // Fetch containers
    fetch('/api/containers')
      .then(r => r.json())
      .then(data => {
        data.forEach(c => {
          const opt = document.createElement('option');
          opt.value = c.id;
          opt.textContent = c.name + (c.program ? ` (${c.program})` : '');
          containerSelect.appendChild(opt);
        });
      })
      .catch(e => console.error('Failed to fetch containers:', e));

    function connect(containerId) {
      if (ws) {
        ws.close();
        ws = null;
      }
      if (!containerId) return;

      logs.innerHTML = '';
      const proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
      ws = new WebSocket(`${proto}//${location.host}/ws/logs/${containerId}`);

      ws.onmessage = (e) => {
        try {
          const entry = JSON.parse(e.data);
          if (entry.error) {
            appendLine(`<span class="error">${entry.error}</span>`);
            return;
          }
          appendLine(entry.html);
        } catch {
          appendLine(e.data);
        }
      };

      ws.onerror = () => {
        appendLine('<span class="error">WebSocket error</span>');
      };

      ws.onclose = () => {
        appendLine('<span class="info">Connection closed</span>');
      };
    }

    function appendLine(html) {
      const line = document.createElement('div');
      line.className = 'log-line';
      line.innerHTML = html;
      logs.appendChild(line);

      // Limit lines
      while (logs.children.length > MAX_LINES) {
        logs.removeChild(logs.firstChild);
      }

      if (followCheckbox.checked) {
        logs.scrollTop = logs.scrollHeight;
      }
    }

    containerSelect.onchange = () => connect(containerSelect.value);
    clearBtn.onclick = () => { logs.innerHTML = ''; };
  </script>
</body>
</html>"#;

/// Embedded CSS for the web UI.
const STYLES_CSS: &str = r#"
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  background: #282a36;
  color: #f8f8f2;
  font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
  height: 100vh;
  display: flex;
  flex-direction: column;
}

header {
  background: #44475a;
  padding: 12px 16px;
  display: flex;
  align-items: center;
  gap: 16px;
  border-bottom: 1px solid #6272a4;
}

header h1 {
  font-size: 1.2rem;
  font-weight: 600;
  color: #bd93f9;
}

.controls {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-left: auto;
}

select, button {
  background: #282a36;
  color: #f8f8f2;
  border: 1px solid #6272a4;
  padding: 6px 12px;
  border-radius: 4px;
  font-family: inherit;
  font-size: 0.9rem;
  cursor: pointer;
}

select:hover, button:hover {
  background: #6272a4;
}

select:focus, button:focus {
  outline: 2px solid #bd93f9;
  outline-offset: 2px;
}

label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.9rem;
}

input[type="checkbox"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
}

main {
  flex: 1;
  overflow-y: auto;
  padding: 8px 16px;
  font-size: 13px;
  line-height: 1.5;
}

.log-line {
  white-space: pre-wrap;
  word-wrap: break-word;
  padding: 2px 0;
}

.log-line:hover {
  background: rgba(98, 114, 164, 0.2);
}

.error {
  color: #ff5555;
}

.info {
  color: #8be9fd;
}

/* Scrollbar styling */
main::-webkit-scrollbar {
  width: 8px;
}

main::-webkit-scrollbar-track {
  background: #282a36;
}

main::-webkit-scrollbar-thumb {
  background: #6272a4;
  border-radius: 4px;
}

main::-webkit-scrollbar-thumb:hover {
  background: #bd93f9;
}
"#;
