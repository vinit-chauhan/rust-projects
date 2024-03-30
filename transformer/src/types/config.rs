use serde_derive::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Deserialize, Serialize)]
pub struct Input {
    pub xml_file: Option<String>,
    pub json_file: Option<String>,
}

impl Input {
    pub fn validate(&self) -> Result<(), &'static str> {
        if (self.xml_file.is_none() && self.json_file.is_none())
            || (self.xml_file.is_some() && self.json_file.is_some())
        {
            Err("Only one input is allowed at a time")
        } else {
            Ok(())
        }
    }
}

#[allow(unused)]
#[derive(Deserialize, Serialize)]
pub struct Redis {
    pub host: String,
}

#[allow(unused)]
#[derive(Deserialize, Serialize)]
pub struct SQLite {
    pub db_file: String,
}

#[allow(unused)]
#[derive(Deserialize, Serialize)]
pub struct PostgreSQL {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub database: String,
}

#[allow(unused)]
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub input: Input,
    pub redis: Option<Redis>,
    pub sqlite: Option<SQLite>,
    pub postgresql: Option<PostgreSQL>,
}
