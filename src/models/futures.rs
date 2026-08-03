use serde::{Deserialize, Serialize};

/// A single aggregate bar for a futures contract in a given time window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesAgg {
    pub ticker: Option<String>,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub volume: Option<f64>,
    pub dollar_volume: Option<f64>,
    pub transactions: Option<i64>,
    pub window_start: Option<i64>,
    pub session_end_date: Option<String>,
    pub settlement_price: Option<f64>,
}

/// Represents a single futures contract (or a 'combo' contract).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesContract {
    pub ticker: Option<String>,
    pub product_code: Option<String>,
    pub trading_venue: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub date: Option<String>,
    pub active: Option<bool>,
    pub first_trade_date: Option<String>,
    pub last_trade_date: Option<String>,
    pub days_to_maturity: Option<i64>,
    pub min_order_quantity: Option<i64>,
    pub max_order_quantity: Option<i64>,
    pub settlement_date: Option<String>,
    pub settlement_tick_size: Option<f64>,
    pub spread_tick_size: Option<f64>,
    pub trade_tick_size: Option<f64>,
    pub group_code: Option<String>,
}

/// Represents a single futures product (or product 'combo').
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesProduct {
    pub product_code: Option<String>,
    pub name: Option<String>,
    pub date: Option<String>,
    pub trading_venue: Option<String>,
    pub asset_class: Option<String>,
    pub asset_sub_class: Option<String>,
    pub sector: Option<String>,
    pub sub_sector: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub last_updated: Option<String>,
    pub price_quotation: Option<String>,
    pub settlement_currency_code: Option<String>,
    pub settlement_method: Option<String>,
    pub settlement_type: Option<String>,
    pub trade_currency_code: Option<String>,
    pub unit_of_measure: Option<String>,
    pub unit_of_measure_qty: Option<f64>,
}

/// Represents a futures NBBO quote within a given time range.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesQuote {
    pub ticker: Option<String>,
    pub timestamp: Option<i64>,
    pub session_end_date: Option<String>,
    pub ask_price: Option<f64>,
    pub ask_size: Option<f64>,
    pub ask_timestamp: Option<i64>,
    pub bid_price: Option<f64>,
    pub bid_size: Option<f64>,
    pub bid_timestamp: Option<i64>,
    pub channel: Option<i64>,
    pub report_sequence: Option<i64>,
    pub sequence_number: Option<i64>,
}

/// Represents a futures trade within a given time range.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesTrade {
    pub ticker: Option<String>,
    pub timestamp: Option<i64>,
    pub session_end_date: Option<String>,
    pub channel: Option<i64>,
    pub price: Option<f64>,
    pub size: Option<f64>,
    pub report_sequence: Option<i64>,
    pub sequence_number: Option<i64>,
}

/// Represents a single schedule event for a given session_end_date and product.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesSchedule {
    pub event: Option<String>,
    pub timestamp: Option<String>,
    pub session_end_date: Option<String>,
    pub product_code: Option<String>,
    pub trading_venue: Option<String>,
    pub product_name: Option<String>,
}

/// Represents the market status of a futures product.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesMarketStatus {
    pub market_event: Option<String>,
    pub name: Option<String>,
    pub product_code: Option<String>,
    pub session_end_date: Option<String>,
    pub timestamp: Option<String>,
    pub trading_venue: Option<String>,
}

/// Details section of a futures snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesSnapshotDetails {
    pub open_interest: Option<i64>,
    pub settlement_date: Option<serde_json::Value>,
    pub ticker: Option<String>,
    pub product_code: Option<String>,
}

/// Last-minute aggregate section of a futures snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesSnapshotMinute {
    pub close: Option<f64>,
    pub high: Option<f64>,
    pub last_updated: Option<i64>,
    pub low: Option<f64>,
    pub open: Option<f64>,
    pub timeframe: Option<String>,
    pub volume: Option<f64>,
}

/// Last-quote section of a futures snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesSnapshotQuote {
    pub ask: Option<f64>,
    pub ask_size: Option<i64>,
    pub ask_timestamp: Option<i64>,
    pub bid: Option<f64>,
    pub bid_size: Option<i64>,
    pub bid_timestamp: Option<i64>,
    pub last_updated: Option<i64>,
    pub timeframe: Option<String>,
}

/// Last-trade section of a futures snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesSnapshotTrade {
    pub last_updated: Option<i64>,
    pub price: Option<f64>,
    pub size: Option<i64>,
    pub timeframe: Option<String>,
}

/// Session section of a futures snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesSnapshotSession {
    pub change: Option<f64>,
    pub change_percent: Option<f64>,
    pub close: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub open: Option<f64>,
    pub previous_settlement: Option<f64>,
    pub settlement_price: Option<f64>,
    pub volume: Option<f64>,
}

/// A futures snapshot combining details, last minute/quote/trade, and session data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesSnapshot {
    pub ticker: Option<String>,
    pub product_code: Option<String>,
    pub details: Option<FuturesSnapshotDetails>,
    pub last_minute: Option<FuturesSnapshotMinute>,
    pub last_quote: Option<FuturesSnapshotQuote>,
    pub last_trade: Option<FuturesSnapshotTrade>,
    pub session: Option<FuturesSnapshotSession>,
}

/// Represents a futures exchange or trading venue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesExchange {
    pub acronym: Option<String>,
    pub id: Option<String>,
    pub locale: Option<String>,
    pub mic: Option<String>,
    pub name: Option<String>,
    pub operating_mic: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub url: Option<String>,
}
