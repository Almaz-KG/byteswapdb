use crate::ast::{Ast, ColumnLiteral, Expression, Literal, Ordering, Select};
use crate::parser::Parser;
use crate::token::{Keyword, Token};
use common::errors::ParsingError;

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
        let select_keyword_eaten = self.eat_keyword(Keyword::Select)?;
        assert!(select_keyword_eaten);

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
        match self.current()? {
            Some(Keyword::Distinct) => {
                self.eat_keyword(Keyword::Distinct)
                    .expect("TODO: Internal error");
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn parse_columns(&mut self) -> Result<Vec<ColumnLiteral>, ParsingError> {
        type NextElementExpectedFlag = bool;
        type ParsedColumn = (Option<ColumnLiteral>, NextElementExpectedFlag);

        fn parse_asterisk(parser: &mut Parser) -> Result<ParsedColumn, ParsingError> {
            parser.eat().expect("TODO: Internal error");
            Ok((Some(ColumnLiteral::from_literal("*".into())), false))
        }

        fn parse_column_expr(
            parser: &mut Parser,
            literal: Literal,
            current_val: String,
        ) -> Result<ParsedColumn, ParsingError> {
            parser.eat().expect("TODO: Internal error");

            let mut column = ColumnLiteral::from_literal(literal);

            if !parser.has_next_token() {
                return Ok((Some(column), false));
            }
            let token = parser.current_token()?;

            if token.clone().try_into() == Ok(Keyword::From) {
                Ok((Some(column), false))
            }
            // Case: select name as something
            else if token.clone().try_into() == Ok(Keyword::As) {
                parser.eat().expect("TODO: Internal error");
                match parser.current_token()? {
                    Token::Identifier(alias) => {
                        parser.eat().expect("TODO: Internal error");
                        let comma_eaten = parser.eat_token(Token::Comma)?;
                        column.alias = Some(alias);
                        return Ok((Some(column), comma_eaten));
                    }
                    Token::String(alias) => {
                        parser.eat().expect("TODO: Internal error");
                        let comma_eaten = parser.eat_token(Token::Comma)?;
                        column.alias = Some(alias);
                        return Ok((Some(column), comma_eaten));
                    }
                    _ => todo!(),
                }
            }
            // Case: select table_name,
            else if token == Token::Comma {
                parser.eat().expect("TODO: Internal error");
                Ok((Some(column), true))
            } else if token == Token::Period {
                parser.eat().expect("TODO: Internal error");
                let mut names = current_val.clone();

                loop {
                    let n_token = parser.current_token()?;
                    match n_token {
                        // Case: select table_name.*
                        Token::Asterisk => {
                            names = format!("{names}.*");
                            parser.eat().expect("TODO: Internal error");
                            break;
                        }
                        // Case: select table_name.column_name
                        Token::Identifier(column_name)
                        | Token::String(column_name)
                        | Token::Number(column_name) => {
                            names = format!("{names}.{column_name}");
                            parser.eat().expect("TODO: Internal error");
                        }
                        _ => break,
                    }

                    let period_eaten = parser.eat_token(Token::Period)?;
                    if !period_eaten {
                        break;
                    }
                }
                let as_keyword_eaten = parser.eat_keyword(Keyword::As)?;
                let mut alias = None;
                if as_keyword_eaten {
                    if let Token::Identifier(alias_name) = parser.current_token()? {
                        alias = Some(alias_name);
                        parser.eat().expect("TODO: Internal error");
                    }
                }

                let comma_eaten = parser.eat_token(Token::Comma)?;
                let column = ColumnLiteral {
                    expression: Expression::Literal(Literal::String(names)),
                    alias,
                };
                Ok((Some(column), comma_eaten))
            } else {
                Err(ParsingError::UnexpectedToken(format!("{token}")))
            }
        }

        fn parse_column(parser: &mut Parser) -> Result<ParsedColumn, ParsingError> {
            if !parser.has_next_token() {
                return Ok((None, false));
            }
            let current_token = parser.current_token()?;

            // Stop parsing columns when the FROM keyword is current token
            if current_token.clone().try_into() == Ok(Keyword::From) {
                return Ok((None, false));
            }

            match current_token {
                // Case: select *
                Token::Asterisk => parse_asterisk(parser),
                // Case: select expr
                Token::Identifier(name) => {
                    parse_column_expr(parser, Literal::String(name.clone()), name)
                }

                // Case: select 1
                Token::Number(number) => {
                    let value: f64 = number.parse().map_err(|_| {
                        ParsingError::InvalidDataType(format!("Unable parse {number} to f64"))
                    })?;
                    let literal = Literal::Number(value);
                    parse_column_expr(parser, literal, number)
                }

                // Case: select 'name'
                Token::String(string) => {
                    parse_column_expr(parser, Literal::String(string.clone()), string)
                }
                _ => todo!(),
            }
        }

        let mut columns = vec![];
        let mut next_column_wanted = true;
        loop {
            let (column_optional, next_exression_expected) = parse_column(self)?;
            if column_optional.is_none() && next_column_wanted {
                return Err(ParsingError::UnexpectedEOF);
            }

            next_column_wanted = next_exression_expected;

            if let Some(column) = column_optional {
                columns.push(column);
            } else {
                break;
            }
        }
        if next_column_wanted {
            return Err(ParsingError::UnexpectedEOF);
        }
        Ok(columns)
    }

    fn parse_from(&mut self) -> Result<String, ParsingError> {
        let current_token = self.current_token();
        match current_token {
            Ok(token) if token.clone().try_into() == Ok(Keyword::From) => {
                self.eat().expect("TODO: Internal error");

                let token = self.current_token()?;
                match token {
                    Token::Identifier(table_name) => Ok(table_name),
                    Token::String(table_name) => Ok(table_name),
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
        assert_eq!(select_stmt.columns.len(), 1);
        assert_eq!(
            select_stmt.columns[0],
            ColumnLiteral::from_literal(Literal::Number(1.0))
        );

        let select_stmt = parse_query("SELECT 'abs'").expect("Expected valid select statement");
        assert_eq!(select_stmt.columns.len(), 1);
        assert_eq!(
            select_stmt.columns[0],
            ColumnLiteral::from_literal(Literal::String("abs".into()))
        );
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
        let select_stmt =
            parse_query("SELECT users.* FROM users").expect("Expected valid select statement");
        assert_eq!(
            select_stmt.columns,
            vec![ColumnLiteral::from_expression(Expression::Literal(
                Literal::String("users.*".to_string())
            )),]
        );
    }

    #[test]
    fn test_select_qualified_multiple_columns_query() {
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
    fn test_select_columns_with_aliases() {
        let select_stmt = parse_query("SELECT 1 AS one, 2 AS two FROM users")
            .expect("Expected valid select statement");
        assert_eq!(
            select_stmt.columns,
            vec![
                ColumnLiteral {
                    expression: Expression::Literal(Literal::Number(1.0)),
                    alias: Some("one".to_string()),
                },
                ColumnLiteral {
                    expression: Expression::Literal(Literal::Number(2.0)),
                    alias: Some("two".to_string()),
                }
            ]
        );
    }

    #[test]
    fn test_select_nested_column_with_alias() {
        let select_stmt = parse_query("SELECT col1.value1.body AS column_name FROM users")
            .expect("Expected valid select statement");
        assert_eq!(
            select_stmt.columns,
            vec![ColumnLiteral {
                expression: Expression::Literal(Literal::String("col1.value1.body".to_string())),
                alias: Some("column_name".to_string()),
            }]
        );
    }

    #[test]
    fn test_select_columns_with_mixed_aliases() {
        let select_stmt =
            parse_query(r#"SELECT col1, col2 as name_2, 3 AS name_3, id AS 'some id' FROM users"#)
                .expect("Expected valid select statement");
        assert_eq!(
            select_stmt.columns,
            vec![
                ColumnLiteral::from_literal(Literal::String("col1".into())),
                ColumnLiteral {
                    expression: Expression::Literal(Literal::String("col2".to_string())),
                    alias: Some("name_2".to_string()),
                },
                ColumnLiteral {
                    expression: Expression::Literal(Literal::Number(3.0)),
                    alias: Some("name_3".to_string()),
                },
                ColumnLiteral {
                    expression: Expression::Literal(Literal::String("id".to_string())),
                    alias: Some("some id".to_string()),
                },
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
