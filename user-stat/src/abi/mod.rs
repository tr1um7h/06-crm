use core::fmt;

use chrono::{DateTime, TimeZone, Utc};
use itertools::Itertools;
use prost_types::Timestamp;
use tonic::{Response, Status};
use tracing::info;

use crate::{
    ResponseStream, ServiceResult, UserStatsService,
    pb::{QueryRequest, QueryRequestBuilder, RawQueryRequest, TimeQuery, User},
};

impl UserStatsService {
    pub async fn query(&self, query: QueryRequest) -> ServiceResult<ResponseStream> {
        let sql = query.to_string();

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

impl QueryRequest {
    pub fn new_with_dt(name: &str, lower: DateTime<Utc>, upper: DateTime<Utc>) -> Self {
        let ts: Timestamp = Timestamp {
            seconds: lower.timestamp(),
            nanos: 0,
        };
        let ts1 = Timestamp {
            seconds: upper.timestamp(),
            nanos: 0,
        };
        let tq = TimeQuery {
            lower: Some(ts),
            upper: Some(ts1),
        };
        QueryRequestBuilder::default()
            .timestamp((name.to_string(), tq))
            .build()
            .expect("should build query request ok")
    }
}

impl fmt::Display for QueryRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // generate sql based on query
        let mut sql = "SELECT email, name FROM user_stats WHERE ".to_string();

        let tm_str = self
            .timestamps
            .iter()
            .map(|(k, v)| time_query(k, v.lower, v.upper))
            .join(" AND ");

        sql.push_str(&tm_str);

        let id_str = self
            .ids
            .iter()
            .map(|(k, v)| ids_query(k, &v.ids))
            // use itertools::Itertools to join
            .join(" AND ");

        if !id_str.is_empty() {
            sql.push_str(" AND ");
            sql.push_str(&id_str);
        }

        info!("Generated sql: {}", sql);

        write!(f, "{}", sql)
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

fn ids_query(name: &str, ids: &[u32]) -> String {
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
    use chrono::Duration;
    use futures::StreamExt;

    #[test]
    fn query_request_to_string_should_work() {
        let date = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let query = QueryRequest::new_with_dt("created_at", date, date + Duration::days(1));
        let sql = query.to_string();
        assert_eq!(
            sql,
            "SELECT email, name FROM user_stats WHERE created_at BETWEEN '2024-01-01 00:00:00' AND '2024-01-02 00:00:00'"
        );
    }

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
