use std::fmt::Display;

#[derive(Debug)]
pub enum ParsingError {
    UnexpectedToken(String),
    UnexpectedEOF,
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
            ParsingError::UnexpectedEOF => write!(f, "Unexpected EOF"),
        }
    }
}

#[derive(Debug)]
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
