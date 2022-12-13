use std::fmt::{Debug, Display, Formatter};

pub struct MyhError{
    err: MyhErr,
    path: Vec<String>
}

pub enum MyhErr {
    /// key
    InvalidKey(String),
    /// key
    KeyNotFound(String),
    /// type, input
    ParsePrimitiveError(String, String),
    ///
    NoItem,
    /// index, len
    IndexOutOfBounds(usize, usize),
    /// error, input
    StringError(String, String),
    /// input
    DeserializationError(String),
    /// what, found, line
    Invalid(String, String, usize),
    ///error, file
    FileError(String, String)
}

impl Into<MyhError> for MyhErr {
    fn into(self) -> MyhError {
        MyhError {
            err: self,
            path: vec![],
        }
    }
}

impl MyhError {
    pub fn at(mut self, t: String) -> Self {
        self.path.push(t);
        self
    }
}

impl Debug for MyhError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for MyhError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyhError:\n{}{}", match &self.err {
            MyhErr::InvalidKey(key) => format!("invalid key: '{key}'"),
            MyhErr::KeyNotFound(key) => format!("key not found: '{key}'"),
            MyhErr::ParsePrimitiveError(ty, input) => format!("could not parse primitive {ty}: '{input}'"),
            MyhErr::NoItem => format!("Myh holds no item"),
            MyhErr::IndexOutOfBounds(index, len) => format!("index {index} was out of bounds for list of length {len}"),
            MyhErr::StringError(err, input) => format!("{err} in string or char '{input}'"),
            MyhErr::DeserializationError(input) => format!("could not deserialize '{input}'"),
            MyhErr::Invalid(what, thing, line) => format!("invalid {what} in line {line}: '{thing}'"),
            MyhErr::FileError(err, file) => format!("could not {err} file: '{file}'"),
        }, if self.path.len() > 0 { format!("\nat: '{}'", self.path.clone().into_iter().rev().collect::<Vec<_>>().join("::")) } else { String::new() })
    }
}