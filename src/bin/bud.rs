use clap::Parser;
use bud::{Bud, config::Settings};
use rusqlite::Connection;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let config: Settings = Settings::new()?;
    println!("{:?}", config);

    // Now you can access the database path
    let conn = Connection::open(config.database.path).expect("Failed to connect to database.");

    let cli: Bud = Bud::parse();

    cli.exec(conn)
}