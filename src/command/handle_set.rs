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
                        HashSet::from_iter(command.args.iter().skip(1).map(|x| x.clone()));
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
}
