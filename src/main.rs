#![warn(clippy::all)]
mod args;
mod errors;
mod sql;

use crate::args::{Cli, Commands};
use clap::Parser;
use sql::parser::lexer::{Lexer, Token};

fn parse_sql_query(query: &str) {
    let vec: Vec<Token> = Lexer::new(query).map(|option| option.unwrap()).collect();
    println!("{:?}", vec);
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::ParseSql(query) => match query.query {
            Some(query) => parse_sql_query(&query),
            None => unreachable!(),
        },
    };
}
