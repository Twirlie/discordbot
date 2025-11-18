use colored::Colorize;
use once_cell::sync::OnceCell;
// poise serenity prelude not needed in library surface
// use rand directly for small helpers (use `rand::random` below)
use rusqlite::Connection;

#[derive(serde::Deserialize, Debug)]
pub struct CodenameData {
    pub animals: Vec<String>,
    pub adjectives: Vec<String>,
}

/// Public global storing the codename data. Initialized during framework setup.
pub static CODENAME_DATA: OnceCell<CodenameData> = OnceCell::new();

/// ### Database data structure
pub struct DbData {
    pub db: Connection,
}

/// ### Bot state, which is shared between commands
pub struct BotState {
    /// Path to the SQLite DB file (we open per-call to avoid sharing Connection across threads)
    pub db_path: String,
}

/// ### the Bot's Error type
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// ### The Bot's Context type
pub type Context<'a> = poise::Context<'a, BotState, Error>;

/// ### Sets up the SQLite database and returns the DbData
pub async fn db_setup(path: &str) -> DbData {
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

/// Async helper that logs command usage for a given author. Extracted so tests can
/// call the same async path as `log_command_usage` without needing a `Context`.
pub async fn log_command_usage_with_author(
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

/// Helper that accepts a `poise::Context` to extract the author and delegate
/// to `log_command_usage_with_author`.
pub async fn log_command_usage(
    db_path: &str,
    ctx: &poise::Context<'_, BotState, Error>,
    command_name: &String,
    command_output: &String,
) {
    let author_id = ctx.author().id.to_string();
    let author_name = ctx.author().name.clone();
    log_command_usage_with_author(
        db_path,
        &author_id,
        &author_name,
        command_name,
        command_output,
    )
    .await;
}

// Crate-public helper that performs the DB insert synchronously. Extracted so tests
// and integration tests can call it directly.
pub fn insert_command_history_sync(
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

pub fn generate_codename(codename_data: &CodenameData) -> Result<String, String> {
    if codename_data.adjectives.is_empty() || codename_data.animals.is_empty() {
        return Err("Codename generation failed".to_string());
    }
    let adj_index = (rand::random::<u64>() as usize) % codename_data.adjectives.len();
    let animal_index = (rand::random::<u64>() as usize) % codename_data.animals.len();
    let adjective = &codename_data.adjectives[adj_index];
    let animal = &codename_data.animals[animal_index];
    Ok(format!("{} {}", adjective, animal))
}
