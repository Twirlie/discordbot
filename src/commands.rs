// CodenameData is defined in `main.rs` and referenced as `crate::CodenameData` where needed.
use discordbot::{
    BotError, Context, format_codename_response, format_register_response, log_command_usage,
};

/// Helper to send a text response and log it to the DB.
async fn send_and_log(ctx: Context<'_>, response: String) -> Result<(), BotError> {
    ctx.say(response.clone()).await?;
    let data = ctx.data();
    let command_name = ctx.command().name.to_string();
    log_command_usage(&data.db_path, &ctx, &command_name, &response).await;
    Ok(())
}

/// Registers application commands on discord
#[poise::command(slash_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), BotError> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    let response = format_register_response();
    send_and_log(ctx, response).await?;
    Ok(())
}

/// Generates and displays a random codename
#[poise::command(
    slash_command,
    description_localized("en-US", "Generates a random codename")
)]
pub async fn codename(ctx: Context<'_>) -> Result<(), BotError> {
    let codename_data = crate::CODENAME_DATA
        .get()
        .expect("Codename data not initialized");
    let codename = generate_codename(codename_data)?;
    let response = format_codename_response(&codename);
    send_and_log(ctx, response).await?;
    Ok(())
}

/// Generate a random codename from the provided CodenameData
fn generate_codename(codename_data: &crate::CodenameData) -> Result<String, String> {
    discordbot::generate_codename(codename_data)
}

#[cfg(test)]
mod tests {
    use crate::commands::generate_codename;

    #[test]
    fn test_codename_file_loaded() {
        let data = std::fs::read_to_string("./assets/CodenameData.json")
            .expect("Failed to read CodenameData.json");
        let animal_data: crate::CodenameData =
            serde_json::from_str(&data).expect("Failed to parse JSON");

        assert!(
            !animal_data.animals.is_empty(),
            "Animals list should not be empty"
        );
        assert!(
            !animal_data.adjectives.is_empty(),
            "Adjectives list should not be empty"
        );
    }
    #[test]
    fn test_create_random_codename() {
        let data: std::string::String = std::fs::read_to_string("./assets/CodenameData.json")
            .expect("failed to read CodenameData.json");
        let codenamedata: crate::CodenameData = serde_json::from_str(&data).expect("d");
        let codename: String = generate_codename(&codenamedata).unwrap();
        assert!(
            !codename.is_empty(),
            "Generated codename should not be empty"
        );
    }
}
