pub mod my_toml;

use self::my_toml::use_static::*;

fn main() {
    let path: String = std::env::args()
        .nth(1)
        .expect("Unable to fetch 1st environment variable");

    // toml_dynamic::read(&path);
    // toml_static::write(&path, true)

    let config: Config = read(&path);

    println!("{}", config.postgresql.database)
}
