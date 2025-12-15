use axum::{
    Router,
    body::Body,
    extract::ws::{WebSocket, WebSocketUpgrade},
    http::{Request, StatusCode, header},
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use colored::Colorize;
use poise::futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tower_http::services::ServeDir;

/// FeedItem represents a Discord command usage event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedItem {
    pub item_uuid: String,
    pub timestamp: String,
    pub author_id: String,
    pub author_name: String,
    pub command_name: String,
    pub command_output: String,
    pub test_item: bool,
}

/// Global broadcast channel for command usage events
pub static COMMAND_TX: once_cell::sync::OnceCell<broadcast::Sender<FeedItem>> =
    once_cell::sync::OnceCell::new();

/// Sets up and runs the web server on the specified port
/// # Arguments
/// * `port` - The port number to bind the web server to
/// # Example
/// * `setup_web_server("8080").await;`
pub async fn setup_web_server(port: &str) {
    println!(
        "{}",
        format!("Starting web server on port {}...", port)
            .white()
            .on_green()
    );

    // Create broadcast channel for command events
    let (tx, _rx) = broadcast::channel::<FeedItem>(100);
    COMMAND_TX.set(tx).ok();

    let service = ServeDir::new("./frontend/build");

    let app = Router::new()
        .route("/ws/feed", get(websocket_handler))
        .fallback_service(service)
        .layer(middleware::from_fn(log_requests));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!(
        "{}",
        format!("Web server running on port {}.", port)
            .white()
            .on_green()
    );
    axum::serve(listener, app).await.unwrap()
}

/// WebSocket handler for the feed endpoint
async fn websocket_handler(ws: WebSocketUpgrade) -> Result<Response, StatusCode> {
    Ok(ws.on_upgrade(handle_socket))
}

/// Handles a WebSocket connection and broadcasts command events to the client
async fn handle_socket(socket: WebSocket) {
    let (mut sender, _receiver) = socket.split();
    let tx = match COMMAND_TX.get() {
        Some(tx) => tx.clone(),
        None => return,
    };

    let mut rx = tx.subscribe();

    while let Ok(feed_item) = rx.recv().await {
        if let Ok(json_msg) = serde_json::to_string(&feed_item) {
            if sender
                .send(axum::extract::ws::Message::Text(json_msg.into()))
                .await
                .is_err()
            {
                break;
            }
        }
    }
}

/// Broadcasts a command usage event to all connected WebSocket clients
pub fn broadcast_command_usage(feed_item: FeedItem) {
    if let Some(tx) = COMMAND_TX.get() {
        let _ = tx.send(feed_item);
    }
}

/// Middleware to log incoming requests
/// Logs the HTTP method, path, and User-Agent header if present
async fn log_requests(req: Request<Body>, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let headers = req.headers();
    if let Some(user_agent) = headers.get(header::USER_AGENT) {
        println!(
            "{} {} - User-Agent: {}",
            method.to_string().cyan(),
            path.yellow(),
            user_agent.to_str().unwrap_or("Unknown").green()
        );
    } else {
        println!("{} {}", method.to_string().cyan(), path.yellow(),);
    }

    next.run(req).await
}
