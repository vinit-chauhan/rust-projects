use colorize::AnsiColor;
use redis::{self, Commands, Connection, FromRedisValue, RedisError, RedisResult, ToRedisArgs};

pub struct DBRedis {
    conn: Connection,
    pub host: String,
}

impl DBRedis {
    pub fn new(host: &str) -> RedisResult<DBRedis> {
        let mut host = host.to_owned();
        if host.starts_with("redis://") == false {
            host = format!("redis://{}", host);
        }

        match redis::Client::open(host.as_ref()) {
            Ok(instance) => {
                let conn = match instance.get_connection() {
                    Ok(conn) => Some(conn),
                    Err(err) => {
                        panic!("{}: {}", "Unable to connect to instance".redb(), err);
                    }
                };

                Ok(DBRedis {
                    host,
                    conn: conn.unwrap(),
                })
            }
            Err(err) => {
                println!("{} {}", "Unable to connect to host:".redb(), &host);
                Err(err)
            }
        }
    }

    pub fn get<T: FromRedisValue>(&mut self, key: &str) -> Result<T, RedisError> {
        self.conn.get(key)
    }

    pub fn set<RA: ToRedisArgs>(&mut self, key: &str, val: RA) -> RedisResult<()> {
        self.conn.set(key, val)
    }
}
