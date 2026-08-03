use futures::TryStreamExt;
use massive::{rest::QuotesApi, Client};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn list_quotes_streams_single_page() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v3/quotes/AAPL"))
        .and(query_param("timestamp.lt", "2023-01-04"))
        .and(query_param("limit", "2"))
        .and(query_param("sort", "timestamp"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "results": [
                {"bid_price": 125.4, "bid_size": 5.0, "bid_exchange": 11, "ask_price": 125.45, "ask_size": 3.0, "ask_exchange": 4, "sip_timestamp": 1672750800000000000i64, "sequence_number": 10, "tape": 3},
                {"bid_price": 125.41, "bid_size": 2.0, "bid_exchange": 11, "ask_price": 125.46, "ask_size": 7.0, "ask_exchange": 4, "sip_timestamp": 1672750800000000001i64, "sequence_number": 11, "tape": 3}
            ],
            "count": 2
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let quotes: Vec<_> = client
        .list_quotes(
            "AAPL",
            None,
            Some("2023-01-04"),
            None,
            None,
            None,
            Some(2),
            Some("timestamp"),
            None,
            None,
        )
        .try_collect()
        .await
        .unwrap();

    assert_eq!(quotes.len(), 2);
    assert_eq!(quotes[0].bid_price, Some(125.4));
    assert_eq!(quotes[0].ask_price, Some(125.45));
    assert_eq!(quotes[1].sequence_number, Some(11));
}

#[tokio::test]
async fn get_last_quote_unwraps_results() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/last/nbbo/AAPL"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "request_id": "req-1",
            "results": {
                "T": "AAPL",
                "p": 130.14,
                "P": 130.16,
                "s": 2,
                "S": 5,
                "x": 11,
                "X": 4,
                "t": 1672750800000000000i64,
                "y": 1672750799999000000i64,
                "q": 987654,
                "c": [1],
                "z": 3
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let quote = client.get_last_quote("AAPL", None).await.unwrap();

    assert_eq!(quote.ticker.as_deref(), Some("AAPL"));
    assert_eq!(quote.bid_price, Some(130.14));
    assert_eq!(quote.ask_price, Some(130.16));
    assert_eq!(quote.bid_size, Some(2));
    assert_eq!(quote.ask_size, Some(5));
}

#[tokio::test]
async fn get_last_forex_quote_returns_top_level_object() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/last_quote/currencies/USD/EUR"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "symbol": "USD/EUR",
            "last": {
                "ask": 0.9369,
                "bid": 0.9366,
                "exchange": 48,
                "timestamp": 1672750800000i64
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let quote = client
        .get_last_forex_quote("USD", "EUR", None)
        .await
        .unwrap();

    assert_eq!(quote.symbol.as_deref(), Some("USD/EUR"));
    let last = quote.last.unwrap();
    assert_eq!(last.ask, Some(0.9369));
    assert_eq!(last.bid, Some(0.9366));
    assert_eq!(last.exchange, Some(48));
}

#[tokio::test]
async fn get_real_time_currency_conversion_sends_amount_and_precision() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/conversion/USD/EUR"))
        .and(query_param("amount", "100"))
        .and(query_param("precision", "2"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "from_": "USD",
            "to": "EUR",
            "initialAmount": 100.0,
            "converted": 93.66,
            "last": {
                "ask": 0.9369,
                "bid": 0.9366,
                "exchange": 48,
                "timestamp": 1672750800000i64
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let conversion = client
        .get_real_time_currency_conversion("USD", "EUR", Some(100.0), Some(2), None)
        .await
        .unwrap();

    assert_eq!(conversion.from_.as_deref(), Some("USD"));
    assert_eq!(conversion.to.as_deref(), Some("EUR"));
    assert_eq!(conversion.initial_amount, Some(100.0));
    assert_eq!(conversion.converted, Some(93.66));
}
