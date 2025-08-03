// src/command/mod.rs

use std::fmt::format;
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
            "SET" => {
                if command.args.len() >= 2 {
                    db_guard.set(
                        command.args[0].clone(),
                        Value::String(command.args[1].clone()),
                    );
                    RespValue::SimpleString("OK".to_string())
                } else {
                    RespValue::Error("ERR wrong number of arguments".to_string())
                }
            }
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
                if command.args.len() >= 1 {
                    // let mut removed = Vec::new();
                    let mut not_exist = Vec::new();
                    for k in command.args {
                        if db_guard.del(&k).is_none() {
                            not_exist.push(k);
                        }
                    }
                    if not_exist.is_empty() {
                        RespValue::SimpleString("OK".to_string())
                    } else {
                        RespValue::SimpleString(format!("OK, ignored:{}", not_exist.join(",")))
                    }
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
    use super::*;
    #[test]
    fn f1() {
        let name = RespValue::BulkString(Some("SET".into()));
        let arg1 = RespValue::BulkString(Some("key1".into()));
        let arg2 = RespValue::BulkString(Some("val1".into()));
        let resp = RespValue::Array(vec![name, arg1, arg2]);

        let cmd = Command::parse(resp);
        println!("{:?}", cmd);
    }
}
