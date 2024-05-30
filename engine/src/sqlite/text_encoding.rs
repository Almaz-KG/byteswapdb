use std::fmt::Display;

use crate::errors::DatabaseError;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum TextEncoding {
    UTF_8 = 1,
    UTF_16le = 2,
    UTF_16be = 3,
}

impl TryFrom<u32> for TextEncoding {
    type Error = DatabaseError;

    /// The database text encoding. A value of 1 means UTF-8. A value of 2 means UTF-16le. A value of 3 means UTF-16be.
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(TextEncoding::UTF_8),
            2 => Ok(TextEncoding::UTF_16le),
            3 => Ok(TextEncoding::UTF_16be),
            _ => Err(DatabaseError::StateError(format!(
                "Unsupported text encoding: {value}"
            ))),
        }
    }
}

impl Display for TextEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextEncoding::UTF_8 => write!(f, "1 (utf8)"),
            TextEncoding::UTF_16le => write!(f, "2 (utf16le)"),
            TextEncoding::UTF_16be => write!(f, "3 (utf16be)"),
        }
    }
}
