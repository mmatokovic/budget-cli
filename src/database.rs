use rusqlite::{Connection, Result, params};

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;

        const CREATE_TABLE_TRANSACTIONS_QUERY: &str = "CREATE TABLE IF NOT EXISTS transactions (\
            id INTEGER NOT NULL PRIMARY KEY, \
            operation TEXT NOT NULL, \
            name TEXT NOT NULL, \
            amount REAL NOT NULL, \
            start_date TEXT NOT NULL, \
            end_date TEXT NOT NULL, \
            tags TEXT \
        )";

        conn.execute(CREATE_TABLE_TRANSACTIONS_QUERY, [])?;
        Ok(Database { connection: conn })
    }

    pub fn save(&self, transaction: &Transaction) -> Result<()> {
        const INSERT_QUERY: &str = "INSERT INTO transactions (operation, name, amount, start_date, end_date, tags) VALUES (?, ?, ?, ?, ?, ?)";

        self.connection.execute(
            INSERT_QUERY,
            params![
                transaction.operation.to_string(),
                transaction.name,
                transaction.amount,
                transaction.startdate.to_string(),
                transaction.enddate.to_string(),
            ],
        )?;
        Ok(())
        println!("Transaction added successfully!");

    }
}
