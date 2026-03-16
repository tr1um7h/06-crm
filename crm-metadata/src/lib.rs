pub mod pb;

mod abi;
mod config;
use std::pin::Pin;

pub use config::AppConfig;
use futures::Stream;
use tonic::{Request, Response, Status, Streaming, async_trait};

use crate::pb::{
    Content, MaterializeRequest,
    metadata_server::{Metadata, MetadataServer},
};

type ServiceResult<T> = Result<Response<T>, Status>;

// Stream: futures::Stream
type ResponseStream = Pin<Box<dyn Stream<Item = Result<Content, Status>> + Send>>;

#[async_trait]
impl Metadata for MetadataService {
    type MaterializeStream = ResponseStream;

    async fn materialize(
        &self,
        request: Request<Streaming<MaterializeRequest>>,
    ) -> ServiceResult<ResponseStream> {
        self.materialize(request).await
    }
}

#[allow(unused)]
pub struct MetadataService {
    config: AppConfig,
}

impl MetadataService {
    pub fn new(config: AppConfig) -> Self {
        MetadataService { config }
    }

    pub fn into_server(self) -> MetadataServer<Self> {
        MetadataServer::new(self)
    }
}
