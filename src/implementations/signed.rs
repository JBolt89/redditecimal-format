use crate::implementations::base::*;

impl Redditecimal for i8 {
    fn as_redditecimal(&self) -> String {
        let out = format!("{:x}", self);
        string_to_redditecimal(out.as_str())
    }
}

impl Redditecimal for i16 {
    fn as_redditecimal(&self) -> String {
        let out = format!("{:x}", self);
        string_to_redditecimal(out.as_str())
    }
}

impl Redditecimal for i32 {
    fn as_redditecimal(&self) -> String {
        let out = format!("{:x}", self);
        string_to_redditecimal(out.as_str())
    }
}

impl Redditecimal for i64 {
    fn as_redditecimal(&self) -> String {
        let out = format!("{:x}", self);
        string_to_redditecimal(out.as_str())
    }
}

impl Redditecimal for i128 {
    fn as_redditecimal(&self) -> String {
        let out = format!("{:x}", self);
        string_to_redditecimal(out.as_str())
    }
}

impl Redditecimal for isize {
    fn as_redditecimal(&self) -> String {
        let out = format!("{:x}", self);
        string_to_redditecimal(out.as_str())
    }
}
