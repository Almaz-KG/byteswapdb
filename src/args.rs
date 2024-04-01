// #[derive(Debug, Parser)]
// #[command(author, version, about, long_about = None)]
// pub struct Args {
//     #[arg(short, long)]
//     pub query: String,
// }

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
    ParseSql(SqlQuery),
}

#[derive(Debug, Args)]
pub struct SqlQuery {
    /// An sql string to parse and print
    #[arg(short, long)]
    pub query: Option<String>,
}
