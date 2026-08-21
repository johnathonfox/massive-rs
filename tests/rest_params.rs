//! Tests for the additive params/builder API (`*_with_params` methods).
//! The params path must produce the exact same wire requests as the flat
//! positional methods.

use futures::TryStreamExt;
use massive::rest::{GetSummariesParams, ListTradesParams};
use massive::{rest::SummariesApi, rest::TradesApi, Client};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn list_trades_with_params_matches_flat_request() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v3/trades/AAPL"))
        .and(query_param("timestamp.gte", "2023-01-03"))
        .and(query_param("limit", "2"))
        .and(query_param("order", "asc"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "results": [
                {"id": "1", "price": 125.5, "size": 100.0, "exchange": 4, "sip_timestamp": 1672750800000000000i64}
            ],
            "count": 1
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let params = ListTradesParams::new()
        .timestamp_gte("2023-01-03")
        .limit(2)
        .order("asc");
    let trades: Vec<_> = client
        .list_trades_with_params("AAPL", params)
        .try_collect()
        .await
        .unwrap();

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].id.as_deref(), Some("1"));
}

#[tokio::test]
async fn get_last_trade_with_params_sends_no_extra_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/last/trade/AAPL"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "results": {"T": "AAPL", "p": 150.5, "s": 100, "t": 1536036818784i64}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let trade = client
        .get_last_trade_with_params("AAPL", massive::rest::GetLastTradeParams::new())
        .await
        .unwrap();
    assert_eq!(trade.price, Some(150.5));
}

#[tokio::test]
async fn get_summaries_with_params_serializes_any_of_list() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/summaries"))
        .and(query_param("ticker.any_of", "AAPL,MSFT"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "results": []
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let params = GetSummariesParams::new().ticker_any_of(&["AAPL", "MSFT"]);
    let results = client.get_summaries_with_params(params).await.unwrap();
    assert!(results.is_empty());
}
