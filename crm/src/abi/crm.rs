use anyhow::Result;
use crm_metadata::MetadataService;
use crm_send::NotificationService;
use tonic::{async_trait, Request, Response, Status};
use user_stat::UserStatsService;

use crate::pb::{
    crm_server::Crm, RecallRequest, RecallResponse, RemindRequest, RemindResponse, WelcomeRequest,
    WelcomeResponse,
};

#[allow(dead_code)]
pub struct CrmService {
    user_stat: UserStatsService,
    notification: NotificationService,
    metadata: MetadataService,
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
