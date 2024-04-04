use chrono;
use colorize::AnsiColor;
use rusqlite::{params, Connection, Result};

use crate::types::sales_and_products::{Product, Sale, SalesWithProduct};
use chrono::prelude::*;

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

    pub fn add_sale(&self, prod: u32, date: u128, quantity: f64, unit: &str) -> Result<()> {
        self.conn.as_ref().unwrap().execute(
            "
        INSERT INTO Sales (PRODUCT_ID, DATE, QUANTITY, UNIT) VALUES ($1, $2, $3, $4)",
            params![prod, date as i64, quantity, unit],
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

    pub fn get_sale_by_id(&self, id: u32) -> Result<Sale> {
        let mut command = self
            .conn
            .as_ref()
            .unwrap()
            .prepare("SELECT ID, PRODUCT_ID, DATE, QUANTITY, UNIT FROM Sales WHERE ID = ?1")?;

        let sale = command.query_row([id], |row| {
            Ok(Sale {
                id: row.get(0)?,
                product_id: row.get(1)?,
                date: row.get(2)?,
                quantity: row.get::<_, f64>(3)?,
                unit: row.get(4)?,
            })
        })?;

        Ok(sale)
    }

    pub fn get_prod_by_id(&self, id: u32) -> Result<Product> {
        let mut command = self
            .conn
            .as_ref()
            .unwrap()
            .prepare("SELECT ID, NAME, CATEGORY FROM Products WHERE ID = (?1)")?;

        let prod = command.query_row([id], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
            })
        })?;

        Ok(prod)
    }

    pub fn print_db(&self) -> Result<()> {
        let mut command = self.conn.as_ref().unwrap().prepare(
            "SELECT s.DATE, s.UNIT, s.QUANTITY, p.NAME, p.CATEGORY from Sales s
            JOIN Products p 
            ON p.ID = s.PRODUCT_ID
            ORDER BY s.DATE",
        )?;

        for sale_and_product in command.query_map(params![], |row| {
            // in the closure
            Ok(SalesWithProduct {
                sale_date: row.get(0)?,
                sale_unit: row.get(1)?,
                sale_quantity: row.get::<_, f64>(2)?,
                prod_name: row.get(3)?,
                prod_category: row.get(4)?,
            })
        })? {
            // in for loop
            println!("{}", "Reading the database...".green());
            let item = sale_and_product.unwrap();

            let timestamp = Utc.timestamp_micros(item.sale_date);

            let date = timestamp.unwrap().format("%Y-%m-%d %H:%M:%S");

            println!(
                "{} ({}) x{} were sold on {}",
                item.prod_name, item.prod_category, item.sale_quantity, date
            );
        }

        Ok(())
    }
}
