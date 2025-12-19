use crate::FeedItem;

use crate::websocket::handle_socket_primary;
use crate::websocket::init_command_broadcast;
use axum::{
    Router,
    body::Body,
    extract::ws::WebSocketUpgrade,
    http::{Request, StatusCode, header},
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use colored::Colorize;
use tokio::sync::broadcast;
use tower_http::services::ServeDir;

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

    // Initialize the broadcast channel
    init_command_broadcast();

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
    Ok(ws.on_upgrade(handle_socket_primary))
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
