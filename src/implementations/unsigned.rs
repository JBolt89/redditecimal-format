use crate::implementations::base::*;

impl Redditecimal for u8 {
    fn as_redditecimal(&self) -> String {
        let out = format!("{:x}", self);
        string_to_redditecimal(out.as_str())
    }
}

impl Redditecimal for u16 {
    fn as_redditecimal(&self) -> String {
        let out = format!("{:x}", self);
        string_to_redditecimal(out.as_str())
    }
}

impl Redditecimal for u32 {
    fn as_redditecimal(&self) -> String {
        let out = format!("{:x}", self);
        string_to_redditecimal(out.as_str())
    }
}

impl Redditecimal for u64 {
    fn as_redditecimal(&self) -> String {
        let out = format!("{:x}", self);
        string_to_redditecimal(out.as_str())
    }
}

impl Redditecimal for u128 {
    fn as_redditecimal(&self) -> String {
        let out = format!("{:x}", self);
        string_to_redditecimal(out.as_str())
    }
}

impl Redditecimal for usize {
    fn as_redditecimal(&self) -> String {
        let out = format!("{:x}", self);
        string_to_redditecimal(out.as_str())
    }
}
