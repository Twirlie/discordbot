use axum::{
    Router,
    body::Body,
    http::{Request, header},
    middleware::{self, Next},
    response::Response,
};
use colored::Colorize;
use tower_http::services::ServeDir;

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
    let service = ServeDir::new("./frontend/build");

    let app = Router::new()
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
