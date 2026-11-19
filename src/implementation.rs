pub trait Redditecimal {
    fn as_redditecimal(&self) -> String;
}

pub fn string_to_redditecimal(string: &str) -> String {
    string
        .replace("a", "⬆️")
        .replace("b", "⬇️")
        .replace("c", "❓")
        .replace("d", "🔼")
        .replace("e", "🔽")
        .replace("f", "❔")
}

impl<T: std::fmt::LowerHex> Redditecimal for T {
    fn as_redditecimal(&self) -> String {
        let out = format!("{:x}", self);
        string_to_redditecimal(&out)
    }
}
