pub mod pb;

use futures::Stream;
use pb::{
    QueryRequest, RawQueryRequest, User,
    user_stats_server::{UserStats, UserStatsServer},
};
use std::pin::Pin;
use tonic::{Request, Response, Status, async_trait};

#[derive(Default)]
pub struct UserStatsService {}

type ServiceResult<T> = Result<Response<T>, Status>;

// Stream: futures::Stream
type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;

#[async_trait]
impl UserStats for UserStatsService {
    type QueryStream = ResponseStream;
    type RawQueryStream = ResponseStream;

    async fn query(&self, _request: Request<QueryRequest>) -> ServiceResult<Self::QueryStream> {
        unimplemented!()
    }

    async fn raw_query(
        &self,
        _request: Request<RawQueryRequest>,
    ) -> ServiceResult<Self::RawQueryStream> {
        unimplemented!()
    }
}

impl From<UserStatsService> for UserStatsServer<UserStatsService> {
    fn from(svc: UserStatsService) -> Self {
        UserStatsServer::new(svc)
    }
}
