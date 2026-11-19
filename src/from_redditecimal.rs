pub fn from_redditecimal(string: &str) -> Result<u128, Box<dyn std::error::Error>> {
    let parseable_string: String = string
        .replace("⬆️", "a")
        .replace("⬇️", "b")
        .replace("❓", "c")
        .replace("🔼", "d")
        .replace("🔽", "e")
        .replace("❔", "f")
        .split_whitespace()
        .collect();

    Ok(u128::from_str_radix(parseable_string.as_str(), 16)?)
}
