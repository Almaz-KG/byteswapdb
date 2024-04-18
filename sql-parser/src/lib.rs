mod ast;
mod lexer;
mod parser;
mod token;

pub use lexer::Lexer;
pub use parser::Parser;
pub use token::Token;

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
