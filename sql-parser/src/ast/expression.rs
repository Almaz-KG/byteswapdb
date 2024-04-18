#[derive(Debug, PartialEq)]
pub enum Expresion {
    Value,
    Operation,
    Literal(Literal),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Null,
    String(String),
    Number(f64),
    Boolean(bool),
}
