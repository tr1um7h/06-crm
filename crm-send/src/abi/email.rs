use tonic::Status;
use tracing::warn;

use crate::{
    NotificationService,
    abi::{Sender, to_ts},
    pb::{EmailMessage, SendRequest, SendResponse, send_request::Msg},
};

impl Sender for EmailMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();
        svc.sender.send(Msg::Email(self)).await.map_err(|e| {
            warn!("Failed to send message: {:?}", e);
            Status::internal("Failed to send message")
        })?;
        Ok(SendResponse {
            message_id,
            sent_at: Some(to_ts()),
        })
    }
}

// jchen: Learn it!
impl From<EmailMessage> for Msg {
    fn from(msg: EmailMessage) -> Self {
        Msg::Email(msg)
    }
}

impl From<EmailMessage> for SendRequest {
    fn from(email: EmailMessage) -> Self {
        let msg = email.into();
        SendRequest { msg: Some(msg) }
    }
}

#[cfg(feature = "test_utils")]
mod tests {
    use crate::pb::EmailMessage;
    use fake::{
        Fake,
        faker::{internet::en::SafeEmail, lorem::en::Sentence},
    };
    use uuid::Uuid;

    impl EmailMessage {
        pub fn fake() -> Self {
            EmailMessage {
                message_id: Uuid::new_v4().to_string(),
                sender: SafeEmail().fake(),
                recipients: vec![SafeEmail().fake()],
                subject: "Hello".to_string(),
                body: Sentence(5..10).fake(),
            }
        }
    }
}
