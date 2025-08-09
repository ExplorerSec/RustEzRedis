use std::collections::HashSet;

use crate::command::Command;
use crate::command::handle_macro::*;

pub struct HandleSet;

impl HandleSet {
    pub async fn handle_sadd(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() >= 2 {
            let mut db_guard = db.lock().await;
            match db_guard.get_mut(&command.args[0]) {
                Some(Value::Set(set)) => {
                    let mut num = 0;
                    for v in command.args.iter().skip(1) {
                        if set.insert(v.clone()) {
                            num += 1;
                        }
                    }
                    RespValue::Integer(num)
                }
                None => {
                    let set: HashSet<String> =
                        HashSet::from_iter(command.args.iter().skip(1).cloned());
                    let num = set.len();
                    db_guard.set(command.args[0].clone(), Value::Set(set));
                    RespValue::Integer(num as i64)
                }
                _ => RespErrType!(),
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_scard(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() == 1 {
            let db_guard = db.lock().await;
            match db_guard.get(&command.args[0]) {
                Some(Value::Set(set)) => RespValue::Integer(set.len() as i64),
                None => RespValue::Integer(0),
                _ => RespErrType!(),
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_smembers(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() == 1 {
            let db_guard = db.lock().await;
            match db_guard.get(&command.args[0]) {
                Some(Value::Set(set)) => {
                    let vec = set
                        .iter()
                        .map(|e| RespValue::BulkString(Some(e.clone())))
                        .collect::<Vec<_>>();
                    RespValue::Array(vec)
                }
                None => RespValue::Array(Vec::new()),
                _ => RespErrType!(),
            }
        } else {
            RespErrType!()
        }
    }

    pub async fn handle_srem(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() >= 2 {
            let mut db_guard = db.lock().await;
            match db_guard.get_mut(&command.args[0]) {
                Some(Value::Set(set)) => {
                    let mut num = 0;
                    for v in command.args.iter().skip(1) {
                        if set.remove(v) {
                            num += 1;
                        }
                    }
                    RespValue::Integer(num)
                }
                None => RespValue::Integer(0),
                _ => RespErrType!(),
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_sismember(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() == 2 {
            let db_guard = db.lock().await;
            match db_guard.get(&command.args[0]) {
                Some(Value::Set(set)) => RespValue::Integer(set.contains(&command.args[1]) as i64),
                None => RespValue::Integer(0),
                _ => RespErrType!(),
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_sinter(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() >= 2 {
            let db_guard = db.lock().await;
            let mut set = match db_guard.get(&command.args[0]) {
                Some(Value::Set(set)) => set.clone(),
                None => HashSet::new(),
                _ => return RespErrType!(),
            };
            for other_arg in command.args.iter().skip(1) {
                match db_guard.get(other_arg) {
                    Some(Value::Set(other_set)) => {
                        set = set.intersection(other_set).cloned().collect();
                    }
                    None => return RespValue::Array(Vec::new()),
                    _ => return RespErrType!(),
                }
            }
            RespValue::Array(Vec::from_iter(
                set.into_iter().map(|s| RespValue::BulkString(Some(s))),
            ))
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_sinterstore(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() >= 2 {
            let mut db_guard = db.lock().await;
            let mut set = match db_guard.get(&command.args[1]) {
                Some(Value::Set(set)) => set.clone(),
                None => HashSet::new(),
                _ => return RespErrType!(),
            };
            for other_arg in command.args.iter().skip(2) {
                match db_guard.get(other_arg) {
                    Some(Value::Set(other_set)) => {
                        set = set.intersection(other_set).cloned().collect();
                    }
                    None => {
                        set = HashSet::new();
                        break;
                    }
                    _ => return RespErrType!(),
                }
            }
            let len = set.len() as i64;
            db_guard.set(command.args[0].to_string(), Value::Set(set));
            RespValue::Integer(len)
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_sunion(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() >= 2 {
            let db_guard = db.lock().await;
            let mut set = match db_guard.get(&command.args[0]) {
                Some(Value::Set(set)) => set.clone(),
                None => HashSet::new(),
                _ => return RespErrType!(),
            };
            for other_arg in command.args.iter().skip(1) {
                match db_guard.get(other_arg) {
                    Some(Value::Set(other_set)) => {
                        set = set.union(other_set).cloned().collect();
                    }
                    None => {}
                    _ => return RespErrType!(),
                }
            }
            RespValue::Array(Vec::from_iter(
                set.into_iter().map(|s| RespValue::BulkString(Some(s))),
            ))
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_sunionstore(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() >= 2 {
            let mut db_guard = db.lock().await;
            let mut set = match db_guard.get(&command.args[1]) {
                Some(Value::Set(set)) => set.clone(),
                None => HashSet::new(),
                _ => return RespErrType!(),
            };
            for other_arg in command.args.iter().skip(2) {
                match db_guard.get(other_arg) {
                    Some(Value::Set(other_set)) => {
                        set = set.union(other_set).cloned().collect();
                    }
                    None => {}
                    _ => return RespErrType!(),
                }
            }
            let len = set.len() as i64;
            db_guard.set(command.args[0].to_string(), Value::Set(set));
            RespValue::Integer(len)
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_sdiff(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        match command.args.len() {
            2 => {
                let db_guard = db.lock().await;
                let set0 = match db_guard.get(&command.args[0]) {
                    Some(Value::Set(set)) => set.clone(),
                    None => HashSet::new(),
                    _ => return RespErrType!(),
                };
                let set1 = match db_guard.get(&command.args[1]) {
                    Some(Value::Set(set)) => set.clone(),
                    None => HashSet::new(),
                    _ => return RespErrType!(),
                };
                RespValue::Array(Vec::from_iter(
                    set0.difference(&set1)
                        .map(|s| RespValue::BulkString(Some(s.clone()))),
                ))
            }
            _ => {
                RespErrArgNum!()
            }
        }
    }

    pub async fn handle_sdiffstore(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        match command.args.len() {
            3 => {
                let mut db_guard = db.lock().await;
                let set0 = match db_guard.get(&command.args[1]) {
                    Some(Value::Set(set)) => set.clone(),
                    None => HashSet::new(),
                    _ => return RespErrType!(),
                };
                let set1 = match db_guard.get(&command.args[2]) {
                    Some(Value::Set(set)) => set.clone(),
                    None => HashSet::new(),
                    _ => return RespErrType!(),
                };

                let set: HashSet<String> = set0.difference(&set1).cloned().collect();
                let len = set.len() as i64;
                db_guard.set(command.args[0].to_string(), Value::Set(set));

                RespValue::Integer(len)
            }
            _ => {
                RespErrArgNum!()
            }
        }
    }
}
