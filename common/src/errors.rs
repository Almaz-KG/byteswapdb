use std::fmt::Display;

#[derive(Clone, PartialEq, Debug)]
pub enum DatabaseError {
    ParsingError(String),
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::ParsingError(msg) => write!(f, "ParsingError: {}", msg),
        }
    }
}
