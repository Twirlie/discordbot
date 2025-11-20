use axum::{Router, routing::get};
use colored::Colorize;

pub async fn setup_web_server(port: &str) {
    println!(
        "{}",
        format!("Starting web server on port {}...", port)
            .white()
            .on_green()
    );
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
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
