use crate::DEFAULT_DB_PATH;
use crate::FeedItem;
use crate::load_recent_commands;
use axum::extract::ws::{Message, WebSocket};
use futures::sink::SinkExt;
use futures::stream::SplitSink;
use futures::stream::SplitStream;
use futures::stream::StreamExt;
use tokio::sync::broadcast;

/// Global broadcast channel for command usage events
pub static COMMAND_TX: once_cell::sync::OnceCell<broadcast::Sender<FeedItem>> =
    once_cell::sync::OnceCell::new();

/// Initialize the broadcast channel (call this once at application startup)
pub fn init_command_broadcast() {
    let (tx, _rx) = broadcast::channel(100); // buffer size of 100
    let _ = COMMAND_TX.set(tx);
}

pub async fn handle_socket_primary(socket: WebSocket) {
    let (sender, receiver) = socket.split();

    // Spawn sender task
    tokio::spawn(sender_task(sender));

    // Run receiver task
    receiver_task(receiver).await;
}

// ============================================================================
// SENDER: Broadcasts events to the client
// ============================================================================

async fn sender_task(mut sender: SplitSink<WebSocket, Message>) {
    let tx = match COMMAND_TX.get() {
        Some(tx) => tx.clone(),
        None => return,
    };

    let mut rx = tx.subscribe();

    while let Ok(event) = rx.recv().await {
        if let Ok(json) = serde_json::to_string(&event) {
            if sender.send(Message::Text(json.into())).await.is_err() {
                break;
            }
        }
    }
}

// ============================================================================
// RECEIVER: Handles incoming messages from the client
// ============================================================================

async fn receiver_task(mut receiver: SplitStream<WebSocket>) {
    while let Some(result) = receiver.next().await {
        match result {
            Ok(Message::Text(text)) => {
                // parse text as JSON
                let message = match serde_json::from_str::<serde_json::Value>(&text) {
                    Ok(message) => message,
                    Err(e) => {
                        eprintln!("Failed to parse JSON: {}", e);
                        continue;
                    }
                };
                // check the message action type is equal to request_items
                if let Some(action) = message.get("action") {
                    if action == "request_items" {
                        // handle request for recent commands
                        if let Some(count) = message.get("count") {
                            if let Some(count) = count.as_i64() {
                                handle_request_for_recent_commands(count).await;
                            }
                        }
                    }
                }
            }
            Ok(Message::Binary(data)) => {
                println!("Binary: {} bytes", data.len());
            }
            Ok(Message::Close(_)) => break,
            Ok(Message::Ping(_)) => {}
            Ok(Message::Pong(_)) => {}
            Err(_) => break,
        }
    }
}

// ============================================================================
// HELPERS
// ============================================================================

/// Broadcasts a command usage event to all connected WebSocket clients
pub fn broadcast_command_usage(feed_item: FeedItem) {
    println!("Lets try broadcasting command usage event...");
    if let Some(tx) = COMMAND_TX.get() {
        println!("Broadcasting command usage event");
        println!("feed_item: {:?}", feed_item);
        let _ = tx.send(feed_item);
    } else {
        println!("Broadcast channel not initialized");
    }
}

/// when the frontend requests for recent commands use load_recent_commands to load the recent commands from the database and send them to the client with a websocket message
pub async fn handle_request_for_recent_commands(count: i64) {
    // Load recent commands from the database
    let recent_commands = match load_recent_commands(DEFAULT_DB_PATH, count) {
        Ok(commands) => commands,
        Err(e) => {
            eprintln!("Failed to load recent commands: {}", e);
            return;
        }
    };
    println!(
        "Sending {} recent commands to client",
        recent_commands.len()
    );

    // Send the recent commands to the client via broadcast
    // Small delay ensures subscribers are ready
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    // Send the recent commands to the client with a websocket message
    for feed_item in recent_commands {
        broadcast_command_usage(feed_item);
    }
}
