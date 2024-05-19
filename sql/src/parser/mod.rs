mod select;

use crate::ast::Ast;
use crate::lexer::Lexer;
use crate::parser::select::SelectQueryParser;
use crate::token::{Keyword, Token};
use common::errors::ParsingError;
use std::iter::Peekable;
use std::convert::TryFrom;
use std::convert::TryInto;

pub struct Parser<'a> {
    lexer1: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(query: &'a str) -> Self {
        Parser {
            lexer1: Lexer::new(query).peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Ast, ParsingError> {
        let keyword: Keyword = self.current()?.expect("Expected a keyword");

        match keyword {
            Keyword::Select => self.parse_select(),
            Keyword::Create => unimplemented!(),
            Keyword::Drop => unimplemented!(),
            Keyword::Delete => unimplemented!(),
            Keyword::Insert => unimplemented!(),
            Keyword::Update => unimplemented!(),
            Keyword::Explain => unimplemented!(),
            _ => Err(ParsingError::UnexpectedToken(keyword.to_string())),
        }
    }

    fn current<T: TryFrom<Token>>(&mut self) -> Result<Option<T>, ParsingError> {
        let token = self.current_token()?;
        let result: Result<T, T::Error> = token.try_into();
        match result {
            Ok(t) => Ok(Some(t)),
            Err(_) => Ok(None)
        }
    }

    fn current_token(&mut self) -> Result<Token, ParsingError> {
        match self.lexer1.peek() {
            Some(Ok(token)) => Ok(token.clone()),
            _ => Err(ParsingError::UnexpectedEOF),
        }
    }

    fn has_next_token(&mut self) -> bool {
        self.lexer1.peek().is_some()
    }

    fn eat(&mut self) -> Result<(), ParsingError> {
        match self.lexer1.next() {
            Some(_) => Ok(()),
            None => Err(ParsingError::UnexpectedEOF)
        }
    }

    fn eat_token(&mut self, token: Token) -> Result<bool, ParsingError> {
        let current_token = self.current_token()?;
        if current_token == token {
            self.lexer1.next();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn eat_keyword(&mut self, keyword: Keyword) -> Result<bool, ParsingError> {
        if let Some(current_keyword) = self.current()? {
            if keyword == current_keyword {
                self.lexer1.next();
                return Ok(true);
            }
        }
        Ok(false)
    }
}
