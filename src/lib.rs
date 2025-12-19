use colored::Colorize;
use once_cell::sync::OnceCell;
use rusqlite::Connection;

pub mod web;

pub fn format_register_response() -> String {
    "Registered application commands".to_string()
}

pub fn format_codename_response(codename: &str) -> String {
    format!("Your generated codename is:\n **{}!**", codename)
}

#[derive(serde::Deserialize, Debug)]
pub struct CodenameData {
    pub animals: Vec<String>,
    pub adjectives: Vec<String>,
}

/// ### Database data structure
pub struct DbData {
    pub db: Connection,
}

/// ### Bot state, which is shared between commands
pub struct BotState {
    /// Path to the SQLite DB file (we open per-call to avoid sharing Connection across threads)
    pub db_path: String,
}

/// Public global storing the codename data. Initialized during framework setup.
pub static CODENAME_DATA: OnceCell<CodenameData> = OnceCell::new();

/// ### the Bot's Error type
pub type BotError = Box<dyn std::error::Error + Send + Sync>;

/// ### The Bot's Context type
pub type Context<'a> = poise::Context<'a, BotState, BotError>;

/// ### Sets up the SQLite database and returns the DbData
pub async fn db_setup(path: &str) -> DbData {
    println!("{}", "Setting up the database...".white().on_blue());
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
        Ok(_) => println!("{}", "Database setup complete.".white().on_blue()),
        Err(e) => panic!("Failed to set up database: {}", e),
    }

    DbData { db }
}

/// Async function that logs command usage for a given author. Extracted so tests can
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
    println!(
        "{}",
        format!(
            "Logging command usage:\n  user_id={}\n  username={}\n  command={}",
            author_id, author_name, command_name
        )
        .white()
    );
    tokio::task::spawn_blocking(move || {
        // Perform the synchronous DB insert in a blocking task
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
    ctx: &poise::Context<'_, BotState, BotError>,
    command_name: &str,
    command_output: &str,
) {
    log_command_usage_with_author(
        db_path,
        &ctx.author().id.to_string(),
        &ctx.author().name,
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

/// Generate a random codename for the codename command
pub fn generate_codename(codename_data: &CodenameData) -> Result<String, String> {
    if codename_data.adjectives.is_empty() || codename_data.animals.is_empty() {
        return Err("Codename generation failed".to_string());
    }
    let adj_index = (rand::random::<u64>() as usize) % codename_data.adjectives.len();
    let animal_index = (rand::random::<u64>() as usize) % codename_data.animals.len();
    let adjective = &codename_data.adjectives[adj_index];
    let animal = &codename_data.animals[animal_index];
    Ok(format!(
        "{} {}",
        capitalize_first(adjective),
        capitalize_first(animal)
    ))
}

fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Load codename data from a JSON file into the global `CODENAME_DATA` OnceCell.
/// This is the crate-public version so tests and the binary can call it.
pub async fn codename_data_setup_from_path(path: &str) {
    let codename_path = path.to_string();
    tokio::task::spawn_blocking(move || {
        println!("{}", "Loading codename data...".white().on_green());
        let data =
            std::fs::read_to_string(&codename_path).expect("Failed to read CodenameData.json");
        let codenamedata: CodenameData =
            serde_json::from_str(&data).expect("Failed to parse CodenameData.json");
        CODENAME_DATA
            .set(codenamedata)
            .expect("CODENAME_DATA was already initialized");
        println!("{}", "Codename data loaded.".white().on_green());
    })
    .await
    .expect("spawn_blocking failed when loading codename data");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_first() {
        // normal lowercase
        assert_eq!(capitalize_first("hello"), "Hello");

        // already capitalized
        assert_eq!(capitalize_first("World"), "World");

        // empty string
        assert_eq!(capitalize_first(""), "");

        // single character
        assert_eq!(capitalize_first("a"), "A");

        // multiple words (only first letter capitalized)
        assert_eq!(capitalize_first("rust language"), "Rust language");

        // unicode character
        assert_eq!(capitalize_first("äbc"), "Äbc");
    }
}
