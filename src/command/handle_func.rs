use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::command::Command;
use crate::protocol::RespValue;
use crate::storage::Database;
use crate::storage::Value;

pub struct HandleFunc;

impl HandleFunc {
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
                match command.args[2].as_str() {
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
                Some(value) => RespValue::BulkString(Some(value.to_string())),
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

    fn handle_others(command: Command) -> RespValue {
        RespValue::Error(format!("ERR unknown command '{}'", command.name))
    }
}

impl Command {
    pub async fn handle(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        match command.name.as_str() {
            "PING" => HandleFunc::handle_ping(command),
            "SET" => HandleFunc::handle_set(db, command).await,
            "GET" => HandleFunc::handle_get(db, command).await,
            "DEL" => HandleFunc::handle_del(db, command).await,
            "EXISTS" => HandleFunc::handle_exists(db, command).await,
            "INCR" => HandleFunc::handle_incr(db, command).await,
            "DECR" => HandleFunc::handle_decr(db, command).await,
            _ => HandleFunc::handle_others(command),
        }
    }
}
