use colored::Colorize;

pub fn format_message(msg_content: &str, author_name: &str, channel_id: u64) -> String {
    format!(
        "[Channel {}]\n {}: {}",
        channel_id.to_string().purple().italic(),
        author_name.blue().bold(),
        msg_content.green()
    )
}
pub fn log_message(msg_content: &str, author_name: &str, channel_id: u64) {
    let formatted_message = format_message(msg_content, author_name, channel_id);
    println!("{}", formatted_message);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_log_message() {
        let content = "Hello".green();
        let author = "Alice".blue().bold();
        let channel = 123.to_string().purple().italic();
        let test_message = format!("[Channel {}]\n {}: {}", channel, author, content);
        let s = format_message("Hello", "Alice", 123);
        assert_eq!(s, test_message);
    }
}
