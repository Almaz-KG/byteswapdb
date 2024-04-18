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

    /// Runs a Read-Eval-Print-Loop (REPL)
    Repl,

}

#[derive(Debug, Args)]
pub struct SqlQuery {
    /// An sql string to parse and print
    #[arg(short, long)]
    pub query: String,
}