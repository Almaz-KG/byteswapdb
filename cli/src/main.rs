#![warn(clippy::all)]
mod args;

use crate::args::{Cli, Commands};
use args::{CreateDatabaseCommand, DatabaseName};
use clap::Parser;
use sql::{Lexer, Token};
use std::io::{self, Result, Write};

use engine::sqlite::Database;

fn parse_sql_query(query: &str) {
    let vec: Vec<Token> = Lexer::new(query).map(|option| option.unwrap()).collect();
    println!("{:?}", vec);
}

fn start_repl(database: &mut Database) {
    fn get_command() -> Result<String> {
        print!("[bsdb-cli]> ");
        io::stdout().flush()?;
        let mut command = String::new();
        io::stdin().read_line(&mut command)?;
        Ok(command.trim().to_string())
    }

    while let Ok(command) = get_command() {
        if command == "exit" || command == "q!" {
            println!("Goodbye!");
            break;
        }
        let result = database.execute_sql(command);
        println!("{:?}", result);
    }
}

fn create_database(_command: CreateDatabaseCommand) {
    todo!()
}

fn connect_database(command: DatabaseName) {
    match Database::open(command.name).as_mut() {
        Ok(database) => start_repl(database),
        Err(err) => println!("Unable to open database: {err:?}"),
    }
}

fn database_info(command: DatabaseName) {
    match Database::open(command.name) {
        Ok(database) => {
            database.print_info();
        }
        Err(err) => println!("Unable to open database: {err:?}"),
    }
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Parse(query) => parse_sql_query(&query.query),
        Commands::Plan(query) => parse_sql_query(&query.query),
        Commands::Create(command) => create_database(command),
        Commands::Connect(db) => connect_database(db),
        Commands::DbInfo(db) => database_info(db),
    };
}
