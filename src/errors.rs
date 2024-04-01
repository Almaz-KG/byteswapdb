use std::fmt::Display;

#[derive(Clone, PartialEq, Debug)]
pub enum ByteSwapDBError {
    ParsingError(String),
}

impl Display for ByteSwapDBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ByteSwapDBError::ParsingError(msg) => write!(f, "ParsingError: {}", msg),
        }
    }
}
