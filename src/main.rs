use crate::{protocol::GeneralError, server::RedisServer};

mod command;
mod protocol;
mod server;
mod storage;

#[tokio::main]
async fn main() -> Result<(), Box<GeneralError>> {
    let addr = "127.0.0.1:6379";
    let mut redis_server = RedisServer::new(addr).await?;
    redis_server.run().await?;

    Ok(())
}
