use colorize::AnsiColor;
use types::config::Config;

mod check_sqlite;
pub mod my_json;
pub mod my_toml;
pub mod output;
pub mod types;

fn read_config() -> Config {
    let config: Config = my_toml::use_static::read(
        std::env::args()
            .nth(1)
            .expect("Unable to fetch 1st environment variable")
            .as_str(),
    );

    config
}

pub fn main() {
    let config: Config = read_config();
    let host = config.redis.unwrap().host.clone();

    let mut db = output::redis::DBRedis::new(&host)
        .expect("Error while initializing redis.".redb().as_str());

    let _ = db.set("key1", 32123 as i32); // Add type annotation for the value being set

    let op = db.get::<i32>("key1").unwrap();

    dbg!(op);
}
