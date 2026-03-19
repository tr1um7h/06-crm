use anyhow::Result;

use crm::{pb::crm_client::CrmClient, AppConfig};

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::load().expect("should load config ok");
    let addr = format!("[::1]:{}", config.server.port);
    let _client = CrmClient::connect(addr).await?;

    todo!()
}
