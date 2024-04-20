#[derive(Debug, PartialEq)]
pub enum Expression {
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

impl From<String> for Literal {
    fn from(value: String) -> Literal {
        Literal::String(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct ColumnLiteral {
    pub expression: Expression,
    pub alias: Option<String>,
}

impl ColumnLiteral {
    pub fn from_expression(expression: Expression) -> ColumnLiteral {
        ColumnLiteral {
            expression,
            alias: None,
        }
    }
}
