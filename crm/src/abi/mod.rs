mod crm;

use std::{collections::HashSet, sync::Arc};

use chrono::{DateTime, Duration, Utc};
use crm_metadata::{
    abi::Tpl,
    pb::{Content, MaterializeRequest},
};
use crm_send::pb::{send_request::Msg, EmailMessage, SendRequest};
use futures::{stream, Stream, StreamExt};
use prost_types::Timestamp;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};
use tracing::warn;
use user_stat::pb::{QueryRequest, QueryRequestBuilder, TimeQuery, User};
use uuid::Uuid;

use crate::{
    pb::{WelcomeRequest, WelcomeResponse},
    CrmService,
};

impl CrmService {
    pub async fn welcome(&self, req: WelcomeRequest) -> Result<Response<WelcomeResponse>, Status> {
        let request_id = req.id;
        let date = Utc::now() - Duration::days(req.interval as _);
        let query = get_user_stats_query("created_at", date);
        let mut res = self.user_stat.clone().query(query).await?.into_inner();

        let contents = self
            .metadata
            .clone()
            .materialize(gen_materialize_req(&req.content_ids))
            .await?
            .into_inner();
        let contents: Vec<Content> = contents
            .filter_map(|v| async move { v.ok() })
            .collect()
            .await;
        let contents = Arc::new(contents);
        let sender = self.config.server.sender_email.clone();

        let (tx, rx) = mpsc::channel(1024);
        tokio::spawn(async move {
            while let Some(Ok(user)) = res.next().await {
                let contents = contents.clone();
                let sender = sender.clone();
                let tx = tx.clone();
                let req = gen_send_req("Welcome".to_string(), sender, user, &contents);
                if let Err(e) = tx.send(req).await {
                    warn!("failed to send message: {:?}", e)
                }
            }
        });
        let reqs = ReceiverStream::new(rx);
        //
        // Note: this is an alternative way
        // use move to capture sender
        // let reqs = res.filter_map(move |v| {
        //     let contents = contents.clone();
        //     let sender = sender.clone();
        //     async move {
        //         let v = v.ok()?;
        //         Some(gen_send_req("Welcome".to_string(), sender, v, &contents))
        //     }
        // });

        self.notification.clone().send(reqs).await?;

        Ok(Response::new(WelcomeResponse { id: request_id }))
    }
}

fn gen_materialize_req(ids: &[u32]) -> impl Stream<Item = MaterializeRequest> {
    let reqs: HashSet<_> = ids
        .iter()
        .map(|id| MaterializeRequest { id: *id })
        .collect();
    stream::iter(reqs)
}

fn get_user_stats_query(name: &str, date: DateTime<Utc>) -> QueryRequest {
    let ts = Timestamp {
        seconds: date.timestamp(),
        nanos: date.timestamp_subsec_nanos() as _,
    };
    let tq = TimeQuery {
        lower: Some(ts),
        upper: Some(ts),
    };
    QueryRequestBuilder::default()
        .timestamp((name.to_string(), tq))
        .build()
        .expect("should build query request ok")
}

fn gen_send_req(subject: String, sender: String, user: User, contents: &[Content]) -> SendRequest {
    let tpl = Tpl(contents);
    let msg = Msg::Email(EmailMessage {
        message_id: Uuid::new_v4().to_string(),
        subject,
        sender,
        recipients: vec![user.email],
        body: tpl.to_body(),
    });

    SendRequest { msg: Some(msg) }
}
