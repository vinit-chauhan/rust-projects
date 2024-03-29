use toml::{self, Value};

pub fn read(path: &str) {
    let text: String = std::fs::read_to_string(path).expect("Unable to read file");
    let config: Value = text.parse::<toml::Value>().unwrap();

    println!(
        "[postgresql].database = {}",
        config
            .get("postgresql")
            .expect("Unable to fetch 'postgresql' in config")
            .get("database")
            .expect("Unable to fetch 'database' in config")
            .as_str()
            .expect("Unable to convert to str")
    )
}
