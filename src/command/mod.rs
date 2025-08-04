// src/command/mod.rs

mod handle_func;
use crate::protocol::GeneralError;
use crate::protocol::RespValue;

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
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
