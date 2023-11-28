use clap::{Args, ValueEnum};
use time::Date;
use rusqlite::{Connection, params};
use std::fmt;

use crate::utils::{parse_date, default_date, parse_float};

#[derive(Debug, Args)]
pub struct Transaction {
    /// Descriptive name
    name: String,

    /// The amount in monetary currency
    #[arg(value_parser = parse_float)]
    amount: f64,

    /// The start date since when the lifetime should be computed
    #[arg(value_parser = parse_date, default_value_t = default_date())]
    startdate: Date,

    /// Duration that the transaction applies to
    #[arg(default_value = "1d")]
    lifetime: String,
    
    /// For grouping transactions
    #[arg(short)]
    tags: Vec<String>,

    // Operation income/expense
    #[arg(short, default_value = "expense")]
    operation: OperationChoice
}

/// Doc comment
#[derive(Debug, Clone, ValueEnum)]
pub enum OperationChoice {
    Expense,
    Income,
}

impl fmt::Display for OperationChoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperationChoice::Expense => write!(f, "expense"),
            OperationChoice::Income => write!(f, "income"),
        }
    }
}


pub fn add_data(transaction :Transaction, conn: Connection) {
    println!("{:?}", transaction);
    
    const INSERT_QUERY: &str = "INSERT INTO transactions (operation, name, amount, start_date, end_date, tags) VALUES (?, ?, ?, ?, ?, ?)";
    
    let tags_str = transaction.tags.join(", ");
    conn.execute(
        INSERT_QUERY,
        params![
            transaction.operation.to_string(),
            transaction.name,
            transaction.amount,
            transaction.startdate.to_string(),
            transaction.lifetime.to_string(),
            tags_str,
        ],
    )
    .unwrap();
    println!("Transaction added successfully!");
}