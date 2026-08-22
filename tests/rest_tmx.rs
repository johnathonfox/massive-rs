use futures::TryStreamExt;
use massive::{
    rest::{ListTmxCorporateEventsParams, TmxApi},
    Client,
};
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

#[tokio::test]
async fn list_tmx_corporate_events_with_params_serializes_every_field() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/tmx/v1/corporate-events"))
        .and(header("Authorization", "Bearer test-key"))
        // date (6)
        .and(query_param("date", "2024-06-04"))
        .and(query_param("date.any_of", "2024-06-04,2024-06-05"))
        .and(query_param("date.gt", "2024-01-01"))
        .and(query_param("date.gte", "2024-01-02"))
        .and(query_param("date.lt", "2024-12-31"))
        .and(query_param("date.lte", "2024-12-30"))
        // type (6)
        .and(query_param("type", "dividend"))
        .and(query_param("type.any_of", "dividend,stock_split"))
        .and(query_param("type.gt", "buyback"))
        .and(query_param("type.gte", "dividend"))
        .and(query_param("type.lt", "stock_split"))
        .and(query_param("type.lte", "rights_offering"))
        // status (6)
        .and(query_param("status", "confirmed"))
        .and(query_param("status.any_of", "confirmed,announced"))
        .and(query_param("status.gt", "announced"))
        .and(query_param("status.gte", "cancelled"))
        .and(query_param("status.lt", "withdrawn"))
        .and(query_param("status.lte", "pending"))
        // ticker (6)
        .and(query_param("ticker", "SHOP"))
        .and(query_param("ticker.any_of", "SHOP,RY"))
        .and(query_param("ticker.gt", "A"))
        .and(query_param("ticker.gte", "B"))
        .and(query_param("ticker.lt", "ZZZ"))
        .and(query_param("ticker.lte", "YYY"))
        // isin (6)
        .and(query_param("isin", "CA82509L1076"))
        .and(query_param("isin.any_of", "CA82509L1076,US64110L1061"))
        .and(query_param("isin.gt", "CA0000000000"))
        .and(query_param("isin.gte", "CA1000000000"))
        .and(query_param("isin.lt", "US9999999999"))
        .and(query_param("isin.lte", "US9000000000"))
        // trading_venue (6)
        .and(query_param("trading_venue", "TSX"))
        .and(query_param("trading_venue.any_of", "TSX,TSXV"))
        .and(query_param("trading_venue.gt", "CSE"))
        .and(query_param("trading_venue.gte", "NEO"))
        .and(query_param("trading_venue.lt", "TSXV"))
        .and(query_param("trading_venue.lte", "TSX"))
        // tmx_company_id (6)
        .and(query_param("tmx_company_id", "12345"))
        .and(query_param("tmx_company_id.any_of", "12345,67890"))
        .and(query_param("tmx_company_id.gt", "1"))
        .and(query_param("tmx_company_id.gte", "2"))
        .and(query_param("tmx_company_id.lt", "99999"))
        .and(query_param("tmx_company_id.lte", "99998"))
        // tmx_record_id (6)
        .and(query_param("tmx_record_id", "abc123"))
        .and(query_param("tmx_record_id.any_of", "abc123,def456"))
        .and(query_param("tmx_record_id.gt", "a"))
        .and(query_param("tmx_record_id.gte", "b"))
        .and(query_param("tmx_record_id.lt", "z"))
        .and(query_param("tmx_record_id.lte", "y"))
        // limit, sort
        .and(query_param("limit", "25"))
        .and(query_param("sort", "date.desc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 0,
            "results": []
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let params = ListTmxCorporateEventsParams::new()
        .date("2024-06-04")
        .date_any_of("2024-06-04,2024-06-05")
        .date_gt("2024-01-01")
        .date_gte("2024-01-02")
        .date_lt("2024-12-31")
        .date_lte("2024-12-30")
        .r#type("dividend")
        .type_any_of("dividend,stock_split")
        .type_gt("buyback")
        .type_gte("dividend")
        .type_lt("stock_split")
        .type_lte("rights_offering")
        .status("confirmed")
        .status_any_of("confirmed,announced")
        .status_gt("announced")
        .status_gte("cancelled")
        .status_lt("withdrawn")
        .status_lte("pending")
        .ticker("SHOP")
        .ticker_any_of("SHOP,RY")
        .ticker_gt("A")
        .ticker_gte("B")
        .ticker_lt("ZZZ")
        .ticker_lte("YYY")
        .isin("CA82509L1076")
        .isin_any_of("CA82509L1076,US64110L1061")
        .isin_gt("CA0000000000")
        .isin_gte("CA1000000000")
        .isin_lt("US9999999999")
        .isin_lte("US9000000000")
        .trading_venue("TSX")
        .trading_venue_any_of("TSX,TSXV")
        .trading_venue_gt("CSE")
        .trading_venue_gte("NEO")
        .trading_venue_lt("TSXV")
        .trading_venue_lte("TSX")
        .tmx_company_id(12345)
        .tmx_company_id_any_of("12345,67890")
        .tmx_company_id_gt(1)
        .tmx_company_id_gte(2)
        .tmx_company_id_lt(99999)
        .tmx_company_id_lte(99998)
        .tmx_record_id("abc123")
        .tmx_record_id_any_of("abc123,def456")
        .tmx_record_id_gt("a")
        .tmx_record_id_gte("b")
        .tmx_record_id_lt("z")
        .tmx_record_id_lte("y")
        .limit(25)
        .sort("date.desc");
    let rows: Vec<_> = client
        .list_tmx_corporate_events_with_params(params)
        .try_collect()
        .await
        .unwrap();
    assert_eq!(rows.len(), 0);
}
