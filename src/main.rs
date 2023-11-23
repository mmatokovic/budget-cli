mod utils;

use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

use clap::{Parser, Subcommand, Args, ValueEnum, ValueHint};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(author = "Mislav M. <mislav.mat@hotmail.com>", version = VERSION, about = "Welcome from Budget CLI, your go-to command-line interface for personal finance management", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add new transaction
    Add(Transaction),
    
    //Print summary
    #[command(visible_alias = "view")]
    Sum
}

#[derive(Debug, Args)]
struct Transaction {
    /// Descriptive name
    name: String,

    /// The amount in monetary currency
    amount: String,

    /// The start date since when the lifetime should be computed
    startdate: String,

    /// Duration that the transaction applies to
    lifetime: String,
    
    /// For grouping transactions
    #[arg(value_hint = ValueHint::CommandWithArguments)]
    tags: TagsChoice,
}

/// Doc comment
#[derive(Debug, Clone, ValueEnum)]
enum TagsChoice {
    Subscription,
    Grocerie,
    Utilitie,
    Transportation,
    Personal,
    Entertainment,
    Shopping,
    Loan,
    Cash,
    Rent,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add(transaction) => {
            println!(
                "Name: {}\nAmount: {}\nLifetime: {}\nStart Date: {}\nTag: {:?}",
                transaction.name, transaction.amount, transaction.lifetime, transaction.startdate, transaction.tags,
            );

            // Save the transaction to the file
            if let Err(err) = save_transaction(&transaction) {
                eprintln!("Error saving transaction: {}", err);
            }
        }
        Commands::Sum => println!("Printing summary"),
    }
}

fn save_transaction(transaction: &Transaction) -> io::Result<()> {
    let path = Path::new("data").join("budget.data.csv");
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    writeln!(
        &mut file,
        "{},{},{},{}",
        transaction.name, transaction.amount, transaction.lifetime, transaction.startdate
    )?;

    Ok(())
}