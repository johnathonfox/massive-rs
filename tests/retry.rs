//! Tests for opt-in 429/5xx retry behavior.

use futures::TryStreamExt;
use massive::rest::TradesApi;
use massive::Client;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_retries_on_429_then_succeeds() {
    let server = MockServer::start().await;
    // Mounted first: takes precedence for the first request only.
    Mock::given(method("GET"))
        .and(path("/v2/last/trade/AAPL"))
        .respond_with(ResponseTemplate::new(429).set_body_string("rate limited"))
        .up_to_n_times(1)
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2/last/trade/AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "results": {"T": "AAPL", "p": 150.5, "s": 100, "t": 1536036818784i64}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key")
        .unwrap()
        .with_base(server.uri())
        .with_max_retries(3);
    let trade = client.get_last_trade("AAPL", None).await.unwrap();
    assert_eq!(trade.price, Some(150.5));
}

#[tokio::test]
async fn get_without_retries_returns_error_on_429() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/last/trade/AAPL"))
        .respond_with(ResponseTemplate::new(429).set_body_string("rate limited"))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let err = client.get_last_trade("AAPL", None).await.unwrap_err();
    match err {
        massive::Error::Http { status, .. } => {
            assert_eq!(status, reqwest::StatusCode::TOO_MANY_REQUESTS)
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[tokio::test]
async fn paginated_stream_retries_page_fetch() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v3/trades/AAPL"))
        .respond_with(ResponseTemplate::new(503).set_body_string("unavailable"))
        .up_to_n_times(2)
        .expect(2)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v3/trades/AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "results": [{"id": "1", "participant_timestamp": 1536036818784i64, "price": 150.5, "size": 100}]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key")
        .unwrap()
        .with_base(server.uri())
        .with_max_retries(2);
    let trades: Vec<_> = client
        .list_trades("AAPL", None, None, None, None, None, None, None, None, None)
        .try_collect()
        .await
        .unwrap();
    assert_eq!(trades.len(), 1);
}
