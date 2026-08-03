use futures::TryStreamExt;
use massive::{rest::ReferenceApi, Client};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const AUTH: (&str, &str) = ("Authorization", "Bearer test-key");

fn client(server: &MockServer) -> Client {
    Client::new("test-key").unwrap().with_base(server.uri())
}

#[tokio::test]
async fn get_market_holidays_returns_list() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/marketstatus/upcoming"))
        .and(header(AUTH.0, AUTH.1))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "close": "2024-12-25T05:00:00Z",
                "date": "2024-12-25",
                "exchange": "NYSE",
                "name": "Christmas",
                "open": "2024-12-26T14:30:00Z",
                "status": "closed"
            },
            {
                "close": "2025-01-01T05:00:00Z",
                "date": "2025-01-01",
                "exchange": "NYSE",
                "name": "New Year's Day",
                "open": "2025-01-02T14:30:00Z",
                "status": "closed"
            }
        ])))
        .expect(1)
        .mount(&server)
        .await;

    let holidays = client(&server).get_market_holidays(None).await.unwrap();
    assert_eq!(holidays.len(), 2);
    assert_eq!(holidays[0].name.as_deref(), Some("Christmas"));
    assert_eq!(holidays[0].date.as_deref(), Some("2024-12-25"));
    assert_eq!(holidays[0].exchange.as_deref(), Some("NYSE"));
    assert_eq!(holidays[1].status.as_deref(), Some("closed"));
}

#[tokio::test]
async fn get_market_status_returns_status() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/marketstatus/now"))
        .and(header(AUTH.0, AUTH.1))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "afterHours": true,
            "earlyHours": false,
            "market": "extended-hours",
            "serverTime": "2024-06-14T20:15:00-04:00",
            "currencies": { "crypto": "open", "fx": "open" },
            "exchanges": { "nasdaq": "extended-hours", "nyse": "extended-hours", "otc": "closed" },
            "indicesGroups": { "s_and_p": "closed", "nasdaq": "closed" }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let status = client(&server).get_market_status(None).await.unwrap();
    assert_eq!(status.market.as_deref(), Some("extended-hours"));
    assert_eq!(status.after_hours, Some(true));
    assert_eq!(status.early_hours, Some(false));
    assert_eq!(
        status.exchanges.and_then(|e| e.nasdaq).as_deref(),
        Some("extended-hours")
    );
}

#[tokio::test]
async fn list_tickers_collects_page_and_sends_filters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v3/reference/tickers"))
        .and(header(AUTH.0, AUTH.1))
        .and(query_param("ticker.gte", "A"))
        .and(query_param("market", "stocks"))
        .and(query_param("active", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 2,
            "results": [
                {
                    "ticker": "A",
                    "name": "Agilent Technologies Inc.",
                    "market": "stocks",
                    "locale": "us",
                    "primary_exchange": "XNYS",
                    "type": "CS",
                    "active": true,
                    "currency_name": "usd"
                },
                {
                    "ticker": "AAPL",
                    "name": "Apple Inc.",
                    "market": "stocks",
                    "locale": "us",
                    "primary_exchange": "XNAS",
                    "type": "CS",
                    "active": true,
                    "currency_name": "usd",
                    "cik": "0000320193"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let tickers: Vec<_> = client(&server)
        .list_tickers(
            None,
            None,
            None,
            None,
            Some("A"), // ticker_gte
            Some("CS"),
            Some("stocks"),
            None,
            None,
            None,
            None,
            Some(true), // active
            None,
            Some(10),
            None,
            None,
            None,
        )
        .try_collect()
        .await
        .unwrap();

    assert_eq!(tickers.len(), 2);
    assert_eq!(tickers[1].ticker.as_deref(), Some("AAPL"));
    assert_eq!(tickers[1].name.as_deref(), Some("Apple Inc."));
    assert_eq!(tickers[1].cik.as_deref(), Some("0000320193"));
    assert_eq!(tickers[0].type_.as_deref(), Some("CS"));
}

#[tokio::test]
async fn get_ticker_details_unwraps_results() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v3/reference/tickers/AAPL"))
        .and(header(AUTH.0, AUTH.1))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "request_id": "req-1",
            "results": {
                "ticker": "AAPL",
                "name": "Apple Inc.",
                "market": "stocks",
                "locale": "us",
                "primary_exchange": "XNAS",
                "type": "CS",
                "active": true,
                "currency_name": "usd",
                "cik": "0000320193",
                "market_cap": 3000000000000.0,
                "sic_description": "ELECTRONIC COMPUTERS",
                "total_employees": 154000,
                "list_date": "1980-12-12"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let details = client(&server)
        .get_ticker_details("AAPL", None, None)
        .await
        .unwrap();
    assert_eq!(details.ticker.as_deref(), Some("AAPL"));
    assert_eq!(details.name.as_deref(), Some("Apple Inc."));
    assert_eq!(details.market_cap, Some(3000000000000.0));
    assert_eq!(
        details.sic_description.as_deref(),
        Some("ELECTRONIC COMPUTERS")
    );
}

#[tokio::test]
async fn list_ticker_news_collects_page_and_sends_filters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/reference/news"))
        .and(header(AUTH.0, AUTH.1))
        .and(query_param("ticker", "AAPL"))
        .and(query_param("published_utc.gte", "2024-06-01"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "id": "abc123",
                    "title": "Apple unveils new chip",
                    "author": "Jane Doe",
                    "published_utc": "2024-06-10T12:00:00Z",
                    "article_url": "https://example.com/article",
                    "tickers": ["AAPL"],
                    "publisher": {
                        "name": "Example News",
                        "homepage_url": "https://example.com"
                    }
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let news: Vec<_> = client(&server)
        .list_ticker_news(
            Some("AAPL"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("2024-06-01"), // published_utc_gte
            None,
            None,
            None,
            None,
        )
        .try_collect()
        .await
        .unwrap();

    assert_eq!(news.len(), 1);
    assert_eq!(news[0].title.as_deref(), Some("Apple unveils new chip"));
    assert_eq!(news[0].tickers.as_ref().unwrap(), &vec!["AAPL".to_string()]);
    assert_eq!(
        news[0].publisher.as_ref().unwrap().name.as_deref(),
        Some("Example News")
    );
}

#[tokio::test]
async fn get_ticker_types_unwraps_results_list() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v3/reference/tickers/types"))
        .and(header(AUTH.0, AUTH.1))
        .and(query_param("asset_class", "stocks"))
        .and(query_param("locale", "us"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "results": [
                {
                    "asset_class": "stocks",
                    "code": "CS",
                    "description": "Common Stock",
                    "locale": "us"
                },
                {
                    "asset_class": "stocks",
                    "code": "PFD",
                    "description": "Preferred Stock",
                    "locale": "us"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let types = client(&server)
        .get_ticker_types(Some("stocks"), Some("us"), None)
        .await
        .unwrap();
    assert_eq!(types.len(), 2);
    assert_eq!(types[0].code.as_deref(), Some("CS"));
    assert_eq!(types[0].description.as_deref(), Some("Common Stock"));
    assert_eq!(types[1].asset_class.as_deref(), Some("stocks"));
}

#[tokio::test]
async fn list_splits_collects_page_and_sends_filters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v3/reference/splits"))
        .and(header(AUTH.0, AUTH.1))
        .and(query_param("ticker", "AAPL"))
        .and(query_param("execution_date.gte", "2020-01-01"))
        .and(query_param("reverse_split", "false"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "id": 42,
                    "execution_date": "2020-08-31",
                    "split_from": 1,
                    "split_to": 4,
                    "ticker": "AAPL"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let splits: Vec<_> = client(&server)
        .list_splits(
            Some("AAPL"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("2020-01-01"), // execution_date_gte
            Some(false),        // reverse_split
            None,
            None,
            None,
            None,
        )
        .try_collect()
        .await
        .unwrap();

    assert_eq!(splits.len(), 1);
    assert_eq!(splits[0].ticker.as_deref(), Some("AAPL"));
    assert_eq!(splits[0].split_from, Some(1));
    assert_eq!(splits[0].split_to, Some(4));
}

#[tokio::test]
async fn list_dividends_collects_page_and_sends_filters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v3/reference/dividends"))
        .and(header(AUTH.0, AUTH.1))
        .and(query_param("ticker", "AAPL"))
        .and(query_param("ex_dividend_date.gte", "2024-01-01"))
        .and(query_param("dividend_type", "CD"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "id": 7,
                    "cash_amount": 0.24,
                    "currency": "USD",
                    "declaration_date": "2024-05-02",
                    "dividend_type": "CD",
                    "ex_dividend_date": "2024-05-10",
                    "frequency": 4,
                    "pay_date": "2024-05-16",
                    "record_date": "2024-05-13",
                    "ticker": "AAPL"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let dividends: Vec<_> = client(&server)
        .list_dividends(
            Some("AAPL"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("2024-01-01"), // ex_dividend_date_gte
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
            None,
            None,
            None,
            Some("CD"), // dividend_type
            None,
            None,
            None,
            None,
        )
        .try_collect()
        .await
        .unwrap();

    assert_eq!(dividends.len(), 1);
    assert_eq!(dividends[0].ticker.as_deref(), Some("AAPL"));
    assert_eq!(dividends[0].cash_amount, Some(0.24));
    assert_eq!(dividends[0].frequency, Some(4));
    assert_eq!(dividends[0].dividend_type.as_deref(), Some("CD"));
}

#[tokio::test]
async fn list_conditions_collects_page_and_sends_filters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v3/reference/conditions"))
        .and(header(AUTH.0, AUTH.1))
        .and(query_param("asset_class", "stocks"))
        .and(query_param("data_type", "trade"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "abbreviation": "T",
                    "asset_class": "stocks",
                    "data_types": ["trade"],
                    "description": "Regular trade",
                    "id": 1,
                    "name": "Regular",
                    "type": "sale_condition",
                    "sip_mapping": { "CTA": " ", "UTP": " " }
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let conditions: Vec<_> = client(&server)
        .list_conditions(
            Some("stocks"),
            Some("trade"),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .try_collect()
        .await
        .unwrap();

    assert_eq!(conditions.len(), 1);
    assert_eq!(conditions[0].id, Some(1));
    assert_eq!(conditions[0].name.as_deref(), Some("Regular"));
    assert_eq!(conditions[0].type_.as_deref(), Some("sale_condition"));
}

#[tokio::test]
async fn get_exchanges_unwraps_results_list() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v3/reference/exchanges"))
        .and(header(AUTH.0, AUTH.1))
        .and(query_param("asset_class", "stocks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "results": [
                {
                    "acronym": "NASDAQ",
                    "asset_class": "stocks",
                    "id": 4,
                    "locale": "us",
                    "mic": "XNAS",
                    "name": "Nasdaq Stock Market",
                    "operating_mic": "XNAS",
                    "type": "exchange",
                    "url": "https://www.nasdaq.com"
                },
                {
                    "acronym": "NYSE",
                    "asset_class": "stocks",
                    "id": 10,
                    "locale": "us",
                    "mic": "XNYS",
                    "name": "New York Stock Exchange",
                    "operating_mic": "XNYS",
                    "type": "exchange",
                    "url": "https://www.nyse.com"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let exchanges = client(&server)
        .get_exchanges(Some("stocks"), None, None)
        .await
        .unwrap();
    assert_eq!(exchanges.len(), 2);
    assert_eq!(exchanges[0].mic.as_deref(), Some("XNAS"));
    assert_eq!(
        exchanges[1].name.as_deref(),
        Some("New York Stock Exchange")
    );
    assert_eq!(exchanges[0].type_.as_deref(), Some("exchange"));
}

#[tokio::test]
async fn get_options_contract_unwraps_results() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v3/reference/options/contracts/O:AAPL241220C00170500"))
        .and(header(AUTH.0, AUTH.1))
        .and(query_param("as_of", "2024-06-14"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "request_id": "req-2",
            "results": {
                "cfi": "OCASPS",
                "contract_type": "call",
                "exercise_style": "american",
                "expiration_date": "2024-12-20",
                "primary_exchange": "BATO",
                "shares_per_contract": 100,
                "strike_price": 170.5,
                "ticker": "O:AAPL241220C00170500",
                "underlying_ticker": "AAPL"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let contract = client(&server)
        .get_options_contract("O:AAPL241220C00170500", Some("2024-06-14"), None)
        .await
        .unwrap();
    assert_eq!(contract.contract_type.as_deref(), Some("call"));
    assert_eq!(contract.strike_price, Some(170.5));
    assert_eq!(contract.underlying_ticker.as_deref(), Some("AAPL"));
    assert_eq!(contract.shares_per_contract, Some(100.0));
}

#[tokio::test]
async fn list_options_contracts_collects_page_and_sends_filters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v3/reference/options/contracts"))
        .and(header(AUTH.0, AUTH.1))
        .and(query_param("underlying_ticker", "AAPL"))
        .and(query_param("expiration_date.gte", "2024-06-01"))
        .and(query_param("expired", "false"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 2,
            "results": [
                {
                    "cfi": "OCASPS",
                    "contract_type": "call",
                    "exercise_style": "american",
                    "expiration_date": "2024-12-20",
                    "primary_exchange": "BATO",
                    "shares_per_contract": 100,
                    "strike_price": 170.5,
                    "ticker": "O:AAPL241220C00170500",
                    "underlying_ticker": "AAPL"
                },
                {
                    "cfi": "OPASPS",
                    "contract_type": "put",
                    "exercise_style": "american",
                    "expiration_date": "2024-12-20",
                    "primary_exchange": "BATO",
                    "shares_per_contract": 100,
                    "strike_price": 165.0,
                    "ticker": "O:AAPL241220P00165000",
                    "underlying_ticker": "AAPL"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let contracts: Vec<_> = client(&server)
        .list_options_contracts(
            Some("AAPL"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("2024-06-01"), // expiration_date_gte
            None,
            None,
            None,
            None,
            None,
            None,
            Some(false), // expired
            None,
            None,
            None,
            None,
        )
        .try_collect()
        .await
        .unwrap();

    assert_eq!(contracts.len(), 2);
    assert_eq!(contracts[0].contract_type.as_deref(), Some("call"));
    assert_eq!(contracts[1].contract_type.as_deref(), Some("put"));
    assert_eq!(contracts[0].expiration_date.as_deref(), Some("2024-12-20"));
}

#[tokio::test]
async fn list_short_interest_collects_page_and_sends_filters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/stocks/v1/short-interest"))
        .and(header(AUTH.0, AUTH.1))
        .and(query_param("ticker", "GME"))
        .and(query_param("settlement_date.gte", "2024-01-01"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "avg_daily_volume": 50000000,
                    "days_to_cover": 1.5,
                    "settlement_date": "2024-06-14",
                    "short_interest": 75000000,
                    "ticker": "GME"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let rows: Vec<_> = client(&server)
        .list_short_interest(
            Some("GME"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("2024-01-01"), // settlement_date_gte
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
        .try_collect()
        .await
        .unwrap();

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].ticker.as_deref(), Some("GME"));
    assert_eq!(rows[0].short_interest, Some(75000000));
    assert_eq!(rows[0].days_to_cover, Some(1.5));
}

#[tokio::test]
async fn list_short_volume_collects_page_and_sends_filters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/stocks/v1/short-volume"))
        .and(header(AUTH.0, AUTH.1))
        .and(query_param("ticker", "AAPL"))
        .and(query_param("date.gte", "2024-06-01"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "adf_short_volume": 1000,
                    "date": "2024-06-14",
                    "exempt_volume": 2000,
                    "short_volume": 10000000,
                    "short_volume_ratio": 32.5,
                    "ticker": "AAPL",
                    "total_volume": 30000000
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let rows: Vec<_> = client(&server)
        .list_short_volume(
            Some("AAPL"),
            None,
            None,
            None,
            None,
            Some("2024-06-01"), // date_gte
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
        .try_collect()
        .await
        .unwrap();

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].ticker.as_deref(), Some("AAPL"));
    assert_eq!(rows[0].short_volume, Some(10000000));
    assert_eq!(rows[0].short_volume_ratio, Some(32.5));
    assert_eq!(rows[0].total_volume, Some(30000000));
}

#[tokio::test]
async fn list_stocks_filings_index_collects_page_and_sends_filters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/stocks/filings/vX/index"))
        .and(header(AUTH.0, AUTH.1))
        .and(query_param("ticker", "AAPL"))
        .and(query_param("form_type", "10-K"))
        .and(query_param("filing_date.gte", "2024-01-01"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "accession_number": "0000320193-24-000123",
                    "cik": "0000320193",
                    "filing_date": "2024-11-01",
                    "filing_url": "https://example.com/filing.htm",
                    "form_type": "10-K",
                    "issuer_name": "Apple Inc.",
                    "ticker": "AAPL"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let filings: Vec<_> = client(&server)
        .list_stocks_filings_index(
            None,
            None,
            None,
            None,
            None,
            None,
            Some("AAPL"),
            None,
            None,
            None,
            None,
            None,
            Some("10-K"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("2024-01-01"), // filing_date_gte
            None,
            None,
            None,
            None,
            None,
        )
        .try_collect()
        .await
        .unwrap();

    assert_eq!(filings.len(), 1);
    assert_eq!(filings[0].form_type.as_deref(), Some("10-K"));
    assert_eq!(
        filings[0].accession_number.as_deref(),
        Some("0000320193-24-000123")
    );
    assert_eq!(filings[0].issuer_name.as_deref(), Some("Apple Inc."));
}

#[tokio::test]
async fn list_stocks_filings_8k_text_collects_page_and_sends_filters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/stocks/filings/8-K/vX/text"))
        .and(header(AUTH.0, AUTH.1))
        .and(query_param("ticker", "AAPL"))
        .and(query_param("form_type", "8-K"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "accession_number": "0000320193-24-000100",
                    "cik": "0000320193",
                    "filing_date": "2024-08-05",
                    "filing_url": "https://example.com/8k.htm",
                    "form_type": "8-K",
                    "items_text": "Item 5.02 Departure of Directors or Certain Officers",
                    "ticker": "AAPL"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let filings: Vec<_> = client(&server)
        .list_stocks_filings_8k_text(
            None,
            None,
            None,
            None,
            None,
            None,
            Some("AAPL"),
            None,
            None,
            None,
            None,
            None,
            Some("8-K"),
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
        .try_collect()
        .await
        .unwrap();

    assert_eq!(filings.len(), 1);
    assert_eq!(filings[0].form_type.as_deref(), Some("8-K"));
    assert_eq!(filings[0].ticker.as_deref(), Some("AAPL"));
    assert!(filings[0]
        .items_text
        .as_deref()
        .unwrap()
        .contains("Item 5.02"));
}
