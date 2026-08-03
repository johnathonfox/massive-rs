//! Wiremock integration tests for the Stocks Financials API (src/rest/financials.rs).

use futures::TryStreamExt;
use massive::rest::FinancialsApi;
use massive::Client;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn list_balance_sheets_hits_expected_path_and_params() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/stocks/financials/v1/balance-sheets"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("tickers", "AAPL"))
        .and(query_param("period_end.gte", "2023-01-01"))
        .and(query_param("limit", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [{
                "cik": "0000320193",
                "tickers": ["AAPL"],
                "filing_date": "2023-11-03",
                "fiscal_year": 2023.0,
                "fiscal_quarter": 4.0,
                "period_end": "2023-09-30",
                "timeframe": "quarterly",
                "cash_and_equivalents": 29965000000.0,
                "total_assets": 352583000000.0,
                "total_liabilities": 290437000000.0,
                "total_equity": 62146000000.0
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let sheets = client
        .list_financials_balance_sheets(
            None,
            None,
            None,
            None,
            None,
            None,             // cik group
            Some("AAPL"),     // tickers
            None,
            None,             // tickers_all_of, tickers_any_of
            None,
            None,
            Some("2023-01-01"), // period_end, period_end.gt, period_end.gte
            None,
            None,             // period_end.lt, period_end.lte
            None,
            None,
            None,
            None,
            None, // filing_date group
            None,
            None,
            None,
            None,
            None, // fiscal_year group
            None,
            None,
            None,
            None,
            None, // fiscal_quarter group
            None,
            None,
            None,
            None,
            None,
            None, // timeframe group
            Some(10),
            Some("period_end"),
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(sheets.len(), 1);
    let sheet = &sheets[0];
    assert_eq!(sheet.cik.as_deref(), Some("0000320193"));
    assert_eq!(sheet.tickers.as_deref(), Some(&["AAPL".to_string()][..]));
    assert_eq!(sheet.total_assets, Some(352583000000.0));
    assert_eq!(sheet.period_end.as_deref(), Some("2023-09-30"));
}

#[tokio::test]
async fn list_cash_flow_statements_hits_expected_path_and_params() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/stocks/financials/v1/cash-flow-statements"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("tickers", "AAPL"))
        .and(query_param("filing_date.gte", "2023-01-01"))
        .and(query_param("timeframe", "quarterly"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [{
                "cik": "0000320193",
                "tickers": ["AAPL"],
                "filing_date": "2023-11-03",
                "period_end": "2023-09-30",
                "timeframe": "quarterly",
                "net_cash_from_operating_activities": 30543000000.0,
                "net_cash_from_investing_activities": -2845000000.0,
                "net_cash_from_financing_activities": -29633000000.0,
                "purchase_of_property_plant_and_equipment": -2392000000.0
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let statements = client
        .list_financials_cash_flow_statements(
            None,
            None,
            None,
            None,
            None,
            None, // cik group
            None,
            None,
            None,
            None,
            None, // period_end group
            None,
            None,
            Some("2023-01-01"), // filing_date, filing_date.gt, filing_date.gte
            None,
            None, // filing_date.lt, filing_date.lte
            Some("AAPL"),
            None,
            None, // tickers group
            None,
            None,
            None,
            None,
            None, // fiscal_year group
            None,
            None,
            None,
            None,
            None, // fiscal_quarter group
            Some("quarterly"),
            None,
            None,
            None,
            None,
            None, // timeframe group
            Some(10),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(statements.len(), 1);
    let stmt = &statements[0];
    assert_eq!(stmt.net_cash_from_operating_activities, Some(30543000000.0));
    assert_eq!(stmt.net_cash_from_financing_activities, Some(-29633000000.0));
    assert_eq!(stmt.period_end.as_deref(), Some("2023-09-30"));
}

#[tokio::test]
async fn list_income_statements_hits_expected_path_and_params() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/stocks/financials/v1/income-statements"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("tickers.any_of", "AAPL,MSFT"))
        .and(query_param("fiscal_year", "2023"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [{
                "cik": "0000320193",
                "tickers": ["AAPL"],
                "filing_date": "2023-11-03",
                "fiscal_year": 2023.0,
                "fiscal_quarter": 4.0,
                "period_end": "2023-09-30",
                "timeframe": "quarterly",
                "revenue": 89498000000.0,
                "cost_of_revenue": -49471000000.0,
                "gross_profit": 40027000000.0,
                "operating_income": 26969000000.0,
                "net_income_loss_attributable_common_shareholders": 22956000000.0,
                "basic_earnings_per_share": 1.47,
                "diluted_earnings_per_share": 1.46
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let statements = client
        .list_financials_income_statements(
            None,
            None,
            None,
            None,
            None,
            None, // cik group
            None,
            None,
            Some("AAPL,MSFT"), // tickers, tickers_all_of, tickers.any_of
            None,
            None,
            None,
            None,
            None, // period_end group
            None,
            None,
            None,
            None,
            None, // filing_date group
            Some(2023.0),
            None,
            None,
            None,
            None, // fiscal_year group
            None,
            None,
            None,
            None,
            None, // fiscal_quarter group
            None,
            None,
            None,
            None,
            None,
            None, // timeframe group
            Some(5),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(statements.len(), 1);
    let stmt = &statements[0];
    assert_eq!(stmt.revenue, Some(89498000000.0));
    assert_eq!(stmt.basic_earnings_per_share, Some(1.47));
    assert_eq!(
        stmt.net_income_loss_attributable_common_shareholders,
        Some(22956000000.0)
    );
}

#[tokio::test]
async fn list_ratios_hits_expected_path_and_params() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/stocks/financials/v1/ratios"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("ticker", "AAPL"))
        .and(query_param("market_cap.gte", "1000000000000"))
        .and(query_param("limit", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [{
                "ticker": "AAPL",
                "cik": "0000320193",
                "date": "2023-11-03",
                "price": 176.65,
                "market_cap": 2750000000000.0,
                "earnings_per_share": 6.11,
                "price_to_earnings": 28.9,
                "price_to_sales": 7.2,
                "debt_to_equity": 1.79,
                "return_on_equity": 1.6,
                "dividend_yield": 0.0054,
                "free_cash_flow": 99585000000.0
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let ratios = client
        .list_financials_ratios(
            Some("AAPL"), // ticker
            None,
            None,
            None,
            None,
            None, // ticker_any_of, ticker.gt/gte/lt/lte
            None,
            None,
            None,
            None,
            None,
            None, // cik group
            None,
            None,
            None,
            None,
            None, // price group
            None,
            None,
            None,
            None,
            None, // average_volume group
            None,
            None,
            Some(1e12), // market_cap, market_cap.gt, market_cap.gte
            None,
            None, // market_cap.lt, market_cap.lte
            None,
            None,
            None,
            None,
            None, // earnings_per_share group
            None,
            None,
            None,
            None,
            None, // price_to_earnings group
            None,
            None,
            None,
            None,
            None, // price_to_book group
            None,
            None,
            None,
            None,
            None, // price_to_sales group
            None,
            None,
            None,
            None,
            None, // price_to_cash_flow group
            None,
            None,
            None,
            None,
            None, // price_to_free_cash_flow group
            None,
            None,
            None,
            None,
            None, // dividend_yield group
            None,
            None,
            None,
            None,
            None, // return_on_assets group
            None,
            None,
            None,
            None,
            None, // return_on_equity group
            None,
            None,
            None,
            None,
            None, // debt_to_equity group
            None,
            None,
            None,
            None,
            None, // current group
            None,
            None,
            None,
            None,
            None, // quick group
            None,
            None,
            None,
            None,
            None, // cash group
            None,
            None,
            None,
            None,
            None, // ev_to_sales group
            None,
            None,
            None,
            None,
            None, // ev_to_ebitda group
            None,
            None,
            None,
            None,
            None, // enterprise_value group
            None,
            None,
            None,
            None,
            None, // free_cash_flow group
            Some(10),
            Some("ticker"),
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(ratios.len(), 1);
    let ratio = &ratios[0];
    assert_eq!(ratio.ticker.as_deref(), Some("AAPL"));
    assert_eq!(ratio.price, Some(176.65));
    assert_eq!(ratio.price_to_earnings, Some(28.9));
    assert_eq!(ratio.debt_to_equity, Some(1.79));
}

#[tokio::test]
async fn list_stocks_floats_hits_expected_path_and_params() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/stocks/vX/float"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("ticker.gte", "A"))
        .and(query_param("free_float_percent.gte", "50"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [{
                "ticker": "AAPL",
                "free_float": 15400000000i64,
                "free_float_percent": 99.87,
                "effective_date": "2024-01-01"
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let floats = client
        .list_stocks_floats(
            None,
            None,
            None,
            Some("A"), // ticker, ticker.any_of, ticker.gt, ticker.gte
            None,
            None,      // ticker.lt, ticker.lte
            None,
            None,
            Some(50.0), // free_float_percent, .gt, .gte
            None,
            None, // free_float_percent.lt, .lte
            Some(5),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(floats.len(), 1);
    let float = &floats[0];
    assert_eq!(float.ticker.as_deref(), Some("AAPL"));
    assert_eq!(float.free_float, Some(15400000000));
    assert_eq!(float.free_float_percent, Some(99.87));
}
