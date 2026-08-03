//! Get the last trade and last quote for a ticker.
//!
//! Usage: MASSIVE_API_KEY=<key> cargo run --example last_trade_quote

use massive::rest::{QuotesApi, TradesApi};
use massive::RESTClient;

#[tokio::main]
async fn main() -> massive::Result<()> {
    let client = RESTClient::from_env()?;

    let trade = client.get_last_trade("AAPL", None).await?;
    println!("last trade: {:?}", trade);

    let quote = client.get_last_quote("AAPL", None).await?;
    println!("last quote: {:?}", quote);

    Ok(())
}
