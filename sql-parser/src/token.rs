use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// A number literal
    Number(String),
    /// A string literal
    String(String),
    /// A textual identifier
    Identifier(String),
    /// The period symbol .
    Period,
    /// The equals symbol =
    Equals,
    /// The greater-than symbol >
    GreaterThan,
    /// The less-than symbol <
    LessThan,
    /// The addition symbol +
    Plus,
    /// The subtraction symbol -
    Minus,
    /// The multiplication symbol *
    Asterisk,
    /// The division symbol /
    Slash,
    /// The modulo symbol %
    Percent,
    /// The factorial or not symbol !
    Exclamation,
    /// The query parameter marker ?
    Question,
    /// An opening parenthesis (
    OpenParen,
    /// A closing parenthesis )
    CloseParen,
    /// An expression separator ,
    Comma,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(n) => f.write_str(n),
            Token::String(s) => f.write_str(s),
            Token::Identifier(i) => f.write_str(i),
            Token::Equals => f.write_str("="),
            Token::GreaterThan => f.write_str(">"),
            Token::LessThan => f.write_str("<"),
            Token::Plus => f.write_str("+"),
            Token::Minus => f.write_str("-"),
            Token::Asterisk => f.write_str("*"),
            Token::Slash => f.write_str("/"),
            Token::Percent => f.write_str("%"),
            Token::Exclamation => f.write_str("!"),
            Token::Question => f.write_str("?"),
            Token::OpenParen => f.write_str("("),
            Token::CloseParen => f.write_str(")"),
            Token::Comma => f.write_str(","),
            _ => unreachable!(),
        }
    }
}

/// Reserved SQL Keywords
#[derive(Debug, PartialEq)]
pub enum Keyword {
    And,
    As,
    Asc,
    Begin,
    Boolean,
    By,
    Char,
    Commit,
    Create,
    Cross,
    Delete,
    Desc,
    Distinct,
    Double,
    Drop,
    Exists,
    Explain,
    False,
    From,
    File,
    Group,
    Having,
    If,
    Index,
    Infinity,
    Inner,
    Insert,
    Into,
    Is,
    Join,
    Left,
    Limit,
    Not,
    Null,
    On,
    Or,
    Order,
    Outer,
    Primary,
    Right,
    Rollback,
    Select,
    Table,
    Text,
    Transaction,
    True,
    Unique,
    Update,
    Values,
    Where,
}

impl From<&Keyword> for &str {
    fn from(value: &Keyword) -> Self {
        match value {
            Keyword::And => "and",
            Keyword::As => "as",
            Keyword::Asc => "asc",
            Keyword::Begin => "begin",
            Keyword::Boolean => "boolean",
            Keyword::By => "by",
            Keyword::Char => "char",
            Keyword::Commit => "commit",
            Keyword::Create => "create",
            Keyword::Cross => "cross",
            Keyword::Delete => "delete",
            Keyword::Desc => "desc",
            Keyword::Distinct => "distinct",
            Keyword::Double => "double",
            Keyword::Drop => "drop",
            Keyword::Exists => "exists",
            Keyword::Explain => "explain",
            Keyword::False => "false",
            Keyword::From => "from",
            Keyword::File => "file",
            Keyword::Group => "group",
            Keyword::Having => "having",
            Keyword::If => "if",
            Keyword::Index => "index",
            Keyword::Infinity => "infinity",
            Keyword::Inner => "inner",
            Keyword::Insert => "insert",
            Keyword::Into => "into",
            Keyword::Is => "is",
            Keyword::Join => "join",
            Keyword::Left => "left",
            Keyword::Limit => "limit",
            Keyword::Not => "not",
            Keyword::Null => "null",
            Keyword::On => "on",
            Keyword::Or => "or",
            Keyword::Order => "order",
            Keyword::Outer => "outer",
            Keyword::Primary => "primary",
            Keyword::Right => "right",
            Keyword::Rollback => "rollback",
            Keyword::Select => "select",
            Keyword::Table => "table",
            Keyword::Text => "text",
            Keyword::Transaction => "transaction",
            Keyword::True => "true",
            Keyword::Unique => "unique",
            Keyword::Update => "update",
            Keyword::Values => "values",
            Keyword::Where => "where",
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.into())
    }
}

impl FromStr for Keyword {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "and" => Ok(Keyword::And),
            "as" => Ok(Keyword::As),
            "asc" => Ok(Keyword::Asc),
            "begin" => Ok(Keyword::Begin),
            "boolean" => Ok(Keyword::Boolean),
            "by" => Ok(Keyword::By),
            "char" => Ok(Keyword::Char),
            "commit" => Ok(Keyword::Commit),
            "create" => Ok(Keyword::Create),
            "cross" => Ok(Keyword::Cross),
            "delete" => Ok(Keyword::Delete),
            "desc" => Ok(Keyword::Desc),
            "distinct" => Ok(Keyword::Distinct),
            "double" => Ok(Keyword::Double),
            "drop" => Ok(Keyword::Drop),
            "exists" => Ok(Keyword::Exists),
            "explain" => Ok(Keyword::Explain),
            "false" => Ok(Keyword::False),
            "from" => Ok(Keyword::From),
            "file" => Ok(Keyword::File),
            "group" => Ok(Keyword::Group),
            "having" => Ok(Keyword::Having),
            "if" => Ok(Keyword::If),
            "index" => Ok(Keyword::Index),
            "infinity" => Ok(Keyword::Infinity),
            "inner" => Ok(Keyword::Inner),
            "insert" => Ok(Keyword::Insert),
            "into" => Ok(Keyword::Into),
            "is" => Ok(Keyword::Is),
            "join" => Ok(Keyword::Join),
            "left" => Ok(Keyword::Left),
            "limit" => Ok(Keyword::Limit),
            "not" => Ok(Keyword::Not),
            "null" => Ok(Keyword::Null),
            "on" => Ok(Keyword::On),
            "or" => Ok(Keyword::Or),
            "order" => Ok(Keyword::Order),
            "outer" => Ok(Keyword::Outer),
            "primary" => Ok(Keyword::Primary),
            "right" => Ok(Keyword::Right),
            "rollback" => Ok(Keyword::Rollback),
            "select" => Ok(Keyword::Select),
            "table" => Ok(Keyword::Table),
            "text" => Ok(Keyword::Text),
            "transaction" => Ok(Keyword::Transaction),
            "true" => Ok(Keyword::True),
            "unique" => Ok(Keyword::Unique),
            "update" => Ok(Keyword::Update),
            "values" => Ok(Keyword::Values),
            "where" => Ok(Keyword::Where),
            _ => Err(()),
        }
    }
}
