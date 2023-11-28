use clap::{Parser, Subcommand};
use rusqlite::Connection;

mod add;
mod database;

use self::add::{add_data, Transaction};
use self::database::{manage_db, Database};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Parser)]
#[command(author = "Mislav M. <mislav.mat@hotmail.com>", version = VERSION)]
#[command(about = "Welcome from Budget CLI, your go-to command-line interface for personal finance management", long_about = None)]
pub struct Bud {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add new transaction
    Add(Transaction),
    
    //Print summary
    #[command(visible_alias = "view")]
    Sum,

    /// Manage database operations
    #[command(subcommand)]
    Database(Database),
}

impl Bud {
    pub fn exec(self, conn: Connection) -> color_eyre::Result<()> {
        match self.command {
            Some(Commands::Add(transaction)) => add_data(transaction, conn),
            Some(Commands::Sum) => {
                println!("The value of my-arg is {:?}", self.command)
            }
            Some(Commands::Database(database)) => manage_db(database, conn),
            None => {
                println!("Please provide a command or use `--help`")
            }
        }
        Ok(())
    }
}