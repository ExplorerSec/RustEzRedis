mod command;
mod protocol;
mod server;
mod storage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut addr = "127.0.0.1:6379".to_string();
    let mut args = std::env::args();
    if args.len() == 2 {
        addr = args.next_back().unwrap();
    }

    let mut redis_server = server::RedisServer::new(&addr).await?;
    redis_server.run().await?;

    Ok(())
}
