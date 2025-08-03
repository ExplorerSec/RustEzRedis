// src/command/mod.rs

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::protocol::GeneralError;
use crate::protocol::RespValue;
use crate::storage::Database;
use crate::storage::Value;

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

pub trait CommandHandler {
    async fn handle(db: Arc<Mutex<Database>>, command: Command) -> RespValue;
}

impl Command {
    pub fn parse(resp_value: RespValue) -> Result<Self, Box<GeneralError>> {
        match resp_value {
            RespValue::Array(items) => {
                if items.is_empty() {
                    return Err("Empty command".into());
                }

                let name = match &items[0] {
                    RespValue::BulkString(Some(s)) => s.to_uppercase(),
                    _ => return Err("Invalid command name".into()),
                };

                let args = items[1..]
                    .iter()
                    .map(|item| match item {
                        RespValue::BulkString(Some(s)) => s.clone(),
                        _ => String::new(),
                    })
                    .collect();

                Ok(Command { name, args })
            }
            _ => Err("Invalid command format".into()),
        }
    }
}

impl CommandHandler for Command {
    async fn handle(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        let mut db_guard = db.lock().await;

        match command.name.as_str() {
            "PING" => {
                if command.args.is_empty() {
                    RespValue::SimpleString("PONG".to_string())
                } else {
                    RespValue::BulkString(Some(command.args[0].clone()))
                }
            }
            "SET" => match command.args.len() {
                2 => {
                    db_guard.set(
                        command.args[0].clone(),
                        Value::String(command.args[1].clone()),
                    );
                    RespValue::SimpleString("OK".to_string())
                }
                3 => {
                    todo!()
                    // db_guard.set_with_duration(key, value, duration);
                    // RespValue::SimpleString("OK".to_string())
                }
                _ => RespValue::Error("ERR wrong number of arguments".to_string()),
            },
            "GET" => {
                if command.args.len() == 1 {
                    match db_guard.get(&command.args[0]) {
                        Some(value) => RespValue::BulkString(Some(value.to_string())),
                        None => RespValue::Null,
                    }
                } else {
                    RespValue::Error("ERR wrong number of arguments".to_string())
                }
            }
            "DEL" => {
                match command.args.len() {
                    0 => {
                        RespValue::Error("ERR wrong number of arguments".to_string())
                    },
                    1 => {
                        db_guard.del(&command.args[0]);
                        RespValue::SimpleString("OK".to_string())
                    },
                    _ => {
                        let mut num = 0;
                        for k in command.args {
                            if db_guard.del(&k).is_some() {
                                num += 1;
                            }
                        }
                        RespValue::Integer(num)
                    },
                }

            }
            "EXISTS" => {
                if command.args.len() >= 1 {
                    let mut num = 0;
                    for k in command.args {
                        if db_guard.exists(&k) {
                            num += 1;
                        }
                    }
                    RespValue::Integer(num)
                } else {
                    RespValue::Error("ERR wrong number of arguments".to_string())
                }
            }
            _ => RespValue::Error(format!("ERR unknown command '{}'", command.name)),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::protocol;

    use super::*;
    #[test]
    fn f1() {
        let name = RespValue::BulkString(Some("SET".into()));
        let arg1 = RespValue::BulkString(Some("key1".into()));
        let arg2 = RespValue::BulkString(Some("val1".into()));
        let resp = RespValue::Array(vec![name, arg1, arg2]);

        let cmd = Command::parse(resp.clone());
        let hex = protocol::RespParser::serializer(resp);
        println!("{:?}", cmd);
        println!("{:?}", String::from_utf8(hex));
    }
}
