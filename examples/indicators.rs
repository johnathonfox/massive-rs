//! Technical indicators: SMA, RSI, and MACD for a ticker.
//!
//! Usage: MASSIVE_API_KEY=<key> cargo run --example indicators

use massive::rest::IndicatorsApi;
use massive::RESTClient;

#[tokio::main]
async fn main() -> massive::Result<()> {
    let client = RESTClient::from_env()?;

    // 10-day simple moving average.
    let sma = client
        .get_sma(
            "AAPL",
            None,
            None,
            None,
            None,
            Some("2023-05-01"), // timestamp_gte
            Some("day"),
            Some(10),
            Some(true),
            None,
            None,
            Some(5),
            None,
            None,
        )
        .await?;
    println!("sma: {:?}", sma);

    // 14-period relative strength index.
    let rsi = client
        .get_rsi(
            "AAPL",
            None,
            None,
            None,
            None,
            Some("2023-05-01"),
            Some("day"),
            Some(14),
            Some(true),
            None,
            None,
            Some(5),
            None,
            None,
        )
        .await?;
    println!("rsi: {:?}", rsi);

    // MACD with the standard 12/26/9 windows.
    let macd = client
        .get_macd(
            "AAPL",
            None,
            None,
            None,
            None,
            Some("2023-05-01"),
            Some("day"),
            Some(12),
            Some(26),
            Some(9),
            Some(true),
            None,
            None,
            Some(5),
            None,
            None,
        )
        .await?;
    println!("macd: {:?}", macd);

    Ok(())
}
