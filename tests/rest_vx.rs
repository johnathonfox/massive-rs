//! Wiremock integration tests for the experimental vX API (src/rest/vx.rs).

use futures::TryStreamExt;
use massive::rest::VxApi;
use massive::Client;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn list_stock_financials_hits_expected_path_and_params() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/vX/reference/financials"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("ticker", "AAPL"))
        .and(query_param("filing_date.gte", "2023-01-01"))
        .and(query_param("timeframe", "quarterly"))
        .and(query_param("include_sources", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [{
                "cik": "0000320193",
                "company_name": "Apple Inc.",
                "start_date": "2023-07-02",
                "end_date": "2023-09-30",
                "filing_date": "2023-11-03",
                "fiscal_period": "Q4",
                "fiscal_year": "2023",
                "source_filing_url": "https://www.sec.gov/Archives/edgar/data/320193/example.htm",
                "financials": {
                    "balance_sheet": {
                        "assets": {
                            "label": "Assets",
                            "value": 352583000000.0,
                            "unit": "USD"
                        }
                    },
                    "income_statement": {
                        "revenues": {
                            "label": "Revenues",
                            "value": 89498000000.0,
                            "unit": "USD"
                        }
                    }
                }
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let financials = client
        .list_stock_financials(
            Some("AAPL"), // ticker
            None,         // cik
            None,         // company_name
            None,         // company_name_search
            None,         // sic
            None,         // filing_date
            None,         // filing_date.lt
            None,         // filing_date.lte
            None,         // filing_date.gt
            Some("2023-01-01"), // filing_date.gte
            None,         // period_of_report_date
            None,         // period_of_report_date.lt
            None,         // period_of_report_date.lte
            None,         // period_of_report_date.gt
            None,         // period_of_report_date.gte
            Some("quarterly"), // timeframe
            Some(true),   // include_sources
            Some(10),     // limit
            None,         // sort
            None,         // order
            None,         // options
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(financials.len(), 1);
    let fin = &financials[0];
    assert_eq!(fin.company_name.as_deref(), Some("Apple Inc."));
    assert_eq!(fin.fiscal_period.as_deref(), Some("Q4"));
    assert_eq!(fin.fiscal_year.as_deref(), Some("2023"));
    let assets = fin
        .financials
        .as_ref()
        .and_then(|f| f.balance_sheet.as_ref())
        .and_then(|bs| bs.assets.as_ref())
        .expect("balance sheet assets data point");
    assert_eq!(assets.value, Some(352583000000.0));
}

#[tokio::test]
async fn list_ipos_hits_expected_path_and_params() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/vX/reference/ipos"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("listing_date.gte", "2023-01-01"))
        .and(query_param("ipo_status", "history"))
        .and(query_param("order", "asc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [{
                "ticker": "ARM",
                "issuer_name": "Arm Holdings plc",
                "announced_date": "2023-08-21",
                "listing_date": "2023-09-14",
                "ipo_status": "history",
                "primary_exchange": "XNAS",
                "isin": "US0420682058",
                "us_code": "042068205",
                "currency_code": "USD",
                "final_issue_price": 51.0,
                "lowest_offer_price": 47.0,
                "highest_offer_price": 51.0,
                "lot_size": 100,
                "total_offer_size": 4870000000.0,
                "shares_outstanding": 1025544118i64,
                "last_updated": "2023-09-14T00:00:00Z"
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let ipos = client
        .list_ipos(
            None,               // ticker
            None,               // us_code
            None,               // isin
            None,               // listing_date
            None,               // listing_date.lt
            None,               // listing_date.lte
            None,               // listing_date.gt
            Some("2023-01-01"), // listing_date.gte
            Some("history"),    // ipo_status
            Some(10),           // limit
            Some("listing_date"), // sort
            Some("asc"),        // order
            None,               // options
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(ipos.len(), 1);
    let ipo = &ipos[0];
    assert_eq!(ipo.ticker.as_deref(), Some("ARM"));
    assert_eq!(ipo.issuer_name.as_deref(), Some("Arm Holdings plc"));
    assert_eq!(ipo.ipo_status.as_deref(), Some("history"));
    assert_eq!(ipo.final_issue_price, Some(51.0));
    assert_eq!(ipo.shares_outstanding, Some(1025544118));
}

#[tokio::test]
async fn list_stock_financials_with_params_serializes_every_field() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/vX/reference/financials"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("ticker", "AAPL"))
        .and(query_param("cik", "0000320193"))
        .and(query_param("company_name", "Apple Inc."))
        .and(query_param("company_name_search", "Apple"))
        .and(query_param("sic", "3571"))
        .and(query_param("filing_date", "2023-11-03"))
        .and(query_param("filing_date.lt", "2024-01-01"))
        .and(query_param("filing_date.lte", "2023-12-31"))
        .and(query_param("filing_date.gt", "2022-12-31"))
        .and(query_param("filing_date.gte", "2023-01-01"))
        .and(query_param("period_of_report_date", "2023-09-30"))
        .and(query_param("period_of_report_date.lt", "2024-01-01"))
        .and(query_param("period_of_report_date.lte", "2023-12-31"))
        .and(query_param("period_of_report_date.gt", "2022-12-31"))
        .and(query_param("period_of_report_date.gte", "2023-01-01"))
        .and(query_param("timeframe", "quarterly"))
        .and(query_param("include_sources", "true"))
        .and(query_param("limit", "10"))
        .and(query_param("sort", "filing_date"))
        .and(query_param("order", "desc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 0,
            "results": []
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let params = massive::rest::ListStockFinancialsParams::new()
        .ticker("AAPL")
        .cik("0000320193")
        .company_name("Apple Inc.")
        .company_name_search("Apple")
        .sic("3571")
        .filing_date("2023-11-03")
        .filing_date_lt("2024-01-01")
        .filing_date_lte("2023-12-31")
        .filing_date_gt("2022-12-31")
        .filing_date_gte("2023-01-01")
        .period_of_report_date("2023-09-30")
        .period_of_report_date_lt("2024-01-01")
        .period_of_report_date_lte("2023-12-31")
        .period_of_report_date_gt("2022-12-31")
        .period_of_report_date_gte("2023-01-01")
        .timeframe("quarterly")
        .include_sources(true)
        .limit(10)
        .sort("filing_date")
        .order("desc");
    let financials = client
        .list_stock_financials_with_params(params)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(financials.len(), 0);
}
