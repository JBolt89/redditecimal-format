pub trait Redditecimal {
    /// Converts a number to a redditecimal string.
    fn to_redditecimal(&self) -> String;
}

/// Replaces hexadecimal characters with redditecimal characters.
fn string_to_redditecimal(string: &str) -> String {
    string
        .replace("a", "⬆️")
        .replace("b", "⬇️")
        .replace("c", "❓")
        .replace("d", "🔼")
        .replace("e", "🔽")
        .replace("f", "❔")
}

impl<T: std::fmt::LowerHex> Redditecimal for T {
    /// Converts a number to a redditecimal string.
    fn to_redditecimal(&self) -> String {
        let out = format!("{:x}", self);
        string_to_redditecimal(&out)
    }
}
