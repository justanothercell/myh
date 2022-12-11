pub fn split_tuple(str: &str) -> Vec<String>{
    let mut out = vec![];
    let mut s = String::new();
    let mut in_str = false;
    let mut in_char = false;
    let mut escaped = false;
    let mut chars = str.chars();
    while let Some(c) = chars.next() {
        let was_escaped = escaped;
        match c {
            '\\' if !escaped && (in_str || in_char) => { escaped = true; s.push('\\') },
            '"' if !(in_str || in_char) => { in_str = true; s.push('"') },
            '"' if !escaped && in_str => { in_str = false; s.push('"') },
            '\'' if !(in_str || in_char) => { in_char = true; s.push('\'') },
            '\'' if !escaped && in_char => { in_char = false; s.push('\'') },
            ',' if !(in_str || in_char) => { out.push(s.trim().to_string()); s.clear() }
            _ => s.push(c)
        }
        if was_escaped {
            escaped = false;
        }
    }
    out.push(s.trim().to_string());
    out
}

pub fn escape_str(str: &str) -> String{
    str.escape_debug().collect()
}

pub fn unescape_str(str: &str) -> Result<String, String>{
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
                            out.push(char::from_u32(u32::from_str_radix(&v, 0x10).map_err(|e|e.to_string())?)
                                .ok_or(format!("invalid ascii literal '\\u{{{v}}}' in '{str}'"))?)
                        } else {
                            return Err(format!("unterminated ascii literal in '{str}'"))
                        }
                    }
                    'u' => {
                        if let Some('{') = chars.next() {} else {
                            return Err(format!("expected '{{' in unicode literal in '{str}'"))
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
                            return Err(format!("unterminated unicode literal in '{str}', expected '}}'"))
                        }
                        out.push(char::from_u32(u32::from_str_radix(&v, 0x10).map_err(|e|e.to_string())?)
                            .ok_or(format!("invalid unicode literal '\\u{{{v}}}' in '{str}'"))?)
                    }
                    _ => return Err(format!("invalid escape sequence '\\{c}' in '{str}'"))
                }
            }
        } else {
            out.push(c);
        }
    }
    Ok(out)
}