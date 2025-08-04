use std::collections::HashMap;

use crate::command::Command;
use crate::command::handle_macro::*;

pub struct HandleHash;
impl HandleHash {
    pub async fn handle_hset(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
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
                    RespErrType!()
                }
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_hget(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
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
                Some(_) => RespErrType!(),
                _ => RespValue::Null,
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_hgetall(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
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
                _ => RespErrType!(),
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_hdel(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
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
                Some(_) => RespErrType!(),
                None => RespValue::Integer(0),
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_hexists(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() == 2 {
            let db_guard = db.lock().await;
            match db_guard.get(&command.args[0]) {
                Some(Value::Hash(hashmap)) => {
                    let num = hashmap.contains_key(&command.args[1]) as i64;
                    RespValue::Integer(num)
                }
                Some(_) => RespErrType!(),
                None => RespValue::Integer(0),
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_hlen(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() == 1 {
            let db_guard = db.lock().await;
            match db_guard.get(&command.args[0]) {
                Some(Value::Hash(hashmap)) => RespValue::Integer(hashmap.len() as i64),
                Some(_) => RespErrType!(),
                None => RespValue::Integer(0),
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_hkeys(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
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
                Some(_) => RespErrType!(),
                None => RespValue::Array(Vec::new()),
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_hvals(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
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
                Some(_) => RespErrType!(),
                None => RespValue::Array(Vec::new()),
            }
        } else {
            RespErrArgNum!()
        }
    }
}
