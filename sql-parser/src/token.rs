use std::fmt::Display;

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
