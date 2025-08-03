// src/server/mod.rs
use crate::command::Command;
use crate::protocol::{RespParser, RespPacker, RespValue, GeneralError};
use crate::storage::{Database, Value};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

pub struct RedisServer {
    db: Arc<Mutex<Database>>,
    listener: TcpListener,
}

impl RedisServer {
    pub async fn new(addr: &str) -> Result<Self, Box<GeneralError>> {
        let listener = TcpListener::bind(addr).await?;
        let db = Arc::new(Mutex::new(Database::new()));

        Ok(Self { db, listener })
    }

    pub async fn run(&mut self) -> Result<(), Box<GeneralError>> {
        println!("Redis server listening on {}", self.listener.local_addr()?);

        loop {
            let (socket, addr) = self.listener.accept().await?;
            let db = self.db.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(socket, db).await {
                    eprintln!("Error handling connection {}: {}", addr, e);
                }
            });
        }
    }

    async fn handle_connection(
        mut socket: TcpStream,
        db: Arc<Mutex<Database>>,
    ) -> Result<(), Box<GeneralError>> {
        let mut buf = bytes::BytesMut::with_capacity(1024);

        loop {
            let mut temp_buf = [0u8; 1024];
            let n = socket.read(&mut temp_buf).await?;

            if n == 0 {
                return Ok(());
            }

            buf.extend_from_slice(&temp_buf[..n]);

            // 处理完整命令
            while let Some(command_respvalue) = RespParser::parse(&mut buf)? {
                let command = Command::parse(command_respvalue)?;
                let response = Self::execute_command(db.clone(), command).await;
                let response_bytes = RespPacker::pack(response);
                socket.write_all(&response_bytes).await?;
            }
        }
    }

    async fn execute_command(db: Arc<Mutex<Database>>, command: Command) -> RespValue {
        let mut db_guard = db.lock().await;

        match command.name.as_str() {
            "PING" => {
                if command.args.is_empty() {
                    RespValue::SimpleString("PONG".to_string())
                } else {
                    RespValue::BulkString(Some(command.args[0].clone()))
                }
            }
            "SET" => {
                if command.args.len() >= 2 {
                    db_guard.set(
                        command.args[0].clone(),
                        Value::String(command.args[1].clone()),
                    );
                    RespValue::SimpleString("OK".to_string())
                } else {
                    RespValue::Error("ERR wrong number of arguments".to_string())
                }
            }
            "GET" => {
                if command.args.len() == 1 {
                    match db_guard.get(&command.args[0]) {
                        Some(value) => RespValue::BulkString(Some(value.to_string())),
                        None => RespValue::Null,
                    }
                } else {
                    RespValue::Error("ERR wrong number of arguments".to_string())
                }
            }
            _ => RespValue::Error(format!("ERR unknown command '{}'", command.name)),
        }
    }

}
