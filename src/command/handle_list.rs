use std::collections::VecDeque;

use crate::command::Command;
use crate::command::handle_macro::*;

pub struct HandleList;
impl HandleList {
    pub async fn handle_lpush(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() > 1 {
            let mut db_guard = db.lock().await;
            match db_guard.get_mut(&command.args[0]) {
                Some(Value::List(list)) => {
                    for element in command.args.iter().skip(1) {
                        list.push_front(element.clone());
                    }
                    RespValue::Integer(list.len() as i64)
                }
                Some(_) => RespErrType!(),
                None => {
                    let mut list = VecDeque::with_capacity(command.args.len());
                    for element in command.args.iter().skip(1) {
                        list.push_front(element.clone());
                    }
                    let len = list.len();
                    db_guard.set(command.args[0].clone(), Value::List(list));
                    RespValue::Integer(len as i64)
                }
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_rpush(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() > 1 {
            let mut db_guard = db.lock().await;
            match db_guard.get_mut(&command.args[0]) {
                Some(Value::List(list)) => {
                    for element in command.args.iter().skip(1) {
                        list.push_back(element.clone());
                    }
                    RespValue::Integer(list.len() as i64)
                }
                Some(_) => RespErrType!(),
                None => {
                    let mut list = VecDeque::with_capacity(command.args.len());
                    for element in command.args.iter().skip(1) {
                        list.push_back(element.clone());
                    }
                    let len = list.len();
                    db_guard.set(command.args[0].clone(), Value::List(list));
                    RespValue::Integer(len as i64)
                }
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_lpop(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() == 1 {
            let mut db_guard = db.lock().await;
            match db_guard.get_mut(&command.args[0]) {
                Some(Value::List(list)) => match list.pop_front() {
                    Some(s) => RespValue::BulkString(Some(s)),
                    None => RespValue::Null,
                },
                Some(_) => RespErrType!(),
                None => RespValue::Null,
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_rpop(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() == 1 {
            let mut db_guard = db.lock().await;
            match db_guard.get_mut(&command.args[0]) {
                Some(Value::List(list)) => match list.pop_back() {
                    Some(s) => RespValue::BulkString(Some(s)),
                    None => RespValue::Null,
                },
                Some(_) => RespErrType!(),
                None => RespValue::Null,
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_llen(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() == 1 {
            let db_guard = db.lock().await;
            match db_guard.get(&command.args[0]) {
                Some(Value::List(list)) => RespValue::Integer(list.len() as i64),
                Some(_) => RespErrType!(),
                None => RespValue::Integer(0),
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_lindex(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() == 2 {
            let db_guard = db.lock().await;
            match db_guard.get(&command.args[0]) {
                Some(Value::List(list)) => {
                    if let Ok(mut index) = command.args[1].parse::<i64>() {
                        if index < 0 {
                            index += list.len() as i64;
                        };
                        if index >= 0 && index < list.len() as i64 {
                            return RespValue::BulkString(Some(list[index as usize].clone()));
                        }
                    }
                    RespErrNumWrong!()
                },
                Some(_) => RespErrType!(),
                None => RespValue::Null,
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_lset(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() == 3 {
            let mut db_guard = db.lock().await;
            match db_guard.get_mut(&command.args[0]) {
                Some(Value::List(list)) => {
                    if let Ok(mut index) = command.args[1].parse::<i64>() {
                        if index < 0 {
                            index += list.len() as i64;
                        };
                        if index >= 0 && index < list.len() as i64 {
                            list[index as usize] = command.args[2].clone();
                            return RespOK!();
                        }
                    }
                    RespErrNumWrong!()
                }
                Some(_) => {
                    RespErrType!()
                }
                None => RespValue::Error("ERR no such key".to_string()),
            }
        } else {
            RespErrArgNum!()
        }
    }

    pub async fn handle_lrange(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        if command.args.len() == 3 {
            let db_guard = db.lock().await;
            match db_guard.get(&command.args[0]) {
                Some(Value::List(list)) => {
                    if let (Ok(mut idx_from), Ok(mut idx_end)) = (
                        command.args[1].parse::<i64>(),
                        command.args[2].parse::<i64>(),
                    ) {
                        if idx_from < 0 {
                            idx_from += list.len() as i64;
                        };
                        if idx_end < 0 {
                            idx_end += list.len() as i64;
                        };
                        if idx_from >= 0 && idx_end < list.len() as i64 && idx_from < idx_end {
                            let vec = Vec::from_iter(
                                list.range(idx_from as usize..=idx_end as usize)
                                    .map(|val| RespValue::BulkString(Some(val.clone()))),
                            );
                            return RespValue::Array(vec);
                        }
                    }
                    RespErrNumWrong!()
                }
                Some(_) => RespErrType!(),
                None => RespValue::Null,
            }
        } else {
            RespErrArgNum!()
        }
    }
}
