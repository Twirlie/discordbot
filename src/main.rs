/// # Discord Bot with SQLite Command History Logging
/// This Rust program implements a Discord bot using the `poise` framework and `serenity` library.
/// It connects to Discord, registers slash commands, and logs command usage into a SQLite database.
/// It includes commands to register application commands, display user account age, and generate random codenames.
/// It uses the `rusqlite` crate for SQLite interactions and `dotenvy` for environment variable management.
use dotenvy::dotenv;
use std::env;

use poise::serenity_prelude as serenity;
use serenity::prelude::*;

use rusqlite::Connection;

/// ### Database data structure
pub struct DbData {
    pub db: Connection,
}

// Setup State, Error and Context types
/// ### Bot state, which is shared between commands
struct BotState {
    /// Path to the SQLite DB file (we open per-call to avoid sharing Connection across threads)
    pub db_path: String,
}
/// ### the Bot's Error type
type Error = Box<dyn std::error::Error + Send + Sync>;
/// ### The Bot's Context type
type Context<'a> = poise::Context<'a, BotState, Error>;

mod commands;

/// ### Sets up the SQLite database and returns the DbData
pub async fn db_setup() -> DbData {
    println!("Setting up the database...");
    let db = Connection::open("history.db").expect("Failed to open SQLite DB");

    match db.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS command_history (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp   INTEGER NOT NULL,
            user_id     TEXT NOT NULL,
            username    TEXT NOT NULL,
            command     TEXT NOT NULL,
            output      TEXT NOT NULL
        );
    ",
    ) {
        Ok(_) => println!("Database setup complete."),
        Err(e) => eprintln!("Failed to set up database: {}", e),
    }

    DbData { db }
}

async fn log_command_usage(
    db_path: &str,
    ctx: &poise::Context<'_, BotState, Error>,
    command_name: &String,
    command_output: &String,
) {
    // Do DB work in a blocking thread to avoid blocking the async runtime
    let db_path = db_path.to_string();
    let command_name = command_name.clone();
    let command_output = command_output.clone();
    let author_id = ctx.author().id.to_string();
    let author_name = ctx.author().name.clone();
    tokio::task::spawn_blocking(move || {
        let conn = Connection::open(&db_path).expect("Failed to open SQLite DB");
        let timestamp = chrono::Utc::now().timestamp();
        conn.execute(
            "INSERT INTO command_history (timestamp, user_id, username, command, output) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![timestamp, author_id, author_name, command_name, command_output],
        )
        .expect("Failed to log command usage");
    })
    .await
    .ok();
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    println!("Bot starting...");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS;

    let framework = poise::Framework::<BotState, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                // Add commands here
                commands::register(),
                commands::age(),
                commands::codename(),
            ],
            ..Default::default()
        })
        .setup(|_ctx, _ready, _framework| {
            println!("Running framework setup...");
            Box::pin(async move {
                poise::builtins::register_globally(_ctx, &_framework.options().commands).await?;
                // Ensure the DB file and schema exist
                db_setup().await;
                Ok(BotState {
                    db_path: "history.db".to_string(),
                })
            })
        })
        .build();

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
