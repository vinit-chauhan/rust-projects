use colorize::AnsiColor;
use rusqlite::{params, Connection, Error};

pub struct DbSQLite {
    file: String,
    pub conn: Option<Connection>,
}

impl DbSQLite {
    pub fn new(file: String) -> DbSQLite {
        DbSQLite { file, conn: None }
    }

    pub fn get_conn(&mut self) -> Result<(), Error> {
        if self.conn.is_none() {
            let conn: Connection = Connection::open(&self.file)?;
            self.conn = Some(conn);
        }

        Ok(())
    }

    pub fn create_db(&self) -> Result<(), Error> {
        let _ = self
            .conn
            .as_ref()
            .unwrap()
            .execute("DROP TABLE Sales;", params![]);

        let _ = self
            .conn
            .as_ref()
            .unwrap()
            .execute("DROP TABLE Products;", params![]);

        match self.conn.as_ref().unwrap().execute(
            "CREATE TABLE Products (
            id INTEGER PRIMARY KEY,
            category TEXT NOT NULL,
            name TEXT NOT NULL UNIQUE)",
            params![],
        ) {
            Ok(_) => println!("{}", "Product table Created".green()),
            Err(err) => println!("creation failed: {}", err),
        };

        match self.conn.as_ref().unwrap().execute(
            "CREATE TABLE Sales (
            id INTEGER PRIMARY KEY,
            product_id INTEGER NOT NULL REFERENCES Products,
            date BIGINT NOT NULL,
            quantity DOUBLE PRECISION NOT NULL,
            unit TEXT NOT NULL)",
            params![],
        ) {
            Ok(_) => println!("{}", "Sales table Created".green()),
            Err(err) => println!("creation failed: {}", err),
        };

        Ok(())
    }

    pub fn add_sale(&self) -> Result<(), Error> {
        // self.conn.as_ref().unwrap().execute("
        // INSERT INTO Sales VALUES ($1, $2, $3)", params![]);
        todo!()
    }

    pub fn add_prod(&self) -> Result<(), Error> {
        self.conn.as_ref().unwrap().execute(
            "
        INSERT INTO Products VALUES ($1, $2, $3)",
            params![1, "Cat 1", "Product 1"],
        )?;
        Ok(())
    }
}
