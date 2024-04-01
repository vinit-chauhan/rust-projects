pub mod my_json;
pub mod my_toml;
pub mod output;
pub mod types;

use std::time::SystemTime;

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

    db.add_prod("Category", "some item 1")?;
    let now = SystemTime::now();

    db.add_sale(1, now.elapsed().unwrap().as_secs(), 1.2, "some unit")?;

    Ok(())
}
