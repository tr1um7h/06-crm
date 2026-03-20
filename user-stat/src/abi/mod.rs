use chrono::{DateTime, TimeZone, Utc};
use itertools::Itertools;
use prost_types::Timestamp;
use tonic::{Response, Status};
use tracing::info;

use crate::{
    ResponseStream, ServiceResult, UserStatsService,
    pb::{QueryRequest, RawQueryRequest, User},
};

impl UserStatsService {
    pub async fn query(&self, query: QueryRequest) -> ServiceResult<ResponseStream> {
        // generate sql based on query
        let mut sql = "SELECT email, name FROM user_stats WHERE ".to_string();

        let tm_str = query
            .timestamps
            .into_iter()
            .map(|(k, v)| time_query(&k, v.lower, v.upper))
            .join(" AND ");

        sql.push_str(&tm_str);

        let id_str = query
            .ids
            .into_iter()
            .map(|(k, v)| ids_query(&k, v.ids))
            // use itertools::Itertools to join
            .join(" AND ");

        if !id_str.is_empty() {
            sql.push_str(" AND ");
            sql.push_str(&id_str);
        }

        info!("Generated sql: {}", sql);

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

fn time_query(name: &str, lower: Option<Timestamp>, upper: Option<Timestamp>) -> String {
    if lower.is_none() && upper.is_none() {
        return "TRUE".to_string();
    }

    if lower.is_none() {
        let upper = ts_to_utc(upper.unwrap());
        return format!("{} <= '{}'", name, upper.format("%Y-%m-%d %H:%M:%S"));
    }

    if upper.is_none() {
        let lower = ts_to_utc(lower.unwrap());
        return format!("{} >= '{}'", name, lower.format("%Y-%m-%d %H:%M:%S"));
    }

    format!(
        "{} BETWEEN '{}' AND '{}'",
        name,
        ts_to_utc(lower.unwrap()).format("%Y-%m-%d %H:%M:%S"),
        ts_to_utc(upper.unwrap()).format("%Y-%m-%d %H:%M:%S")
    )
}

fn ts_to_utc(ts: Timestamp) -> DateTime<Utc> {
    Utc.timestamp_opt(ts.seconds, ts.nanos as u32).unwrap()
}

fn ids_query(name: &str, ids: Vec<u32>) -> String {
    if ids.is_empty() {
        return "TRUE".to_string();
    }

    format!("array{:?} && {}", ids, name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        UserStatsService,
        pb::QueryRequestBuilder,
        test_utils::{id, tq},
    };
    use anyhow::Result;
    use futures::StreamExt;

    #[tokio::test]
    async fn raw_query_should_work() -> Result<()> {
        let (_tdb, svc) = UserStatsService::new_for_test().await?;
        let request = RawQueryRequest {
            query: "SELECT email, name FROM user_stats WHERE created_at > '2024-01-01' LIMIT 10;"
                .to_string(),
        };
        let mut stream = svc.raw_query(request).await?.into_inner();

        let mut i = 0;
        while let Some(item) = stream.next().await {
            i += 1;
            println!("{:?}", item);
        }

        // print if failed
        assert_eq!(i, 10);
        Ok(())
    }

    #[tokio::test]
    async fn query_should_work() -> Result<()> {
        let (_tdb, svc) = UserStatsService::new_for_test().await?;
        let query = QueryRequestBuilder::default()
            .timestamp(("created_at".to_string(), tq(Some(120), None)))
            .timestamp(("last_visited_at".to_string(), tq(Some(90), None)))
            .id(("viewed_but_not_started".to_string(), id(&[207348])))
            .build()
            .unwrap();
        let mut stream = svc.query(query).await?.into_inner();

        let mut i = 0;
        while let Some(res) = stream.next().await {
            i += 1;
            println!("{:?}", res);
        }

        // print if failed
        assert_eq!(i, 1);
        Ok(())
    }
}
