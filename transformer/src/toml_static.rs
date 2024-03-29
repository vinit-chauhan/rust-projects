use serde_derive::Deserialize;

#[allow(unused)]
#[derive(Deserialize)]
struct Input {
    xml_file: String,
    json_file: String,
}

#[allow(unused)]
#[derive(Deserialize)]
struct Redis {
    host: String,
}

#[allow(unused)]
#[derive(Deserialize)]
struct SQLite {
    db_file: String,
}

#[allow(unused)]
#[derive(Deserialize)]
struct PostgreSQL {
    username: String,
    password: String,
    host: String,
    port: String,
    database: String,
}

#[allow(unused)]
#[derive(Deserialize)]
struct Config {
    input: Input,
    redis: Redis,
    sqlite: SQLite,
    postgresql: PostgreSQL,
}

pub fn read(path: &str) {
    let config: Config = {
        let config_string: String = std::fs::read_to_string(path).unwrap();
        toml::from_str(&config_string).unwrap()
    };

    println!("[postgresql].database: {}", config.postgresql.database)
}
