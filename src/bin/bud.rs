use clap::Parser;
use bud::{Bud, config::get_config};
use rusqlite::Connection;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let config = get_config().expect("Failed to read configuration.");

    let conn = Connection::open(config.database.path).expect("Failed to connect to database.");
    
    let cli: Bud = Bud::parse();

    cli.exec(conn)
}