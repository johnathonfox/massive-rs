//! Snapshots: whole-market snapshot and an options chain.
//!
//! Usage: MASSIVE_API_KEY=<key> cargo run --example snapshot

use futures::TryStreamExt;
use massive::rest::SnapshotApi;
use massive::RESTClient;

#[tokio::main]
async fn main() -> massive::Result<()> {
    let client = RESTClient::from_env()?;

    let tickers = client
        .get_snapshot_all("stocks", Some("AAPL,MSFT"), None, None)
        .await?;
    println!("{} snapshots", tickers.len());

    // Options chain for an underlying.
    let mut chain = client.list_snapshot_options_chain("HCP", None);
    let mut count = 0usize;
    while let Some(o) = chain.try_next().await? {
        count += 1;
        if count <= 3 {
            println!("{:?}", o.details.as_ref().and_then(|d| d.ticker.as_ref()));
        }
    }
    println!("{} option contracts", count);

    // Universal snapshots with a .gte filter operator on the ticker.
    let mut filtered = client.list_universal_snapshots(
        Some("stocks"),
        None,
        None,
        Some(10),
        None,
        None,
        None,
        None,
        Some("A"), // ticker_gte
        None,
    );
    while let Some(s) = filtered.try_next().await? {
        println!("{:?}", s.ticker);
    }

    Ok(())
}
