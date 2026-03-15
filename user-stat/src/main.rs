use anyhow::Result;
use tonic::transport::Server;
use user_stat::{AppConfig, UserStatsService};

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::load().expect("should load config ok");
    let addr = format!("[::1]:{}", config.server.port)
        .parse()
        .expect("should parse addr ok");
    println!("UserStatsServer is listening to {}", addr);

    let svc = UserStatsService::new(config).await.into_server();
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
