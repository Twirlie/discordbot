/// # Discord Bot with SQLite Command History Logging
/// This Rust program implements a Discord bot using the `poise` framework and `serenity` library.
/// It connects to Discord, registers slash commands, and logs command usage into a SQLite database.
/// It includes commands to register application commands, display user account age, and generate random codenames.
/// It uses the `rusqlite` crate for SQLite interactions and `dotenvy` for environment variable management.
use dotenvy::dotenv;
use std::env;

use colored::Colorize;
use once_cell::sync::OnceCell;
use poise::serenity_prelude as serenity;
use serenity::prelude::*;

use rusqlite::Connection;

/// ### Database data structure
pub struct DbData {
    pub db: Connection,
}

#[derive(serde::Deserialize, Debug)]
pub struct CodenameData {
    pub animals: Vec<String>,
    pub adjectives: Vec<String>,
}

/// Public global storing the codename data. Initialized during framework setup.
pub static CODENAME_DATA: OnceCell<CodenameData> = OnceCell::new();

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
    db_setup_at("history.db").await
}

/// Variant of `db_setup` that writes to an explicit file path. Useful for tests.
pub async fn db_setup_at(path: &str) -> DbData {
    println!("{}", "Setting up the database...".green());
    let db = Connection::open(path).expect("Failed to open SQLite DB");

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
        Ok(_) => println!("{}", "Database setup complete.".green()),
        Err(e) => panic!("Failed to set up database: {}", e),
    }

    DbData { db }
}

/// ### Logs command usage into the SQLite database
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
    println!(
        "{}",
        format!(
            "Logging command usage: user_id={}, username={}, command={}, output={}",
            author_id, author_name, command_name, command_output
        )
        .blue()
    );
    // Delegate to the author-aware helper so tests can exercise the same path
    log_command_usage_with_author(
        &db_path,
        &author_id,
        &author_name,
        &command_name,
        &command_output,
    )
    .await;
}

// Async helper that logs command usage for a given author. Extracted so tests can
// call the same async path as `log_command_usage` without needing a `Context`.
async fn log_command_usage_with_author(
    db_path: &str,
    author_id: &str,
    author_name: &str,
    command_name: &str,
    command_output: &str,
) {
    let db_path = db_path.to_string();
    let author_id = author_id.to_string();
    let author_name = author_name.to_string();
    let command_name = command_name.to_string();
    let command_output = command_output.to_string();
    tokio::task::spawn_blocking(move || {
        insert_command_history_sync(
            &db_path,
            &author_id,
            &author_name,
            &command_name,
            &command_output,
        )
        .expect("Failed to log command usage");
    })
    .await
    .ok();
}

// Crate-private helper that performs the DB insert synchronously. Extracted so tests
// can call it directly without needing a `Context`.
fn insert_command_history_sync(
    db_path: &str,
    author_id: &str,
    author_name: &str,
    command_name: &str,
    command_output: &str,
) -> rusqlite::Result<()> {
    let conn = Connection::open(db_path)?;
    let timestamp = chrono::Utc::now().timestamp();
    conn.execute(
        "INSERT INTO command_history (timestamp, user_id, username, command, output) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![timestamp, author_id, author_name, command_name, command_output],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_db_setup_creates_table() {
        let tmp = NamedTempFile::new().expect("create temp file");
        let path = tmp.path().to_str().expect("path to str");

        // Call the path-specific setup
        let dbdata = db_setup_at(path).await;

        // Verify the table exists
        let mut stmt = dbdata
            .db
            .prepare("SELECT count(name) FROM sqlite_master WHERE type='table' AND name='command_history';")
            .expect("prepare stmt");
        let count: i64 = stmt.query_row([], |r| r.get(0)).expect("query row");
        assert_eq!(count, 1, "command_history table should exist");
    }

    #[tokio::test]
    async fn test_log_command_usage_inserts_row() {
        let tmp = NamedTempFile::new().expect("create temp file");
        let path = tmp.path().to_str().expect("path to str");

        // Initialize schema
        let _ = db_setup_at(path).await;

        // Insert a test row using the sync helper
        insert_command_history_sync(path, "42", "testuser", "testcmd", "ok").expect("insert");

        // Open a connection and verify the inserted row exists
        let conn = Connection::open(path).expect("open conn");
        let mut stmt = conn
            .prepare(
                "SELECT user_id, username, command, output FROM command_history WHERE user_id = ?1",
            )
            .expect("prepare");
        let mut rows = stmt.query(["42"]).expect("query");
        let row = rows.next().expect("row").expect("row unwrap");
        let user_id: String = row.get(0).expect("get user_id");
        let username: String = row.get(1).expect("get username");
        let command: String = row.get(2).expect("get command");
        let output: String = row.get(3).expect("get output");

        assert_eq!(user_id, "42");
        assert_eq!(username, "testuser");
        assert_eq!(command, "testcmd");
        assert_eq!(output, "ok");
    }

    #[tokio::test]
    #[should_panic]
    async fn test_db_setup_fails_on_directory_path() {
        // Create a temporary directory and pass the directory path to db_setup_at.
        // Opening a SQLite database at a path that is a directory should fail.
        let tmpdir = tempfile::tempdir().expect("create temp dir");
        let dir_path = tmpdir.path().to_str().expect("path to str");

        // This should panic due to the underlying sqlite open/exec failing on a directory path
        let _ = db_setup_at(dir_path).await;
    }

    #[tokio::test]
    async fn test_log_command_usage_async_helper_inserts_row() {
        let tmp = NamedTempFile::new().expect("create temp file");
        let path = tmp.path().to_str().expect("path to str");

        // Initialize schema
        let _ = db_setup_at(path).await;

        // Call the author-aware async helper to insert a row
        log_command_usage_with_author(path, "7", "asyncuser", "acmd", "done").await;

        // Verify row exists
        let conn = Connection::open(path).expect("open conn");
        let mut stmt = conn
            .prepare(
                "SELECT user_id, username, command, output FROM command_history WHERE user_id = ?1",
            )
            .expect("prepare");
        let mut rows = stmt.query(["7"]).expect("query");
        let row = rows.next().expect("row").expect("row unwrap");
        let user_id: String = row.get(0).expect("get user_id");
        let username: String = row.get(1).expect("get username");
        let command: String = row.get(2).expect("get command");
        let output: String = row.get(3).expect("get output");

        assert_eq!(user_id, "7");
        assert_eq!(username, "asyncuser");
        assert_eq!(command, "acmd");
        assert_eq!(output, "done");
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    println!("{}", "Bot starting...".yellow());
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
            println!("{}", "Running framework setup...".cyan());
            Box::pin(async move {
                poise::builtins::register_globally(_ctx, &_framework.options().commands).await?;
                // Load codename data into the global once cell. Panic early if loading fails.
                let codename_path = "./assets/CodenameData.json".to_string();
                tokio::task::spawn_blocking(move || {
                    println!("{}", "Loading codename data...".green());
                    let data = std::fs::read_to_string(&codename_path)
                        .expect("Failed to read CodenameData.json");
                    let codenamedata: CodenameData =
                        serde_json::from_str(&data).expect("Failed to parse CodenameData.json");
                    CODENAME_DATA
                        .set(codenamedata)
                        .expect("CODENAME_DATA was already initialized");
                    println!("{}", "Codename data loaded.".green());
                })
                .await
                .expect("spawn_blocking failed when loading codename data");

                // Ensure the DB file and schema exist
                db_setup().await;
                println!("{}", "Framework setup complete.".cyan());
                // Confirm everything finished and the bot is running
                println!("{}", "Bot is running.".green());
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
        println!("{}", format!("Client error: {why:?}").red());
    }
}
