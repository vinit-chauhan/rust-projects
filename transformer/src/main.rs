pub mod my_json;
pub mod my_toml;

use self::my_toml::use_static::*;

#[allow(unused)]
fn read_toml() {
    let config: Config = read(
        std::env::args()
            .nth(1)
            .expect("Unable to fetch 1st environment variable")
            .as_str(),
    );

    println!("{}", config.postgresql.database);
}

fn update_json(is_dynamic: bool) {
    if is_dynamic {
        my_json::use_dynamic::read_and_update(
            std::env::args()
                .nth(1)
                .expect("Unable to fetch 1st environment variable")
                .as_str(),
            std::env::args()
                .nth(2)
                .expect("Unable to fetch 2nd environment variable")
                .as_str(),
        )
    } else {
        my_json::use_static::read_and_update(
            std::env::args()
                .nth(1)
                .expect("Unable to fetch 1st environment variable")
                .as_str(),
            std::env::args()
                .nth(2)
                .expect("Unable to fetch 2nd environment variable")
                .as_str(),
        )
    }
}

fn main() {
    update_json(false);
}
