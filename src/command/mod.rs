// src/command/mod.rs
mod handle_macro;
use handle_macro::*;
mod handle_sys;
use handle_sys::HandleSys;
mod handle_string;
use handle_string::HandleString;
mod handle_hash;
use handle_hash::HandleHash;
mod handle_list;
use handle_list::HandleList;
mod handle_set;
use handle_set::HandleSet;

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

    pub async fn handle(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        match command.name.as_str() {
            // System
            "PING" => HandleSys::handle_ping(command),
            "ECHO" => HandleSys::handle_echo(command),
            "CLEAN" => HandleSys::handle_clean(db, command).await,
            // String
            "SET" => HandleString::handle_set(db, command).await,
            "GET" => HandleString::handle_get(db, command).await,
            "DEL" => HandleString::handle_del(db, command).await,
            "EXISTS" => HandleString::handle_exists(db, command).await,
            "INCR" => HandleString::handle_incr(db, command).await,
            "DECR" => HandleString::handle_decr(db, command).await,
            // Hash
            "HSET" => HandleHash::handle_hset(db, command).await,
            "HGET" => HandleHash::handle_hget(db, command).await,
            "HGETALL" => HandleHash::handle_hgetall(db, command).await,
            "HDEL" => HandleHash::handle_hdel(db, command).await,
            "HEXISTS" => HandleHash::handle_hexists(db, command).await,
            "HLEN" => HandleHash::handle_hlen(db, command).await,
            "HKEYS" => HandleHash::handle_hkeys(db, command).await,
            "HVALS" => HandleHash::handle_hvals(db, command).await,
            // List
            "LPUSH" => HandleList::handle_lpush(db, command).await,
            "RPUSH" => HandleList::handle_rpush(db, command).await,
            "LPOP" => HandleList::handle_lpop(db, command).await,
            "RPOP" => HandleList::handle_rpop(db, command).await,
            "LLEN" => HandleList::handle_llen(db, command).await,
            "LINDEX" => HandleList::handle_lindex(db, command).await,
            "LSET" => HandleList::handle_lset(db, command).await,
            "LRANGE" => HandleList::handle_lrange(db, command).await,
            // Set
            "SADD" => HandleSet::handle_sadd(db, command).await,
            "SCARD" => HandleSet::handle_scard(db, command).await,
            "SMEMBERS" => HandleSet::handle_smembers(db, command).await,
            "SREM" => HandleSet::handle_srem(db, command).await,
            "SISMEMBER" => HandleSet::handle_sismember(db, command).await,
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
