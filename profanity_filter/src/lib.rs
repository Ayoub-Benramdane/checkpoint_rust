pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.contains("stupid") || message.trim().is_empty() {
        return Err("ERROR: illegal")
    }
    Ok("hello there")
}