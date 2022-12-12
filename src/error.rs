use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum MyhError {
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
    /// error, input
    DeserializationError(String, String)
}

impl Display for MyhError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "self")
    }
}