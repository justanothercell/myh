use crate::parsing::{escape_str, unescape_str};
use crate::Primitive;

impl Primitive for char {
    fn stringify(&self) -> String {
        format!("'{self}'")
    }

    fn from_string(str: &str) -> Option<Self>{
        if &str[0..=0] != "'" || &str[str.len()-1..=str.len()-1] != "'" {
            return None
        }
        let c = unescape_str(&str[1..=str.len()-2]).ok()?;
        if c.len() == 1 {
            c.chars().next()
        } else {
            None
        }
    }
}

impl Primitive for String {
    fn stringify(&self) -> String {
        format!("\"{}\"", escape_str(self))
    }

    fn from_string(str: &str) -> Option<Self>{
        if &str[0..=0] != "\"" || &str[str.len()-1..=str.len()-1] != "\"" {
            return None
        }
        unescape_str(&str[1..=str.len()-2]).ok()
    }
}