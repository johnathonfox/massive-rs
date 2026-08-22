use futures::TryStreamExt;
use massive::{
    rest::{BenzingaApi, ListBenzingaEarningsParams},
    Client,
};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn list_benzinga_analyst_insights_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/benzinga/v1/analyst-insights"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("date", "2024-01-15"))
        .and(query_param("ticker.gte", "A"))
        .and(query_param("limit", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [{
                "benzinga_id": "ins-1",
                "ticker": "AAPL",
                "firm": "Morgan Stanley",
                "rating": "Overweight",
                "rating_action": "Maintains",
                "price_target": 220.5,
                "date": "2024-01-15"
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let insights = client
        .list_benzinga_analyst_insights(
            Some("2024-01-15"), // date
            None,               // date_any_of
            None,               // date_gt
            None,               // date_gte
            None,               // date_lt
            None,               // date_lte
            None,               // ticker
            None,               // ticker_any_of
            None,               // ticker_gt
            Some("A"),          // ticker_gte
            None,               // ticker_lt
            None,               // ticker_lte
            None,               // last_updated
            None,
            None,
            None,
            None,
            None,
            None, // firm
            None,
            None,
            None,
            None,
            None,
            None, // rating_action
            None,
            None,
            None,
            None,
            None,
            None, // benzinga_firm_id
            None,
            None,
            None,
            None,
            None,
            None, // benzinga_rating_id
            None,
            None,
            None,
            None,
            None,
            Some(10), // limit
            None,     // sort
            None,     // options
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(insights.len(), 1);
    assert_eq!(insights[0].ticker.as_deref(), Some("AAPL"));
    assert_eq!(insights[0].rating.as_deref(), Some("Overweight"));
    assert_eq!(insights[0].price_target, Some(220.5));
}

#[tokio::test]
async fn list_benzinga_analysts_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/benzinga/v1/analysts"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("firm_name", "Goldman Sachs"))
        .and(query_param("sort", "full_name.asc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [{
                "benzinga_id": "ana-1",
                "full_name": "Jane Doe",
                "firm_name": "Goldman Sachs",
                "smart_score": 8.7,
                "total_ratings": 150
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let analysts = client
        .list_benzinga_analysts(
            None, // benzinga_id
            None,
            None,
            None,
            None,
            None,
            None, // benzinga_firm_id
            None,
            None,
            None,
            None,
            None,
            Some("Goldman Sachs"), // firm_name
            None,
            None,
            None,
            None,
            None,
            None, // full_name
            None,
            None,
            None,
            None,
            None,
            None,                  // limit
            Some("full_name.asc"), // sort
            None,                  // options
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(analysts.len(), 1);
    assert_eq!(analysts[0].full_name.as_deref(), Some("Jane Doe"));
    assert_eq!(analysts[0].smart_score, Some(8.7));
}

#[tokio::test]
async fn list_benzinga_consensus_ratings_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/benzinga/v1/consensus-ratings/AAPL"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("date.gt", "2024-01-01"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [{
                "ticker": "AAPL",
                "consensus_rating": "buy",
                "consensus_price_target": 205.5,
                "buy_ratings": 20,
                "sell_ratings": 2
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let ratings = client
        .list_benzinga_consensus_ratings(
            "AAPL",
            None,               // date
            Some("2024-01-01"), // date_gt
            None,               // date_gte
            None,               // date_lt
            None,               // date_lte
            None,               // limit
            None,               // options
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(ratings.len(), 1);
    assert_eq!(ratings[0].ticker.as_deref(), Some("AAPL"));
    assert_eq!(ratings[0].consensus_rating.as_deref(), Some("buy"));
    assert_eq!(ratings[0].buy_ratings, Some(20));
}

#[tokio::test]
async fn list_benzinga_earnings_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/benzinga/v1/earnings"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("ticker", "AAPL"))
        .and(query_param("importance.gte", "3"))
        .and(query_param("fiscal_year", "2024"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [{
                "ticker": "AAPL",
                "date": "2024-01-25",
                "fiscal_period": "Q1",
                "fiscal_year": 2024,
                "actual_eps": 2.18,
                "estimated_eps": 2.10,
                "importance": 5
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let earnings = client
        .list_benzinga_earnings(
            None, // date
            None,
            None,
            None,
            None,
            None,
            Some("AAPL"), // ticker
            None,
            None,
            None,
            None,
            None,
            None,    // importance
            None,    // importance_any_of
            None,    // importance_gt
            Some(3), // importance_gte
            None,    // importance_lt
            None,    // importance_lte
            None,    // last_updated
            None,
            None,
            None,
            None,
            None,
            None, // date_status
            None,
            None,
            None,
            None,
            None,
            None, // eps_surprise_percent
            None,
            None,
            None,
            None,
            None,
            None, // revenue_surprise_percent
            None,
            None,
            None,
            None,
            None,
            Some(2024), // fiscal_year
            None,
            None,
            None,
            None,
            None,
            Some("Q1"), // fiscal_period
            None,
            None,
            None,
            None,
            None,
            None, // limit
            None, // sort
            None, // options
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(earnings.len(), 1);
    assert_eq!(earnings[0].ticker.as_deref(), Some("AAPL"));
    assert_eq!(earnings[0].actual_eps, Some(2.18));
    assert_eq!(earnings[0].fiscal_year, Some(2024));
}

#[tokio::test]
async fn list_benzinga_firms_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/benzinga/v1/firms"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("benzinga_id", "firm-123"))
        .and(query_param("benzinga_id.gt", "a"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [{
                "benzinga_id": "firm-123",
                "name": "Morgan Stanley",
                "currency": "USD"
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let firms = client
        .list_benzinga_firms(
            Some("firm-123"), // benzinga_id
            None,             // benzinga_id_any_of
            Some("a"),        // benzinga_id_gt
            None,             // benzinga_id_gte
            None,             // benzinga_id_lt
            None,             // benzinga_id_lte
            None,             // limit
            None,             // sort
            None,             // options
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(firms.len(), 1);
    assert_eq!(firms[0].name.as_deref(), Some("Morgan Stanley"));
    assert_eq!(firms[0].currency.as_deref(), Some("USD"));
}

#[tokio::test]
async fn list_benzinga_guidance_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/benzinga/v1/guidance"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("ticker", "MSFT"))
        .and(query_param("positioning", "positive"))
        .and(query_param("date.lte", "2024-12-31"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [{
                "ticker": "MSFT",
                "positioning": "positive",
                "fiscal_period": "Q4",
                "fiscal_year": 2024,
                "estimated_eps_guidance": 2.95,
                "date": "2024-07-30"
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let guidance = client
        .list_benzinga_guidance(
            None,               // date
            None,               // date_any_of
            None,               // date_gt
            None,               // date_gte
            None,               // date_lt
            Some("2024-12-31"), // date_lte
            Some("MSFT"),       // ticker
            None,
            None,
            None,
            None,
            None,
            Some("positive"), // positioning
            None,
            None,
            None,
            None,
            None,
            None, // importance
            None,
            None,
            None,
            None,
            None,
            None, // last_updated
            None,
            None,
            None,
            None,
            None,
            None, // fiscal_year
            None,
            None,
            None,
            None,
            None,
            None, // fiscal_period
            None,
            None,
            None,
            None,
            None,
            None, // limit
            None, // sort
            None, // options
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(guidance.len(), 1);
    assert_eq!(guidance[0].ticker.as_deref(), Some("MSFT"));
    assert_eq!(guidance[0].positioning.as_deref(), Some("positive"));
    assert_eq!(guidance[0].estimated_eps_guidance, Some(2.95));
}

#[tokio::test]
async fn list_benzinga_news_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/benzinga/v1/news"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("published.gte", "2024-01-01T00:00:00Z"))
        .and(query_param("tickers", "AAPL,MSFT"))
        .and(query_param("channels.any_of", "news"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [{
                "benzinga_id": 12345,
                "title": "Apple beats expectations",
                "author": "Jane Doe",
                "published": "2024-01-15T14:30:00Z",
                "tickers": ["AAPL"],
                "channels": ["News"]
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let news = client
        .list_benzinga_news(
            None,                         // published
            None,                         // published_any_of
            None,                         // published_gt
            Some("2024-01-01T00:00:00Z"), // published_gte
            None,                         // published_lt
            None,                         // published_lte
            None,                         // last_updated
            None,
            None,
            None,
            None,
            None,
            Some("AAPL,MSFT"), // tickers
            None,              // tickers_all_of
            None,              // tickers_any_of
            None,              // channels
            None,              // channels_all_of
            Some("news"),      // channels_any_of
            None,              // tags
            None,
            None,
            None, // author
            None,
            None,
            None,
            None,
            None,
            None, // limit
            None, // sort
            None, // options
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(news.len(), 1);
    assert_eq!(news[0].benzinga_id, Some(12345));
    assert_eq!(news[0].title.as_deref(), Some("Apple beats expectations"));
    assert_eq!(news[0].tickers.as_deref(), Some(&["AAPL".to_string()][..]));
}

#[tokio::test]
async fn list_benzinga_news_v2_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/benzinga/v2/news"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("published.gt", "2024-06-01"))
        .and(query_param("tickers", "AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [{
                "benzinga_id": 67890,
                "title": "Microsoft unveils new AI features",
                "author": "John Smith",
                "published": "2024-06-15T09:00:00Z",
                "stocks": [{"name": "MSFT"}],
                "tickers": ["MSFT"]
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let news = client
        .list_benzinga_news_v2(
            None,               // published
            Some("2024-06-01"), // published_gt
            None,               // published_gte
            None,               // published_lt
            None,               // published_lte
            None,               // channels
            None,
            None,
            None, // tags
            None,
            None,
            None, // author
            None,
            None,
            None,
            None,
            None,
            None, // stocks
            None,
            None,
            Some("AAPL"), // tickers
            None,         // tickers_all_of
            None,         // tickers_any_of
            None,         // limit
            None,         // sort
            None,         // options
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(news.len(), 1);
    assert_eq!(news[0].benzinga_id, Some(67890));
    assert_eq!(
        news[0].title.as_deref(),
        Some("Microsoft unveils new AI features")
    );
}

#[tokio::test]
async fn list_benzinga_ratings_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/benzinga/v1/ratings"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("ticker", "AAPL"))
        .and(query_param("rating_action", "upgrades"))
        .and(query_param("date.gte", "2024-01-01"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [{
                "ticker": "AAPL",
                "firm": "Morgan Stanley",
                "analyst": "Jane Doe",
                "rating_action": "upgrades",
                "rating": "Buy",
                "price_target": 225.0,
                "date": "2024-01-15"
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let ratings = client
        .list_benzinga_ratings(
            None,               // date
            None,               // date_any_of
            None,               // date_gt
            Some("2024-01-01"), // date_gte
            None,               // date_lt
            None,               // date_lte
            Some("AAPL"),       // ticker
            None,
            None,
            None,
            None,
            None,
            None, // importance
            None,
            None,
            None,
            None,
            None,
            None, // last_updated
            None,
            None,
            None,
            None,
            None,
            Some("upgrades"), // rating_action
            None,
            None,
            None,
            None,
            None,
            None, // price_target_action
            None,
            None,
            None,
            None,
            None,
            None, // benzinga_id
            None,
            None,
            None,
            None,
            None,
            None, // benzinga_analyst_id
            None,
            None,
            None,
            None,
            None,
            None, // benzinga_firm_id
            None,
            None,
            None,
            None,
            None,
            None, // limit
            None, // sort
            None, // options
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(ratings.len(), 1);
    assert_eq!(ratings[0].rating.as_deref(), Some("Buy"));
    assert_eq!(ratings[0].rating_action.as_deref(), Some("upgrades"));
    assert_eq!(ratings[0].price_target, Some(225.0));
}

#[tokio::test]
async fn list_benzinga_bulls_bears_say_hits_expected_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/benzinga/v1/bulls-bears-say"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("ticker", "TSLA"))
        .and(query_param("limit", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [{
                "benzinga_id": "bb-1",
                "ticker": "TSLA",
                "bull_case": "Strong EV demand and energy growth.",
                "bear_case": "Margin compression and competition.",
                "last_updated": "2024-01-15T00:00:00Z"
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let say = client
        .list_benzinga_bulls_bears_say(
            Some("TSLA"), // ticker
            None,
            None,
            None,
            None,
            None,
            None, // benzinga_id
            None,
            None,
            None,
            None,
            None,
            None, // last_updated
            None,
            None,
            None,
            None,
            Some(10), // limit
            None,     // sort
            None,     // options
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(say.len(), 1);
    assert_eq!(say[0].ticker.as_deref(), Some("TSLA"));
    assert!(say[0].bull_case.as_deref().unwrap().contains("EV"));
    assert!(say[0].bear_case.as_deref().unwrap().contains("Margin"));
}

#[tokio::test]
async fn list_benzinga_earnings_with_params_sends_every_query_param() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/benzinga/v1/earnings"))
        .and(header("Authorization", "Bearer test-key"))
        .and(query_param("date", "2024-01-15"))
        .and(query_param("date.any_of", "2024-01-15,2024-01-16"))
        .and(query_param("date.gt", "2024-01-01"))
        .and(query_param("date.gte", "2024-01-02"))
        .and(query_param("date.lt", "2024-12-31"))
        .and(query_param("date.lte", "2024-12-30"))
        .and(query_param("ticker", "AAPL"))
        .and(query_param("ticker.any_of", "AAPL,MSFT"))
        .and(query_param("ticker.gt", "A"))
        .and(query_param("ticker.gte", "B"))
        .and(query_param("ticker.lt", "Z"))
        .and(query_param("ticker.lte", "Y"))
        .and(query_param("importance", "3"))
        .and(query_param("importance.any_of", "3,4"))
        .and(query_param("importance.gt", "1"))
        .and(query_param("importance.gte", "2"))
        .and(query_param("importance.lt", "5"))
        .and(query_param("importance.lte", "4"))
        .and(query_param("last_updated", "2024-01-15T00:00:00Z"))
        .and(query_param("last_updated.any_of", "2024-01-15T00:00:00Z"))
        .and(query_param("last_updated.gt", "2024-01-01T00:00:00Z"))
        .and(query_param("last_updated.gte", "2024-01-02T00:00:00Z"))
        .and(query_param("last_updated.lt", "2024-12-31T00:00:00Z"))
        .and(query_param("last_updated.lte", "2024-12-30T00:00:00Z"))
        .and(query_param("date_status", "confirmed"))
        .and(query_param("date_status.any_of", "confirmed,projected"))
        .and(query_param("date_status.gt", "a"))
        .and(query_param("date_status.gte", "b"))
        .and(query_param("date_status.lt", "z"))
        .and(query_param("date_status.lte", "y"))
        // Whole-number floats must serialize as "5"/"10", not "5.0"/"10.0".
        .and(query_param("eps_surprise_percent", "5"))
        .and(query_param("eps_surprise_percent.any_of", "5,6"))
        .and(query_param("eps_surprise_percent.gt", "1.5"))
        .and(query_param("eps_surprise_percent.gte", "2.5"))
        .and(query_param("eps_surprise_percent.lt", "9.5"))
        .and(query_param("eps_surprise_percent.lte", "8.5"))
        .and(query_param("revenue_surprise_percent", "10"))
        .and(query_param("revenue_surprise_percent.any_of", "10,11"))
        .and(query_param("revenue_surprise_percent.gt", "1.25"))
        .and(query_param("revenue_surprise_percent.gte", "2.25"))
        .and(query_param("revenue_surprise_percent.lt", "9.75"))
        .and(query_param("revenue_surprise_percent.lte", "8.75"))
        .and(query_param("fiscal_year", "2024"))
        .and(query_param("fiscal_year.any_of", "2023,2024"))
        .and(query_param("fiscal_year.gt", "2022"))
        .and(query_param("fiscal_year.gte", "2023"))
        .and(query_param("fiscal_year.lt", "2026"))
        .and(query_param("fiscal_year.lte", "2025"))
        .and(query_param("fiscal_period", "Q1"))
        .and(query_param("fiscal_period.any_of", "Q1,Q2"))
        .and(query_param("fiscal_period.gt", "Q1"))
        .and(query_param("fiscal_period.gte", "Q2"))
        .and(query_param("fiscal_period.lt", "Q4"))
        .and(query_param("fiscal_period.lte", "Q3"))
        .and(query_param("limit", "25"))
        .and(query_param("sort", "date.desc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "OK",
            "count": 1,
            "results": [{
                "benzinga_id": "earn-1",
                "ticker": "AAPL",
                "date": "2024-01-15",
                "fiscal_period": "Q1",
                "fiscal_year": 2024,
                "importance": 3,
                "eps_surprise_percent": 5.0
            }]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::new("test-key").unwrap().with_base(server.uri());
    let earnings = client
        .list_benzinga_earnings_with_params(
            ListBenzingaEarningsParams::new()
                .date("2024-01-15")
                .date_any_of("2024-01-15,2024-01-16")
                .date_gt("2024-01-01")
                .date_gte("2024-01-02")
                .date_lt("2024-12-31")
                .date_lte("2024-12-30")
                .ticker("AAPL")
                .ticker_any_of("AAPL,MSFT")
                .ticker_gt("A")
                .ticker_gte("B")
                .ticker_lt("Z")
                .ticker_lte("Y")
                .importance(3)
                .importance_any_of("3,4")
                .importance_gt(1)
                .importance_gte(2)
                .importance_lt(5)
                .importance_lte(4)
                .last_updated("2024-01-15T00:00:00Z")
                .last_updated_any_of("2024-01-15T00:00:00Z")
                .last_updated_gt("2024-01-01T00:00:00Z")
                .last_updated_gte("2024-01-02T00:00:00Z")
                .last_updated_lt("2024-12-31T00:00:00Z")
                .last_updated_lte("2024-12-30T00:00:00Z")
                .date_status("confirmed")
                .date_status_any_of("confirmed,projected")
                .date_status_gt("a")
                .date_status_gte("b")
                .date_status_lt("z")
                .date_status_lte("y")
                .eps_surprise_percent(5.0)
                .eps_surprise_percent_any_of("5,6")
                .eps_surprise_percent_gt(1.5)
                .eps_surprise_percent_gte(2.5)
                .eps_surprise_percent_lt(9.5)
                .eps_surprise_percent_lte(8.5)
                .revenue_surprise_percent(10.0)
                .revenue_surprise_percent_any_of("10,11")
                .revenue_surprise_percent_gt(1.25)
                .revenue_surprise_percent_gte(2.25)
                .revenue_surprise_percent_lt(9.75)
                .revenue_surprise_percent_lte(8.75)
                .fiscal_year(2024)
                .fiscal_year_any_of("2023,2024")
                .fiscal_year_gt(2022)
                .fiscal_year_gte(2023)
                .fiscal_year_lt(2026)
                .fiscal_year_lte(2025)
                .fiscal_period("Q1")
                .fiscal_period_any_of("Q1,Q2")
                .fiscal_period_gt("Q1")
                .fiscal_period_gte("Q2")
                .fiscal_period_lt("Q4")
                .fiscal_period_lte("Q3")
                .limit(25)
                .sort("date.desc"),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    assert_eq!(earnings.len(), 1);
    assert_eq!(earnings[0].ticker.as_deref(), Some("AAPL"));
}
