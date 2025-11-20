use colored::Colorize;
use discordbot::{BotState, CODENAME_DATA, CodenameData, Error, db_setup};
use dotenvy::dotenv;
use poise::serenity_prelude as serenity;
use serenity::prelude::*;
use std::env;
mod commands;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    println!("{}", "Bot starting...".black().on_yellow());
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
            println!("{}", "Running framework setup...".white().on_cyan());
            Box::pin(async move { run_setup(_ctx, _ready, _framework).await })
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
    // Start the Discord client and begin listening for events.
    let start_result = client.start().await;
    match start_result {
        Ok(_) => {
            println!("Discord client started successfully!");
        }
        Err(error) => {
            // Print the error in red for visibility.
            println!("{}", format!("Discord client failed: {:?}", error).red());
        }
    }
}
/// Framework setup function
/// - Registers application commands globally
/// - Loads codename data from JSON file
/// - Sets up the SQLite database
/// - Returns the initial BotState
async fn run_setup(
    ctx: &Context,
    _ready: &serenity::Ready,
    _framework: &poise::Framework<BotState, Error>,
) -> Result<BotState, Error> {
    // Register application commands globally
    poise::builtins::register_globally(ctx, &_framework.options().commands).await?;
    //load codename data
    discordbot::codename_data_setup_from_path("./assets/CodenameData.json").await;
    // Ensure the DB file and schema exist
    db_setup("history.db").await;
    println!("{}", "Framework setup complete.".white().on_cyan());
    // Confirm everything finished and the bot is running
    println!("{}", "Bot is running!".white().on_bright_magenta());
    Ok(BotState {
        db_path: "history.db".to_string(),
    })
}
