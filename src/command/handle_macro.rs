macro_rules! RespErrArgNum {
    () => {
        RespValue::Error("ERR wrong number of arguments".to_string())
    };
}

macro_rules! RespErrType {
    () => {
        RespValue::Error(
            "WRONGTYPE Operation against a key holding the wrong kind of value".to_string(),
        )
    };
}

macro_rules! RespOK {
    () => {
        RespValue::SimpleString("OK".to_string())
    };
}

pub(super) use RespErrArgNum;
pub(super) use RespErrType;
pub(super) use RespOK;

pub(super) use std::sync::Arc;
pub(super) use tokio::sync::Mutex;

pub(super) use crate::protocol::GeneralError;
pub(super) use crate::protocol::RespValue;
pub(super) use crate::storage::Database;
pub(super) use crate::storage::Value;
