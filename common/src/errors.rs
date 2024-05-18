use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    UnexpectedToken(String),
    UnexpectedKeyword(String),
    UnexpectedEOF,
    InvalidDataType(String),
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
            ParsingError::UnexpectedKeyword(keyword) => {
                write!(f, "Unexpected keyword: {}", keyword)
            }
            ParsingError::UnexpectedEOF => write!(f, "Unexpected EOF"),
            ParsingError::InvalidDataType(message) => write!(f, "Invalid Data Type: {message}"),
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
