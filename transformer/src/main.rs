pub mod my_json;
pub mod my_toml;
pub mod output;
pub mod types;

use std::time::{SystemTime, UNIX_EPOCH};

use colorize::AnsiColor;
use rusqlite::Result;
use types::config::Config;

use crate::output::sqlite::DbSQLite;

#[allow(unused)]
fn read_config() -> Config {
    let config: Config = my_toml::use_static::read(
        std::env::args()
            .nth(1)
            .expect("Unable to fetch 1st environment variable")
            .as_str(),
    );

    config
}

fn main() -> Result<()> {
    let config: Config = read_config();
    let db_file = config.sqlite.unwrap().db_file.clone();

    let mut db = DbSQLite::new(db_file.to_string());

    db.get_conn()?;
    db.cleanup()?;

    match db.create_db() {
        Ok(()) => println!("{}", "Database created successfully.".green()),
        Err(err) => {
            println!("{}", err.sqlite_error().unwrap().to_string().redb());
        }
    };

    db.add_prod("Category", "some item 1")
        .expect("Unable to add data to table Products".redb().as_str());

    db.add_sale(
        1,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros(),
        1.2,
        "some unit",
    )
    .expect("Unable to add data to table Sales".redb().as_str());

    // let sale = db.get_sale_by_id(1)?;
    // dbg!(&sale);

    // let prod = db.get_prod_by_id(1)?;
    // dbg!(&prod);

    db.print_db()?;
    Ok(())
}
