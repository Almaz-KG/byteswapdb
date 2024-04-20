mod select;

use crate::parser::select::SelectQueryParser;
use std::iter::Peekable;

use crate::ast::Ast;
use crate::lexer::Lexer;
use crate::token::{Keyword, Token};
use common::errors::ParsingError;

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
        match self.get_current_token()? {
            Token::Identifier(ident) if ident.to_lowercase() == Keyword::Select.to_string() => {
                
                self.parse_select()
            }
            _ => unimplemented!(),
        }
    }

    fn assert_current_token_is(&mut self, keyword: Keyword) -> Result<(), ParsingError> {
        match self.get_current_token()? {
            Token::Identifier(ident) if ident.to_lowercase() == keyword.to_string() => Ok(()),
            _ => Err(ParsingError::UnexpectedToken(format!("Expected: {}", keyword))),
        }
    }

    fn get_current_token(&mut self) -> Result<Token, ParsingError> {
        match self.lexer.peek() {
            Some(Ok(token)) => Ok(token.clone()),
            _ => Err(ParsingError::UnexpectedEOF),
        }
    }
}
