//! WebSocket streaming client for the Massive API.
//!
//! Port of the Python client's `massive.websocket` module. Python's `run`
//! (a thin `asyncio.run` wrapper) and the `raw`/`custom_json` options are
//! intentionally omitted: Rust callers drive `connect` from their own
//! runtime and always receive parsed [`WebSocketMessage`]s.

pub mod models;

pub use models::*;

use crate::error::{Error, Result};
use futures::{SinkExt, StreamExt};
use std::collections::HashSet;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::{debug, info, warn};

const ENV_KEY: &str = "MASSIVE_API_KEY";
const RECV_TIMEOUT: Duration = Duration::from_secs(1);

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WsSink = futures::stream::SplitSink<WsStream, Message>;

/// WebSocket streaming client, mirroring the Python client's `WebSocketClient`.
#[derive(Debug)]
pub struct WebSocketClient {
    api_key: String,
    feed: Feed,
    market: Market,
    secure: bool,
    trace: bool,
    max_reconnects: Option<u64>,
    scheduled_subs: HashSet<String>,
    subs: HashSet<String>,
    schedule_resub: bool,
    sink: Option<WsSink>,
}

impl WebSocketClient {
    /// Create a new client with the given API key.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        let api_key = api_key.into();
        if api_key.is_empty() {
            return Err(Error::MissingApiKey);
        }
        Ok(Self {
            api_key,
            feed: Feed::RealTime,
            market: Market::Stocks,
            secure: true,
            trace: false,
            max_reconnects: Some(5),
            scheduled_subs: HashSet::new(),
            subs: HashSet::new(),
            schedule_resub: true,
            sink: None,
        })
    }

    /// Create a client from the `MASSIVE_API_KEY` environment variable.
    pub fn from_env() -> Result<Self> {
        let key = std::env::var(ENV_KEY).map_err(|_| Error::MissingApiKey)?;
        Self::new(key)
    }

    /// Set the feed to subscribe to (default: [`Feed::RealTime`]).
    pub fn with_feed(mut self, feed: Feed) -> Self {
        self.feed = feed;
        self
    }

    /// Set the market to subscribe to (default: [`Market::Stocks`]).
    pub fn with_market(mut self, market: Market) -> Self {
        self.market = market;
        self
    }

    /// Set the initial subscriptions (e.g. `&["T.AAPL", "Q.*"]`).
    pub fn with_subscriptions(mut self, subscriptions: &[&str]) -> Self {
        self.scheduled_subs = subscriptions.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Set how many times to reconnect on outage before giving up (default: `Some(5)`).
    pub fn with_max_reconnects(mut self, max_reconnects: Option<u64>) -> Self {
        self.max_reconnects = max_reconnects;
        self
    }

    /// Use a secure (`wss://`) or plain (`ws://`) connection (default: true).
    pub fn with_secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    /// Log status and connection messages at info level (default: false).
    pub fn with_trace(mut self, trace: bool) -> Self {
        self.trace = trace;
        self
    }

    /// The WebSocket URL: `ws{s}://{feed}/{market}`.
    fn url(&self) -> String {
        format!(
            "ws{}://{}/{}",
            if self.secure { "s" } else { "" },
            self.feed.as_str(),
            self.market.as_str()
        )
    }

    /// Send a JSON action frame on the open connection.
    async fn send_action(&mut self, action: &str, params: &str) -> Result<()> {
        if let Some(sink) = self.sink.as_mut() {
            let frame = serde_json::json!({"action": action, "params": params}).to_string();
            sink.send(Message::Text(frame))
                .await
                .map_err(|e| Error::WebSocket(e.to_string()))?;
        }
        Ok(())
    }

    /// Connect to the server and run `processor(msgs)` on every new message batch.
    ///
    /// Reconnects on connection errors up to `max_reconnects` times; returns
    /// `Err(Error::Auth)` if authentication fails.
    pub async fn connect<F, Fut>(&mut self, mut processor: F) -> Result<()>
    where
        F: FnMut(Vec<WebSocketMessage>) -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        let url = self.url();
        debug!("connect: {}", url);
        let mut reconnects: u64 = 0;

        loop {
            let ws = match connect_async(&url).await {
                Ok((ws, _)) => ws,
                Err(e) => {
                    reconnects += 1;
                    warn!("connection error: {}", e);
                    if self.max_reconnects.is_some_and(|max| reconnects > max) {
                        return Err(Error::WebSocket(e.to_string()));
                    }
                    continue;
                }
            };
            let (sink, mut stream) = ws.split();
            self.sink = Some(sink);

            let session: Result<()> = async {
                // Server greets with a "connected" status message.
                match stream.next().await {
                    Some(Ok(msg)) => debug!("connected: {}", msg),
                    other => debug!("connected (no greeting): {:?}", other),
                }
                debug!("authing...");
                self.send_action("auth", &self.api_key.clone()).await?;
                let auth_msg = match stream.next().await {
                    Some(Ok(Message::Text(t))) => t,
                    Some(Ok(other)) => {
                        return Err(Error::WebSocket(format!("unexpected auth frame: {other}")))
                    }
                    Some(Err(e)) => return Err(Error::WebSocket(e.to_string())),
                    None => return Err(Error::WebSocket("connection closed during auth".into())),
                };
                let auth_parsed: Vec<serde_json::Value> = serde_json::from_str(&auth_msg)?;
                debug!("authed: {}", auth_msg);
                if auth_parsed
                    .first()
                    .and_then(|m| m.get("status"))
                    .and_then(|s| s.as_str())
                    == Some("auth_failed")
                {
                    let message = auth_parsed
                        .first()
                        .and_then(|m| m.get("message"))
                        .and_then(|s| s.as_str())
                        .unwrap_or("authentication failed")
                        .to_string();
                    return Err(Error::Auth(message));
                }

                loop {
                    if self.schedule_resub {
                        debug!("reconciling: {:?} {:?}", self.subs, self.scheduled_subs);
                        let new_subs: Vec<String> =
                            self.scheduled_subs.difference(&self.subs).cloned().collect();
                        if !new_subs.is_empty() {
                            self.send_action("subscribe", &new_subs.join(",")).await?;
                        }
                        let old_subs: Vec<String> =
                            self.subs.difference(&self.scheduled_subs).cloned().collect();
                        if !old_subs.is_empty() {
                            self.send_action("unsubscribe", &old_subs.join(",")).await?;
                        }
                        self.subs = self.scheduled_subs.clone();
                        self.schedule_resub = false;
                    }

                    let frame = match tokio::time::timeout(RECV_TIMEOUT, stream.next()).await {
                        Ok(Some(frame)) => frame,
                        // No frame within the timeout; loop to reconcile subs.
                        Ok(None) => return Err(Error::WebSocket("connection closed".into())),
                        Err(_) => continue,
                    };

                    let text = match frame {
                        Ok(Message::Text(t)) => t,
                        Ok(Message::Close(_)) => {
                            debug!("connection closed (OK)");
                            self.sink = None;
                            return Ok(());
                        }
                        Ok(_) => continue,
                        Err(e) => return Err(Error::WebSocket(e.to_string())),
                    };

                    let values: Vec<serde_json::Value> = serde_json::from_str(&text)?;
                    let msgs = parse_messages(values, self.market);
                    if !msgs.is_empty() {
                        if self.trace {
                            info!("received {} messages", msgs.len());
                        }
                        processor(msgs).await;
                    }
                }
            }
            .await;

            match session {
                Ok(()) => return Ok(()),
                // Auth failures are not retried.
                Err(e @ Error::Auth(_)) => return Err(e),
                Err(e) => {
                    debug!("connection closed (ERR): {}", e);
                    reconnects += 1;
                    self.sink = None;
                    self.scheduled_subs = std::mem::take(&mut self.subs);
                    self.schedule_resub = true;
                    if self.max_reconnects.is_some_and(|max| reconnects > max) {
                        return Err(e);
                    }
                }
            }
        }
    }

    /// Schedule subscriptions, applying `X.*` wildcard semantics.
    pub fn subscribe(&mut self, subscriptions: &[&str]) {
        for s in subscriptions {
            let Some((topic, sym)) = parse_subscription(s) else {
                continue;
            };
            debug!("sub desired: {}", s);
            self.scheduled_subs.insert(s.to_string());
            // If user subs to X.*, remove other X.<sym> entries.
            if sym == "*" {
                for t in self.subs.clone() {
                    if t.starts_with(topic) {
                        self.scheduled_subs.remove(&t);
                    }
                }
            }
        }
        self.schedule_resub = true;
    }

    /// Schedule unsubscriptions, applying `X.*` wildcard semantics.
    pub fn unsubscribe(&mut self, subscriptions: &[&str]) {
        for s in subscriptions {
            let Some((topic, sym)) = parse_subscription(s) else {
                continue;
            };
            debug!("sub undesired: {}", s);
            self.scheduled_subs.remove(*s);
            // If user unsubs to X.*, remove other X.<sym> entries.
            if sym == "*" {
                for t in self.subs.clone() {
                    if t.starts_with(topic) {
                        self.scheduled_subs.remove(&t);
                    }
                }
            }
        }
        self.schedule_resub = true;
    }

    /// Unsubscribe from all subscriptions.
    pub fn unsubscribe_all(&mut self) {
        self.scheduled_subs.clear();
        self.schedule_resub = true;
    }

    /// Close the websocket connection, if one is open.
    pub async fn close(&mut self) {
        debug!("closing");
        if let Some(mut sink) = self.sink.take() {
            if let Err(e) = sink.send(Message::Close(None)).await {
                warn!("error sending close frame: {}", e);
            }
        } else {
            warn!("no websocket open to close");
        }
    }
}

/// Split a subscription string into `(topic, symbol)` at the first period.
fn parse_subscription(s: &str) -> Option<(&str, &str)> {
    let s = s.trim();
    match s.split_once('.') {
        Some((topic, sym)) => Some((topic, sym)),
        None => {
            warn!("invalid subscription: {}", s);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client() -> WebSocketClient {
        WebSocketClient::new("test-key").unwrap()
    }

    #[test]
    fn new_rejects_empty_api_key() {
        assert!(matches!(
            WebSocketClient::new(""),
            Err(crate::error::Error::MissingApiKey)
        ));
    }

    #[test]
    fn url_matches_python_scheme() {
        let c = client();
        assert_eq!(c.url(), "wss://socket.massive.com/stocks");
        let c = client().with_secure(false).with_market(Market::FuturesCME);
        assert_eq!(c.url(), "ws://socket.massive.com/futures/cme");
    }

    #[test]
    fn subscribe_wildcard_replaces_same_topic_symbols() {
        let mut c = client().with_subscriptions(&["T.AAPL"]);
        // Simulate an active subscription, then subscribe to the wildcard.
        c.subs.insert("T.AAPL".to_string());
        c.subscribe(&["T.*"]);
        assert!(c.scheduled_subs.contains("T.*"));
        assert!(!c.scheduled_subs.contains("T.AAPL"));
    }

    #[test]
    fn unsubscribe_removes_scheduled() {
        let mut c = client().with_subscriptions(&["T.AAPL", "Q.AAPL"]);
        c.unsubscribe(&["T.AAPL"]);
        assert!(!c.scheduled_subs.contains("T.AAPL"));
        assert!(c.scheduled_subs.contains("Q.AAPL"));
    }

    #[test]
    fn unsubscribe_all_clears() {
        let mut c = client().with_subscriptions(&["T.*"]);
        c.unsubscribe_all();
        assert!(c.scheduled_subs.is_empty());
    }

    #[test]
    fn invalid_subscription_is_skipped() {
        let mut c = client();
        c.subscribe(&["no-dot-here"]);
        assert!(c.scheduled_subs.is_empty());
    }
}

