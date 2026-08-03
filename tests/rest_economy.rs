use futures::TryStreamExt;
use massive::{rest::EconomyApi, Client};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn list_treasury_yields_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/fed/v1/treasury-yields"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("date.gte", "2024-01-01"))
        .and(query_param("limit", "10"))
        .and(query_param("order", "asc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "date": "2024-01-02",
                    "yield_2_year": 4.25,
                    "yield_10_year": 3.95,
                    "yield_30_year": 4.10
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let yields: Vec<_> = client
        .list_treasury_yields(
            None,
            None,
            None,
            Some("2024-01-01"),
            None,
            None,
            Some(10),
            None,
            Some("asc"),
            None,
        )
        .try_collect()
        .await
        .unwrap();
    assert_eq!(yields.len(), 1);
    assert_eq!(yields[0].date.as_deref(), Some("2024-01-02"));
    assert_eq!(yields[0].yield_10_year, Some(3.95));
    assert_eq!(yields[0].yield_2_year, Some(4.25));
}

#[tokio::test]
async fn list_inflation_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/fed/v1/inflation"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("date.any_of", "2024-01-01,2024-02-01"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "date": "2024-01-01",
                    "cpi": 308.417,
                    "cpi_year_over_year": 3.1,
                    "pce": 120.3
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let rows: Vec<_> = client
        .list_inflation(
            None,
            Some("2024-01-01,2024-02-01"),
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
    assert_eq!(rows[0].date.as_deref(), Some("2024-01-01"));
    assert_eq!(rows[0].cpi, Some(308.417));
    assert_eq!(rows[0].cpi_year_over_year, Some(3.1));
}

#[tokio::test]
async fn list_inflation_expectations_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/fed/v1/inflation-expectations"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("date.lte", "2024-06-30"))
        .and(query_param("sort", "date"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "date": "2024-06-28",
                    "market_5_year": 2.31,
                    "market_10_year": 2.28,
                    "forward_years_5_to_10": 2.24
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let rows: Vec<_> = client
        .list_inflation_expectations(
            None,
            None,
            None,
            None,
            None,
            Some("2024-06-30"),
            None,
            Some("date"),
            None,
        )
        .try_collect()
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].market_5_year, Some(2.31));
    assert_eq!(rows[0].forward_years_5_to_10, Some(2.24));
}

#[tokio::test]
async fn list_labor_market_indicators_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/fed/v1/labor-market"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("date.gt", "2024-01-01"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "date": "2024-05-01",
                    "unemployment_rate": 4.0,
                    "labor_force_participation_rate": 62.5,
                    "job_openings": 8059.0
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let rows: Vec<_> = client
        .list_labor_market_indicators(
            None,
            None,
            Some("2024-01-01"),
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
    assert_eq!(rows[0].unemployment_rate, Some(4.0));
    assert_eq!(rows[0].job_openings, Some(8059.0));
}

#[tokio::test]
async fn list_eu_merchant_aggregates_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/consumer-spending/eu/v1/merchant-aggregates"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("name.any_of", "Tesco,Lidl"))
        .and(query_param("transaction_date.gte", "2024-01-01"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "name": "Tesco",
                    "parent_name": "Tesco PLC",
                    "transaction_date": "2024-03-15",
                    "type": "merchant",
                    "user_country": "GB",
                    "channel": "in_store",
                    "total_spend": 1234567.89,
                    "total_transactions": 43210
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let rows: Vec<_> = client
        .list_eu_merchant_aggregates(
            None,
            None,
            Some("2024-01-01"),
            None,
            None,
            None,
            Some("Tesco,Lidl"),
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
        )
        .try_collect()
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].name.as_deref(), Some("Tesco"));
    assert_eq!(rows[0].type_.as_deref(), Some("merchant"));
    assert_eq!(rows[0].total_spend, Some(1234567.89));
}

#[tokio::test]
async fn list_eu_merchant_hierarchy_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/consumer-spending/eu/v1/merchant-hierarchy"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("ticker.gte", "A"))
        .and(query_param("listing_status", "listed"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "lookup_name": "Tesco",
                    "normalized_name": "tesco",
                    "ticker": "TSCO.L",
                    "parent_name": "Tesco PLC",
                    "parent_ticker": "TSCO.L",
                    "sector": "Consumer Staples",
                    "listing_status": "listed",
                    "active_from": "2020-01-01"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let rows: Vec<_> = client
        .list_eu_merchant_hierarchy(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("A"),
            None,
            None,
            Some("listed"),
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
    assert_eq!(rows[0].lookup_name.as_deref(), Some("Tesco"));
    assert_eq!(rows[0].ticker.as_deref(), Some("TSCO.L"));
    assert_eq!(rows[0].sector.as_deref(), Some("Consumer Staples"));
}
