use anyhow::Result;
use tonic::transport::Server;
use user_stat::{UserStatsService, pb::user_stats_server::UserStatsServer};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::1]:50051".parse().expect("should parse addr ok");
    let svc = UserStatsServer::from(UserStatsService::default());

    println!("UserStatsServer is listening to {}", addr);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
