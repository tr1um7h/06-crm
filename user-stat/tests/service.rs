use std::{net::SocketAddr, time::Duration};

use anyhow::Result;
use futures::StreamExt;
use tokio::time::sleep;
use tonic::transport::Server;
use user_stat::{
    AppConfig, UserStatsService,
    pb::{QueryRequestBuilder, RawQueryRequestBuilder, user_stats_client::UserStatsClient},
    test_utils::{id, tq},
};

const PORT_BASE: u32 = 50000;

#[tokio::test]
async fn raw_query_should_work() -> Result<()> {
    let addr = start_server(PORT_BASE).await?;
    let mut client = UserStatsClient::connect(format!("http://{addr}")).await?;

    let req = RawQueryRequestBuilder::default()
        .query("SELECT * FROM user_stats WHERE created_at > '2026-01-01' LIMIT 5")
        .build()?;
    let stream = client.raw_query(req).await?.into_inner();
    let ret = stream
        .then(|res| async move { res.unwrap() })
        .collect::<Vec<_>>()
        .await;

    assert_eq!(ret.len(), 5);

    Ok(())
}

#[tokio::test]
async fn query_should_work() -> Result<()> {
    let addr = start_server(PORT_BASE + 1).await?;
    let mut client = UserStatsClient::connect(format!("http://{addr}")).await?;

    let req = QueryRequestBuilder::default()
        .timestamp(("created_at".to_string(), tq(Some(120), None)))
        .timestamp(("last_visited_at".to_string(), tq(Some(90), None)))
        .id((
            "viewed_but_not_started".to_string(),
            id(&[124760872, 1776987229, 1944726278]),
        ))
        .build()
        .unwrap();

    let stream = client.query(req).await?.into_inner();
    let ret = stream
        .then(|res| async move { res.unwrap() })
        .collect::<Vec<_>>()
        .await;

    assert!(!ret.is_empty());

    Ok(())
}

async fn start_server(port: u32) -> Result<SocketAddr> {
    let config = AppConfig::load()?;
    let addr = format!("[::1]:{}", port).parse()?;
    let svc = UserStatsService::new(config).await.into_server();

    tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve(addr)
            .await
            .unwrap();
    });

    sleep(Duration::from_micros(1)).await;

    Ok(addr)
}
