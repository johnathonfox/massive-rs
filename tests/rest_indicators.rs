use massive::rest::IndicatorsApi;
use massive::Client;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn single_indicator_body() -> serde_json::Value {
    serde_json::json!({
        "results": {
            "values": [
                { "timestamp": 1683057600000i64, "value": 129.42 },
                { "timestamp": 1683144000000i64, "value": 130.05 }
            ],
            "underlying": {
                "url": "https://api.massive.com/v2/aggs/ticker/AAPL/range/1/day/1683057600000/1683144000000?limit=120"
            }
        },
        "status": "OK",
        "request_id": "a1b2c3d4"
    })
}

fn macd_body() -> serde_json::Value {
    serde_json::json!({
        "results": {
            "values": [
                {
                    "timestamp": 1683057600000i64,
                    "value": 1.234,
                    "signal": 1.001,
                    "histogram": 0.233
                },
                {
                    "timestamp": 1683144000000i64,
                    "value": 1.410,
                    "signal": 1.083,
                    "histogram": 0.327
                }
            ],
            "underlying": {
                "url": "https://api.massive.com/v2/aggs/ticker/MSFT/range/1/day/1683057600000/1683144000000?limit=120"
            }
        },
        "status": "OK",
        "request_id": "e5f6g7h8"
    })
}

#[tokio::test]
async fn get_sma_hits_expected_path_and_unwraps_results() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/indicators/sma/AAPL"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("timestamp.gte", "2023-05-01"))
        .and(query_param("window", "10"))
        .and(query_param("adjusted", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(single_indicator_body()))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let results = client
        .get_sma(
            "AAPL",
            None,
            None,
            None,
            None,
            Some("2023-05-01"),
            Some("day"),
            Some(10),
            Some(true),
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

    let values = results.values.as_ref().unwrap();
    assert_eq!(values.len(), 2);
    assert_eq!(values[0].timestamp, Some(1683057600000));
    assert_eq!(values[0].value, Some(129.42));
    let underlying = results.underlying.as_ref().unwrap();
    assert!(underlying.url.as_ref().unwrap().contains("/v2/aggs/ticker/AAPL"));
}

#[tokio::test]
async fn get_ema_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/indicators/ema/MSFT"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("timestamp.lt", "2023-06-01"))
        .and(query_param("series_type", "close"))
        .respond_with(ResponseTemplate::new(200).set_body_json(single_indicator_body()))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let results = client
        .get_ema(
            "MSFT",
            None,
            Some("2023-06-01"),
            None,
            None,
            None,
            Some("day"),
            None,
            None,
            None,
            Some("desc"),
            Some(2),
            Some("close"),
            None,
        )
        .await
        .unwrap();

    let values = results.values.as_ref().unwrap();
    assert_eq!(values.len(), 2);
    assert_eq!(values[1].value, Some(130.05));
}

#[tokio::test]
async fn get_rsi_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/indicators/rsi/AAPL"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("window", "14"))
        .and(query_param("timespan", "day"))
        .respond_with(ResponseTemplate::new(200).set_body_json(single_indicator_body()))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let results = client
        .get_rsi(
            "AAPL",
            None,
            None,
            None,
            None,
            None,
            Some("day"),
            Some(14),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

    let values = results.values.as_ref().unwrap();
    assert_eq!(values.len(), 2);
    assert_eq!(values[0].timestamp, Some(1683057600000));
}

#[tokio::test]
async fn get_macd_hits_expected_path_and_unwraps_results() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/indicators/macd/MSFT"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("short_window", "12"))
        .and(query_param("long_window", "26"))
        .and(query_param("signal_window", "9"))
        .respond_with(ResponseTemplate::new(200).set_body_json(macd_body()))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let results = client
        .get_macd(
            "MSFT",
            None,
            None,
            None,
            None,
            None,
            Some("day"),
            Some(12),
            Some(26),
            Some(9),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

    let values = results.values.as_ref().unwrap();
    assert_eq!(values.len(), 2);
    assert_eq!(values[0].value, Some(1.234));
    assert_eq!(values[0].signal, Some(1.001));
    assert_eq!(values[0].histogram, Some(0.233));
    let underlying = results.underlying.as_ref().unwrap();
    assert!(underlying.url.as_ref().unwrap().contains("/v2/aggs/ticker/MSFT"));
}
