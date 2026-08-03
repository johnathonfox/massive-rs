# massive-rs

Async Rust client for the [Massive.com](https://massive.com) (formerly Polygon.io) REST
and WebSocket APIs, with feature parity to the official Python client
([massive-com/client-python](https://github.com/massive-com/client-python)).

## Features

- Full REST coverage: aggregates, trades, quotes, snapshots, reference data
  (tickers, news, splits, dividends, conditions, exchanges, options contracts,
  SEC filings, short interest/volume), financials, technical indicators
  (SMA/EMA/RSI/MACD), futures, economy (Fed data, EU consumer spending),
  ETF Global, TMX corporate events, Benzinga, summaries, and experimental vX endpoints.
- Automatic pagination for `list_*` methods as a `futures::Stream` (follows `next_url`).
- WebSocket streaming client for stocks, options, forex, crypto, indices, and futures
  with automatic reconnect and resubscribe.
- Filter operators (`.gt/.gte/.lt/.lte/.any_of`) as typed optional arguments.

## Getting started

Add to `Cargo.toml`:

```toml
[dependencies]
massive = { path = "..." } # crates.io release pending
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
futures = "0.3"
```

Set your API key in the `MASSIVE_API_KEY` environment variable, or pass it directly.

```rust
use futures::TryStreamExt;
use massive::rest::{AggsApi, TradesApi};
use massive::RESTClient;

#[tokio::main]
async fn main() -> massive::Result<()> {
    let client = RESTClient::from_env()?;

    // List aggregates (bars) — streams all pages automatically.
    let mut aggs = client.list_aggs(
        "AAPL", 1, "minute", "2023-01-01", "2023-06-13",
        Some(true), Some("asc"), Some(50000), None,
    );
    while let Some(agg) = aggs.try_next().await? {
        println!("{:?}", agg);
    }

    // Get last trade.
    let trade = client.get_last_trade("AAPL", None).await?;
    println!("{:?}", trade);

    Ok(())
}
```

Each REST module is a trait implemented on `Client` (`RESTClient` is an alias):
`AggsApi`, `TradesApi`, `QuotesApi`, `SnapshotApi`, `ReferenceApi`, `FinancialsApi`,
`IndicatorsApi`, `FuturesApi`, `EconomyApi`, `EtfGlobalApi`, `TmxApi`,
`SummariesApi`, `BenzingaApi`, `VxApi`. Import the traits you use.

## Pagination

Pagination is enabled by default: `limit` controls the *page size* and the stream
follows `next_url` until all results are delivered. To fetch a single page:

```rust
let client = RESTClient::from_env()?.with_pagination(false);
```

## Filter operators

Python's `params={"ticker.gte": "A"}` filters are typed optional arguments, e.g.
`list_dividends(.., ex_dividend_date_gte: Some("2024-01-01"), ..)` serializes as
`ex_dividend_date.gte=2024-01-01`.

## WebSocket

```rust
use massive::websocket::{Market, WebSocketClient, WebSocketMessage};

#[tokio::main]
async fn main() -> massive::Result<()> {
    let mut client = WebSocketClient::from_env()?
        .with_market(Market::Stocks)
        .with_subscriptions(&["T.*", "Q.AAPL"]); // all trades, AAPL quotes

    client
        .connect(|msgs| async move {
            for m in msgs {
                if let WebSocketMessage::EquityTrade(t) = m {
                    println!("{:?}", t);
                }
            }
        })
        .await
}
```

Feeds (`Feed::RealTime`, `Feed::Delayed`, …) and markets (including
`Market::FuturesCME` etc.) mirror the Python client; the client authenticates,
reconciles `subscribe`/`unsubscribe` calls live, and reconnects with resubscribe
on connection loss (`with_max_reconnects`, default 5).

## Examples

```sh
cargo run --example aggs              # list aggregate bars
cargo run --example last_trade_quote  # last trade/quote
cargo run --example snapshot          # market snapshot + options chain
cargo run --example websocket         # streaming trades/quotes
```

## API documentation

The full API reference (every trait method, parameter, and response model) is
documented with rustdoc. Build and open it locally:

```sh
cargo doc --open
```

## Testing

`cargo test` runs the full wiremock-based integration suite (no API key required).

## Parity notes and deliberate deviations

- Method names, paths, parameter names/order, and response models track the Python
  client 1:1. `Option<T>` arguments serialize only when `Some`; server-side defaults
  apply otherwise (Python client-side defaults like `limit=10` are not sent).
- The Python `raw: bool` and free-form `params: dict` escape hatches are omitted —
  all parameters are typed. Launchpad edge headers are supported via
  `RequestOptions::with_edge_headers` as the trailing `options` argument.
- Python's `WebSocketClient.run` (an `asyncio.run` wrapper) and `custom_json` are
  omitted; use your own Tokio runtime.
- Quirk preserved for parity: `RealTimeCurrencyConversion.from_` deserializes the
  wire key `from_` (matching the Python model, which reads `from_` rather than `from`).

## License

MIT OR Apache-2.0
