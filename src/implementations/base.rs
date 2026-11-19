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
