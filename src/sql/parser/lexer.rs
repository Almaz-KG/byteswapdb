use crate::errors::ByteSwapDBError;
use std::fmt::Display;
use std::iter::Peekable;
use std::str::Chars;

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

pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, ByteSwapDBError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.scan() {
            Ok(Some(token)) => Some(Ok(token)),
            Ok(None) => self.iter.peek().map(|c| {
                Err(ByteSwapDBError::ParsingError(format!(
                    "Unexpected character {}",
                    c
                )))
            }),
            Err(err) => Some(Err(err)),
        }
    }
}

impl<'a> Lexer<'a> {
    #[allow(dead_code)]
    pub fn new(input: &'a str) -> Self {
        Self {
            iter: input.chars().peekable(),
        }
    }

    pub fn scan(&mut self) -> Result<Option<Token>, ByteSwapDBError> {
        self.consume_whitespaces();

        match self.iter.peek() {
            Some('\'') => self.scan_string(),
            Some(c) if c.is_ascii_digit() => Ok(self.scan_number()),
            Some(c) if c.is_alphabetic() => Ok(self.scan_ident()),
            Some(_) => Ok(self.scan_symbol()),
            None => Ok(None),
        }
    }

    fn next_while<F: Fn(char) -> bool>(&mut self, predicate: F) -> Option<String> {
        let mut value = String::new();
        while let Some(c) = self.next_if(&predicate) {
            value.push(c)
        }
        Some(value).filter(|v| !v.is_empty())
    }

    fn next_if<F: Fn(char) -> bool>(&mut self, predicate: F) -> Option<char> {
        self.iter.peek().filter(|&c| predicate(*c))?;
        self.iter.next()
    }

    fn next_if_token<F: Fn(char) -> Option<Token>>(&mut self, tokenizer: F) -> Option<Token> {
        let token = self.iter.peek().and_then(|&c| tokenizer(c))?;
        self.iter.next();
        Some(token)
    }

    fn consume_whitespaces(&mut self) {
        self.next_while(|c| c.is_whitespace());
    }

    fn scan_string(&mut self) -> Result<Option<Token>, ByteSwapDBError> {
        if self.next_if(|c| c == '\'').is_none() {
            return Ok(None);
        }

        let mut result = String::new();
        loop {
            match self.iter.next() {
                Some('\'') => {
                    if let Some(c) = self.next_if(|c| c == '\'') {
                        result.push(c)
                    } else {
                        break;
                    }
                }
                Some(c) => result.push(c),
                None => {
                    return Err(ByteSwapDBError::ParsingError(
                        "Unterminated string literal".to_string(),
                    ))
                }
            }
        }

        Ok(Some(Token::String(result)))
    }

    fn scan_number(&mut self) -> Option<Token> {
        let mut num = self.next_while(|c| c.is_ascii_digit())?;

        if let Some(sep) = self.next_if(|c| c == '.') {
            num.push(sep);
            while let Some(dec) = self.next_if(|c| c.is_ascii_digit()) {
                num.push(dec)
            }
        }
        if let Some(exp) = self.next_if(|c| c == 'e' || c == 'E') {
            num.push(exp);
            if let Some(sign) = self.next_if(|c| c == '+' || c == '-') {
                num.push(sign)
            }
            while let Some(c) = self.next_if(|c| c.is_ascii_digit()) {
                num.push(c)
            }
        }
        Some(Token::Number(num))
    }

    fn scan_ident(&mut self) -> Option<Token> {
        let mut name = self.next_if(|c| c.is_alphabetic())?.to_string();
        while let Some(c) = self.next_if(|c| c.is_alphanumeric() || c == '_') {
            name.push(c)
        }
        Some(Token::Identifier(name))
    }

    fn scan_symbol(&mut self) -> Option<Token> {
        self.next_if_token(|c| match c {
            '*' => Some(Token::Asterisk),
            '.' => Some(Token::Period),
            '=' => Some(Token::Equals),
            '>' => Some(Token::GreaterThan),
            '<' => Some(Token::LessThan),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '/' => Some(Token::Slash),
            '%' => Some(Token::Percent),
            '!' => Some(Token::Exclamation),
            '?' => Some(Token::Question),
            '(' => Some(Token::OpenParen),
            ')' => Some(Token::CloseParen),
            ',' => Some(Token::Comma),
            _ => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_scan(input: &str, expect: Vec<Token>) {
        let actual: Vec<Token> = Lexer::new(input).map(|r| r.unwrap()).collect();
        assert_eq!(expect, actual);
    }

    #[test]
    fn literal_string() {
        assert_scan(
            r#"A 'literal string with ''single'' and "double" quotes inside 😀'."#,
            vec![
                Token::Identifier("A".into()),
                Token::String(
                    r#"literal string with 'single' and "double" quotes inside 😀"#.into(),
                ),
                Token::Period,
            ],
        );
    }

    #[test]
    fn literal_number() {
        assert_scan(
            "0 1 3.14 293. -2.718 3.14e3 2.718E-2",
            vec![
                Token::Number("0".into()),
                Token::Number("1".into()),
                Token::Number("3.14".into()),
                Token::Number("293.".into()),
                Token::Minus,
                Token::Number("2.718".into()),
                Token::Number("3.14e3".into()),
                Token::Number("2.718E-2".into()),
            ],
        )
    }

    #[test]
    fn select() {
        use Token::*;
        assert_scan(
            "
            SELECT artist.name, album.name, EXTRACT(YEAR FROM NOW()) - album.release_year AS age
            FROM artist INNER JOIN album ON album.artist_id = artist.id
            WHERE album.genre != 'country' AND album.release_year >= 1980
            ORDER BY artist.name ASC, age DESC",
            vec![
                Identifier("SELECT".into()),
                Identifier("artist".into()),
                Period,
                Identifier("name".into()),
                Comma,
                Identifier("album".into()),
                Period,
                Identifier("name".into()),
                Comma,
                Identifier("EXTRACT".into()),
                OpenParen,
                Identifier("YEAR".into()),
                Identifier("FROM".into()),
                Identifier("NOW".into()),
                OpenParen,
                CloseParen,
                CloseParen,
                Minus,
                Identifier("album".into()),
                Period,
                Identifier("release_year".into()),
                Identifier("AS".into()),
                Identifier("age".into()),
                Identifier("FROM".into()),
                Identifier("artist".into()),
                Identifier("INNER".into()),
                Identifier("JOIN".into()),
                Identifier("album".into()),
                Identifier("ON".into()),
                Identifier("album".into()),
                Period,
                Identifier("artist_id".into()),
                Equals,
                Identifier("artist".into()),
                Period,
                Identifier("id".into()),
                Identifier("WHERE".into()),
                Identifier("album".into()),
                Period,
                Identifier("genre".into()),
                Exclamation,
                Equals,
                String("country".into()),
                Identifier("AND".into()),
                Identifier("album".into()),
                Period,
                Identifier("release_year".into()),
                GreaterThan,
                Equals,
                Number("1980".into()),
                Identifier("ORDER".into()),
                Identifier("BY".into()),
                Identifier("artist".into()),
                Period,
                Identifier("name".into()),
                Identifier("ASC".into()),
                Comma,
                Identifier("age".into()),
                Identifier("DESC".into()),
            ],
        )
    }
}
