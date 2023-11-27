use clap::{Args, ValueHint, ValueEnum};

#[derive(Debug, Args)]
pub struct Transaction {
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
pub enum TagsChoice {
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

pub fn add_data(transaction :Transaction) {
    println!(
        "Name: {}\nAmount: {}\nLifetime: {}\nStart Date: {}\nTag: {:?}",
        transaction.name, transaction.amount, transaction.lifetime, transaction.startdate, transaction.tags,
    );
}