use chrono::{Timelike, Utc};
use colorize::AnsiColor;
use std::{fs::File, io::Write};

pub enum LogType {
    SUCCESS,
    FAILURE,
    INFO,
}

pub struct Logger {
    fd: File,
}

impl Logger {
    pub fn init(path: &str) -> Logger {
        let fd: File = File::create(path).expect("Unable to create log file");
        Logger { fd }
    }

    pub fn write_info(&mut self, msg: &str) {
        self.write(msg, LogType::INFO);
    }

    pub fn write(&mut self, msg: &str, log_type: LogType) {
        let now = Utc::now();
        let date = format!(
            "[ {} {}:{}:{} ] ",
            now.date_naive(),
            now.hour(),
            now.minute(),
            if now.second() < 10 {
                format!("0{}", now.second())
            } else {
                now.second().to_string()
            }
        );
        let mut buf: String = String::with_capacity(&msg.len() + &date.len() + 11);
        buf.push_str(&date);
        match log_type {
            LogType::INFO => buf.push_str("[INFO] "),
            LogType::FAILURE => buf.push_str("[FAIL] "),
            LogType::SUCCESS => buf.push_str("[SUCCESS] "),
        }
        buf.push_str(&msg);
        buf.push('\n');

        match self.fd.write_all(buf.as_bytes()) {
            Ok(_) => (),
            Err(_) => println!("{}", "Unable to write to logs!!!".redb()),
        }
    }
}
