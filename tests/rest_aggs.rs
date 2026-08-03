use futures::TryStreamExt;
use massive::{rest::AggsApi, Client};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn list_aggs_streams_single_page() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/aggs/ticker/AAPL/range/1/day/2023-01-03/2023-01-04"))
        .and(query_param("adjusted", "true"))
        .and(query_param("sort", "asc"))
        .and(query_param("limit", "2"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "ticker": "AAPL",
            "queryCount": 2,
            "resultsCount": 2,
            "adjusted": true,
            "results": [
                {"o": 130.28, "h": 130.90, "l": 124.17, "c": 125.07, "v": 112117471.0, "vw": 126.0, "t": 1672723200000i64, "n": 100},
                {"o": 126.89, "h": 128.66, "l": 125.08, "c": 126.36, "v": 89113633.0, "vw": 127.0, "t": 1672809600000i64, "n": 90}
            ],
            "count": 2
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let aggs: Vec<_> = client
        .list_aggs(
            "AAPL",
            1,
            "day",
            "2023-01-03",
            "2023-01-04",
            Some(true),
            Some("asc"),
            Some(2),
            None,
        )
        .try_collect()
        .await
        .unwrap();

    assert_eq!(aggs.len(), 2);
    assert_eq!(aggs[0].close, Some(125.07));
    assert_eq!(aggs[1].open, Some(126.89));
    assert_eq!(aggs[0].timestamp, Some(1672723200000));
}

#[tokio::test]
async fn get_aggs_unwraps_results() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/aggs/ticker/MSFT/range/5/minute/2023-01-03/2023-01-03"))
        .and(query_param("adjusted", "false"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "ticker": "MSFT",
            "results": [
                {"o": 239.0, "h": 240.1, "l": 238.5, "c": 239.8, "v": 12345.0, "t": 1672747200000i64}
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let aggs = client
        .get_aggs(
            "MSFT",
            5,
            "minute",
            "2023-01-03",
            "2023-01-03",
            Some(false),
            None,
            None,
            None,
        )
        .await
        .unwrap();

    assert_eq!(aggs.len(), 1);
    assert_eq!(aggs[0].high, Some(240.1));
    assert_eq!(aggs[0].volume, Some(12345.0));
}

#[tokio::test]
async fn get_grouped_daily_aggs_hits_grouped_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/aggs/grouped/locale/us/market/stocks/2023-01-09"))
        .and(query_param("adjusted", "true"))
        .and(query_param("include_otc", "false"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "adjusted": true,
            "resultsCount": 2,
            "results": [
                {"T": "AAPL", "o": 130.47, "h": 133.41, "l": 129.89, "c": 130.15, "v": 7.0790813e7, "t": 1673240400000i64},
                {"T": "MSFT", "o": 227.2, "h": 231.94, "l": 226.42, "c": 231.93, "v": 2.6621021e7, "t": 1673240400000i64}
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let aggs = client
        .get_grouped_daily_aggs("2023-01-09", Some(true), None, None, Some(false), None)
        .await
        .unwrap();

    assert_eq!(aggs.len(), 2);
    assert_eq!(aggs[0].ticker.as_deref(), Some("AAPL"));
    assert_eq!(aggs[0].close, Some(130.15));
    assert_eq!(aggs[1].ticker.as_deref(), Some("MSFT"));
}

#[tokio::test]
async fn get_daily_open_close_agg_returns_top_level_object() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/open-close/AAPL/2023-01-09"))
        .and(query_param("adjusted", "true"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "from": "2023-01-09",
            "symbol": "AAPL",
            "open": 130.465,
            "high": 133.41,
            "low": 129.89,
            "close": 130.15,
            "volume": 7.0790813e7,
            "afterHours": 129.85,
            "preMarket": 129.6
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let agg = client
        .get_daily_open_close_agg("AAPL", "2023-01-09", Some(true), None)
        .await
        .unwrap();

    assert_eq!(agg.symbol.as_deref(), Some("AAPL"));
    assert_eq!(agg.open, Some(130.465));
    assert_eq!(agg.close, Some(130.15));
    assert_eq!(agg.after_hours, Some(129.85));
    assert_eq!(agg.pre_market, Some(129.6));
}

#[tokio::test]
async fn get_previous_close_agg_unwraps_results() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/aggs/ticker/AAPL/prev"))
        .and(query_param("adjusted", "false"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "ticker": "AAPL",
            "adjusted": false,
            "resultsCount": 1,
            "results": [
                {"T": "AAPL", "o": 130.47, "h": 133.41, "l": 129.89, "c": 130.15, "v": 7.0790813e7, "vw": 131.0, "t": 1673240400000.0}
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let aggs = client
        .get_previous_close_agg("AAPL", Some(false), None)
        .await
        .unwrap();

    assert_eq!(aggs.len(), 1);
    assert_eq!(aggs[0].ticker.as_deref(), Some("AAPL"));
    assert_eq!(aggs[0].close, Some(130.15));
    assert_eq!(aggs[0].vwap, Some(131.0));
}
