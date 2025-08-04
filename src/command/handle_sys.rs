use crate::command::Command;
use crate::command::handle_macro::*;


pub struct HandleSys;
impl HandleSys{
    pub fn handle_ping(command: Command) -> RespValue {
        if command.args.is_empty() {
            RespValue::SimpleString("PONG".to_string())
        } else {
            RespValue::BulkString(Some(command.args[0].clone()))
        }
    }

    pub fn handle_echo(command: Command) -> RespValue{
        if command.args.len() ==1{
            RespValue::BulkString(Some(command.args[0].clone()))
        }else {
            RespErrArgNum!()
        }
    }
}
