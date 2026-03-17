use tonic::Status;
use tracing::warn;

use crate::{
    NotificationService,
    abi::{Sender, to_ts},
    pb::{SendRequest, SendResponse, SmsMessage, send_request::Msg},
};

impl Sender for SmsMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();
        svc.sender.send(Msg::Sms(self)).await.map_err(|e| {
            warn!("Failed to send message: {:?}", e);
            Status::internal("Failed to send message")
        })?;
        Ok(SendResponse {
            message_id,
            sent_at: Some(to_ts()),
        })
    }
}

impl From<SmsMessage> for Msg {
    fn from(msg: SmsMessage) -> Self {
        Msg::Sms(msg)
    }
}

impl From<SmsMessage> for SendRequest {
    fn from(sms: SmsMessage) -> Self {
        let msg = sms.into();
        SendRequest { msg: Some(msg) }
    }
}

#[cfg(feature = "test_utils")]
mod tests {
    use crate::pb::SmsMessage;
    use fake::{
        Fake,
        faker::{lorem::en::Sentence, phone_number::en::PhoneNumber},
    };
    use uuid::Uuid;

    impl SmsMessage {
        pub fn fake() -> Self {
            SmsMessage {
                message_id: Uuid::new_v4().to_string(),
                sender: PhoneNumber().fake(),
                recipients: vec![PhoneNumber().fake()],
                body: Sentence(5..10).fake(),
            }
        }
    }
}
