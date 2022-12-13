use crate::error::{MyhErr, MyhError};

pub fn split_tuple(str: &str) -> Vec<String>{
    let mut out = vec![];
    let mut s = String::new();
    let mut in_str = false;
    let mut in_char = false;
    let mut escaped = false;
    let mut chars = str.chars();
    let mut bracket_level = 0;
    while let Some(c) = chars.next() {
        let was_escaped = escaped;
        match c {
            '(' if !(in_str || in_char) => { bracket_level += 1 },
            ')' if !(in_str || in_char) => { bracket_level -= 1 }
            '\\' if !escaped && (in_str || in_char) => { escaped = true; s.push('\\') },
            '"' if bracket_level == 0 && !(in_str || in_char) => { in_str = true; s.push('"') },
            '"' if !escaped && in_str => { in_str = false; s.push('"') },
            '\'' if bracket_level == 0 && !(in_str || in_char) => { in_char = true; s.push('\'') },
            '\'' if !escaped && in_char => { in_char = false; s.push('\'') },
            ',' if bracket_level == 0 && !(in_str || in_char) => { out.push(s.trim().to_string()); s.clear() }
            _ => s.push(c)
        }
        if was_escaped {
            escaped = false;
        }
    }
    out.push(s.trim().to_string());
    out
}

pub fn assert_str(a: &str, b: &str, err: MyhError) -> Result<(), MyhError> {
    if a != b {
        Err(err)
    } else { Ok(()) }
}

pub fn validate_key(key: &str) -> Result<(), MyhError> {
    for c in key.chars() {
        if !(c.is_ascii_alphanumeric() || c == '_') {
            return Err(MyhErr::InvalidKey(key.to_string()).into())
        }
    }
    return Ok(())
}

pub fn key_index(str: &str) -> Option<usize> {
    let mut index = 0;
    let mut chars = str.chars();
    while let Some(c) = chars.next() {
        if c == ':' {
            return Some(index)
        } else if !(c.is_ascii_alphanumeric() || c == '_') {
            return None
        }
        index += 1;
    }
    None
}

pub fn escape_str(str: &str) -> String{
    str.escape_debug().collect()
}

pub fn unescape_str(str: &str) -> Result<String, MyhError>{
    let mut out = String::new();
    let mut chars = str.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(c) = chars.next() {
                match c {
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    '"' => out.push('"'),
                    '\'' => out.push('\''),
                    'x' => {
                        if let Some(c1) = chars.next() && let Some(c2) = chars.next() {
                            let mut v = String::new();
                            v.push(c1);
                            v.push(c2);
                            out.push(char::from_u32(u32::from_str_radix(&v, 0x10)
                                .map_err(|_e| MyhErr::StringError(format!("invalid ascii literal '\\u{{{v}}}'"), str.to_string()).into())?)
                                .ok_or(MyhErr::StringError(format!("invalid ascii literal '\\u{{{v}}}'"), str.to_string()).into())?)
                        } else {
                            return Err(MyhErr::StringError("unterminated ascii literal".to_string(), str.to_string()).into())
                        }
                    }
                    'u' => {
                        if let Some('{') = chars.next() {} else {
                            return Err(MyhErr::StringError("expected '{{' in unicode literal".to_string(), str.to_string()).into())
                        }
                        let mut ok = false;
                        let mut v = String::new();
                        while let Some(c) = chars.next() {
                            if c == '}' {
                                ok = true;
                                break
                            }
                            v.push(c);
                        }
                        if !ok {
                            return Err(MyhErr::StringError("unterminated unicode literal".to_string(), str.to_string()).into())
                        }
                        out.push(char::from_u32(u32::from_str_radix(&v, 0x10)
                            .map_err(|_e| MyhErr::StringError(format!("invalid unicode literal '\\u{{{v}}}'"), str.to_string()).into())?)
                            .ok_or(MyhErr::StringError(format!("invalid unicode literal '\\u{{{v}}}'"), str.to_string()).into())?)
                    }
                    _ => return Err(MyhErr::StringError(format!("invalid escape sequence '\\{c}'"), str.to_string()).into())
                }
            }
        } else {
            out.push(c);
        }
    }
    Ok(out)
}