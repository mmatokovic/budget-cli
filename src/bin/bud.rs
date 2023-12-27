use clap::Parser;
use bud::{Bud, Settings, Database};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let config: Settings = Settings::config()?;

    let database: Database = Database::new(&config.database.path)?;
    
    let cli: Bud = Bud::parse();
   
    cli.exec()
}