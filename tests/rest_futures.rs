use futures::TryStreamExt;
use massive::rest::FuturesApi;
use massive::Client;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn list_futures_aggregates_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/futures/v1/aggs/ESZ4"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("resolution", "day"))
        .and(query_param("window_start.gte", "2024-01-01"))
        .and(query_param("limit", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 2,
            "results": [
                {
                    "ticker": "ESZ4",
                    "open": 5200.25,
                    "high": 5215.0,
                    "low": 5198.5,
                    "close": 5210.75,
                    "volume": 1500000.0,
                    "window_start": 1704067200000i64,
                    "session_end_date": "2024-01-02",
                    "settlement_price": 5211.0
                },
                {
                    "ticker": "ESZ4",
                    "open": 5210.75,
                    "high": 5230.0,
                    "low": 5205.0,
                    "close": 5225.5,
                    "volume": 1400000.0,
                    "window_start": 1704153600000i64,
                    "session_end_date": "2024-01-03",
                    "settlement_price": 5226.0
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let aggs = client
        .list_futures_aggregates(
            "ESZ4",
            Some("day"),
            None,
            None,
            None,
            None,
            Some("2024-01-01"),
            Some(10),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(aggs.len(), 2);
    assert_eq!(aggs[0].ticker.as_deref(), Some("ESZ4"));
    assert_eq!(aggs[0].close, Some(5210.75));
    assert_eq!(aggs[0].window_start, Some(1704067200000));
}

#[tokio::test]
async fn list_futures_contracts_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/futures/v1/contracts"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("product_code", "ES"))
        .and(query_param("active", "true"))
        .and(query_param("ticker.gte", "ESH4"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "ticker": "ESH4",
                    "product_code": "ES",
                    "trading_venue": "XCME",
                    "name": "E-mini S&P 500 Futures",
                    "type": "future",
                    "date": "2024-01-01",
                    "active": true,
                    "first_trade_date": "2023-03-01",
                    "last_trade_date": "2024-03-15",
                    "days_to_maturity": 74
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let contracts = client
        .list_futures_contracts(
            None,
            None,
            None,
            None,
            None,
            Some("ES"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("ESH4"),
            None,
            None,
            Some(true),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(contracts.len(), 1);
    assert_eq!(contracts[0].ticker.as_deref(), Some("ESH4"));
    assert_eq!(contracts[0].product_code.as_deref(), Some("ES"));
    assert_eq!(contracts[0].active, Some(true));
    assert_eq!(contracts[0].days_to_maturity, Some(74));
}

#[tokio::test]
async fn list_futures_products_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/futures/v1/products"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("product_code", "CL"))
        .and(query_param("trading_venue", "XNYM"))
        .and(query_param("date.gte", "2024-01-01"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "product_code": "CL",
                    "name": "WTI Crude Oil Futures",
                    "date": "2024-01-02",
                    "trading_venue": "XNYM",
                    "asset_class": "commodity",
                    "sector": "energy",
                    "type": "future",
                    "settlement_currency_code": "USD",
                    "unit_of_measure": "barrels",
                    "unit_of_measure_qty": 1000.0
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let products = client
        .list_futures_products(
            None,
            None,
            None,
            None,
            None,
            None,
            Some("CL"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("2024-01-01"),
            None,
            None,
            Some("XNYM"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(products.len(), 1);
    assert_eq!(products[0].product_code.as_deref(), Some("CL"));
    assert_eq!(products[0].name.as_deref(), Some("WTI Crude Oil Futures"));
    assert_eq!(products[0].trading_venue.as_deref(), Some("XNYM"));
}

#[tokio::test]
async fn list_futures_quotes_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/futures/v1/quotes/ESZ4"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("timestamp.gte", "1704067200000"))
        .and(query_param("session_end_date", "2024-01-02"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "ticker": "ESZ4",
                    "timestamp": 1704067200123i64,
                    "session_end_date": "2024-01-02",
                    "ask_price": 5211.0,
                    "ask_size": 15.0,
                    "bid_price": 5210.75,
                    "bid_size": 12.0,
                    "channel": 1,
                    "sequence_number": 100001i64
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let quotes = client
        .list_futures_quotes(
            "ESZ4",
            None,
            None,
            None,
            None,
            Some("1704067200000"),
            Some("2024-01-02"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(quotes.len(), 1);
    assert_eq!(quotes[0].ticker.as_deref(), Some("ESZ4"));
    assert_eq!(quotes[0].bid_price, Some(5210.75));
    assert_eq!(quotes[0].ask_price, Some(5211.0));
}

#[tokio::test]
async fn list_futures_trades_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/futures/v1/trades/CLZ4"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("timestamp.lt", "1704153600000"))
        .and(query_param("limit", "5"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 2,
            "results": [
                {
                    "ticker": "CLZ4",
                    "timestamp": 1704067200456i64,
                    "session_end_date": "2024-01-02",
                    "channel": 1,
                    "price": 71.25,
                    "size": 3.0,
                    "sequence_number": 200001i64
                },
                {
                    "ticker": "CLZ4",
                    "timestamp": 1704067200789i64,
                    "session_end_date": "2024-01-02",
                    "channel": 1,
                    "price": 71.30,
                    "size": 1.0,
                    "sequence_number": 200002i64
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let trades = client
        .list_futures_trades(
            "CLZ4",
            None,
            Some("1704153600000"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(5),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(trades.len(), 2);
    assert_eq!(trades[0].ticker.as_deref(), Some("CLZ4"));
    assert_eq!(trades[0].price, Some(71.25));
    assert_eq!(trades[1].size, Some(1.0));
}

#[tokio::test]
async fn list_futures_schedules_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/futures/v1/schedules"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("product_code", "ES"))
        .and(query_param("session_end_date", "2024-01-02"))
        .and(query_param("trading_venue", "XCME"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "event": "open",
                    "timestamp": "2024-01-01T18:00:00Z",
                    "session_end_date": "2024-01-02",
                    "product_code": "ES",
                    "trading_venue": "XCME",
                    "product_name": "E-mini S&P 500 Futures"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let schedules = client
        .list_futures_schedules(
            Some("ES"),
            None,
            None,
            None,
            None,
            None,
            Some("2024-01-02"),
            None,
            None,
            None,
            None,
            Some("XCME"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(schedules.len(), 1);
    assert_eq!(schedules[0].event.as_deref(), Some("open"));
    assert_eq!(schedules[0].product_code.as_deref(), Some("ES"));
    assert_eq!(schedules[0].timestamp.as_deref(), Some("2024-01-01T18:00:00Z"));
}

#[tokio::test]
async fn list_futures_market_statuses_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/futures/v1/market-status"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("product_code", "ES"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "market_event": "open",
                    "name": "E-mini S&P 500 Futures",
                    "product_code": "ES",
                    "session_end_date": "2024-01-02",
                    "timestamp": "2024-01-01T18:00:00Z",
                    "trading_venue": "XCME"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let statuses = client
        .list_futures_market_statuses(
            Some("ES"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(statuses.len(), 1);
    assert_eq!(statuses[0].market_event.as_deref(), Some("open"));
    assert_eq!(statuses[0].product_code.as_deref(), Some("ES"));
    assert_eq!(statuses[0].trading_venue.as_deref(), Some("XCME"));
}

#[tokio::test]
async fn get_futures_snapshot_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/futures/v1/snapshot"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("ticker.any_of", "ESZ4,ESH5"))
        .and(query_param("product_code", "ES"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "ticker": "ESZ4",
                    "product_code": "ES",
                    "details": {
                        "open_interest": 2100000,
                        "settlement_date": "2024-12-20",
                        "ticker": "ESZ4",
                        "product_code": "ES"
                    },
                    "last_quote": {
                        "ask": 5211.0,
                        "ask_size": 15,
                        "bid": 5210.75,
                        "bid_size": 12,
                        "timeframe": "REAL-TIME"
                    },
                    "last_trade": {
                        "price": 5210.75,
                        "size": 5,
                        "timeframe": "REAL-TIME"
                    },
                    "session": {
                        "change": 5.25,
                        "change_percent": 0.10,
                        "close": 5210.75,
                        "high": 5230.0,
                        "low": 5198.5,
                        "open": 5205.5,
                        "settlement_price": 5211.0,
                        "volume": 1500000.0
                    }
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let snapshots = client
        .get_futures_snapshot(
            None,
            Some("ESZ4,ESH5"),
            None,
            None,
            None,
            None,
            Some("ES"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(snapshots.len(), 1);
    let snap = &snapshots[0];
    assert_eq!(snap.ticker.as_deref(), Some("ESZ4"));
    assert_eq!(snap.product_code.as_deref(), Some("ES"));
    let trade = snap.last_trade.as_ref().unwrap();
    assert_eq!(trade.price, Some(5210.75));
    let session = snap.session.as_ref().unwrap();
    assert_eq!(session.change, Some(5.25));
}

#[tokio::test]
async fn list_futures_exchanges_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/futures/v1/exchanges"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("limit", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "acronym": "CME",
                    "id": "1",
                    "locale": "us",
                    "mic": "XCME",
                    "name": "Chicago Mercantile Exchange",
                    "operating_mic": "XCME",
                    "type": "exchange",
                    "url": "https://www.cmegroup.com"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let exchanges = client
        .list_futures_exchanges(Some(10), None)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(exchanges.len(), 1);
    assert_eq!(exchanges[0].mic.as_deref(), Some("XCME"));
    assert_eq!(exchanges[0].name.as_deref(), Some("Chicago Mercantile Exchange"));
    assert_eq!(exchanges[0].acronym.as_deref(), Some("CME"));
}
