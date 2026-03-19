pub mod pb;

mod abi;
mod config;

pub use config::AppConfig;

use anyhow::Result;
use crm_metadata::pb::metadata_client::MetadataClient;
use crm_send::pb::notification_client::NotificationClient;
use tonic::{async_trait, transport::Channel, Request, Response, Status};
use user_stat::pb::user_stats_client::UserStatsClient;

use crate::pb::{
    crm_server::{Crm, CrmServer},
    RecallRequest, RecallResponse, RemindRequest, RemindResponse, WelcomeRequest, WelcomeResponse,
};

#[allow(unused)]
pub struct CrmService {
    config: AppConfig,
    user_stat: UserStatsClient<Channel>,
    notification: NotificationClient<Channel>,
    metadata: MetadataClient<Channel>,
}

#[async_trait]
impl Crm for CrmService {
    async fn welcome(
        &self,
        _request: Request<WelcomeRequest>,
    ) -> Result<Response<WelcomeResponse>, Status> {
        // let user = self.user_stat.get_user(request.id).await?;
        // let message = format!("Welcome back, {}!", user.name);
        // self.notification.send_email(user.email, &message).await?;
        // Ok(WelcomeResponse { message })
        unimplemented!()
    }

    async fn recall(
        &self,
        _request: Request<RecallRequest>,
    ) -> Result<Response<RecallResponse>, Status> {
        unimplemented!()
    }

    async fn remind(
        &self,
        _request: Request<RemindRequest>,
    ) -> Result<Response<RemindResponse>, Status> {
        unimplemented!()
    }
}

impl CrmService {
    pub async fn try_new(config: AppConfig) -> Result<Self> {
        let user_stat = UserStatsClient::connect(config.server.user_stats.clone()).await?;
        let notification = NotificationClient::connect(config.server.notification.clone()).await?;
        let metadata = MetadataClient::connect(config.server.metadata.clone()).await?;
        Ok(Self {
            config,
            user_stat,
            notification,
            metadata,
        })
    }

    pub fn into_server(self) -> CrmServer<Self> {
        CrmServer::new(self)
    }
}
