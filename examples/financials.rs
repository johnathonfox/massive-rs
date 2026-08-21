//! Financials: income statements and share float data.
//!
//! Usage: MASSIVE_API_KEY=<key> cargo run --example financials

use futures::TryStreamExt;
use massive::rest::FinancialsApi;
use massive::RESTClient;

#[tokio::main]
async fn main() -> massive::Result<()> {
    let client = RESTClient::from_env()?;

    // Annual income statements for a ticker, streamed with pagination.
    let mut stream = client.list_financials_income_statements(
        None,
        None,
        None,
        None,
        None,
        None, // cik group
        Some("AAPL"),
        None,
        None, // tickers, tickers_all_of, tickers.any_of
        None,
        None,
        None,
        None,
        None, // period_end group
        None,
        None,
        None,
        None,
        None, // filing_date group
        None,
        None,
        None,
        None,
        None, // fiscal_year group
        None,
        None,
        None,
        None,
        None, // fiscal_quarter group
        Some("annual"),
        None,
        None,
        None,
        None,
        None, // timeframe group
        Some(5),
        None,
        None,
    );
    let mut count = 0usize;
    while let Some(stmt) = stream.try_next().await? {
        count += 1;
        if count <= 3 {
            println!("{:?}", stmt);
        }
    }
    println!("{} income statements", count);

    // Share float data with a free-float-percent floor.
    let mut floats = client.list_stocks_floats(
        None,
        None,
        None,
        Some("A"), // ticker, ticker.any_of, ticker.gt, ticker.gte
        None,
        None, // ticker.lt, ticker.lte
        None,
        None,
        Some(50.0), // free_float_percent, .gt, .gte
        None,
        None, // free_float_percent.lt, .lte
        Some(5),
        None,
        None,
    );
    while let Some(f) = floats.try_next().await? {
        println!("{:?}", f);
    }

    Ok(())
}
