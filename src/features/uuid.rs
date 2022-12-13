use uuid::Uuid;
use crate::error::{MyhErr, MyhError};
use crate::Primitive;

impl Primitive for Uuid {
    fn stringify(&self) -> String {
        self.to_string()
    }

    fn from_string(str: &str) -> Result<Self, MyhError> where Self: Sized {
        str.parse().map_err(|_| MyhErr::ParsePrimitiveError("Uuid".to_string(), str.to_string()).into())
    }
}