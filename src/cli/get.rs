use clap::{Args, ValueEnum};

#[derive(Debug, Args)]
pub struct Filter {
    // id of Transiction
    id: u32,

    /// Descriptive name
    name: String,

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

/// OperationChoice
#[derive(Debug, Clone, ValueEnum)]
pub enum OperationChoice {
    Expense,
    Income,
}