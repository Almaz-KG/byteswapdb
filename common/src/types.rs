use std::fmt::Display;


#[derive(Debug, PartialEq)]
pub enum DataType {
    Boolean,
    Integer,
    Double,
    Char,
    Text,
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Boolean => write!(f, "boolean"),
            DataType::Integer => write!(f, "integer"),
            DataType::Double => write!(f, "double"),
            DataType::Char => write!(f, "char"),
            DataType::Text => write!(f, "text"),
        }
    }
}


#[derive(Debug)]
pub enum Value {
    Boolean(bool),
    Integer(i32),
    Double(f64),
    Char(char),
    Text(String),
    Null,
}


#[derive(Debug, PartialEq)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
}

