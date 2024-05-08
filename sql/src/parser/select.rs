use crate::ast::{Ast, ColumnLiteral, Expression, Literal, Ordering, Select};
use crate::parser::Parser;
use crate::token::{Keyword, Token};
use common::errors::ParsingError;
use std::str::FromStr;

pub trait SelectQueryParser<'a> {
    fn parse_select(&mut self) -> Result<Ast, ParsingError>;

    fn parse_distinct(&mut self) -> Result<bool, ParsingError>;

    fn parse_columns(&mut self) -> Result<Vec<ColumnLiteral>, ParsingError>;

    fn parse_from(&mut self) -> Result<String, ParsingError>;

    fn parse_where_clause(&mut self) -> Result<Option<Expression>, ParsingError>;

    fn parse_group_by_clause(&mut self) -> Result<Option<Vec<String>>, ParsingError>;

    fn parse_having_clause(&mut self) -> Result<Option<Expression>, ParsingError>;

    fn parse_order_by_clause(&mut self) -> Result<Option<Vec<(String, Ordering)>>, ParsingError>;

    fn parse_limit(&mut self) -> Result<Option<usize>, ParsingError>;
}

impl<'a> SelectQueryParser<'a> for Parser<'a> {
    fn parse_select(&mut self) -> Result<Ast, ParsingError> {
        self.assert_current_token_is(Keyword::Select)?;
        self.lexer.next();
        Ok(Ast::Select(Select {
            distinct: self.parse_distinct()?,
            columns: self.parse_columns()?,
            from: self.parse_from()?,
            where_clause: self.parse_where_clause()?,
            group_by: self.parse_group_by_clause()?,
            having: self.parse_having_clause()?,
            order_by: self.parse_order_by_clause()?,
            limit: self.parse_limit()?,
        }))
    }

    fn parse_distinct(&mut self) -> Result<bool, ParsingError> {
        let current_token = self.get_current_token_as_keyword()?;
        match current_token {
            Some(Keyword::Distinct) => {
                self.lexer.next();
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn parse_columns(&mut self) -> Result<Vec<ColumnLiteral>, ParsingError> {
        let current_token = self.get_current_token()?;
        match current_token {
            Token::Asterisk => {
                self.lexer.next();
                match self.get_current_token_as_keyword()? {
                    Some(Keyword::As) => {
                        self.lexer.next();
                        let alias_token = self.get_current_token()?;
                        
                        if let Token::Identifier(alias) = alias_token {
                            self.lexer.next();
                            Ok(vec![ColumnLiteral {
                                expression: Expression::Literal(Literal::String("*".into())),
                                alias: Some(alias),
                            }])
                        } else {
                            Ok(vec![ColumnLiteral::from_expression(Expression::Literal(
                                Literal::String("*".into()),
                            ))])
                        }
                    },
                    Some(_) => {
                        Ok(vec![ColumnLiteral::from_expression(Expression::Literal(
                            Literal::String("*".into()),
                        ))])
                    },
                    None => Ok(vec![ColumnLiteral::from_expression(Expression::Literal(
                        Literal::String("*".into()),
                    ))]),
                }
            }
            _ => {
                let mut columns = vec![];
                let mut expected_next = false;

                while let Ok(token) = self.get_current_token() {
                    expected_next = false;
                    match token {
                        Token::Identifier(identifier) => {
                            let keyword_option = Keyword::from_str(&identifier.to_lowercase());
                            match keyword_option {
                                Ok(Keyword::As) => {
                                    self.lexer.next();
                                    let last_column: &mut ColumnLiteral =
                                        columns.last_mut().ok_or(ParsingError::UnexpectedToken(
                                            Keyword::As.to_string(),
                                        ))?;
                                    
                                    let alias_token = self.get_current_token()?;

                                    if let Token::Identifier(alias) = alias_token {
                                        last_column.alias = Some(alias);
                                        self.lexer.next();
                                        continue;
                                    } else {
                                        return Err(ParsingError::UnexpectedToken(format!(
                                            "Expected alias, got: {}",
                                            alias_token
                                        )));
                                    }
                                }
                                Ok(_) => {
                                    break;
                                }
                                Err(_) => {
                                    // ignore as current token is not a keyword
                                }
                            }
                            columns.push(ColumnLiteral::from_expression(Expression::Literal(
                                Literal::String(identifier),
                            )));
                            self.lexer.next();
                        }
                        Token::Number(num) => {
                            columns.push(ColumnLiteral::from_expression(Expression::Literal(
                                Literal::String(num),
                            )));
                            self.lexer.next();
                        }
                        Token::Period => {
                            let last_column = columns
                                .last_mut()
                                .ok_or(ParsingError::UnexpectedToken(".".into()))?;

                            match &mut last_column.expression {
                                Expression::Value => unimplemented!(),
                                Expression::Operation => unimplemented!(),
                                Expression::Literal(literal) => {
                                    match literal {
                                        Literal::String(ref mut value) => {
                                            self.lexer.next(); // skipping current . period-token
                                            let col_name = self.get_current_token()?;
                                            match col_name {
                                                Token::Identifier(identifier) => {
                                                    *value = format!("{value}.{identifier}");
                                                }
                                                _ => Err(ParsingError::UnexpectedToken(format!(
                                                    "Expected identifier, got: {}",
                                                    col_name
                                                )))?,
                                            }
                                        }
                                        _ => Err(ParsingError::UnexpectedToken(".".into()))?,
                                    }
                                }
                            };
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
        }
    }

    fn parse_from(&mut self) -> Result<String, ParsingError> {
        let current_token = self.get_current_token();
        match current_token {
            Ok(Token::Identifier(identifier)) if Keyword::from_str(&identifier) == Ok(Keyword::From) => {
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

    fn parse_where_clause(&mut self) -> Result<Option<Expression>, ParsingError> {
        Ok(None)
    }

    fn parse_group_by_clause(&mut self) -> Result<Option<Vec<String>>, ParsingError> {
        Ok(None)
    }

    fn parse_having_clause(&mut self) -> Result<Option<Expression>, ParsingError> {
        Ok(None)
    }

    fn parse_order_by_clause(&mut self) -> Result<Option<Vec<(String, Ordering)>>, ParsingError> {
        Ok(None)
    }

    fn parse_limit(&mut self) -> Result<Option<usize>, ParsingError> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_query(query: &str) -> Result<Select, ParsingError> {
        match Parser::new(query).parse()? {
            Ast::Select(select) => Ok(select),
            _ => Err(ParsingError::UnexpectedToken("Expected select AST".into())),
        }
    }

    #[test]
    fn test_select_all_query() {
        let select_stmt =
            parse_query("SELECT * FROM users").expect("Expected valid select statement");
        dbg!(&select_stmt);

        assert_eq!(
            select_stmt.columns,
            vec![ColumnLiteral::from_expression(Expression::Literal(
                Literal::String("*".to_string())
            ))]
        );
        assert_eq!(select_stmt.from, "users".to_string());
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
    fn test_select_lower_and_upper_cases() {
        let select_stmt = parse_query("SELECT 1");
        assert!(select_stmt.is_ok());
        let select_stmt = parse_query("select 1");
        assert!(select_stmt.is_ok());
    }

    #[test]
    fn test_select_constant() {
        let select_stmt = parse_query("SELECT 1").expect("Expected valid select statement");
        assert_eq!(
            select_stmt.columns,
            vec![ColumnLiteral::from_expression(Expression::Literal(
                Literal::String("1".to_string())
            ))]
        );
        assert_eq!(select_stmt.from, "".to_string());
    }

    #[test]
    fn test_select_distinct_query() {
        let select_stmt =
            parse_query("SELECT DISTINCT * FROM users").expect("Expected valid select statement");
        assert!(select_stmt.distinct);

        let select_stmt =
            parse_query("SELECT * FROM users").expect("Expected valid select statement");
        assert!(!select_stmt.distinct);

        let select_stmt = parse_query("SELECT distinct col1, col2 FROM users")
            .expect("Expected valid select statement");
        assert!(select_stmt.distinct);
    }

    #[test]
    fn test_select_single_column_query() {
        let select_stmt =
            parse_query("SELECT col1 FROM users").expect("Expected valid select statement");
        assert_eq!(
            select_stmt.columns,
            vec![ColumnLiteral::from_expression(Expression::Literal(
                Literal::String("col1".to_string())
            ))]
        );
    }

    #[test]
    fn test_select_columns_query() {
        let select_stmt = parse_query("SELECT col1, col2, col3 FROM users")
            .expect("Expected valid select statement");
        assert_eq!(
            select_stmt.columns,
            vec![
                ColumnLiteral::from_expression(Expression::Literal(Literal::String(
                    "col1".to_string()
                ))),
                ColumnLiteral::from_expression(Expression::Literal(Literal::String(
                    "col2".to_string()
                ))),
                ColumnLiteral::from_expression(Expression::Literal(Literal::String(
                    "col3".to_string()
                ))),
            ]
        );
    }

    #[test]
    fn test_select_qualified_column_query() {
        let select_stmt =
            parse_query("SELECT users.col1 FROM users").expect("Expected valid select statement");
        assert_eq!(
            select_stmt.columns,
            vec![ColumnLiteral::from_expression(Expression::Literal(
                Literal::String("users.col1".to_string())
            )),]
        );
    }

    #[test]
    fn test_select_qualified_columns_query() {
        let select_stmt = parse_query("SELECT users.col1, users.col2 FROM users")
            .expect("Expected valid select statement");
        assert_eq!(
            select_stmt.columns,
            vec![
                ColumnLiteral::from_expression(Expression::Literal(Literal::String(
                    "users.col1".to_string()
                ))),
                ColumnLiteral::from_expression(Expression::Literal(Literal::String(
                    "users.col2".to_string()
                ))),
            ]
        );
    }

    #[test]
    fn test_select_qualified_columns2_query() {
        let select_stmt = parse_query("SELECT users.id, orders.order_id FROM users, orders")
            .expect("Expected valid select statement");
        assert_eq!(
            select_stmt.columns,
            vec![
                ColumnLiteral::from_expression(Expression::Literal(Literal::String(
                    "users.id".to_string()
                ))),
                ColumnLiteral::from_expression(Expression::Literal(Literal::String(
                    "orders.order_id".to_string()
                ))),
            ]
        );
    }

    #[test]
    fn test_select_qualified_column_nested_query() {
        let select_stmt = parse_query("SELECT users.id.value FROM users")
            .expect("Expected valid select statement");
        assert_eq!(
            select_stmt.columns,
            vec![ColumnLiteral::from_expression(Expression::Literal(
                Literal::String("users.id.value".to_string())
            )),]
        );
    }

    #[test]
    fn test_select_qualified_columns_nested_query() {
        let select_stmt = parse_query("SELECT users.id.value, orders.order_id FROM users, orders")
            .expect("Expected valid select statement");
        assert_eq!(
            select_stmt.columns,
            vec![
                ColumnLiteral::from_expression(Expression::Literal(Literal::String(
                    "users.id.value".to_string()
                ))),
                ColumnLiteral::from_expression(Expression::Literal(Literal::String(
                    "orders.order_id".to_string()
                ))),
            ]
        );
    }

    #[test]
    fn test_select_column_with_alias() {
        let select_stmt = parse_query("SELECT col1 AS alias FROM users")
            .expect("Expected valid select statement");
        assert_eq!(
            select_stmt.columns,
            vec![ColumnLiteral {
                expression: Expression::Literal(Literal::String("col1".to_string())),
                alias: Some("alias".to_string()),
            }]
        );
    }

    #[test]
    fn test_select_all_columns_with_alias() {
        let select_stmt =
            parse_query("SELECT * AS alias FROM users").expect("Expected valid select statement");
        assert_eq!(
            select_stmt.columns,
            vec![ColumnLiteral {
                expression: Expression::Literal(Literal::String("*".to_string())),
                alias: Some("alias".to_string()),
            }]
        );
    }

    #[test]
    fn test_select_columns_with_aliases() {
        let select_stmt = parse_query("SELECT 1 AS one, 2 AS two FROM users")
            .expect("Expected valid select statement");
        assert_eq!(
            select_stmt.columns,
            vec![
                ColumnLiteral {
                    expression: Expression::Literal(Literal::String("1".to_string())),
                    alias: Some("one".to_string()),
                },
                ColumnLiteral {
                    expression: Expression::Literal(Literal::String("2".to_string())),
                    alias: Some("two".to_string()),
                }
            ]
        );
    }

    #[test]
    fn test_select_nested_column_with_alias() {
        let select_stmt = parse_query("SELECT col1.value1.body AS body FROM users")
            .expect("Expected valid select statement");
        assert_eq!(
            select_stmt.columns,
            vec![ColumnLiteral {
                expression: Expression::Literal(Literal::String("col1.value1.body".to_string())),
                alias: Some("body".to_string()),
            }]
        );
    }

    #[test]
    fn test_select_columns_with_mixed_aliases() {
        let select_stmt = parse_query(
            // r#"SELECT col1, col2 as name_2, 3 AS name_3, id AS "some id" FROM users"#,
            r#"SELECT col1, col2 as name_2, 3 AS name_3 FROM users"#,
        )
        .expect("Expected valid select statement");
        assert_eq!(
            select_stmt.columns,
            vec![
                ColumnLiteral::from_expression(Expression::Literal(Literal::String(
                    "col1".to_string()
                ))),
                ColumnLiteral {
                    expression: Expression::Literal(Literal::String("col2".to_string())),
                    alias: Some("name_2".to_string()),
                },
                ColumnLiteral {
                    expression: Expression::Literal(Literal::String("3".to_string())),
                    alias: Some("name_3".to_string()),
                },
                // ColumnLiteral {
                //     expression: Expression::Literal(Literal::String("id".to_string())),
                //     alias: Some("some id".to_string()),
                // },
            ]
        );
    }

    #[test]
    fn test_select_column_with_alias_bare() {
        let select_stmt = parse_query("SELECT col1 AS");
        assert!(select_stmt.is_err());
        assert_eq!(select_stmt.err(), Some(ParsingError::UnexpectedEOF));
    }

    // field_ambiguous: "SELECT id FROM movies, genres",
    // field_unknown: "SELECT unknown FROM movies",
    // alias: SELECT col1 AS table.body.value FROM users

    // field_unknown_aliased: "SELECT movies.id FROM movies AS m",
    // field_unknown_qualified: "SELECT movies.unknown FROM movies",
    // field_unknown_table: "SELECT unknown.id FROM movies",
    // field_aliased: "SELECT m.id, g.id FROM movies AS m, genres g",

    // expr_dynamic: "SELECT 2020 - year AS age FROM movies",
    // expr_static: "SELECT 1 + 2 * 3, 'abc' LIKE 'x%' AS nope",
    // expr_mixed: "SELECT 1 + 2 * 3, 2020 - released AS age FROM movies",

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
