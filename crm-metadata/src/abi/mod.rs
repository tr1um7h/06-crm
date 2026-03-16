use chrono::{DateTime, Duration, Utc};
use fake::{
    Fake, Faker,
    faker::{chrono::en::DateTimeBetween, lorem::en::Sentence, name::en::Name},
};
use futures::{Stream, StreamExt};
use prost_types::Timestamp;
use rand::Rng;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};

use crate::{
    MetadataService, ResponseStream, ServiceResult,
    pb::{Content, MaterializeRequest, Publisher},
};

const CHANNEL_SIZE: usize = 1024;

impl MetadataService {
    pub async fn meterialize(
        &self,
        mut stream: impl Stream<Item = Result<MaterializeRequest, Status>> + Send + Unpin + 'static,
    ) -> ServiceResult<ResponseStream> {
        let (tx, rx) = mpsc::channel(CHANNEL_SIZE);
        tokio::spawn(async move {
            while let Some(Ok(req)) = stream.next().await {
                let content = Content::meterialize(req.id);
                tx.send(Ok(content)).await.expect("Should send ok")
            }
        });

        let stream = ReceiverStream::new(rx);

        Ok(Response::new(Box::pin(stream)))
    }
}

impl Content {
    pub fn meterialize(id: u32) -> Self {
        let mut rng = rand::rng();
        Content {
            id,
            name: Name().fake(),
            description: Sentence(3..7).fake(),
            publishers: (1..rng.random_range(1..10))
                .map(|_| Publisher::new())
                .collect(),
            url: "https://placehold.co/400x320".to_string(),
            image: "https://placehold.co/400x320".to_string(),
            content_type: Faker.fake(),
            created_at: created_at(),
            views: rng.random_range(200..10000),
            likes: rng.random_range(100..5000),
            dislikes: rng.random_range(10..2000),
        }
    }
}

impl Publisher {
    pub fn new() -> Self {
        Publisher {
            id: (1000..200000).fake(),
            name: Name().fake(),
            avatar: "https://placehold.co/200x200".to_string(),
        }
    }
}

fn before(days: i64) -> DateTime<Utc> {
    Utc::now() - Duration::days(days)
}

fn created_at() -> Option<Timestamp> {
    let date: DateTime<Utc> = DateTimeBetween(before(365), before(0)).fake();

    Some(Timestamp {
        seconds: date.timestamp(),
        nanos: date.timestamp_subsec_nanos() as i32,
    })
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::config;

    use super::*;

    #[tokio::test]
    async fn meterialize_should_work() -> Result<()> {
        let config = config::AppConfig::load()?;
        let service = MetadataService::new(config);

        let stream = tokio_stream::iter(vec![
            Ok(MaterializeRequest { id: 1 }),
            Ok(MaterializeRequest { id: 2 }),
            Ok(MaterializeRequest { id: 3 }),
        ]);

        let response = service.meterialize(stream).await?;
        let ret = response.into_inner().collect::<Vec<_>>().await;

        assert_eq!(ret.len(), 3);
        Ok(())
    }
}
