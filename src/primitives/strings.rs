use crate::error::{MyhErr, MyhError};
use crate::parsing::{escape_str, unescape_str};
use crate::Primitive;

impl Primitive for char {
    fn stringify(&self) -> String {
        format!("'{self}'")
    }

    fn from_string(str: &str) -> Result<Self, MyhError>{
        if &str[0..=0] != "'" || &str[str.len()-1..=str.len()-1] != "'" {
            return Err(MyhErr::ParsePrimitiveError("char".to_string(), str.to_string()).into())
        }
        let c = unescape_str(&str[1..=str.len()-2])?;
        if c.len() == 1 {
            Ok(c.chars().next().unwrap())
        } else {
            Err(MyhErr::ParsePrimitiveError("char".to_string(), str.to_string()).into())
        }
    }
}

impl Primitive for String {
    fn stringify(&self) -> String {
        format!("\"{}\"", escape_str(self))
    }

    fn from_string(str: &str) -> Result<Self, MyhError>{
        if &str[0..=0] != "\"" || &str[str.len()-1..=str.len()-1] != "\"" {
            return Err(MyhErr::ParsePrimitiveError("string".to_string(), str.to_string()).into())
        }
        unescape_str(&str[1..=str.len()-2])
    }
}