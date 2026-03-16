use anyhow::Result;
use crm_metadata::{AppConfig, MetadataService};
use tonic::transport::Server;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = AppConfig::load().expect("should load config ok");
    let addr = format!("[::1]:{}", config.server.port)
        .parse()
        .expect("should parse addr ok");
    info!("UserStatsServer is listening to {}", addr);

    let svc = MetadataService::new(config).into_server();
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
