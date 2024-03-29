use colorize::AnsiColor;
use serde_derive::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Deserialize, Serialize)]
struct Input {
    xml_file: String,
    json_file: String,
}

#[allow(unused)]
#[derive(Deserialize, Serialize)]
struct Redis {
    host: String,
}

#[allow(unused)]
#[derive(Deserialize, Serialize)]
struct SQLite {
    db_file: String,
}

#[allow(unused)]
#[derive(Deserialize, Serialize)]
struct PostgreSQL {
    username: String,
    password: String,
    host: String,
    port: String,
    database: String,
}

#[allow(unused)]
#[derive(Deserialize, Serialize)]
struct Config {
    input: Input,
    redis: Redis,
    sqlite: SQLite,
    postgresql: PostgreSQL,
}

pub fn read(path: &str) {
    let config: Config = {
        let config_string: String =
            std::fs::read_to_string(path).expect("Unable to read config file".red().as_str());
        toml::from_str(&config_string).unwrap()
    };

    println!("[postgresql].database: {}", config.postgresql.database)
}

pub fn write(path: &str, formatted: bool) {
    let config: Config = Config {
        input: Input {
            xml_file: String::from("../data/sales_new.xml"),
            json_file: String::from("../data/sales_new.json"),
        },
        redis: Redis {
            host: String::from("localhost"),
        },
        sqlite: SQLite {
            db_file: String::from("../data/sales_new.db"),
        },
        postgresql: PostgreSQL {
            username: String::from("admin"),
            password: String::from("Admin@123"),
            host: String::from("localhost"),
            port: String::from("5432"),
            database: String::from("new-rust-db"),
        },
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
