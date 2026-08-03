//! List aggregate bars for a ticker (mirrors examples/rest/stocks-aggregates_bars.py).
//!
//! Usage: MASSIVE_API_KEY=<key> cargo run --example aggs

use futures::TryStreamExt;
use massive::rest::AggsApi;
use massive::{Client, RESTClient};

#[tokio::main]
async fn main() -> massive::Result<()> {
    let client = RESTClient::from_env()?;

    let mut stream = client.list_aggs(
        "AAPL",
        1,
        "day",
        "2023-01-01",
        "2023-02-01",
        Some(true),
        Some("asc"),
        Some(120),
        None,
    );

    let mut aggs = Vec::new();
    while let Some(agg) = stream.try_next().await? {
        aggs.push(agg);
    }
    println!("{} aggs", aggs.len());
    for a in aggs.iter().take(3) {
        println!("{:?}", a);
    }

    // Single-page fetch (no pagination follow).
    let page = Client::from_env()?
        .with_pagination(false)
        .get_aggs(
            "AAPL",
            1,
            "day",
            "2023-01-01",
            "2023-02-01",
            Some(true),
            Some("asc"),
            Some(5),
            None,
        )
        .await?;
    println!("single page: {} aggs", page.len());

    Ok(())
}
