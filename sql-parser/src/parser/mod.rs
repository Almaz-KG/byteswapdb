mod select;

use crate::ast::Ast;
use crate::lexer::Lexer;
use crate::parser::select::SelectQueryParser;
use crate::token::{Keyword, Token};
use common::errors::ParsingError;
use std::iter::Peekable;
use std::str::FromStr;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(query: &'a str) -> Self {
        Parser {
            lexer: Lexer::new(query).peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Ast, ParsingError> {
        match self.get_current_token_as_keyword()? {
            Some(keyword) => match keyword {
                Keyword::Select => self.parse_select(),
                Keyword::Create => unimplemented!(),
                Keyword::Drop => unimplemented!(),
                Keyword::Delete => unimplemented!(),
                Keyword::Insert => unimplemented!(),
                Keyword::Update => unimplemented!(),
                Keyword::Explain => unimplemented!(),
                _ => Err(ParsingError::UnexpectedToken(keyword.to_string())),
            },
            None => Err(ParsingError::UnexpectedToken(
                "Expected: keyword".to_string(),
            )),
        }
    }

    fn get_current_token_as_keyword(&mut self) -> Result<Option<Keyword>, ParsingError> {
        match self.get_current_token()? {
            Token::Identifier(ident) => match Keyword::from_str(&ident.to_lowercase()) {
                Ok(keyword) => Ok(Some(keyword)),
                Err(_) => Ok(None),
            },
            _ => Err(ParsingError::UnexpectedToken(
                "Expected: keyword".to_string(),
            )),
        }
    }

    fn assert_current_token_is(&mut self, keyword: Keyword) -> Result<(), ParsingError> {
        match self.get_current_token()? {
            Token::Identifier(ident) if ident.to_lowercase() == keyword.to_string() => Ok(()),
            _ => Err(ParsingError::UnexpectedToken(format!(
                "Expected: {}",
                keyword
            ))),
        }
    }

    fn get_current_token(&mut self) -> Result<Token, ParsingError> {
        match self.lexer.peek() {
            Some(Ok(token)) => Ok(token.clone()),
            _ => Err(ParsingError::UnexpectedEOF),
        }
    }
}
