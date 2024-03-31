pub mod my_json;
pub mod my_toml;
pub mod output;
pub mod types;

use colorize::AnsiColor;
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

fn main() {
    let config: Config = read_config();
    let db_file = config.sqlite.unwrap().db_file.clone();

    let mut db = DbSQLite::new(db_file.to_string());

    db.get_conn().expect("Error creating db".greenb().as_str());

    println!("Fetching SQlite db: {}", &db_file);
    match db.create_db() {
        Ok(()) => println!("{}", "Database created successfully.".green()),
        Err(err) => {
            println!("{}", err.sqlite_error().unwrap().to_string().redb());
            return;
        }
    };

    db.add_prod().expect("Unable to add data.");
}
