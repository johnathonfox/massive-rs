use futures::TryStreamExt;
use massive::{rest::TmxApi, Client};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn list_tmx_corporate_events_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/tmx/v1/corporate-events"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("ticker", "SHOP"))
        .and(query_param("date.gte", "2024-01-01"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [
                {
                    "company_name": "Shopify Inc.",
                    "ticker": "SHOP",
                    "date": "2024-06-04",
                    "type": "stock_split",
                    "status": "confirmed",
                    "isin": "CA82509L1076",
                    "trading_venue": "TSX",
                    "tmx_company_id": 12345,
                    "tmx_record_id": "abc123"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let rows: Vec<_> = client
        .list_tmx_corporate_events(
            // date (6)
            None,
            None,
            None,
            Some("2024-01-01"),
            None,
            None,
            // type (6)
            None,
            None,
            None,
            None,
            None,
            None,
            // status (6)
            None,
            None,
            None,
            None,
            None,
            None,
            // ticker (6)
            Some("SHOP"),
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
            // trading_venue (6)
            None,
            None,
            None,
            None,
            None,
            None,
            // tmx_company_id (6)
            None,
            None,
            None,
            None,
            None,
            None,
            // tmx_record_id (6)
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
    assert_eq!(rows[0].ticker.as_deref(), Some("SHOP"));
    assert_eq!(rows[0].type_.as_deref(), Some("stock_split"));
    assert_eq!(rows[0].tmx_company_id, Some(12345));
}

#[tokio::test]
async fn list_tmx_corporate_events_serializes_numeric_and_operator_filters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/tmx/v1/corporate-events"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("tmx_company_id", "12345"))
        .and(query_param("type.any_of", "dividend,stock_split"))
        .and(query_param("limit", "25"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 2,
            "results": [
                {
                    "company_name": "Shopify Inc.",
                    "ticker": "SHOP",
                    "date": "2024-06-04",
                    "type": "stock_split",
                    "status": "confirmed",
                    "tmx_company_id": 12345,
                    "tmx_record_id": "abc123"
                },
                {
                    "company_name": "Shopify Inc.",
                    "ticker": "SHOP",
                    "date": "2024-03-15",
                    "type": "dividend",
                    "status": "announced",
                    "tmx_company_id": 12345,
                    "tmx_record_id": "def456"
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let rows: Vec<_> = client
        .list_tmx_corporate_events(
            // date (6)
            None,
            None,
            None,
            None,
            None,
            None,
            // type (6)
            None,
            Some("dividend,stock_split"),
            None,
            None,
            None,
            None,
            // status (6)
            None,
            None,
            None,
            None,
            None,
            None,
            // ticker (6)
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
            // trading_venue (6)
            None,
            None,
            None,
            None,
            None,
            None,
            // tmx_company_id (6)
            Some(12345),
            None,
            None,
            None,
            None,
            None,
            // tmx_record_id (6)
            None,
            None,
            None,
            None,
            None,
            None,
            // limit, sort, options
            Some(25),
            None,
            None,
        )
        .try_collect()
        .await
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[1].type_.as_deref(), Some("dividend"));
    assert_eq!(rows[1].tmx_record_id.as_deref(), Some("def456"));
}
