//! Reference data: list tickers, ticker details, and market status.
//!
//! Usage: MASSIVE_API_KEY=<key> cargo run --example reference

use futures::TryStreamExt;
use massive::rest::ReferenceApi;
use massive::RESTClient;

#[tokio::main]
async fn main() -> massive::Result<()> {
    let client = RESTClient::from_env()?;

    // Active common stocks, streamed with pagination.
    let mut stream = client.list_tickers(
        None,
        None,
        None,
        None,
        Some("A"), // ticker_gte
        Some("CS"),
        Some("stocks"),
        None,
        None,
        None,
        None,
        Some(true), // active
        None,
        Some(10),
        None,
        None,
        None,
    );
    let mut count = 0usize;
    while let Some(t) = stream.try_next().await? {
        count += 1;
        if count <= 3 {
            println!("{:?}", t);
        }
    }
    println!("{} tickers", count);

    // Details for a single ticker.
    let details = client.get_ticker_details("AAPL", None, None).await?;
    println!("ticker details: {:?}", details);

    // Current market status.
    let status = client.get_market_status(None).await?;
    println!("market status: {:?}", status);

    Ok(())
}
