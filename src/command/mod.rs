// src/command/mod.rs
use crate::storage::Database;
use crate::protocol::RespValue;

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

pub trait CommandHandler {
    fn handle(&self, db: &mut Database, command: Command) -> RespValue;
}

impl Command {
    pub fn parse(resp_value: RespValue) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        match resp_value {
            RespValue::Array(items) => {
                if items.is_empty() {
                    return Err("Empty command".into());
                }
                
                let name = match &items[0] {
                    RespValue::BulkString(Some(s)) => s.to_uppercase(),
                    _ => return Err("Invalid command name".into()),
                };
                
                let args = items[1..].iter().map(|item| {
                    match item {
                        RespValue::BulkString(Some(s)) => s.clone(),
                        _ => String::new(),
                    }
                }).collect();
                
                Ok(Command { name, args })
            }
            _ => Err("Invalid command format".into()),
        }
    }
}
