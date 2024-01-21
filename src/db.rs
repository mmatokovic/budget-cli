use rusqlite::{Connection, Result};

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let mut db = Database { connection: Connection::open(path)? };
        
        db.init()?;

        Ok(db)
    }

    fn init(&mut self) -> Result<()> {
        const CREATE_TABLE_TRANSACTIONS_QUERY: &str = "CREATE TABLE IF NOT EXISTS transactions (\
            id INTEGER NOT NULL PRIMARY KEY, \
            operation TEXT NOT NULL, \
            name TEXT NOT NULL, \
            amount REAL NOT NULL, \
            start_date TEXT NOT NULL, \
            end_date TEXT NOT NULL, \
            tags TEXT \
        )";

        self.connection.execute(CREATE_TABLE_TRANSACTIONS_QUERY, [])?;

        Ok(())
    }
}
