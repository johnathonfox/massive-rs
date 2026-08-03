//! Tests for WebSocket message parsing (market-dependent event dispatch).

use massive::websocket::{parse_messages, Market, WebSocketMessage};
use serde_json::json;

fn parse(value: serde_json::Value, market: Market) -> Vec<WebSocketMessage> {
    parse_messages(vec![value], market)
}

#[test]
fn stocks_trade_maps_to_equity_trade() {
    let msgs = parse(
        json!({"ev":"T","sym":"AAPL","x":4,"i":"1","z":3,"p":150.5,"s":100,"c":[1],"t":1536036818784i64,"q":123}),
        Market::Stocks,
    );
    match &msgs[..] {
        [WebSocketMessage::EquityTrade(t)] => {
            assert_eq!(t.symbol.as_deref(), Some("AAPL"));
            assert_eq!(t.price, Some(150.5));
            assert_eq!(t.size, Some(100));
            assert_eq!(t.timestamp, Some(1536036818784));
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn stocks_minute_agg_maps_to_equity_agg() {
    let msgs = parse(
        json!({"ev":"AM","sym":"AAPL","v":1000,"av":50000,"op":149.0,"vw":150.1,"o":149.5,"c":150.2,"h":150.4,"l":149.9,"a":150.0,"z":50,"s":1536036816000i64,"e":1536036817000i64}),
        Market::Stocks,
    );
    match &msgs[..] {
        [WebSocketMessage::EquityAgg(a)] => {
            assert_eq!(a.symbol.as_deref(), Some("AAPL"));
            assert_eq!(a.official_open_price, Some(149.0));
            assert_eq!(a.end_timestamp, Some(1536036817000));
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn futures_trade_uses_same_ev_code_as_equities() {
    // "T" means FuturesTrade on futures markets but EquityTrade on stocks.
    let msgs = parse(
        json!({"ev":"T","sym":"ESZ4","p":4500.25,"s":2,"t":1536036818784i64,"q":456}),
        Market::Futures,
    );
    match &msgs[..] {
        [WebSocketMessage::FuturesTrade(t)] => {
            assert_eq!(t.symbol.as_deref(), Some("ESZ4"));
            assert_eq!(t.sequence_number, Some(456));
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn futures_cme_market_also_maps_to_futures_models() {
    let msgs = parse(
        json!({"ev":"Q","sym":"ESZ4","bp":4500.0,"bs":10,"bt":1536036818783i64,"ap":4500.5,"as":12,"at":1536036818783i64,"t":1536036818784i64}),
        Market::FuturesCME,
    );
    assert!(matches!(msgs[..], [WebSocketMessage::FuturesQuote(_)]));
}

#[test]
fn crypto_trade_maps_to_crypto_trade() {
    let msgs = parse(
        json!({"ev":"XT","pair":"BTC-USD","x":1,"i":"123","p":30000.0,"s":0.5,"c":[1],"t":1536036818784i64,"r":1536036819000i64}),
        Market::Crypto,
    );
    match &msgs[..] {
        [WebSocketMessage::CryptoTrade(t)] => {
            assert_eq!(t.pair.as_deref(), Some("BTC-USD"));
            assert_eq!(t.price, Some(30000.0));
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn forex_quote_maps_to_forex_quote() {
    let msgs = parse(
        json!({"ev":"C","p":"EUR/USD","x":1,"a":1.09,"b":1.08,"t":1536036818784i64}),
        Market::Forex,
    );
    match &msgs[..] {
        [WebSocketMessage::ForexQuote(q)] => {
            assert_eq!(q.pair.as_deref(), Some("EUR/USD"));
            assert_eq!(q.ask_price, Some(1.09));
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn index_value_maps_on_indices_market() {
    let msgs = parse(
        json!({"ev":"V","val":4500.5,"T":"I:SPX","t":"2023-09-05T15:00:00Z"}),
        Market::Indices,
    );
    match &msgs[..] {
        [WebSocketMessage::IndexValue(v)] => {
            assert_eq!(v.ticker.as_deref(), Some("I:SPX"));
            assert_eq!(v.value, Some(4500.5));
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn status_messages_parse_as_status() {
    let msgs = parse(
        json!({"ev":"status","status":"connected","message":"Connected Successfully"}),
        Market::Stocks,
    );
    match &msgs[..] {
        [WebSocketMessage::Status(s)] => {
            assert_eq!(s.status.as_deref(), Some("connected"));
        }
        other => panic!("unexpected: {:?}", other),
    }
}

#[test]
fn unknown_event_type_is_skipped() {
    let msgs = parse(json!({"ev":"ZZZ","foo":1}), Market::Stocks);
    assert!(msgs.is_empty());
}
