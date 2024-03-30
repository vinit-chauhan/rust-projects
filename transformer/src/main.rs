pub mod my_json;
pub mod my_toml;
pub mod types;

use types::config::Config;

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
    println!("Fetching SQlite db: {}", config.sqlite.unwrap().db_file);
}
