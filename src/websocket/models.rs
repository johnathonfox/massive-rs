//! WebSocket feed/market enums and streamed message models.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::warn;

/// WebSocket feed host, mirroring the Python client's `Feed` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feed {
    Delayed,
    RealTime,
    Nasdaq,
    PolyFeed,
    PolyFeedPlus,
    StarterFeed,
    Launchpad,
    Business,
    EdgxBusiness,
    IEXBusiness,
    DelayedBusiness,
    DelayedEdgxBusiness,
    DelayedNasdaqLastSaleBusiness,
    DelayedNasdaqBasic,
    DelayedFullMarketBusiness,
    FullMarketBusiness,
    NasdaqLastSaleBusiness,
    NasdaqBasicBusiness,
}

impl Feed {
    /// The feed's hostname.
    pub fn as_str(&self) -> &'static str {
        match self {
            Feed::Delayed => "delayed.massive.com",
            Feed::RealTime => "socket.massive.com",
            Feed::Nasdaq => "nasdaqfeed.massive.com",
            Feed::PolyFeed => "polyfeed.massive.com",
            Feed::PolyFeedPlus => "polyfeedplus.massive.com",
            Feed::StarterFeed => "starterfeed.massive.com",
            Feed::Launchpad => "launchpad.massive.com",
            Feed::Business => "business.massive.com",
            Feed::EdgxBusiness => "edgx-business.massive.com",
            Feed::IEXBusiness => "iex-business.massive.com",
            Feed::DelayedBusiness => "delayed-business.massive.com",
            Feed::DelayedEdgxBusiness => "delayed-edgx-business.massive.com",
            Feed::DelayedNasdaqLastSaleBusiness => "delayed-nasdaq-last-sale-business.massive.com",
            Feed::DelayedNasdaqBasic => "delayed-nasdaq-basic-business.massive.com",
            Feed::DelayedFullMarketBusiness => "delayed-fullmarket-business.massive.com",
            Feed::FullMarketBusiness => "fullmarket-business.massive.com",
            Feed::NasdaqLastSaleBusiness => "nasdaq-last-sale-business.massive.com",
            Feed::NasdaqBasicBusiness => "nasdaq-basic-business.massive.com",
        }
    }
}

/// WebSocket market cluster, mirroring the Python client's `Market` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Market {
    Stocks,
    Options,
    Forex,
    Crypto,
    Indices,
    Futures,
    FuturesCME,
    FuturesCBOT,
    FuturesNYMEX,
    FuturesCOMEX,
}

impl Market {
    /// The market's URL path segment.
    pub fn as_str(&self) -> &'static str {
        match self {
            Market::Stocks => "stocks",
            Market::Options => "options",
            Market::Forex => "forex",
            Market::Crypto => "crypto",
            Market::Indices => "indices",
            Market::Futures => "futures",
            Market::FuturesCME => "futures/cme",
            Market::FuturesCBOT => "futures/cbot",
            Market::FuturesNYMEX => "futures/nymex",
            Market::FuturesCOMEX => "futures/comex",
        }
    }
}

/// Aggregate data for either stock tickers, option contracts or index tickers.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EquityAgg {
    #[serde(rename = "ev")]
    pub event_type: Option<String>,
    #[serde(rename = "sym")]
    pub symbol: Option<String>,
    #[serde(rename = "v")]
    pub volume: Option<f64>,
    #[serde(rename = "av")]
    pub accumulated_volume: Option<f64>,
    #[serde(rename = "op")]
    pub official_open_price: Option<f64>,
    #[serde(rename = "vw")]
    pub vwap: Option<f64>,
    #[serde(rename = "o")]
    pub open: Option<f64>,
    #[serde(rename = "c")]
    pub close: Option<f64>,
    #[serde(rename = "h")]
    pub high: Option<f64>,
    #[serde(rename = "l")]
    pub low: Option<f64>,
    #[serde(rename = "a")]
    pub aggregate_vwap: Option<f64>,
    #[serde(rename = "z")]
    pub average_size: Option<f64>,
    #[serde(rename = "s")]
    pub start_timestamp: Option<i64>,
    #[serde(rename = "e")]
    pub end_timestamp: Option<i64>,
    pub otc: Option<bool>,
    #[serde(rename = "dv")]
    pub fractional_volume: Option<String>,
    #[serde(rename = "dav")]
    pub fractional_accumulated_volume: Option<String>,
}

/// Aggregate data for either forex currency pairs or crypto pairs.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CurrencyAgg {
    #[serde(rename = "ev")]
    pub event_type: Option<String>,
    pub pair: Option<String>,
    #[serde(rename = "o")]
    pub open: Option<f64>,
    #[serde(rename = "c")]
    pub close: Option<f64>,
    #[serde(rename = "h")]
    pub high: Option<f64>,
    #[serde(rename = "l")]
    pub low: Option<f64>,
    #[serde(rename = "v")]
    pub volume: Option<f64>,
    #[serde(rename = "vw")]
    pub vwap: Option<f64>,
    #[serde(rename = "s")]
    pub start_timestamp: Option<i64>,
    #[serde(rename = "e")]
    pub end_timestamp: Option<i64>,
    #[serde(rename = "z")]
    pub avg_trade_size: Option<f64>,
}

/// Trade data for either stock tickers or option contracts.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EquityTrade {
    #[serde(rename = "ev")]
    pub event_type: Option<String>,
    #[serde(rename = "sym")]
    pub symbol: Option<String>,
    #[serde(rename = "x")]
    pub exchange: Option<i64>,
    #[serde(rename = "i")]
    pub id: Option<String>,
    #[serde(rename = "z")]
    pub tape: Option<i64>,
    #[serde(rename = "p")]
    pub price: Option<f64>,
    #[serde(rename = "s")]
    pub size: Option<i64>,
    #[serde(rename = "ds")]
    pub fractional_shares: Option<String>,
    #[serde(rename = "c")]
    pub conditions: Option<Vec<i64>>,
    #[serde(rename = "t")]
    pub timestamp: Option<i64>,
    #[serde(rename = "q")]
    pub sequence_number: Option<i64>,
    #[serde(rename = "trfi")]
    pub trf_id: Option<i64>,
    #[serde(rename = "trft")]
    pub trf_timestamp: Option<i64>,
}

/// Trade data for a crypto pair.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CryptoTrade {
    #[serde(rename = "ev")]
    pub event_type: Option<String>,
    pub pair: Option<String>,
    #[serde(rename = "x")]
    pub exchange: Option<i64>,
    #[serde(rename = "i")]
    pub id: Option<String>,
    #[serde(rename = "p")]
    pub price: Option<f64>,
    #[serde(rename = "s")]
    pub size: Option<f64>,
    #[serde(rename = "c")]
    pub conditions: Option<Vec<i64>>,
    #[serde(rename = "t")]
    pub timestamp: Option<i64>,
    #[serde(rename = "r")]
    pub received_timestamp: Option<i64>,
}

/// Quote data for either stock tickers or option contracts.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EquityQuote {
    #[serde(rename = "ev")]
    pub event_type: Option<String>,
    #[serde(rename = "sym")]
    pub symbol: Option<String>,
    #[serde(rename = "bx")]
    pub bid_exchange_id: Option<i64>,
    #[serde(rename = "bp")]
    pub bid_price: Option<f64>,
    #[serde(rename = "bs")]
    pub bid_size: Option<i64>,
    #[serde(rename = "ax")]
    pub ask_exchange_id: Option<i64>,
    #[serde(rename = "ap")]
    pub ask_price: Option<f64>,
    #[serde(rename = "as")]
    pub ask_size: Option<i64>,
    #[serde(rename = "c")]
    pub condition: Option<i64>,
    #[serde(rename = "i")]
    pub indicators: Option<Vec<i64>>,
    #[serde(rename = "t")]
    pub timestamp: Option<i64>,
    #[serde(rename = "z")]
    pub tape: Option<i64>,
    #[serde(rename = "q")]
    pub sequence_number: Option<i64>,
    #[serde(rename = "trfi")]
    pub trf_id: Option<i64>,
    #[serde(rename = "trft")]
    pub trf_timestamp: Option<i64>,
}

/// Quote data for a forex currency pair.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ForexQuote {
    #[serde(rename = "ev")]
    pub event_type: Option<String>,
    #[serde(rename = "p")]
    pub pair: Option<String>,
    #[serde(rename = "x")]
    pub exchange_id: Option<i64>,
    #[serde(rename = "a")]
    pub ask_price: Option<f64>,
    #[serde(rename = "b")]
    pub bid_price: Option<f64>,
    #[serde(rename = "t")]
    pub timestamp: Option<i64>,
}

/// Quote data for a crypto pair.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CryptoQuote {
    #[serde(rename = "ev")]
    pub event_type: Option<String>,
    pub pair: Option<String>,
    #[serde(rename = "bp")]
    pub bid_price: Option<i64>,
    #[serde(rename = "bs")]
    pub bid_size: Option<f64>,
    #[serde(rename = "ap")]
    pub ask_price: Option<i64>,
    #[serde(rename = "as")]
    pub ask_size: Option<i64>,
    #[serde(rename = "t")]
    pub timestamp: Option<f64>,
    #[serde(rename = "x")]
    pub exchange_id: Option<i64>,
    #[serde(rename = "r")]
    pub received_timestamp: Option<i64>,
}

/// Imbalance event data for a given stock ticker symbol.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Imbalance {
    #[serde(rename = "ev")]
    pub event_type: Option<String>,
    #[serde(rename = "T")]
    pub symbol: Option<String>,
    #[serde(rename = "t")]
    pub time_stamp: Option<i64>,
    #[serde(rename = "at")]
    pub auction_time: Option<i64>,
    #[serde(rename = "a")]
    pub auction_type: Option<String>,
    #[serde(rename = "i")]
    pub symbol_sequence: Option<i64>,
    #[serde(rename = "x")]
    pub exchange_id: Option<i64>,
    #[serde(rename = "o")]
    pub imbalance_quantity: Option<i64>,
    #[serde(rename = "p")]
    pub paired_quantity: Option<i64>,
    #[serde(rename = "b")]
    pub book_clearing_price: Option<f64>,
}

/// Limit up/limit down (LULD) event data for a given stock ticker symbol.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LimitUpLimitDown {
    #[serde(rename = "ev")]
    pub event_type: Option<String>,
    #[serde(rename = "T")]
    pub symbol: Option<String>,
    #[serde(rename = "h")]
    pub high_price: Option<f64>,
    #[serde(rename = "l")]
    pub low_price: Option<f64>,
    #[serde(rename = "i")]
    pub indicators: Option<Vec<i64>>,
    #[serde(rename = "z")]
    pub tape: Option<i64>,
    #[serde(rename = "t")]
    pub timestamp: Option<i64>,
    #[serde(rename = "q")]
    pub sequence_number: Option<i64>,
}

/// Level 2 book data for a given crypto pair.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Level2Book {
    #[serde(rename = "ev")]
    pub event_type: Option<String>,
    pub pair: Option<String>,
    #[serde(rename = "b")]
    pub bid_prices: Option<f64>,
    #[serde(rename = "a")]
    pub ask_prices: Option<f64>,
    #[serde(rename = "t")]
    pub timestamp: Option<i64>,
    #[serde(rename = "x")]
    pub exchange_id: Option<i64>,
    #[serde(rename = "r")]
    pub received_timestamp: Option<i64>,
}

/// Value data for a given index ticker.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IndexValue {
    #[serde(rename = "ev")]
    pub event_type: Option<String>,
    #[serde(rename = "val")]
    pub value: Option<f64>,
    #[serde(rename = "T")]
    pub ticker: Option<String>,
    #[serde(rename = "t")]
    pub timestamp: Option<String>,
}

/// Launchpad value data for a given ticker symbol.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LaunchpadValue {
    #[serde(rename = "ev")]
    pub event_type: Option<String>,
    #[serde(rename = "val")]
    pub value: Option<f64>,
    #[serde(rename = "sym")]
    pub symbol: Option<String>,
    #[serde(rename = "t")]
    pub timestamp: Option<i64>,
}

/// Fair market value data for a given ticker symbol.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FairMarketValue {
    #[serde(rename = "ev")]
    pub event_type: Option<String>,
    pub fmv: Option<f64>,
    #[serde(rename = "sym")]
    pub ticker: Option<String>,
    #[serde(rename = "t")]
    pub timestamp: Option<i64>,
}

/// Trade data for a futures contract.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FuturesTrade {
    #[serde(rename = "ev")]
    pub event_type: Option<String>,
    #[serde(rename = "sym")]
    pub symbol: Option<String>,
    #[serde(rename = "p")]
    pub price: Option<f64>,
    #[serde(rename = "s")]
    pub size: Option<i64>,
    #[serde(rename = "t")]
    pub timestamp: Option<i64>,
    #[serde(rename = "q")]
    pub sequence_number: Option<i64>,
}

/// Quote data for a futures contract.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FuturesQuote {
    #[serde(rename = "ev")]
    pub event_type: Option<String>,
    #[serde(rename = "sym")]
    pub symbol: Option<String>,
    #[serde(rename = "bp")]
    pub bid_price: Option<f64>,
    #[serde(rename = "bs")]
    pub bid_size: Option<i64>,
    #[serde(rename = "bt")]
    pub bid_timestamp: Option<i64>,
    #[serde(rename = "ap")]
    pub ask_price: Option<f64>,
    #[serde(rename = "as")]
    pub ask_size: Option<i64>,
    #[serde(rename = "at")]
    pub ask_timestamp: Option<i64>,
    #[serde(rename = "t")]
    pub sip_timestamp: Option<i64>,
}

/// Aggregate data for a futures contract.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FuturesAgg {
    #[serde(rename = "ev")]
    pub event_type: Option<String>,
    #[serde(rename = "sym")]
    pub symbol: Option<String>,
    #[serde(rename = "v")]
    pub volume: Option<i64>,
    #[serde(rename = "dv")]
    pub total_value: Option<i64>,
    #[serde(rename = "o")]
    pub open: Option<f64>,
    #[serde(rename = "c")]
    pub close: Option<f64>,
    #[serde(rename = "h")]
    pub high: Option<f64>,
    #[serde(rename = "l")]
    pub low: Option<f64>,
    #[serde(rename = "n")]
    pub transactions: Option<i64>,
    #[serde(rename = "s")]
    pub start_timestamp: Option<i64>,
    #[serde(rename = "e")]
    pub end_timestamp: Option<i64>,
}

/// Connection status envelope message.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StatusMessage {
    #[serde(rename = "ev")]
    pub event_type: Option<String>,
    pub status: Option<String>,
    pub message: Option<String>,
}

/// A parsed WebSocket message, one variant per streamed model.
#[derive(Debug, Clone, PartialEq)]
pub enum WebSocketMessage {
    EquityAgg(EquityAgg),
    CurrencyAgg(CurrencyAgg),
    EquityTrade(EquityTrade),
    CryptoTrade(CryptoTrade),
    EquityQuote(EquityQuote),
    ForexQuote(ForexQuote),
    CryptoQuote(CryptoQuote),
    Imbalance(Imbalance),
    LimitUpLimitDown(LimitUpLimitDown),
    Level2Book(Level2Book),
    IndexValue(IndexValue),
    LaunchpadValue(LaunchpadValue),
    FairMarketValue(FairMarketValue),
    FuturesTrade(FuturesTrade),
    FuturesQuote(FuturesQuote),
    FuturesAgg(FuturesAgg),
    Status(StatusMessage),
}

/// Deserialize a value into `T`, warning and returning `None` on failure.
fn de<T: serde::de::DeserializeOwned>(value: &Value) -> Option<T> {
    match serde_json::from_value(value.clone()) {
        Ok(v) => Some(v),
        Err(e) => {
            warn!("could not parse message {}: {}", value, e);
            None
        }
    }
}

/// Parse a single raw message value into a typed message for the given market.
pub fn parse_single(value: &Value, market: Market) -> Option<WebSocketMessage> {
    let event_type = value.get("ev").and_then(Value::as_str).unwrap_or_default();
    if event_type == "status" {
        return de::<StatusMessage>(value).map(WebSocketMessage::Status);
    }
    match market {
        Market::Stocks => match event_type {
            "A" | "AM" => de::<EquityAgg>(value).map(WebSocketMessage::EquityAgg),
            "T" => de::<EquityTrade>(value).map(WebSocketMessage::EquityTrade),
            "Q" => de::<EquityQuote>(value).map(WebSocketMessage::EquityQuote),
            "LULD" => de::<LimitUpLimitDown>(value).map(WebSocketMessage::LimitUpLimitDown),
            "FMV" => de::<FairMarketValue>(value).map(WebSocketMessage::FairMarketValue),
            "NOI" => de::<Imbalance>(value).map(WebSocketMessage::Imbalance),
            "LV" => de::<LaunchpadValue>(value).map(WebSocketMessage::LaunchpadValue),
            _ => warn_unknown(event_type, market),
        },
        Market::Options => match event_type {
            "A" | "AM" => de::<EquityAgg>(value).map(WebSocketMessage::EquityAgg),
            "T" => de::<EquityTrade>(value).map(WebSocketMessage::EquityTrade),
            "Q" => de::<EquityQuote>(value).map(WebSocketMessage::EquityQuote),
            "FMV" => de::<FairMarketValue>(value).map(WebSocketMessage::FairMarketValue),
            "LV" => de::<LaunchpadValue>(value).map(WebSocketMessage::LaunchpadValue),
            _ => warn_unknown(event_type, market),
        },
        Market::Indices => match event_type {
            "A" | "AM" => de::<EquityAgg>(value).map(WebSocketMessage::EquityAgg),
            "V" => de::<IndexValue>(value).map(WebSocketMessage::IndexValue),
            _ => warn_unknown(event_type, market),
        },
        Market::Futures
        | Market::FuturesCME
        | Market::FuturesCBOT
        | Market::FuturesNYMEX
        | Market::FuturesCOMEX => match event_type {
            "A" | "AM" => de::<FuturesAgg>(value).map(WebSocketMessage::FuturesAgg),
            "T" => de::<FuturesTrade>(value).map(WebSocketMessage::FuturesTrade),
            "Q" => de::<FuturesQuote>(value).map(WebSocketMessage::FuturesQuote),
            _ => warn_unknown(event_type, market),
        },
        Market::Crypto => match event_type {
            "XA" | "XAS" => de::<CurrencyAgg>(value).map(WebSocketMessage::CurrencyAgg),
            "XT" => de::<CryptoTrade>(value).map(WebSocketMessage::CryptoTrade),
            "XQ" => de::<CryptoQuote>(value).map(WebSocketMessage::CryptoQuote),
            "XL2" => de::<Level2Book>(value).map(WebSocketMessage::Level2Book),
            "FMV" => de::<FairMarketValue>(value).map(WebSocketMessage::FairMarketValue),
            "AM" => de::<EquityAgg>(value).map(WebSocketMessage::EquityAgg),
            "LV" => de::<LaunchpadValue>(value).map(WebSocketMessage::LaunchpadValue),
            _ => warn_unknown(event_type, market),
        },
        Market::Forex => match event_type {
            "CA" | "CAS" => de::<CurrencyAgg>(value).map(WebSocketMessage::CurrencyAgg),
            "C" => de::<ForexQuote>(value).map(WebSocketMessage::ForexQuote),
            "FMV" => de::<FairMarketValue>(value).map(WebSocketMessage::FairMarketValue),
            "AM" => de::<EquityAgg>(value).map(WebSocketMessage::EquityAgg),
            "LV" => de::<LaunchpadValue>(value).map(WebSocketMessage::LaunchpadValue),
            _ => warn_unknown(event_type, market),
        },
    }
}

/// Warn about an unrecognized event type.
fn warn_unknown(event_type: &str, market: Market) -> Option<WebSocketMessage> {
    warn!("Unknown event type '{}' for market {:?}", event_type, market);
    None
}

/// Parse a batch of raw message values, skipping unparseable ones.
pub fn parse_messages(values: Vec<Value>, market: Market) -> Vec<WebSocketMessage> {
    values
        .iter()
        .filter_map(|v| parse_single(v, market))
        .collect()
}
