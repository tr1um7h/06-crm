pub mod pb;

mod abi;
mod config;
use std::{pin::Pin, sync::Arc};

pub use config::AppConfig;
use futures::Stream;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status, Streaming, async_trait};

use crate::pb::{SendRequest, SendResponse, notification_server::Notification, send_request::Msg};

type ServiceResult<T> = Result<Response<T>, Status>;

// Stream: futures::Stream
type ResponseStream = Pin<Box<dyn Stream<Item = Result<SendResponse, Status>> + Send>>;

#[allow(unused)]
#[derive(Clone)]
pub struct NotificationService {
    inner: Arc<NotificationServiceInner>,
}

#[allow(unused)]
pub struct NotificationServiceInner {
    config: AppConfig,
    // queue
    sender: mpsc::Sender<Msg>,
}

#[async_trait]
impl Notification for NotificationService {
    type SendStream = ResponseStream;

    async fn send(
        &self,
        request: Request<Streaming<SendRequest>>,
    ) -> Result<Response<Self::SendStream>, Status> {
        let stream = request.into_inner();
        self.send(stream).await
    }
}
