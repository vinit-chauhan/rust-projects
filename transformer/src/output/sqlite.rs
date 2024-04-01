use colorize::AnsiColor;
use rusqlite::{params, Connection, Result};

pub struct DbSQLite {
    file: String,
    pub conn: Option<Connection>,
}

impl DbSQLite {
    pub fn new(file: String) -> DbSQLite {
        DbSQLite { file, conn: None }
    }

    pub fn get_conn(&mut self) -> Result<()> {
        if self.conn.is_none() {
            let conn: Connection = Connection::open(&self.file)?;
            self.conn = Some(conn);
        }

        Ok(())
    }

    pub fn cleanup(&self) -> Result<()> {
        self.conn
            .as_ref()
            .unwrap()
            .execute("DROP TABLE Sales;", params![])?;

        self.conn
            .as_ref()
            .unwrap()
            .execute("DROP TABLE Products;", params![])?;

        Ok(())
    }

    pub fn create_db(&self) -> Result<()> {
        match self.conn.as_ref().unwrap().execute(
            "CREATE TABLE Products (
            ID INTEGER PRIMARY KEY,
            CATEGORY TEXT NOT NULL,
            NAME TEXT NOT NULL UNIQUE)",
            params![],
        ) {
            Ok(_) => println!("{}", "Product table Created".green()),
            Err(err) => println!("creation failed: {:?}", err),
        };

        match self.conn.as_ref().unwrap().execute(
            "CREATE TABLE Sales (
            ID INTEGER PRIMARY KEY,
            PRODUCT_ID INTEGER NOT NULL REFERENCES Products,
            DATE BIGINT NOT NULL,
            QUANTITY DOUBLE PRECISION NOT NULL,
            UNIT TEXT NOT NULL)",
            params![],
        ) {
            Ok(_) => println!("{}", "Sales table Created".green()),
            Err(err) => println!("creation failed: {}", err),
        };

        Ok(())
    }

    pub fn add_sale(&self, prod: u32, date: u64, quantity: f64, unit: &str) -> Result<()> {
        self.conn.as_ref().unwrap().execute(
            "
        INSERT INTO Sales (PRODUCT_ID, DATE, QUANTITY, UNIT) VALUES ($1, $2, $3, $4)",
            params![prod, date, quantity, unit],
        )?;

        Ok(())
    }

    pub fn add_prod(&self, category: &str, name: &str) -> Result<()> {
        self.conn.as_ref().unwrap().execute(
            "
        INSERT INTO Products (CATEGORY, NAME) VALUES ($1, $2)",
            params![category, name],
        )?;
        Ok(())
    }
}
