#![warn(clippy::all)]
mod args;

use crate::args::{Cli, Commands};
use clap::Parser;
use sql_parser::{Lexer, Token};
use std::io::{Result, Write};

fn parse_sql_query(query: &str) {
    let vec: Vec<Token> = Lexer::new(query).map(|option| option.unwrap()).collect();
    println!("{:?}", vec);
}

fn start_repl() {
    fn get_command() -> Result<String> {
        print!("[bsdb-cli]> ");
        std::io::stdout().flush()?;
        let mut command = String::new();
        std::io::stdin().read_line(&mut command)?;
        Ok(command.trim().to_string())
    }

    while let Ok(command) = get_command() {
        if command == "exit" || command == "q!" {
            println!("Goodbye!");
            break;
        }
        parse_sql_query(&command);
    }
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Parse(query) => parse_sql_query(&query.query),
        Commands::Repl => start_repl(),
    };
}
