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

// --- Local test-server harness --------------------------------------------
//
// These tests run the real `WebSocketClient` against a local
// tokio-tungstenite server to cover the auth handshake, live
// subscribe/unsubscribe reconciliation, and reconnect-with-resubscribe.

use futures::{SinkExt, StreamExt};
use massive::websocket::WebSocketClient;
use massive::Error;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

const TEST_TIMEOUT: Duration = Duration::from_secs(10);

type ServerWs = WebSocketStream<TcpStream>;

/// Bind a localhost websocket server; returns the listener and `host:port`.
async fn bind_server() -> (TcpListener, String) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let host = listener.local_addr().unwrap().to_string();
    (listener, host)
}

async fn accept_ws(listener: &TcpListener) -> ServerWs {
    let (stream, _) = listener.accept().await.unwrap();
    tokio_tungstenite::accept_async(stream).await.unwrap()
}

fn status_frame(status: &str, message: &str) -> Message {
    Message::Text(
        json!([{"ev":"status","status":status,"message":message}])
            .to_string()
            .into(),
    )
}

/// Read the next text frame as JSON, skipping non-text frames.
async fn next_json(ws: &mut ServerWs) -> serde_json::Value {
    loop {
        match timeout(TEST_TIMEOUT, ws.next()).await.unwrap() {
            Some(Ok(Message::Text(t))) => return serde_json::from_str(&t).unwrap(),
            Some(Ok(_)) => continue,
            other => panic!("unexpected frame: {:?}", other),
        }
    }
}

/// Perform the server side of the auth handshake, asserting the API key.
async fn expect_auth(ws: &mut ServerWs, expected_key: &str) {
    let auth = next_json(ws).await;
    assert_eq!(auth["action"], "auth");
    assert_eq!(auth["params"], expected_key);
    ws.send(status_frame("auth_success", "authenticated"))
        .await
        .unwrap();
}

#[tokio::test]
async fn auth_handshake_succeeds_and_subscribes() {
    let (listener, host) = bind_server().await;
    let server = tokio::spawn(async move {
        let mut ws = accept_ws(&listener).await;
        ws.send(status_frame("connected", "Connected Successfully"))
            .await
            .unwrap();
        expect_auth(&mut ws, "test-key").await;
        let sub = next_json(&mut ws).await;
        assert_eq!(sub["action"], "subscribe");
        assert_eq!(sub["params"], "T.AAPL");
        ws.send(Message::Close(None)).await.unwrap();
    });

    let mut client = WebSocketClient::new("test-key")
        .unwrap()
        .with_secure(false)
        .with_host(host)
        .with_subscriptions(&["T.AAPL"]);
    timeout(
        TEST_TIMEOUT,
        client.connect(|_: Vec<WebSocketMessage>| async {}),
    )
    .await
    .unwrap()
    .unwrap();
    timeout(TEST_TIMEOUT, server).await.unwrap().unwrap();
}

#[tokio::test]
async fn auth_failure_returns_auth_error() {
    let (listener, host) = bind_server().await;
    let server = tokio::spawn(async move {
        let mut ws = accept_ws(&listener).await;
        ws.send(status_frame("connected", "Connected Successfully"))
            .await
            .unwrap();
        let auth = next_json(&mut ws).await;
        assert_eq!(auth["action"], "auth");
        ws.send(status_frame("auth_failed", "Invalid API key"))
            .await
            .unwrap();
    });

    let mut client = WebSocketClient::new("bad-key")
        .unwrap()
        .with_secure(false)
        .with_host(host);
    let err = timeout(
        TEST_TIMEOUT,
        client.connect(|_: Vec<WebSocketMessage>| async {}),
    )
    .await
    .unwrap()
    .unwrap_err();
    match err {
        Error::Auth(msg) => assert_eq!(msg, "Invalid API key"),
        other => panic!("expected auth error, got {:?}", other),
    }
    timeout(TEST_TIMEOUT, server).await.unwrap().unwrap();
}

#[tokio::test]
async fn live_subscribe_unsubscribe_reconciles_on_server() {
    let (listener, host) = bind_server().await;
    let (observed_tx, mut observed_rx) = mpsc::unbounded_channel::<serde_json::Value>();

    let server = tokio::spawn(async move {
        let mut ws = accept_ws(&listener).await;
        ws.send(status_frame("connected", "Connected Successfully"))
            .await
            .unwrap();
        expect_auth(&mut ws, "test-key").await;
        // Report every action frame the client sends, then close after three.
        for _ in 0..3 {
            let msg = next_json(&mut ws).await;
            observed_tx.send(msg).unwrap();
        }
        ws.send(Message::Close(None)).await.unwrap();
    });

    let mut client = WebSocketClient::new("test-key")
        .unwrap()
        .with_secure(false)
        .with_host(host)
        .with_subscriptions(&["T.AAPL"]);
    let control = client.control();
    let client_task =
        tokio::spawn(async move { client.connect(|_: Vec<WebSocketMessage>| async {}).await });

    // Initial subscription from connect.
    let msg = timeout(TEST_TIMEOUT, observed_rx.recv())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(msg["action"], "subscribe");
    assert_eq!(msg["params"], "T.AAPL");

    // A live subscribe is pushed to the open connection.
    control.subscribe(&["Q.AAPL"]).unwrap();
    let msg = timeout(TEST_TIMEOUT, observed_rx.recv())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(msg["action"], "subscribe");
    assert_eq!(msg["params"], "Q.AAPL");

    // A live unsubscribe reconciles only the removed channel.
    control.unsubscribe(&["T.AAPL"]).unwrap();
    let msg = timeout(TEST_TIMEOUT, observed_rx.recv())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(msg["action"], "unsubscribe");
    assert_eq!(msg["params"], "T.AAPL");

    timeout(TEST_TIMEOUT, client_task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    timeout(TEST_TIMEOUT, server).await.unwrap().unwrap();
}

#[tokio::test]
async fn reconnect_resubscribes_active_subscriptions() {
    let (listener, host) = bind_server().await;
    let (observed_tx, mut observed_rx) = mpsc::unbounded_channel::<serde_json::Value>();

    let server = tokio::spawn(async move {
        // First connection: auth + initial subscribe, then drop the socket
        // abruptly (no Close frame).
        let mut ws = accept_ws(&listener).await;
        ws.send(status_frame("connected", "Connected Successfully"))
            .await
            .unwrap();
        expect_auth(&mut ws, "test-key").await;
        let sub = next_json(&mut ws).await;
        observed_tx.send(sub).unwrap();
        drop(ws);

        // Second connection: auth again + resubscribe of the same channel.
        let mut ws = accept_ws(&listener).await;
        ws.send(status_frame("connected", "Connected Successfully"))
            .await
            .unwrap();
        expect_auth(&mut ws, "test-key").await;
        let sub = next_json(&mut ws).await;
        observed_tx.send(sub).unwrap();
        ws.send(Message::Close(None)).await.unwrap();
    });

    let mut client = WebSocketClient::new("test-key")
        .unwrap()
        .with_secure(false)
        .with_host(host)
        .with_max_reconnects(Some(1))
        .with_subscriptions(&["T.AAPL"]);
    timeout(
        TEST_TIMEOUT,
        client.connect(|_: Vec<WebSocketMessage>| async {}),
    )
    .await
    .unwrap()
    .unwrap();
    timeout(TEST_TIMEOUT, server).await.unwrap().unwrap();

    let first = observed_rx.recv().await.unwrap();
    let second = observed_rx.recv().await.unwrap();
    assert_eq!(first["action"], "subscribe");
    assert_eq!(first["params"], "T.AAPL");
    assert_eq!(second["action"], "subscribe");
    assert_eq!(second["params"], "T.AAPL");
}
