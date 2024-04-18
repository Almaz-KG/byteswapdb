use std::iter::Peekable;
use std::str::FromStr;

use crate::ast::{Ast, Expresion, Literal, Ordering};
use crate::lexer::Lexer;
use crate::token::{Keyword, Token};
use common::errors::ParsingError;
use common::types::Column;

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
                self.lexer.next();
                self.parse_select()
            }
            _ => unimplemented!(),
        }
    }

    fn parse_select(&mut self) -> Result<Ast, ParsingError> {
        Ok(Ast::Select {
            distinct: self.parse_distinct()?,
            columns: self.parse_columns()?,
            from: self.parse_from()?,
            where_clause: self.parse_where_clause()?,
            group_by: self.parse_group_by_clause()?,
            having: self.parse_having_clause()?,
            order_by: self.parse_order_by_clause()?,
            limit: self.parse_limit()?,
        })
    }

    fn get_current_token(&mut self) -> Result<Token, ParsingError> {
        match self.lexer.peek() {
            Some(Ok(token)) => Ok(token.clone()),
            _ => Err(ParsingError::UnexpectedEOF),
        }
    }

    #[allow(clippy::let_and_return)]
    fn parse_distinct(&mut self) -> Result<bool, ParsingError> {
        let current_token = self.get_current_token()?;

        let result = match current_token {
            Token::Identifier(identifier)
                if identifier.to_lowercase() == Keyword::Distinct.to_string() =>
            {
                self.lexer.next();
                Ok(true)
            }
            _ => Ok(false),
        };

        // dbg!(&result);
        result
    }

    #[allow(clippy::let_and_return)]
    fn parse_columns(&mut self) -> Result<Vec<Expresion>, ParsingError> {
        let current_token = self.get_current_token()?;
        let result = match current_token {
            Token::Asterisk => {
                self.lexer.next();
                Ok(vec![Expresion::Literal(Literal::String("*".into()))])
            }
            _ => {
                let mut columns = vec![];
                let mut expected_next = false;
                while let Ok(token) = self.get_current_token() {
                    expected_next = false;
                    match token {
                        Token::Identifier(identifier) => {
                            if Keyword::from_str(&identifier.to_lowercase()).is_ok() {
                                break;
                            }
                            // dbg!(&identifier);
                            columns.push(Expresion::Literal(Literal::String(identifier)));
                            self.lexer.next();
                        }
                        Token::Number(num) => {
                            columns.push(Expresion::Literal(Literal::String(num)));
                            self.lexer.next();
                        }
                        Token::Comma => {
                            self.lexer.next();
                            expected_next = true;
                        }
                        _ => {
                            break;
                        }
                    }
                }
                if expected_next {
                    Err(ParsingError::UnexpectedEOF)
                } else {
                    Ok(columns)
                }
            }
        };

        // dbg!(&result);
        result
    }

    fn parse_from(&mut self) -> Result<String, ParsingError> {
        let current_token = self.get_current_token();

        match current_token {
            Ok(Token::Identifier(identifier))
                if identifier.to_lowercase() == Keyword::From.to_string() =>
            {
                self.lexer.next();
                let token = self.get_current_token()?;
                match token {
                    Token::Identifier(table_name) => Ok(table_name),
                    _ => Err(ParsingError::UnexpectedToken(token.to_string())),
                }
            }
            // The case where single select query was given (without FROM keyword)
            Err(ParsingError::UnexpectedEOF) => Ok("".into()),
            _ => Err(ParsingError::UnexpectedEOF),
        }
    }

    fn parse_where_clause(&mut self) -> Result<Option<Expresion>, ParsingError> {
        Ok(None)
    }

    fn parse_group_by_clause(&mut self) -> Result<Option<Vec<Column>>, ParsingError> {
        Ok(None)
    }

    fn parse_having_clause(&mut self) -> Result<Option<Expresion>, ParsingError> {
        Ok(None)
    }

    fn parse_order_by_clause(&mut self) -> Result<Option<Vec<(Column, Ordering)>>, ParsingError> {
        Ok(None)
    }

    fn parse_limit(&mut self) -> Result<Option<usize>, ParsingError> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_query(query: &str) -> Result<Ast, ParsingError> {
        Parser::new(query).parse()
    }

    #[test]
    fn test_select_query() {
        let select_stmt = parse_query("SELECT * FROM users");
        assert!(select_stmt.is_ok());
        let select_stmt = select_stmt.unwrap();

        let expected = Ast::Select {
            distinct: false,
            columns: vec![Expresion::Literal(Literal::String("*".to_string()))],
            from: "users".into(),
            where_clause: None,
            group_by: None,
            having: None,
            order_by: None,
            limit: None,
        };
        assert_eq!(select_stmt, expected);
    }

    #[test]
    fn test_bare_select_statement() {
        let select_stmt = parse_query("SELECT");
        assert!(select_stmt.is_err());
    }

    #[test]
    fn test_select_with_trailing_comma() {
        let select_stmt = parse_query("SELECT 1,");
        dbg!(&select_stmt);
        assert!(select_stmt.is_err());
    }

    #[test]
    fn test_select_constant() {
        let select_stmt = parse_query("SELECT 1");
        assert!(select_stmt.is_ok());

        let select_stmt = select_stmt.unwrap();

        let expected = Ast::Select {
            distinct: false,
            columns: vec![Expresion::Literal(Literal::String("1".to_string()))],
            from: "".into(),
            where_clause: None,
            group_by: None,
            having: None,
            order_by: None,
            limit: None,
        };
        assert_eq!(select_stmt, expected);
    }

    #[test]
    fn test_select_distinct_query() {
        let select_stmt = parse_query("SELECT DISTINCT * FROM users");
        println!("{:?}", select_stmt);
        assert!(select_stmt.is_ok());
        let select_stmt = select_stmt.unwrap();

        let expected = Ast::Select {
            distinct: true,
            columns: vec![Expresion::Literal(Literal::String("*".to_string()))],
            from: "users".into(),
            where_clause: None,
            group_by: None,
            having: None,
            order_by: None,
            limit: None,
        };
        assert_eq!(select_stmt, expected);
    }

    #[test]
    fn test_select_columns_query() {
        let select_stmt = parse_query("SELECT col1, col2 FROM users");
        println!("{:?}", select_stmt);
        assert!(select_stmt.is_ok());
        let select_stmt = select_stmt.unwrap();

        let expected = Ast::Select {
            distinct: false,
            columns: vec![
                Expresion::Literal(Literal::String("col1".to_string())),
                Expresion::Literal(Literal::String("col2".to_string())),
            ],
            from: "users".into(),
            where_clause: None,
            group_by: None,
            having: None,
            order_by: None,
            limit: None,
        };
        assert_eq!(select_stmt, expected);
    }

    #[test]
    fn test_select_distinct_columns_query() {
        let select_stmt = parse_query("SELECT distinct col1, col2 FROM users");
        println!("{:?}", select_stmt);
        assert!(select_stmt.is_ok());
        let select_stmt = select_stmt.unwrap();

        let expected = Ast::Select {
            distinct: true,
            columns: vec![
                Expresion::Literal(Literal::String("col1".to_string())),
                Expresion::Literal(Literal::String("col2".to_string())),
            ],
            from: "users".into(),
            where_clause: None,
            group_by: None,
            having: None,
            order_by: None,
            limit: None,
        };
        assert_eq!(select_stmt, expected);
    }

    // bare: "SELECT",
    // trailing_comma: "SELECT 1,",
    // lowercase: "select 1",

    // field_single: "SELECT id FROM movies",
    // field_multi: "SELECT id, title FROM movies",
    // field_ambiguous: "SELECT id FROM movies, genres",
    // field_qualified: "SELECT movies.id FROM movies",
    // field_qualified_multi: "SELECT movies.id, genres.id FROM movies, genres",
    // field_qualified_nested: "SELECT movies.id.value FROM movies",
    // field_unknown: "SELECT unknown FROM movies",
    // field_unknown_aliased: "SELECT movies.id FROM movies AS m",
    // field_unknown_qualified: "SELECT movies.unknown FROM movies",
    // field_unknown_table: "SELECT unknown.id FROM movies",
    // field_aliased: "SELECT m.id, g.id FROM movies AS m, genres g",

    // expr_dynamic: "SELECT 2020 - year AS age FROM movies",
    // expr_static: "SELECT 1 + 2 * 3, 'abc' LIKE 'x%' AS nope",
    // expr_mixed: "SELECT 1 + 2 * 3, 2020 - released AS age FROM movies",

    // as_: r#"SELECT 1, 2 b, 3 AS c, 4 AS "👋", id AS "some id" FROM movies"#,
    // as_bare: "SELECT 1 AS",
    // as_all: "SELECT * AS all FROM movies",
    // as_duplicate: "SELECT 1 AS a, 2 AS a",
    // as_qualified: r#"SELECT 1 AS a.b FROM movies"#,

    // from_bare: "SELECT * FROM",
    // from_multiple: "SELECT * FROM movies, genres, countries",
    // from_unknown: "SELECT * FROM unknown",
    // from_alias_duplicate: "SELECT * FROM movies a, genres a",
    // from_alias_duplicate_join: "SELECT * FROM movies a JOIN genres a ON TRUE",
    // from_duplicate: "SELECT * FROM movies, movies",

    // where_bare: "SELECT * FROM movies WHERE",
    // where_true: "SELECT * FROM movies WHERE TRUE",
    // where_false: "SELECT * FROM movies WHERE FALSE",
    // where_null: "SELECT * FROM movies WHERE NULL",
    // where_expr: "SELECT * FROM movies WHERE released >= 2000 AND ultrahd",
    // where_float: "SELECT * FROM movies WHERE 3.14",
    // where_integer: "SELECT * FROM movies WHERE 7",
    // where_string: "SELECT * FROM movies WHERE 'abc'",
    // where_multi: "SELECT * FROM movies WHERE TRUE, TRUE",
    // where_pk: "SELECT * FROM movies WHERE id = 3",
    // where_pk_or: "SELECT * FROM movies WHERE id = 3 OR id = 5 OR id = 7",
    // where_pk_or_partial: "SELECT * FROM movies WHERE (id = 2 OR id = 3 OR id = 4 OR id = 5) AND genre_id = 1",
    // where_index: "SELECT * FROM movies WHERE genre_id = 2 ORDER BY id",
    // where_index_or: "SELECT * FROM movies WHERE genre_id = 2 OR genre_id = 3 OR genre_id = 4 OR genre_id = 5 ORDER BY id",
    // where_index_or_partial: "SELECT * FROM movies WHERE (genre_id = 2 OR genre_id = 3) AND studio_id = 2 ORDER BY id",
    // where_field_unknown: "SELECT * FROM movies WHERE unknown",
    // where_field_qualified: "SELECT movies.id, genres.id FROM movies, genres WHERE movies.id >= 3 AND genres.id = 1",
    // where_field_ambiguous: "SELECT movies.id, genres.id FROM movies, genres WHERE id >= 3",
    // where_field_aliased_select: "SELECT m.id AS movie_id, g.id AS genre_id FROM movies m, genres g WHERE movie_id >= 3 AND genre_id = 1",
    // where_field_aliased_table: "SELECT m.id, g.id FROM movies m, genres g WHERE m.id >= 3 AND g.id = 1",
    // where_join_inner: "SELECT * FROM movies, genres WHERE movies.genre_id = genres.id",

    // order: "SELECT * FROM movies ORDER BY released",
    // order_asc: "SELECT * FROM movies ORDER BY released ASC",
    // order_asc_lowercase: "SELECT * FROM movies ORDER BY released asc",
    // order_desc: "SELECT * FROM movies ORDER BY released DESC",
    // order_desc_lowercase: "SELECT * FROM movies ORDER BY released desc",
    // order_expr: "SELECT id, title, released, released % 4 AS ord FROM movies ORDER BY released % 4 ASC",
    // order_multi: "SELECT * FROM movies ORDER BY ultrahd ASC, id DESC",
    // order_noselect: "SELECT id, title FROM movies ORDER BY released",
    // order_unknown_dir: "SELECT * FROM movies ORDER BY id X",
    // order_field_unknown: "SELECT * FROM movies ORDER BY unknown",
    // order_field_qualified: "SELECT movies.id, title, name FROM movies, genres WHERE movies.genre_id = genres.id ORDER BY genres.name, movies.title",
    // order_field_aliased: "SELECT movies.id, title, genres.name AS genre FROM movies, genres WHERE movies.genre_id = genres.id ORDER BY genre, title",
    // order_field_ambiguous: "SELECT * FROM movies, genres WHERE movies.genre_id = genres.id ORDER BY id",
    // order_trailing_comma: "SELECT * FROM movies ORDER BY id,",
    // order_aggregate: "SELECT studio_id, MAX(rating) FROM movies GROUP BY studio_id ORDER BY MAX(rating)",
    // order_aggregate_noselect: "SELECT studio_id, MAX(rating) FROM movies GROUP BY studio_id ORDER BY MIN(rating)",
    // order_group_by_noselect: "SELECT MAX(rating) FROM movies GROUP BY studio_id ORDER BY studio_id",
}
