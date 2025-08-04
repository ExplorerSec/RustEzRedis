use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::command::Command;
use crate::protocol::RespValue;
use crate::storage::Database;
use crate::storage::Value;

pub struct HandleString;

impl HandleString {
    fn handle_ping(command: Command) -> RespValue {
        if command.args.is_empty() {
            RespValue::SimpleString("PONG".to_string())
        } else {
            RespValue::BulkString(Some(command.args[0].clone()))
        }
    }

    async fn handle_set(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        let mut db_guard = db.lock().await;
        match command.args.len() {
            2 => {
                db_guard.set(
                    command.args[0].clone(),
                    Value::String(command.args[1].clone()),
                );
                RespValue::SimpleString("OK".to_string())
            }
            4 => {
                let duration: Duration;
                match command.args[2].to_uppercase().as_str() {
                    "EX" => {
                        if let Ok(secs) = command.args[3].parse::<u64>() {
                            duration = Duration::from_secs(secs);
                        } else {
                            return RespValue::Error(
                                "ERR invalid expire time in 'set' command".to_string(),
                            );
                        }
                    }
                    "PX" => {
                        if let Ok(millis) = command.args[3].parse::<u64>() {
                            duration = Duration::from_millis(millis)
                        } else {
                            return RespValue::Error(
                                "ERR invalid expire time in 'set' command".to_string(),
                            );
                        }
                    }
                    _ => {
                        return RespValue::Error("ERR syntax error".to_string());
                    }
                }
                db_guard.set_with_duration(
                    command.args[0].clone(),
                    Value::String(command.args[1].clone()),
                    Some(duration),
                );
                RespValue::SimpleString("OK".to_string())
            }
            _ => RespValue::Error("ERR wrong number of arguments".to_string()),
        }
    }

    async fn handle_get(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        let db_guard = db.lock().await;
        if command.args.len() == 1 {
            match db_guard.get(&command.args[0]) {
                Some(Value::String(s)) => RespValue::BulkString(Some(s.to_string())),
                Some(_) => RespValue::Error(
                    "WRONGTYPE Operation against a key holding the wrong kind of value".to_string(),
                ),
                None => RespValue::Null,
            }
        } else {
            RespValue::Error("ERR wrong number of arguments".to_string())
        }
    }

    async fn handle_del(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        let mut db_guard = db.lock().await;
        match command.args.len() {
            0 => RespValue::Error("ERR wrong number of arguments".to_string()),
            1 => {
                db_guard.del(&command.args[0]);
                RespValue::SimpleString("OK".to_string())
            }
            _ => {
                let mut num = 0;
                for k in command.args {
                    if db_guard.del(&k).is_some() {
                        num += 1;
                    }
                }
                RespValue::Integer(num)
            }
        }
    }

    async fn handle_exists(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        let db_guard = db.lock().await;
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

    async fn handle_incr(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        let mut db_guard = db.lock().await;
        if command.args.len() == 1 {
            let key = command.args[0].clone();
            match db_guard.get(&key) {
                Some(val) => {
                    if let Value::String(s) = val {
                        if let Ok(mut n) = s.parse::<i64>() {
                            n = n + 1;
                            db_guard.set(key, Value::String(n.to_string()));
                            return RespValue::Integer(n);
                        }
                    }
                    RespValue::Error("ERR value is not an integer or out of range".to_string())
                }
                None => {
                    db_guard.set(key, Value::String("1".to_string()));
                    RespValue::Integer(1)
                }
            }
        } else {
            RespValue::Error("ERR wrong number of arguments for 'incr' command".to_string())
        }
    }

    async fn handle_decr(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        let mut db_guard = db.lock().await;
        if command.args.len() == 1 {
            let key = command.args[0].clone();
            match db_guard.get(&key) {
                Some(val) => {
                    if let Value::String(s) = val {
                        if let Ok(mut n) = s.parse::<i64>() {
                            n = n - 1;
                            db_guard.set(key, Value::String(n.to_string()));
                            return RespValue::Integer(n);
                        }
                    }
                    RespValue::Error("ERR value is not an integer or out of range".to_string())
                }
                None => {
                    db_guard.set(key, Value::String("-1".to_string()));
                    RespValue::Integer(-1)
                }
            }
        } else {
            RespValue::Error("ERR wrong number of arguments for 'incr' command".to_string())
        }
    }
}

struct HandleHash;
impl HandleHash {
    async fn handle_hset(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        let len = command.args.len();
        if len % 2 == 1 && len != 1 {
            let key = command.args[0].clone();
            let mut db_guard = db.lock().await;
            match db_guard.get(&key) {
                None | Some(Value::Hash(_)) => {
                    // 新建或覆盖原有表
                    let mut hashmap = HashMap::new();
                    for i in (1..len).step_by(2) {
                        hashmap.insert(command.args[i].clone(), command.args[i + 1].clone());
                    }
                    db_guard.set(key, Value::Hash(hashmap));
                    RespValue::Integer((len as i64) / 2)
                }
                _ => {
                    // 被其他类型占据，直接返回错误
                    RespValue::Error(
                        "WRONGTYPE Operation against a key holding the wrong kind of value"
                            .to_string(),
                    )
                }
            }
        } else {
            RespValue::Error("ERR wrong number of arguments for 'hset' command".to_string())
        }
    }

    async fn handle_hget(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() == 2 {
            let db_guard = db.lock().await;
            let map = db_guard.get(&command.args[0]);
            match map {
                Some(Value::Hash(map)) => {
                    let v = map.get(&command.args[1]);
                    if let Some(v) = v {
                        RespValue::BulkString(Some(v.clone()))
                    } else {
                        RespValue::Null
                    }
                }
                Some(_) => RespValue::Error(
                    "WRONGTYPE Operation against a key holding the wrong kind of value".to_string(),
                ),
                _ => RespValue::Null,
            }
        } else {
            RespValue::Error("ERR wrong number of arguments for 'hget' command".to_string())
        }
    }

    async fn handle_hgetall(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() == 1 {
            let db_guard = db.lock().await;
            match db_guard.get(&command.args[0]) {
                None => RespValue::Array(Vec::new()),
                Some(Value::Hash(hashmap)) => {
                    let mut vec: Vec<RespValue> = Vec::with_capacity(hashmap.len() * 2);
                    for (k, v) in hashmap {
                        vec.extend([
                            RespValue::BulkString(Some(k.clone())),
                            RespValue::BulkString(Some(v.clone())),
                        ]);
                    }
                    RespValue::Array(vec)
                }
                _ => RespValue::Error(
                    "WRONGTYPE Operation against a key holding the wrong kind of value".to_string(),
                ),
            }
        } else {
            RespValue::Error("ERR wrong number of arguments for 'hgetall' command".to_string())
        }
    }

    async fn handle_hdel(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() >= 2 {
            let mut db_guard = db.lock().await;
            match db_guard.get_mut(&command.args[0]) {
                Some(Value::Hash(hashmap)) => {
                    let mut num = 0;
                    for k in command.args.iter().skip(1) {
                        if hashmap.remove(k).is_some() {
                            num += 1;
                        }
                    }
                    RespValue::Integer(num)
                }
                Some(_) => RespValue::Error(
                    "WRONGTYPE Operation against a key holding the wrong kind of value".to_string(),
                ),
                None => RespValue::Integer(0),
            }
        } else {
            RespValue::Error("ERR wrong number of arguments for 'hdel' command".to_string())
        }
    }

    async fn handle_hexists(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() == 2 {
            let db_guard = db.lock().await;
            match db_guard.get(&command.args[0]) {
                Some(Value::Hash(hashmap)) => {
                    let num = hashmap.contains_key(&command.args[1]) as i64;
                    RespValue::Integer(num)
                }
                Some(_) => RespValue::Error(
                    "WRONGTYPE Operation against a key holding the wrong kind of value".to_string(),
                ),
                None => RespValue::Integer(0),
            }
        } else {
            RespValue::Error("ERR wrong number of arguments for 'hexists' command".to_string())
        }
    }

    async fn handle_hlen(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() == 1 {
            let db_guard = db.lock().await;
            match db_guard.get(&command.args[0]) {
                Some(Value::Hash(hashmap)) => RespValue::Integer(hashmap.len() as i64),
                Some(_) => RespValue::Error(
                    "WRONGTYPE Operation against a key holding the wrong kind of value".to_string(),
                ),
                None => RespValue::Integer(0),
            }
        } else {
            RespValue::Error("ERR wrong number of arguments for 'hlen' command".to_string())
        }
    }

    async fn handle_hkeys(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() == 1 {
            let db_guard = db.lock().await;
            match db_guard.get(&command.args[0]) {
                Some(Value::Hash(hashmap)) => {
                    let vec = hashmap
                        .keys()
                        .map(|k| RespValue::BulkString(Some(k.clone())))
                        .collect::<Vec<_>>();
                    RespValue::Array(vec)
                }
                Some(_) => RespValue::Error(
                    "WRONGTYPE Operation against a key holding the wrong kind of value".to_string(),
                ),
                None => RespValue::Array(Vec::new()),
            }
        } else {
            RespValue::Error("ERR wrong number of arguments for 'hkeys' command".to_string())
        }
    }

    async fn handle_hvals(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() == 1 {
            let db_guard = db.lock().await;
            match db_guard.get(&command.args[0]) {
                Some(Value::Hash(hashmap)) => {
                    let vec = hashmap
                        .values()
                        .map(|k| RespValue::BulkString(Some(k.clone())))
                        .collect::<Vec<_>>();
                    RespValue::Array(vec)
                }
                Some(_) => RespValue::Error(
                    "WRONGTYPE Operation against a key holding the wrong kind of value".to_string(),
                ),
                None => RespValue::Array(Vec::new()),
            }
        } else {
            RespValue::Error("ERR wrong number of arguments for 'hvals' command".to_string())
        }
    }
}

impl Command {
    pub async fn handle(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        match command.name.as_str() {
            // System
            "PING" => HandleString::handle_ping(command),
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
            _ => RespValue::Error(format!("ERR unknown command '{}'", command.name)),
        }
    }
}
