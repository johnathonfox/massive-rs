//! Stream trades and quotes over WebSocket (mirrors the Python client's WebSocketClient).
//!
//! Usage: MASSIVE_API_KEY=<key> cargo run --example websocket

use massive::websocket::{Market, WebSocketClient, WebSocketMessage};

#[tokio::main]
async fn main() -> massive::Result<()> {
    let mut client = WebSocketClient::from_env()?
        .with_market(Market::Stocks)
        // Subscribe to all trades, and quotes for AAPL and META.
        .with_subscriptions(&["T.*", "Q.AAPL", "Q.META"]);

    client
        .connect(|msgs| async move {
            for m in msgs {
                match m {
                    WebSocketMessage::EquityTrade(t) => println!("trade: {:?}", t),
                    WebSocketMessage::EquityQuote(q) => println!("quote: {:?}", q),
                    other => println!("other: {:?}", other),
                }
            }
        })
        .await
}
