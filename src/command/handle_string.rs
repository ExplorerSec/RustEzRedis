use std::time::Duration;

use crate::command::Command;
use crate::command::handle_macro::*;

pub struct HandleString;
impl HandleString {
    pub async fn handle_set(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        let mut db_guard = db.lock().await;
        match command.args.len() {
            2 => {
                db_guard.set(
                    command.args[0].clone(),
                    Value::String(command.args[1].clone()),
                );
                RespOK!()
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
                RespOK!()
            }
            _ => RespErrArgNum!(),
        }
    }

    pub async fn handle_get(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        let db_guard = db.lock().await;
        if command.args.len() == 1 {
            match db_guard.get(&command.args[0]) {
                Some(Value::String(s)) => RespValue::BulkString(Some(s.to_string())),
                Some(_) => RespErrType!(),
                None => RespValue::Null,
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_del(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        let mut db_guard = db.lock().await;
        match command.args.len() {
            0 => RespErrArgNum!(),
            1 => {
                db_guard.del(&command.args[0]);
                RespOK!()
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

    pub async fn handle_exists(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
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
            RespErrArgNum!()
        }
    }

    pub async fn handle_incr(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
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
            RespErrArgNum!()
        }
    }

    pub async fn handle_decr(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
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
            RespErrArgNum!()
        }
    }
}
