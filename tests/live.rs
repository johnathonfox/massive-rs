//! Live tests against the real Massive API.
//!
//! These are `#[ignore]`d by default and skip silently without a key.
//! Run with: MASSIVE_API_KEY=<key> cargo test --test live -- --ignored

use futures::{StreamExt, TryStreamExt};
use massive::rest::{AggsApi, ReferenceApi, TradesApi};
use massive::RESTClient;

fn client() -> Option<RESTClient> {
    match RESTClient::from_env() {
        Ok(c) => Some(c),
        Err(_) => {
            eprintln!("MASSIVE_API_KEY not set, skipping live test");
            None
        }
    }
}

#[tokio::test]
#[ignore]
async fn live_get_last_trade() {
    let Some(client) = client() else { return };
    let trade = client.get_last_trade("AAPL", None).await.unwrap();
    assert!(trade.price.is_some());
}

#[tokio::test]
#[ignore]
async fn live_list_aggs_single_page() {
    let Some(client) = client() else { return };
    let aggs = client
        .with_pagination(false)
        .get_aggs(
            "AAPL",
            1,
            "day",
            "2024-01-02",
            "2024-01-05",
            Some(true),
            Some("asc"),
            Some(5),
            None,
        )
        .await
        .unwrap();
    assert!(!aggs.is_empty());
    assert!(aggs[0].close.is_some());
}

#[tokio::test]
#[ignore]
async fn live_list_tickers_stream() {
    let Some(client) = client() else { return };
    let tickers: Vec<_> = client
        .list_tickers(
            Some("AAPL"),
            None, None, None, None, None, None, None, None, None, None, None, None, Some(1), None,
            None, None,
        )
        .take(1)
        .try_collect()
        .await
        .unwrap();
    assert_eq!(tickers.len(), 1);
    assert_eq!(tickers[0].ticker.as_deref(), Some("AAPL"));
}

#[tokio::test]
#[ignore]
async fn live_market_status() {
    let Some(client) = client() else { return };
    let status = client.get_market_status(None).await.unwrap();
    assert!(status.market.is_some());
}
