use futures::TryStreamExt;
use massive::{
    rest::{EtfGlobalApi, GetEtfGlobalAnalyticsParams},
    Client,
};
use rust_decimal_macros::dec;
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
    assert_eq!(rows[0].risk_total_score, Some(dec!(4.2)));
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
    assert_eq!(rows[0].weight, Some(dec!(8.75)));
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
    assert_eq!(rows[0].fund_flow, Some(dec!(1234567.89)));
    assert_eq!(rows[0].nav, Some(dec!(545.23)));
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
    assert_eq!(rows[0].management_fee, Some(dec!(0.0003)));
    assert_eq!(rows[0].num_holdings, Some(dec!(3700.0)));
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

#[tokio::test]
async fn get_etf_global_analytics_with_params_sends_all_query_params() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/etf-global/v1/analytics"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("composite_ticker", "SPY"))
        .and(query_param("composite_ticker.any_of", "SPY,QQQ"))
        .and(query_param("composite_ticker.gt", "A"))
        .and(query_param("composite_ticker.gte", "AA"))
        .and(query_param("composite_ticker.lt", "ZZZ"))
        .and(query_param("composite_ticker.lte", "ZZ"))
        .and(query_param("processed_date", "2024-06-29"))
        .and(query_param("processed_date.gt", "2024-01-01"))
        .and(query_param("processed_date.gte", "2024-01-02"))
        .and(query_param("processed_date.lt", "2024-12-31"))
        .and(query_param("processed_date.lte", "2024-12-30"))
        .and(query_param("effective_date", "2024-06-28"))
        .and(query_param("effective_date.gt", "2024-01-01"))
        .and(query_param("effective_date.gte", "2024-01-02"))
        .and(query_param("effective_date.lt", "2024-12-31"))
        .and(query_param("effective_date.lte", "2024-12-30"))
        .and(query_param("risk_total_score", "1.1"))
        .and(query_param("risk_total_score.gt", "1.2"))
        .and(query_param("risk_total_score.gte", "1.3"))
        .and(query_param("risk_total_score.lt", "1.9"))
        .and(query_param("risk_total_score.lte", "1.8"))
        .and(query_param("reward_score", "2.1"))
        .and(query_param("reward_score.gt", "2.2"))
        .and(query_param("reward_score.gte", "2.3"))
        .and(query_param("reward_score.lt", "2.9"))
        .and(query_param("reward_score.lte", "2.8"))
        .and(query_param("quant_total_score", "3.1"))
        .and(query_param("quant_total_score.gt", "3.2"))
        .and(query_param("quant_total_score.gte", "3.3"))
        .and(query_param("quant_total_score.lt", "3.9"))
        .and(query_param("quant_total_score.lte", "3.8"))
        .and(query_param("quant_grade", "A"))
        .and(query_param("quant_grade.any_of", "A,B"))
        .and(query_param("quant_grade.gt", "A"))
        .and(query_param("quant_grade.gte", "B"))
        .and(query_param("quant_grade.lt", "F"))
        .and(query_param("quant_grade.lte", "E"))
        .and(query_param("quant_composite_technical", "4.1"))
        .and(query_param("quant_composite_technical.gt", "4.2"))
        .and(query_param("quant_composite_technical.gte", "4.3"))
        .and(query_param("quant_composite_technical.lt", "4.9"))
        .and(query_param("quant_composite_technical.lte", "4.8"))
        .and(query_param("quant_composite_sentiment", "5.1"))
        .and(query_param("quant_composite_sentiment.gt", "5.2"))
        .and(query_param("quant_composite_sentiment.gte", "5.3"))
        .and(query_param("quant_composite_sentiment.lt", "5.9"))
        .and(query_param("quant_composite_sentiment.lte", "5.8"))
        .and(query_param("quant_composite_behavioral", "6.1"))
        .and(query_param("quant_composite_behavioral.gt", "6.2"))
        .and(query_param("quant_composite_behavioral.gte", "6.3"))
        .and(query_param("quant_composite_behavioral.lt", "6.9"))
        .and(query_param("quant_composite_behavioral.lte", "6.8"))
        .and(query_param("quant_composite_fundamental", "7.1"))
        .and(query_param("quant_composite_fundamental.gt", "7.2"))
        .and(query_param("quant_composite_fundamental.gte", "7.3"))
        .and(query_param("quant_composite_fundamental.lt", "7.9"))
        .and(query_param("quant_composite_fundamental.lte", "7.8"))
        .and(query_param("quant_composite_global", "8.1"))
        .and(query_param("quant_composite_global.gt", "8.2"))
        .and(query_param("quant_composite_global.gte", "8.3"))
        .and(query_param("quant_composite_global.lt", "8.9"))
        .and(query_param("quant_composite_global.lte", "8.8"))
        .and(query_param("quant_composite_quality", "9.1"))
        .and(query_param("quant_composite_quality.gt", "9.2"))
        .and(query_param("quant_composite_quality.gte", "9.3"))
        .and(query_param("quant_composite_quality.lt", "9.9"))
        .and(query_param("quant_composite_quality.lte", "9.8"))
        .and(query_param("limit", "10"))
        .and(query_param("sort", "composite_ticker.asc"))
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
    let params = GetEtfGlobalAnalyticsParams::new()
        .composite_ticker("SPY")
        .composite_ticker_any_of("SPY,QQQ")
        .composite_ticker_gt("A")
        .composite_ticker_gte("AA")
        .composite_ticker_lt("ZZZ")
        .composite_ticker_lte("ZZ")
        .processed_date("2024-06-29")
        .processed_date_gt("2024-01-01")
        .processed_date_gte("2024-01-02")
        .processed_date_lt("2024-12-31")
        .processed_date_lte("2024-12-30")
        .effective_date("2024-06-28")
        .effective_date_gt("2024-01-01")
        .effective_date_gte("2024-01-02")
        .effective_date_lt("2024-12-31")
        .effective_date_lte("2024-12-30")
        .risk_total_score(1.1)
        .risk_total_score_gt(1.2)
        .risk_total_score_gte(1.3)
        .risk_total_score_lt(1.9)
        .risk_total_score_lte(1.8)
        .reward_score(2.1)
        .reward_score_gt(2.2)
        .reward_score_gte(2.3)
        .reward_score_lt(2.9)
        .reward_score_lte(2.8)
        .quant_total_score(3.1)
        .quant_total_score_gt(3.2)
        .quant_total_score_gte(3.3)
        .quant_total_score_lt(3.9)
        .quant_total_score_lte(3.8)
        .quant_grade("A")
        .quant_grade_any_of("A,B")
        .quant_grade_gt("A")
        .quant_grade_gte("B")
        .quant_grade_lt("F")
        .quant_grade_lte("E")
        .quant_composite_technical(4.1)
        .quant_composite_technical_gt(4.2)
        .quant_composite_technical_gte(4.3)
        .quant_composite_technical_lt(4.9)
        .quant_composite_technical_lte(4.8)
        .quant_composite_sentiment(5.1)
        .quant_composite_sentiment_gt(5.2)
        .quant_composite_sentiment_gte(5.3)
        .quant_composite_sentiment_lt(5.9)
        .quant_composite_sentiment_lte(5.8)
        .quant_composite_behavioral(6.1)
        .quant_composite_behavioral_gt(6.2)
        .quant_composite_behavioral_gte(6.3)
        .quant_composite_behavioral_lt(6.9)
        .quant_composite_behavioral_lte(6.8)
        .quant_composite_fundamental(7.1)
        .quant_composite_fundamental_gt(7.2)
        .quant_composite_fundamental_gte(7.3)
        .quant_composite_fundamental_lt(7.9)
        .quant_composite_fundamental_lte(7.8)
        .quant_composite_global(8.1)
        .quant_composite_global_gt(8.2)
        .quant_composite_global_gte(8.3)
        .quant_composite_global_lt(8.9)
        .quant_composite_global_lte(8.8)
        .quant_composite_quality(9.1)
        .quant_composite_quality_gt(9.2)
        .quant_composite_quality_gte(9.3)
        .quant_composite_quality_lt(9.9)
        .quant_composite_quality_lte(9.8)
        .limit(10)
        .sort("composite_ticker.asc");
    let rows: Vec<_> = client
        .get_etf_global_analytics_with_params(params)
        .try_collect()
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].composite_ticker.as_deref(), Some("SPY"));
    assert_eq!(rows[0].quant_grade.as_deref(), Some("A"));
}
