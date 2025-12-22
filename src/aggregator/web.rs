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
    theme: Option<String>,
}

/// WebSocket handler for container logs.
async fn container_logs_ws(
    ws: WebSocketUpgrade,
    Path(container_id): Path<String>,
    Query(query): Query<LogsQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_logs_ws(socket, container_id, query.program, query.theme, state))
}

/// Handle WebSocket connection for log streaming.
async fn handle_logs_ws(
    socket: WebSocket,
    container_id: String,
    program_override: Option<String>,
    theme_override: Option<String>,
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
    let program_id = program.clone().unwrap_or_else(|| "unknown".to_string());

    // Get theme - use override or default
    let theme = theme_override
        .as_ref()
        .and_then(|name| Theme::builtin(name))
        .unwrap_or_else(Theme::default_dark);

    // For custom themes, stream directly with per-connection colorizer
    // This bypasses the shared LogStreamer to support different themes per connection
    let provider = state.provider.clone();
    let container_id_clone = container.id.clone();
    let container_name = container.name.clone();

    let send_task = tokio::spawn(async move {
        use crate::aggregator::html::ansi_to_html;
        use crate::{Colorizer, programs};

        // Create colorizer with the requested theme
        let registry = programs::default_registry();
        let rules = registry
            .get(&program_id)
            .map(|p| p.rules())
            .unwrap_or_default();
        let mut colorizer = Colorizer::new(rules).with_theme(theme);

        // Get log stream from provider
        let mut stream = match provider.get_logs(&container_id_clone, 100, true).await {
            Ok(s) => s,
            Err(e) => {
                let _ = sender
                    .send(Message::Text(
                        format!("{{\"error\": \"Failed to get logs: {e}\"}}").into(),
                    ))
                    .await;
                return;
            }
        };

        while let Some(result) = stream.next().await {
            match result {
                Ok(log_line) => {
                    let line = log_line.content.trim();
                    if line.is_empty() {
                        continue;
                    }

                    let colorized = colorizer.colorize(line);
                    let html = ansi_to_html(&colorized);

                    let json = serde_json::json!({
                        "container_id": container_id_clone,
                        "container_name": container_name,
                        "program": program_id,
                        "html": html
                    });

                    if sender.send(Message::Text(json.to_string().into())).await.is_err() {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Log stream error for {container_name}: {e}");
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
      <select id="theme">
        <option value="default-dark">default-dark</option>
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
    const themeSelect = document.getElementById('theme');
    const followCheckbox = document.getElementById('follow');
    const clearBtn = document.getElementById('clear');
    let ws = null;
    const MAX_LINES = 10000;

    // Load saved theme from localStorage
    const savedTheme = localStorage.getItem('phos-theme') || 'default-dark';
    themeSelect.value = savedTheme;

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

    // Fetch themes
    fetch('/api/themes')
      .then(r => r.json())
      .then(themes => {
        themeSelect.innerHTML = '';
        themes.forEach(t => {
          const opt = document.createElement('option');
          opt.value = t;
          opt.textContent = t;
          if (t === savedTheme) opt.selected = true;
          themeSelect.appendChild(opt);
        });
      })
      .catch(e => console.error('Failed to fetch themes:', e));

    function connect(containerId) {
      if (ws) {
        ws.close();
        ws = null;
      }
      if (!containerId) return;

      logs.innerHTML = '';
      const proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
      const theme = themeSelect.value;
      ws = new WebSocket(`${proto}//${location.host}/ws/logs/${containerId}?theme=${encodeURIComponent(theme)}`);

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
    themeSelect.onchange = () => {
      localStorage.setItem('phos-theme', themeSelect.value);
      if (containerSelect.value) connect(containerSelect.value);
    };
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
