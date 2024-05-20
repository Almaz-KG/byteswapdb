use std::{iter::Peekable, str::Chars};

use crate::Token;
use common::errors::ParsingError;

#[derive(Debug)]
pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, ParsingError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.scan() {
            Ok(Some(token)) => Some(Ok(token)),
            Ok(None) => self.iter.peek().map(|c| {
                Err(ParsingError::UnexpectedToken(format!(
                    "Unexpected token {}",
                    c
                )))
            }),
            Err(err) => Some(Err(err)),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            iter: input.chars().peekable(),
        }
    }

    pub fn scan(&mut self) -> Result<Option<Token>, ParsingError> {
        self.consume_whitespaces();

        match self.iter.peek() {
            Some(c) if *c == '"' => self.scan_string('"'),
            Some(c) if *c == '\'' => self.scan_string('\''),
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

    fn scan_string(&mut self, opening: char) -> Result<Option<Token>, ParsingError> {
        if self.next_if(|c| c == opening).is_none() {
            return Ok(None);
        }

        let mut result = String::new();
        loop {
            match self.iter.next() {
                Some(c) if c == opening => {
                    if let Some(c) = self.next_if(|c| c == opening) {
                        result.push(c)
                    } else {
                        break;
                    }
                }
                Some(c) => result.push(c),
                None => {
                    return Err(ParsingError::UnexpectedEOF);
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
            ';' => Some(Token::SemiColon),
            '|' => Some(Token::Pipe),
            _ => None,
        })
        .map(|token| match token {
            Token::Equals if self.next_if(|c| c == '=').is_some() => Token::DoubleEquals,
            Token::Exclamation if self.next_if(|c| c == '=').is_some() => Token::NotEquals,
            Token::LessThan if self.next_if(|c| c == '>').is_some() => Token::NotEquals,
            Token::LessThan if self.next_if(|c| c == '=').is_some() => Token::LessOrEqual,
            Token::GreaterThan if self.next_if(|c| c == '=').is_some() => Token::GreaterOrEqual,
            Token::Pipe if self.next_if(|c| c == '|').is_some() => Token::DoublePipe,
            _ => token,
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
            r#"A "literal string with 'single' and ''double'' quotes inside 😀"."#,
            vec![
                Token::Identifier("A".into()),
                Token::String(
                    r#"literal string with 'single' and ''double'' quotes inside 😀"#.into(),
                ),
                Token::Period,
            ],
        );
    }

    #[test]
    fn literal_number() {
        assert_scan(
            "0 00 1 3.14 293. -2.718 3.14e3 2.718E-2",
            vec![
                Token::Number("0".into()),
                Token::Number("00".into()),
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
    fn test_special_characters() {
        for (c, token) in [
            ('(', Token::OpenParen),
            (')', Token::CloseParen),
            ('*', Token::Asterisk),
            ('+', Token::Plus),
            (',', Token::Comma),
            ('-', Token::Minus),
            ('.', Token::Period),
            (';', Token::SemiColon),
        ] {
            let input = format!("{c}");
            assert_scan(&input, vec![token.clone()]);

            let input = format!("{c}abc");
            assert_scan(&input, vec![token, Token::Identifier("abc".into())]);
        }
    }

    #[test]
    fn test_space() {
        assert_scan(" ", vec![]);
        assert_scan("      ", vec![]);
        assert_scan("a", vec![Token::Identifier("a".into())]);
        assert_scan("     a", vec![Token::Identifier("a".into())]);

        assert_scan("\t", vec![]);
        assert_scan("\n", vec![]);
        assert_scan("\x0b", vec![]);
        assert_scan("\x0c", vec![]);
        assert_scan("\r", vec![]);
        assert_scan("  \t\n\x0b\x0c\r", vec![]);
    }

    #[test]
    fn test_binary_operators() {
        for (s, token) in [
            ("*", Token::Asterisk),
            (".", Token::Period),
            ("=", Token::Equals),
            (">", Token::GreaterThan),
            ("<", Token::LessThan),
            ("+", Token::Plus),
            ("-", Token::Minus),
            ("/", Token::Slash),
            ("%", Token::Percent),
            ("!", Token::Exclamation),
            ("?", Token::Question),
            ("(", Token::OpenParen),
            (")", Token::CloseParen),
            (",", Token::Comma),
            (";", Token::SemiColon),
            ("|", Token::Pipe),
            ("==", Token::DoubleEquals),
            ("!=", Token::NotEquals),
            ("<>", Token::NotEquals),
            ("<=", Token::LessOrEqual),
            (">=", Token::GreaterOrEqual),
            ("||", Token::DoublePipe),
        ] {
            assert_scan(s, vec![token]);
        }
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
                NotEquals,
                String("country".into()),
                Identifier("AND".into()),
                Identifier("album".into()),
                Period,
                Identifier("release_year".into()),
                GreaterOrEqual,
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

    /// tesing on spider academic dataset (manual only as for now)
    /// Expected no panic during tokenizing process
    // #[test]
    #[ignore]
    #[allow(dead_code)]
    fn test_from_spider_dataset() {
        use std::fs::File;
        use std::io::{self, BufRead};

        // let filename = "spider/train_gold.sql";
        let filename = "spider/dev_gold.sql";
        let file = File::open(filename).expect("TODO");

        for (idx, line) in io::BufReader::new(file).lines().enumerate() {
            assert!(line.is_ok());
            let query = line.unwrap();
            let lexer = Lexer::new(&query);
            let tokens: Vec<Token> = lexer.map(|option| option.unwrap()).collect();
            println!("{idx}: {:?}", tokens);
        }
    }
}
