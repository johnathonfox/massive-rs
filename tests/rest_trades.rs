use futures::TryStreamExt;
use massive::{rest::TradesApi, Client};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn list_trades_streams_single_page() {
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
                {"id": "1", "price": 125.5, "size": 100.0, "exchange": 4, "conditions": [12], "sip_timestamp": 1672750800000000000i64, "participant_timestamp": 1672750799999000000i64, "sequence_number": 1, "tape": 3},
                {"id": "2", "price": 125.55, "size": 200.0, "exchange": 4, "conditions": [12], "sip_timestamp": 1672750800000000001i64, "participant_timestamp": 1672750799999000001i64, "sequence_number": 2, "tape": 3}
            ],
            "count": 2
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let trades: Vec<_> = client
        .list_trades(
            "AAPL",
            None,
            None,
            None,
            None,
            Some("2023-01-03"),
            Some(2),
            None,
            Some("asc"),
            None,
        )
        .try_collect()
        .await
        .unwrap();

    assert_eq!(trades.len(), 2);
    assert_eq!(trades[0].id.as_deref(), Some("1"));
    assert_eq!(trades[0].price, Some(125.5));
    assert_eq!(trades[1].size, Some(200.0));
}

#[tokio::test]
async fn list_trades_follows_next_url_with_auth_header() {
    let server = MockServer::start().await;
    let page2_url = format!("{}/v3/trades/AAPL?cursor=page2", server.uri());

    Mock::given(method("GET"))
        .and(path("/v3/trades/AAPL"))
        .and(query_param("limit", "2"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "results": [
                {"id": "p1-a", "price": 125.5, "size": 100.0, "sip_timestamp": 1672750800000000000i64},
                {"id": "p1-b", "price": 125.55, "size": 200.0, "sip_timestamp": 1672750800000000001i64}
            ],
            "next_url": page2_url
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v3/trades/AAPL"))
        .and(query_param("cursor", "page2"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "results": [
                {"id": "p2-a", "price": 125.6, "size": 300.0, "sip_timestamp": 1672750800000000002i64},
                {"id": "p2-b", "price": 125.65, "size": 400.0, "sip_timestamp": 1672750800000000003i64}
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let trades: Vec<_> = client
        .list_trades(
            "AAPL", None, None, None, None, None, Some(2), None, None, None,
        )
        .try_collect()
        .await
        .unwrap();

    // Both pages stream in order; wiremock's expect(1) + header matcher on page 2
    // verifies the follow-up request carried the Authorization: Bearer header.
    let ids: Vec<&str> = trades.iter().map(|t| t.id.as_deref().unwrap()).collect();
    assert_eq!(ids, vec!["p1-a", "p1-b", "p2-a", "p2-b"]);
    assert_eq!(trades[2].price, Some(125.6));
}

#[tokio::test]
async fn list_trades_pagination_disabled_stops_after_first_page() {
    let server = MockServer::start().await;
    let page2_url = format!("{}/v3/trades/AAPL?cursor=page2", server.uri());

    // Even though page 1 advertises a next_url, a client built with
    // .with_pagination(false) must not follow it.
    Mock::given(method("GET"))
        .and(path("/v3/trades/AAPL"))
        .and(query_param("limit", "2"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "results": [
                {"id": "only-1", "price": 125.5, "size": 100.0, "sip_timestamp": 1672750800000000000i64},
                {"id": "only-2", "price": 125.55, "size": 200.0, "sip_timestamp": 1672750800000000001i64}
            ],
            "next_url": page2_url
        })))
        .expect(1)
        .mount(&server)
        .await;

    // No mock for page 2: any follow-up request would 404 and surface as a stream error.
    let client = Client::new("test-key")
        .unwrap()
        .with_base(server.uri())
        .with_pagination(false);
    let trades: Vec<_> = client
        .list_trades(
            "AAPL", None, None, None, None, None, Some(2), None, None, None,
        )
        .try_collect()
        .await
        .unwrap();

    let ids: Vec<&str> = trades.iter().map(|t| t.id.as_deref().unwrap()).collect();
    assert_eq!(ids, vec!["only-1", "only-2"]);
}

#[tokio::test]
async fn get_last_trade_unwraps_results() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/last/trade/AAPL"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "request_id": "req-1",
            "results": {
                "T": "AAPL",
                "p": 130.15,
                "s": 250.0,
                "x": 4,
                "t": 1672750800000000000i64,
                "y": 1672750799999000000i64,
                "q": 123456.0,
                "c": [12, 37],
                "i": "6046",
                "z": 3
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let trade = client.get_last_trade("AAPL", None).await.unwrap();

    assert_eq!(trade.ticker.as_deref(), Some("AAPL"));
    assert_eq!(trade.price, Some(130.15));
    assert_eq!(trade.size, Some(250.0));
    assert_eq!(trade.exchange, Some(4));
    assert_eq!(trade.conditions, Some(vec![12, 37]));
}

#[tokio::test]
async fn get_last_crypto_trade_unwraps_last() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/last/crypto/BTC/USD"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "symbol": "BTC-USD",
            "last": {
                "price": 16808.5,
                "size": 0.0025,
                "exchange": 1,
                "conditions": [1],
                "timestamp": 1672750800000i64
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let trade = client.get_last_crypto_trade("BTC", "USD", None).await.unwrap();

    assert_eq!(trade.price, Some(16808.5));
    assert_eq!(trade.size, Some(0.0025));
    assert_eq!(trade.exchange, Some(1));
    assert_eq!(trade.timestamp, Some(1672750800000));
}
