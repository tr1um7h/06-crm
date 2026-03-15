use tonic::{Response, Status};

use crate::{
    ResponseStream, ServiceResult, UserStatsService,
    pb::{QueryRequest, RawQueryRequest, User},
};

impl UserStatsService {
    pub async fn query(&self, _query: QueryRequest) -> ServiceResult<ResponseStream> {
        // generate sql based on query
        let mut sql = "SELECT email, name FROM user_stats WHERE ".to_string();

        sql.push_str("created_at > '2026-02-09' LIMIT 10;");

        println!("Generated sql: {}", sql);

        self.raw_query(RawQueryRequest { query: sql }).await
    }

    pub async fn raw_query(&self, req: RawQueryRequest) -> ServiceResult<ResponseStream> {
        let Ok(ret) = sqlx::query_as::<_, User>(&req.query)
            .fetch_all(&self.pool)
            .await
        else {
            return Err(Status::internal(format!(
                "Failed to query data: {}",
                req.query
            )));
        };

        Ok(Response::new(Box::pin(futures::stream::iter(
            ret.into_iter().map(Ok),
        ))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AppConfig,
        UserStatsService,
        // pb::{IdQuery, QueryRequestBuilder, TimeQuery},
    };
    use anyhow::Result;
    use futures::StreamExt;

    #[tokio::test]
    async fn test_query() -> Result<()> {
        let config = AppConfig::load().expect("should load config ok");
        let svc = UserStatsService::new(config).await;
        let request = RawQueryRequest {
            query: "SELECT email, name FROM user_stats WHERE created_at > '2026-02-09' LIMIT 10;"
                .to_string(),
        };
        let mut stream = svc.raw_query(request).await?.into_inner();

        while let Some(item) = stream.next().await {
            println!("{:?}", item);
        }
        Ok(())
    }
}
