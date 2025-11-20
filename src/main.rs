/// # Discord Bot with SQLite Command History Logging
/// This Rust program implements a Discord bot using the `poise` framework and `serenity` library.
/// It connects to Discord, registers slash commands, and logs command usage into a SQLite database.
/// It includes commands to register application commands, display user account age, and generate random codenames.
/// It uses the `rusqlite` crate for SQLite interactions and `dotenvy` for environment variable management.
use dotenvy::dotenv;
use std::env;

use colored::Colorize;
use poise::serenity_prelude as serenity;
use serenity::prelude::*;

use discordbot::{BotState, CODENAME_DATA, CodenameData, Error, db_setup};

mod commands;

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
                db_setup("history.db").await;
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
