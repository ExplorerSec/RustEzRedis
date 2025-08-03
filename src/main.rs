use crate::{protocol::GeneralError, server::RedisServer};

mod storage;
mod protocol;
mod command;
mod server;

#[tokio::main]
async fn main() ->Result<(),Box<GeneralError>>{
    let addr = "127.0.0.1:14785";
    let mut redis_server = RedisServer::new(addr).await?;
    redis_server.run().await?;

    Ok(())
}
