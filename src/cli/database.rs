use clap::Subcommand;
use rusqlite::Connection;

const CREATE_DATABASE_QUERY: &str = "CREATE TABLE IF NOT EXISTS transactions (\
    id INTEGER NOT NULL PRIMARY KEY, \
    operation TEXT NOT NULL, \
    name TEXT NOT NULL, \
    amount REAL NOT NULL, \
    start_date TEXT NOT NULL, \
    end_date TEXT NOT NULL, \
    tags TEXT \
)";


const DROP_DATABASE_QUERY: &str = "DROP TABLE IF EXISTS transactions";

#[derive(Debug, Subcommand)]
pub enum Database {
    /// Create database tables
    Create,

    /// Reset database tables
    Reset,
}

pub fn manage_db(database: Database, conn: Connection) {
    match database {
        Database::Create => {
            conn.execute(CREATE_DATABASE_QUERY, ()).unwrap();
            println!("Created successfully!");
        }
        Database::Reset => {
            conn.execute(DROP_DATABASE_QUERY, ()).unwrap();
            conn.execute(CREATE_DATABASE_QUERY, ()).unwrap();
            println!("Reset successful!");
        }
    }
}