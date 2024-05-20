use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Parses the given sql string and prints the result to stdout
    Parse(SqlQuery),

    /// Prints the query plan for the given sql query
    Plan(SqlQuery),

    /// Creates a new database with the given name
    Create(CreateDatabaseCommand),

    /// Connects to the given database and opens REPL session
    Connect(ConnectDatabaseCommand),
}

#[derive(Debug, Args)]
pub struct SqlQuery {
    /// An sql string to parse and print
    #[arg(short, long)]
    pub query: String,
}

#[derive(Debug, Args)]
pub struct CreateDatabaseCommand {
    /// A database name
    #[arg(short, long)]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct ConnectDatabaseCommand {
    /// A database name
    pub name: String,
}
