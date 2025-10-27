use colored::Colorize;

pub fn format_message(
    guild_result: Result<(String, String), String>,
    msg_content: &str,
    author_name: &str,
) -> String {
    match guild_result {
        Ok(guild_result) => {
            let content = msg_content.green();
            let author = author_name.blue().bold();
            let channel = guild_result.1.to_string().purple().italic();
            format!(
                "[{} | Channel {}]\n {}: {}",
                guild_result.0.yellow().italic(),
                channel,
                author,
                content
            )
        }
        Err(_) => {
            let content = msg_content.green();
            let author = author_name.blue().bold();
            format!("[DM]\n {}: {}", author, content)
        }
    }
}
pub fn log_message(
    guild_result: Result<(String, String), String>,
    msg_content: &str,
    author_name: &str,
) {
    let formatted_message = format_message(guild_result, msg_content, author_name);
    println!("{}", formatted_message);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_log_message() {
        let guild_result = Ok(("TestServer".to_string(), "general".to_string()));
        let content = "Hello".green();
        let author = "Alice".blue().bold();
        let channel = 123.to_string().purple().italic();
        let test_message = format!("[Channel {}]\n {}: {}", channel, author, content);
        let s = format_message(guild_result, "Hello", "Alice");
        assert_eq!(s, test_message);
    }
}
