use massive::rest::SummariesApi;
use massive::Client;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_summaries_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/summaries"))
        .and(query_param("ticker.any_of", "AAPL,MSFT"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "results": [
                {
                    "ticker": "AAPL",
                    "name": "Apple Inc.",
                    "type": "stocks",
                    "price": 173.15,
                    "market_status": "open",
                    "last_updated": 1675283458000000000i64,
                    "branding": {
                        "logo_url": "https://api.massive.com/v1/reference/company-branding/d29wbmlsLmNvbQ/images/2022-01-10_logo.png",
                        "icon_url": "https://api.massive.com/v1/reference/company-branding/d29wbmlsLmNvbQ/images/2022-01-10_icon.png"
                    },
                    "session": {"change": 1.2, "change_percent": 0.7, "open": 171.9, "close": 173.15, "high": 174.0, "low": 171.0, "volume": 42000000.0}
                },
                {
                    "ticker": "MSFT",
                    "name": "Microsoft Corporation",
                    "type": "stocks",
                    "price": 331.06,
                    "market_status": "open",
                    "last_updated": 1675283458000000000i64,
                    "session": {"change": -2.1, "change_percent": -0.63, "open": 333.0, "close": 331.06}
                }
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let summaries = client
        .get_summaries(Some(&["AAPL", "MSFT"]), None)
        .await
        .unwrap();
    assert_eq!(summaries.len(), 2);
    assert_eq!(summaries[0].ticker.as_deref(), Some("AAPL"));
    assert_eq!(summaries[0].price, Some(173.15));
    assert_eq!(summaries[0].market_status.as_deref(), Some("open"));
    assert_eq!(
        summaries[0].session.as_ref().unwrap().change_percent,
        Some(0.7)
    );
    assert!(summaries[1].branding.is_none());
}

#[tokio::test]
async fn get_summaries_without_tickers_sends_no_filter() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/summaries"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "results": [{
                "ticker": "O:AAPL230120C00155000",
                "type": "options",
                "price": 21.8,
                "market_status": "open",
                "options": {
                    "contract_type": "call",
                    "exercise_style": "american",
                    "expiration_date": "2023-01-20",
                    "shares_per_contract": 100.0,
                    "strike_price": 155.0
                }
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let summaries = client.get_summaries(None, None).await.unwrap();
    assert_eq!(summaries.len(), 1);
    assert_eq!(summaries[0].type_.as_deref(), Some("options"));
    assert_eq!(
        summaries[0].options.as_ref().unwrap().strike_price,
        Some(155.0)
    );
}
