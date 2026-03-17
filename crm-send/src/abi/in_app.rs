use tonic::Status;
use tracing::warn;

use crate::{
    NotificationService,
    abi::{Sender, to_ts},
    pb::{InAppMessage, SendRequest, SendResponse, send_request::Msg},
};

impl Sender for InAppMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();
        svc.sender.send(Msg::InApp(self)).await.map_err(|e| {
            warn!("Failed to send message: {:?}", e);
            Status::internal("Failed to send message")
        })?;
        Ok(SendResponse {
            message_id,
            sent_at: Some(to_ts()),
        })
    }
}

impl From<InAppMessage> for Msg {
    fn from(msg: InAppMessage) -> Self {
        Msg::InApp(msg)
    }
}

impl From<InAppMessage> for SendRequest {
    fn from(in_app: InAppMessage) -> Self {
        let msg = in_app.into();
        SendRequest { msg: Some(msg) }
    }
}

#[cfg(feature = "test_utils")]
mod tests {
    use super::*;
    use fake::{Fake, faker::lorem::en::Sentence};
    use uuid::Uuid;

    impl InAppMessage {
        pub fn fake() -> Self {
            InAppMessage {
                message_id: Uuid::new_v4().to_string(),
                device_id: Uuid::new_v4().to_string(),
                title: Sentence(3..7).fake(),
                body: Sentence(5..10).fake(),
            }
        }
    }
}
