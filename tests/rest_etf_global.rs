use futures::TryStreamExt;
use massive::{rest::EtfGlobalApi, Client};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_etf_global_analytics_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/etf-global/v1/analytics"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("composite_ticker", "SPY"))
        .and(query_param("processed_date.gte", "2024-01-01"))
        .and(query_param("risk_total_score.gte", "3.5"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "composite_ticker": "SPY",
                    "effective_date": "2024-06-28",
                    "processed_date": "2024-06-29",
                    "quant_grade": "A",
                    "quant_total_score": 7.5,
                    "reward_score": 8.1,
                    "risk_total_score": 4.2
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let rows: Vec<_> = client
        .get_etf_global_analytics(
            // composite_ticker (6)
            Some("SPY"),
            None,
            None,
            None,
            None,
            None,
            // processed_date (5)
            None,
            None,
            Some("2024-01-01"),
            None,
            None,
            // effective_date (5)
            None,
            None,
            None,
            None,
            None,
            // risk_total_score (5)
            None,
            None,
            Some(3.5),
            None,
            None,
            // reward_score (5)
            None,
            None,
            None,
            None,
            None,
            // quant_total_score (5)
            None,
            None,
            None,
            None,
            None,
            // quant_grade (6)
            None,
            None,
            None,
            None,
            None,
            None,
            // quant_composite_technical (5)
            None,
            None,
            None,
            None,
            None,
            // quant_composite_sentiment (5)
            None,
            None,
            None,
            None,
            None,
            // quant_composite_behavioral (5)
            None,
            None,
            None,
            None,
            None,
            // quant_composite_fundamental (5)
            None,
            None,
            None,
            None,
            None,
            // quant_composite_global (5)
            None,
            None,
            None,
            None,
            None,
            // quant_composite_quality (5)
            None,
            None,
            None,
            None,
            None,
            // limit, sort, options
            Some(10),
            None,
            None,
        )
        .try_collect()
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].composite_ticker.as_deref(), Some("SPY"));
    assert_eq!(rows[0].quant_grade.as_deref(), Some("A"));
    assert_eq!(rows[0].risk_total_score, Some(4.2));
}

#[tokio::test]
async fn get_etf_global_constituents_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/etf-global/v1/constituents"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("composite_ticker", "QQQ"))
        .and(query_param("constituent_ticker.any_of", "AAPL,MSFT"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "composite_ticker": "QQQ",
                    "constituent_ticker": "AAPL",
                    "constituent_name": "Apple Inc.",
                    "weight": 8.75,
                    "shares_held": 123456.0,
                    "isin": "US0378331005",
                    "effective_date": "2024-06-28"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let rows: Vec<_> = client
        .get_etf_global_constituents(
            // composite_ticker (6)
            Some("QQQ"),
            None,
            None,
            None,
            None,
            None,
            // constituent_ticker (6)
            None,
            Some("AAPL,MSFT"),
            None,
            None,
            None,
            None,
            // effective_date (5)
            None,
            None,
            None,
            None,
            None,
            // processed_date (5)
            None,
            None,
            None,
            None,
            None,
            // us_code (6)
            None,
            None,
            None,
            None,
            None,
            None,
            // isin (6)
            None,
            None,
            None,
            None,
            None,
            None,
            // figi (6)
            None,
            None,
            None,
            None,
            None,
            None,
            // sedol (6)
            None,
            None,
            None,
            None,
            None,
            None,
            // limit, sort, options
            None,
            None,
            None,
        )
        .try_collect()
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].constituent_ticker.as_deref(), Some("AAPL"));
    assert_eq!(rows[0].weight, Some(8.75));
    assert_eq!(rows[0].isin.as_deref(), Some("US0378331005"));
}

#[tokio::test]
async fn get_etf_global_fund_flows_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/etf-global/v1/fund-flows"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("composite_ticker", "SPY"))
        .and(query_param("effective_date.lt", "2024-07-01"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "composite_ticker": "SPY",
                    "effective_date": "2024-06-28",
                    "fund_flow": 1234567.89,
                    "nav": 545.23,
                    "shares_outstanding": 1023000000.0
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let rows: Vec<_> = client
        .get_etf_global_fund_flows(
            // processed_date (5)
            None,
            None,
            None,
            None,
            None,
            // effective_date (5)
            None,
            None,
            None,
            Some("2024-07-01"),
            None,
            // composite_ticker (6)
            Some("SPY"),
            None,
            None,
            None,
            None,
            None,
            // limit, sort, options
            None,
            None,
            None,
        )
        .try_collect()
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].composite_ticker.as_deref(), Some("SPY"));
    assert_eq!(rows[0].fund_flow, Some(1234567.89));
    assert_eq!(rows[0].nav, Some(545.23));
}

#[tokio::test]
async fn get_etf_global_profiles_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/etf-global/v1/profiles"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("composite_ticker", "VTI"))
        .and(query_param("limit", "5"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "composite_ticker": "VTI",
                    "issuer": "Vanguard",
                    "asset_class": "Equity",
                    "aum": 1600000000000.0,
                    "management_fee": 0.0003,
                    "num_holdings": 3700.0,
                    "inception_date": "2001-05-24",
                    "listing_exchange": "NYSE Arca"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let rows: Vec<_> = client
        .get_etf_global_profiles(
            // processed_date (5)
            None,
            None,
            None,
            None,
            None,
            // effective_date (5)
            None,
            None,
            None,
            None,
            None,
            // composite_ticker (6)
            Some("VTI"),
            None,
            None,
            None,
            None,
            None,
            // limit, sort, options
            Some(5),
            None,
            None,
        )
        .try_collect()
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].issuer.as_deref(), Some("Vanguard"));
    assert_eq!(rows[0].management_fee, Some(0.0003));
    assert_eq!(rows[0].num_holdings, Some(3700.0));
}

#[tokio::test]
async fn get_etf_global_taxonomies_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/etf-global/v1/taxonomies"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("composite_ticker", "ARKK"))
        .and(query_param("processed_date.gt", "2024-01-01"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "composite_ticker": "ARKK",
                    "issuer": "ARK Invest",
                    "asset_class": "Equity",
                    "category": "Sector Equity",
                    "focus": "Disruptive Innovation",
                    "region": "North America",
                    "product_type": "ETF",
                    "inception_date": "2014-10-31"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let rows: Vec<_> = client
        .get_etf_global_taxonomies(
            // processed_date (5)
            None,
            Some("2024-01-01"),
            None,
            None,
            None,
            // effective_date (5)
            None,
            None,
            None,
            None,
            None,
            // composite_ticker (6)
            Some("ARKK"),
            None,
            None,
            None,
            None,
            None,
            // limit, sort, options
            None,
            None,
            None,
        )
        .try_collect()
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].composite_ticker.as_deref(), Some("ARKK"));
    assert_eq!(rows[0].focus.as_deref(), Some("Disruptive Innovation"));
    assert_eq!(rows[0].region.as_deref(), Some("North America"));
}
