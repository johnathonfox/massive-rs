use futures::TryStreamExt;
use massive::rest::SnapshotApi;
use massive::Client;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn ticker_snapshot_json(ticker: &str) -> serde_json::Value {
    serde_json::json!({
        "ticker": ticker,
        "todaysChange": 1.23,
        "todaysChangePerc": 0.82,
        "updated": 1675227600000000000i64,
        "day": {"o": 150.1, "h": 152.4, "l": 149.9, "c": 151.8, "v": 1234567.0, "vw": 151.2},
        "min": {"o": 151.7, "h": 151.9, "l": 151.6, "c": 151.8, "v": 4321.0, "t": 1675227540000i64},
        "prevDay": {"o": 148.0, "h": 151.0, "l": 147.5, "c": 150.57, "v": 987654.0},
        "lastTrade": {"p": 151.8, "s": 100, "t": 1675227599000000000i64},
        "lastQuote": {"P": 151.85, "S": 3, "p": 151.79, "s": 5, "t": 1675227599000000000i64}
    })
}

fn option_contract_snapshot_json(ticker: &str) -> serde_json::Value {
    serde_json::json!({
        "break_even_price": 179.075,
        "implied_volatility": 0.304,
        "open_interest": 8921.0,
        "day": {"change": -1.05, "change_percent": -4.6, "close": 21.87, "high": 22.2, "low": 21.0, "open": 21.5, "previous_close": 22.92, "volume": 37.0},
        "details": {
            "contract_type": "call",
            "exercise_style": "american",
            "expiration_date": "2023-01-20",
            "shares_per_contract": 100.0,
            "strike_price": 155.0,
            "ticker": ticker
        },
        "greeks": {"delta": 0.552, "gamma": 0.00667, "theta": -0.028, "vega": 0.727},
        "last_quote": {"ask": 21.25, "ask_size": 110.0, "bid": 20.9, "bid_size": 172.0, "midpoint": 21.075, "timeframe": "REAL-TIME"},
        "last_trade": {"price": 21.8, "size": 2, "sip_timestamp": 1675283458785000000i64, "timeframe": "REAL-TIME"},
        "underlying_asset": {"change_to_break_even": 5.825, "price": 173.15, "ticker": "AAPL", "timeframe": "REAL-TIME"}
    })
}

#[tokio::test]
async fn list_universal_snapshots_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v3/snapshot"))
        .and(query_param("type", "stocks"))
        .and(query_param("ticker.gte", "A"))
        .and(query_param("limit", "10"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 2,
            "results": [
                {
                    "ticker": "A",
                    "type": "stocks",
                    "market_status": "open",
                    "session": {"price": 142.13, "change": 1.13, "change_percent": 0.8, "open": 141.0, "close": 142.13, "volume": 2000000.0},
                    "last_trade": {"price": 142.13, "size": 100, "timeframe": "REAL-TIME"},
                    "last_quote": {"ask": 142.14, "bid": 142.12, "timeframe": "REAL-TIME"},
                    "last_minute": {"open": 142.1, "close": 142.13, "volume": 12345.0},
                    "last_updated": 1675283458000000000i64
                },
                {
                    "ticker": "AA",
                    "type": "stocks",
                    "market_status": "open",
                    "session": {"price": 50.25, "change": -0.5, "change_percent": -0.98},
                    "last_updated": 1675283458000000000i64
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let snapshots = client
        .list_universal_snapshots(
            Some("stocks"),
            None,
            None,
            Some(10),
            None,
            None,
            None,
            None,
            Some("A"),
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(snapshots.len(), 2);
    assert_eq!(snapshots[0].ticker.as_deref(), Some("A"));
    assert_eq!(snapshots[0].market_status.as_deref(), Some("open"));
    assert_eq!(
        snapshots[0].session.as_ref().unwrap().change_percent,
        Some(0.8)
    );
}

#[tokio::test]
async fn get_snapshot_all_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/snapshot/locale/us/markets/stocks/tickers"))
        .and(query_param("tickers", "AAPL,MSFT"))
        .and(query_param("include_otc", "false"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 2,
            "tickers": [ticker_snapshot_json("AAPL"), ticker_snapshot_json("MSFT")]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let snapshots = client
        .get_snapshot_all("stocks", Some("AAPL,MSFT"), Some(false), None)
        .await
        .unwrap();
    assert_eq!(snapshots.len(), 2);
    assert_eq!(snapshots[0].ticker.as_deref(), Some("AAPL"));
    assert_eq!(snapshots[0].todays_change, Some(1.23));
    assert_eq!(
        snapshots[0].day.as_ref().unwrap().close,
        Some(151.8)
    );
}

#[tokio::test]
async fn get_snapshot_direction_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/snapshot/locale/us/markets/stocks/gainers"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "tickers": [ticker_snapshot_json("TSLA")]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let snapshots = client
        .get_snapshot_direction("stocks", "gainers", None, None)
        .await
        .unwrap();
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].ticker.as_deref(), Some("TSLA"));
    assert_eq!(snapshots[0].todays_change_percent, Some(0.82));
}

#[tokio::test]
async fn get_snapshot_ticker_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/snapshot/locale/us/markets/stocks/tickers/AAPL"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "ticker": ticker_snapshot_json("AAPL")
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let snapshot = client
        .get_snapshot_ticker("stocks", "AAPL", None)
        .await
        .unwrap();
    assert_eq!(snapshot.ticker.as_deref(), Some("AAPL"));
    assert_eq!(snapshot.day.as_ref().unwrap().vwap, Some(151.2));
    assert_eq!(
        snapshot.last_trade.as_ref().unwrap().price,
        Some(151.8)
    );
}

#[tokio::test]
async fn get_snapshot_option_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v3/snapshot/options/AAPL/O:AAPL230120C00155000"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "results": option_contract_snapshot_json("O:AAPL230120C00155000")
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let snapshot = client
        .get_snapshot_option("AAPL", "O:AAPL230120C00155000", None)
        .await
        .unwrap();
    assert_eq!(snapshot.break_even_price, Some(179.075));
    assert_eq!(
        snapshot.details.as_ref().unwrap().contract_type.as_deref(),
        Some("call")
    );
    assert_eq!(snapshot.greeks.as_ref().unwrap().delta, Some(0.552));
    assert_eq!(
        snapshot.underlying_asset.as_ref().unwrap().price,
        Some(173.15)
    );
}

#[tokio::test]
async fn list_snapshot_options_chain_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v3/snapshot/options/AAPL"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 2,
            "results": [
                option_contract_snapshot_json("O:AAPL230120C00155000"),
                option_contract_snapshot_json("O:AAPL230120P00155000")
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let chain = client
        .list_snapshot_options_chain("AAPL", None)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(chain.len(), 2);
    assert_eq!(
        chain[0].details.as_ref().unwrap().strike_price,
        Some(155.0)
    );
    assert_eq!(chain[1].open_interest, Some(8921.0));
}

#[tokio::test]
async fn get_snapshot_crypto_book_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/v2/snapshot/locale/global/markets/crypto/tickers/X:BTCUSD/book",
        ))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "data": {
                "ticker": "X:BTCUSD",
                "bidCount": 694.5393709,
                "askCount": 593.1411254,
                "spread": 483.61,
                "updated": 1605297592000000000i64,
                "bids": [{"p": 16354.17, "x": {"1": 1.5, "6": 0.25}}, {"p": 16354.0, "x": {"2": 0.75}}],
                "asks": [{"p": 16837.78, "x": {"1": 0.5}}, {"p": 16838.0, "x": {"6": 1.2}}]
            }
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let book = client
        .get_snapshot_crypto_book("X:BTCUSD", None)
        .await
        .unwrap();
    assert_eq!(book.ticker.as_deref(), Some("X:BTCUSD"));
    assert_eq!(book.spread, Some(483.61));
    assert_eq!(book.bids.as_ref().unwrap().len(), 2);
    assert_eq!(
        book.bids.as_ref().unwrap()[0].price,
        Some(16354.17)
    );
}

#[tokio::test]
async fn get_snapshot_indices_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v3/snapshot/indices"))
        .and(query_param("ticker_any_of", "I:SPX"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "results": [{
                "ticker": "I:SPX",
                "name": "S&P 500",
                "type": "indices",
                "value": 4401.05,
                "market_status": "open",
                "session": {"change": 15.2, "change_percent": 0.35, "close": 4401.05, "open": 4390.0, "previous_close": 4385.85}
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let snapshots = client
        .get_snapshot_indices(Some("I:SPX"), None)
        .await
        .unwrap();
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].ticker.as_deref(), Some("I:SPX"));
    assert_eq!(snapshots[0].value, Some(4401.05));
    assert_eq!(
        snapshots[0].session.as_ref().unwrap().change_percent,
        Some(0.35)
    );
}
