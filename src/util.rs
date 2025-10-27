use serenity::prelude::*;

// get server name from guild id
pub fn get_server_and_channel_name(
    ctx: &Context,
    message_guild_id: &Option<serenity::model::id::GuildId>,
    channel_id: &serenity::model::id::ChannelId,
) -> Result<(String, String), String> {
    // get the guild id from the message
    let guild_id = match message_guild_id {
        Some(id) => *id,
        None => {
            return Err("No guild ID".to_string());
        }
    };
    match guild_id.to_guild_cached(&ctx.cache) {
        Some(guild) => {
            let channel_name = &guild.channels.get(channel_id).unwrap().name;
            let guild_name = &guild.name;
            return Ok((guild_name.to_string(), channel_name.to_string()));
        }
        None => {
            return Err("Guild not found".to_string());
        }
    };
}
