//! Wiremock integration tests for the Stocks Financials API (src/rest/financials.rs).

use rust_decimal_macros::dec;

use futures::TryStreamExt;
use massive::rest::{FinancialsApi, ListFinancialsRatiosParams};
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
            None,         // cik group
            Some("AAPL"), // tickers
            None,
            None, // tickers_all_of, tickers_any_of
            None,
            None,
            Some("2023-01-01"), // period_end, period_end.gt, period_end.gte
            None,
            None, // period_end.lt, period_end.lte
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
    assert_eq!(sheet.total_assets, Some(dec!(352583000000.0)));
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
    assert_eq!(
        stmt.net_cash_from_operating_activities,
        Some(dec!(30543000000.0))
    );
    assert_eq!(
        stmt.net_cash_from_financing_activities,
        Some(dec!(-29633000000.0))
    );
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
    assert_eq!(stmt.revenue, Some(dec!(89498000000.0)));
    assert_eq!(stmt.basic_earnings_per_share, Some(dec!(1.47)));
    assert_eq!(
        stmt.net_income_loss_attributable_common_shareholders,
        Some(dec!(22956000000.0))
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
    assert_eq!(ratio.price, Some(dec!(176.65)));
    assert_eq!(ratio.price_to_earnings, Some(dec!(28.9)));
    assert_eq!(ratio.debt_to_equity, Some(dec!(1.79)));
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
            None, // ticker.lt, ticker.lte
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
    assert_eq!(float.free_float_percent, Some(dec!(99.87)));
}

#[tokio::test]
async fn list_ratios_with_params_serializes_every_field() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/stocks/financials/v1/ratios"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("ticker", "v_ticker"))
        .and(query_param("ticker.any_of", "v_ticker_any_of"))
        .and(query_param("ticker.gt", "v_ticker_gt"))
        .and(query_param("ticker.gte", "v_ticker_gte"))
        .and(query_param("ticker.lt", "v_ticker_lt"))
        .and(query_param("ticker.lte", "v_ticker_lte"))
        .and(query_param("cik", "v_cik"))
        .and(query_param("cik.any_of", "v_cik_any_of"))
        .and(query_param("cik.gt", "v_cik_gt"))
        .and(query_param("cik.gte", "v_cik_gte"))
        .and(query_param("cik.lt", "v_cik_lt"))
        .and(query_param("cik.lte", "v_cik_lte"))
        .and(query_param("price", "1"))
        .and(query_param("price.gt", "2"))
        .and(query_param("price.gte", "3"))
        .and(query_param("price.lt", "4"))
        .and(query_param("price.lte", "5"))
        .and(query_param("average_volume", "6"))
        .and(query_param("average_volume.gt", "7"))
        .and(query_param("average_volume.gte", "8"))
        .and(query_param("average_volume.lt", "9"))
        .and(query_param("average_volume.lte", "10"))
        .and(query_param("market_cap", "11"))
        .and(query_param("market_cap.gt", "12"))
        .and(query_param("market_cap.gte", "13"))
        .and(query_param("market_cap.lt", "14"))
        .and(query_param("market_cap.lte", "15"))
        .and(query_param("earnings_per_share", "16"))
        .and(query_param("earnings_per_share.gt", "17"))
        .and(query_param("earnings_per_share.gte", "18"))
        .and(query_param("earnings_per_share.lt", "19"))
        .and(query_param("earnings_per_share.lte", "20"))
        .and(query_param("price_to_earnings", "21"))
        .and(query_param("price_to_earnings.gt", "22"))
        .and(query_param("price_to_earnings.gte", "23"))
        .and(query_param("price_to_earnings.lt", "24"))
        .and(query_param("price_to_earnings.lte", "25"))
        .and(query_param("price_to_book", "26"))
        .and(query_param("price_to_book.gt", "27"))
        .and(query_param("price_to_book.gte", "28"))
        .and(query_param("price_to_book.lt", "29"))
        .and(query_param("price_to_book.lte", "30"))
        .and(query_param("price_to_sales", "31"))
        .and(query_param("price_to_sales.gt", "32"))
        .and(query_param("price_to_sales.gte", "33"))
        .and(query_param("price_to_sales.lt", "34"))
        .and(query_param("price_to_sales.lte", "35"))
        .and(query_param("price_to_cash_flow", "36"))
        .and(query_param("price_to_cash_flow.gt", "37"))
        .and(query_param("price_to_cash_flow.gte", "38"))
        .and(query_param("price_to_cash_flow.lt", "39"))
        .and(query_param("price_to_cash_flow.lte", "40"))
        .and(query_param("price_to_free_cash_flow", "41"))
        .and(query_param("price_to_free_cash_flow.gt", "42"))
        .and(query_param("price_to_free_cash_flow.gte", "43"))
        .and(query_param("price_to_free_cash_flow.lt", "44"))
        .and(query_param("price_to_free_cash_flow.lte", "45"))
        .and(query_param("dividend_yield", "46"))
        .and(query_param("dividend_yield.gt", "47"))
        .and(query_param("dividend_yield.gte", "48"))
        .and(query_param("dividend_yield.lt", "49"))
        .and(query_param("dividend_yield.lte", "50"))
        .and(query_param("return_on_assets", "51"))
        .and(query_param("return_on_assets.gt", "52"))
        .and(query_param("return_on_assets.gte", "53"))
        .and(query_param("return_on_assets.lt", "54"))
        .and(query_param("return_on_assets.lte", "55"))
        .and(query_param("return_on_equity", "56"))
        .and(query_param("return_on_equity.gt", "57"))
        .and(query_param("return_on_equity.gte", "58"))
        .and(query_param("return_on_equity.lt", "59"))
        .and(query_param("return_on_equity.lte", "60"))
        .and(query_param("debt_to_equity", "61"))
        .and(query_param("debt_to_equity.gt", "62"))
        .and(query_param("debt_to_equity.gte", "63"))
        .and(query_param("debt_to_equity.lt", "64"))
        .and(query_param("debt_to_equity.lte", "65"))
        .and(query_param("current", "66"))
        .and(query_param("current.gt", "67"))
        .and(query_param("current.gte", "68"))
        .and(query_param("current.lt", "69"))
        .and(query_param("current.lte", "70"))
        .and(query_param("quick", "71"))
        .and(query_param("quick.gt", "72"))
        .and(query_param("quick.gte", "73"))
        .and(query_param("quick.lt", "74"))
        .and(query_param("quick.lte", "75"))
        .and(query_param("cash", "76"))
        .and(query_param("cash.gt", "77"))
        .and(query_param("cash.gte", "78"))
        .and(query_param("cash.lt", "79"))
        .and(query_param("cash.lte", "80"))
        .and(query_param("ev_to_sales", "81"))
        .and(query_param("ev_to_sales.gt", "82"))
        .and(query_param("ev_to_sales.gte", "83"))
        .and(query_param("ev_to_sales.lt", "84"))
        .and(query_param("ev_to_sales.lte", "85"))
        .and(query_param("ev_to_ebitda", "86"))
        .and(query_param("ev_to_ebitda.gt", "87"))
        .and(query_param("ev_to_ebitda.gte", "88"))
        .and(query_param("ev_to_ebitda.lt", "89"))
        .and(query_param("ev_to_ebitda.lte", "90"))
        .and(query_param("enterprise_value", "91"))
        .and(query_param("enterprise_value.gt", "92"))
        .and(query_param("enterprise_value.gte", "93"))
        .and(query_param("enterprise_value.lt", "94"))
        .and(query_param("enterprise_value.lte", "95"))
        .and(query_param("free_cash_flow", "96"))
        .and(query_param("free_cash_flow.gt", "97"))
        .and(query_param("free_cash_flow.gte", "98"))
        .and(query_param("free_cash_flow.lt", "99"))
        .and(query_param("free_cash_flow.lte", "100"))
        .and(query_param("limit", "100"))
        .and(query_param("sort", "v_sort"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 0,
            "results": []
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let ratios = client
        .list_financials_ratios_with_params(
            ListFinancialsRatiosParams::new()
                .ticker("v_ticker")
                .ticker_any_of("v_ticker_any_of")
                .ticker_gt("v_ticker_gt")
                .ticker_gte("v_ticker_gte")
                .ticker_lt("v_ticker_lt")
                .ticker_lte("v_ticker_lte")
                .cik("v_cik")
                .cik_any_of("v_cik_any_of")
                .cik_gt("v_cik_gt")
                .cik_gte("v_cik_gte")
                .cik_lt("v_cik_lt")
                .cik_lte("v_cik_lte")
                .price(1.0)
                .price_gt(2.0)
                .price_gte(3.0)
                .price_lt(4.0)
                .price_lte(5.0)
                .average_volume(6.0)
                .average_volume_gt(7.0)
                .average_volume_gte(8.0)
                .average_volume_lt(9.0)
                .average_volume_lte(10.0)
                .market_cap(11.0)
                .market_cap_gt(12.0)
                .market_cap_gte(13.0)
                .market_cap_lt(14.0)
                .market_cap_lte(15.0)
                .earnings_per_share(16.0)
                .earnings_per_share_gt(17.0)
                .earnings_per_share_gte(18.0)
                .earnings_per_share_lt(19.0)
                .earnings_per_share_lte(20.0)
                .price_to_earnings(21.0)
                .price_to_earnings_gt(22.0)
                .price_to_earnings_gte(23.0)
                .price_to_earnings_lt(24.0)
                .price_to_earnings_lte(25.0)
                .price_to_book(26.0)
                .price_to_book_gt(27.0)
                .price_to_book_gte(28.0)
                .price_to_book_lt(29.0)
                .price_to_book_lte(30.0)
                .price_to_sales(31.0)
                .price_to_sales_gt(32.0)
                .price_to_sales_gte(33.0)
                .price_to_sales_lt(34.0)
                .price_to_sales_lte(35.0)
                .price_to_cash_flow(36.0)
                .price_to_cash_flow_gt(37.0)
                .price_to_cash_flow_gte(38.0)
                .price_to_cash_flow_lt(39.0)
                .price_to_cash_flow_lte(40.0)
                .price_to_free_cash_flow(41.0)
                .price_to_free_cash_flow_gt(42.0)
                .price_to_free_cash_flow_gte(43.0)
                .price_to_free_cash_flow_lt(44.0)
                .price_to_free_cash_flow_lte(45.0)
                .dividend_yield(46.0)
                .dividend_yield_gt(47.0)
                .dividend_yield_gte(48.0)
                .dividend_yield_lt(49.0)
                .dividend_yield_lte(50.0)
                .return_on_assets(51.0)
                .return_on_assets_gt(52.0)
                .return_on_assets_gte(53.0)
                .return_on_assets_lt(54.0)
                .return_on_assets_lte(55.0)
                .return_on_equity(56.0)
                .return_on_equity_gt(57.0)
                .return_on_equity_gte(58.0)
                .return_on_equity_lt(59.0)
                .return_on_equity_lte(60.0)
                .debt_to_equity(61.0)
                .debt_to_equity_gt(62.0)
                .debt_to_equity_gte(63.0)
                .debt_to_equity_lt(64.0)
                .debt_to_equity_lte(65.0)
                .current(66.0)
                .current_gt(67.0)
                .current_gte(68.0)
                .current_lt(69.0)
                .current_lte(70.0)
                .quick(71.0)
                .quick_gt(72.0)
                .quick_gte(73.0)
                .quick_lt(74.0)
                .quick_lte(75.0)
                .cash(76.0)
                .cash_gt(77.0)
                .cash_gte(78.0)
                .cash_lt(79.0)
                .cash_lte(80.0)
                .ev_to_sales(81.0)
                .ev_to_sales_gt(82.0)
                .ev_to_sales_gte(83.0)
                .ev_to_sales_lt(84.0)
                .ev_to_sales_lte(85.0)
                .ev_to_ebitda(86.0)
                .ev_to_ebitda_gt(87.0)
                .ev_to_ebitda_gte(88.0)
                .ev_to_ebitda_lt(89.0)
                .ev_to_ebitda_lte(90.0)
                .enterprise_value(91.0)
                .enterprise_value_gt(92.0)
                .enterprise_value_gte(93.0)
                .enterprise_value_lt(94.0)
                .enterprise_value_lte(95.0)
                .free_cash_flow(96.0)
                .free_cash_flow_gt(97.0)
                .free_cash_flow_gte(98.0)
                .free_cash_flow_lt(99.0)
                .free_cash_flow_lte(100.0)
                .limit(100)
                .sort("v_sort"),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert!(ratios.is_empty());
}
