use crate::types::config::*;
use colorize::AnsiColor;

pub fn read(path: &str) -> Config {
    let config: Config = {
        let config_string: String =
            std::fs::read_to_string(path).expect("Unable to read config file".red().as_str());

        toml::from_str(&config_string).expect("Unable to Deserialize the config")
    };

    config
}

pub fn write(path: &str, formatted: bool) {
    let config: Config = Config {
        input: Input {
            json_file: Some(String::from("../data/sales_new.json")),
            xml_file: None,
        },
        redis: Option::Some(Redis {
            host: String::from("localhost"),
        }),
        sqlite: Option::Some(SQLite {
            db_file: String::from("../data/sales_new.db"),
        }),
        postgresql: Option::Some(PostgreSQL {
            username: String::from("admin"),
            password: String::from("Admin@123"),
            host: String::from("localhost"),
            port: String::from("5432"),
            database: String::from("new-rust-db"),
        }),
    };

    let config_string: String = if formatted {
        toml::to_string_pretty(&config)
    } else {
        toml::to_string(&config)
    }
    .expect("Unable to serialize TOML struct".red().as_str());

    std::fs::write(&path, config_string).expect("Unable to write to config file".red().as_str());

    println!("{}", "Done!".green())
}
