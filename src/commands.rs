use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use rand::prelude::IndexedRandom;
#[allow(unused_imports)] // .choose() is used but it seems to think it's unused
use rand::seq::SliceRandom;
use serde::Deserialize;

#[derive(Deserialize)]
struct CodenameData {
    animals: Vec<String>,
    adjectives: Vec<String>,
}

fn get_codenamedata() -> Result<CodenameData, Error> {
    let data: std::string::String = std::fs::read_to_string("./CodenameData.json")?;
    let codenamedata: CodenameData = serde_json::from_str(&data)?;
    Ok(codenamedata)
}

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn codename(
    ctx: Context<'_>,
    #[description = "get a new codename"] _test: Option<String>,
) -> Result<(), Error> {
    let codename_data = get_codenamedata()?;
    let codename = generate_codename(&codename_data)?;
    let response = format!("Your generated codename is: {}", codename);
    ctx.say(response).await?;
    Ok(())
}

fn generate_codename(codename_data: &CodenameData) -> Result<String, String> {
    // randomly select an adjective and an animal from the provided data and concatenate them
    let mut rng = rand::rng();
    if let (Some(adjective), Some(animal)) = (
        codename_data.adjectives.choose(&mut rng),
        codename_data.animals.choose(&mut rng),
    ) {
        Ok(format!("{} {}", adjective, animal))
    } else {
        Err("Codename generation failed".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::CodenameData;
    use crate::commands::generate_codename;

    #[test]
    fn test_codename_file_loaded() {
        let data = std::fs::read_to_string("./CodenameData.json")
            .expect("Failed to read CodenameData.json");
        let animal_data: super::CodenameData =
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
        let data: std::string::String = std::fs::read_to_string("./CodenameData.json")
            .expect("failed to read CodenameData.json");
        let codenamedata: CodenameData = serde_json::from_str(&data).expect("d");
        let codename: String = generate_codename(&codenamedata).unwrap();
        assert!(
            !codename.is_empty(),
            "Generated codename should not be empty"
        );
    }
}
