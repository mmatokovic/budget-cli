use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod add;
mod view;

use self::add::{add_data, Transaction};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
#[command(arg_required_else_help(true), subcommand_required(true))]
pub struct Bud {   
    /// Sets a custom config file
    #[arg(short, long, value_name = "file")]
    config: Option<PathBuf>,

    /// Optional name to operate on
    operation: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,

    /// Insted of stdout, write output to this file
    #[arg(long, value_name = "file", help_heading = "GLOBAL OPTIONS", global = true)]
    output_file: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add new transaction
    Add(Transaction),
       
    /// Print summary
    #[command(subcommand)]
    View,
}

impl Bud {
    pub fn exec(self) -> color_eyre::Result<()> {
        if let Some(config_path) = self.config.as_deref() {
            println!("Value for config: {}", config_path.display());
        }

        if let Some(action) = self.operation.as_deref() {
            println!("Pass command for operation: {action}");
        }

        match self.command {
            Some(Commands::Add(transaction)) => add_data(transaction),
            Some(Commands::View) => println!("The value of my-arg is {:?}", self.command),
            _ => unreachable!(),
        }
        Ok(())
    }
}