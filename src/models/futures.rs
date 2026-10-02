use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// A single aggregate bar for a futures contract in a given time window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesAgg {
    pub ticker: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub open: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub high: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub low: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub close: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub volume: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub dollar_volume: Option<Decimal>,
    pub transactions: Option<i64>,
    pub window_start: Option<i64>,
    pub session_end_date: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub settlement_price: Option<Decimal>,
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
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub settlement_tick_size: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub spread_tick_size: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub trade_tick_size: Option<Decimal>,
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
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub unit_of_measure_qty: Option<Decimal>,
}

/// Represents a futures NBBO quote within a given time range.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesQuote {
    pub ticker: Option<String>,
    pub timestamp: Option<i64>,
    pub session_end_date: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub ask_price: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub ask_size: Option<Decimal>,
    pub ask_timestamp: Option<i64>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub bid_price: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub bid_size: Option<Decimal>,
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
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub price: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub size: Option<Decimal>,
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
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub close: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub high: Option<Decimal>,
    pub last_updated: Option<i64>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub low: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub open: Option<Decimal>,
    pub timeframe: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub volume: Option<Decimal>,
}

/// Last-quote section of a futures snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesSnapshotQuote {
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub ask: Option<Decimal>,
    pub ask_size: Option<i64>,
    pub ask_timestamp: Option<i64>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub bid: Option<Decimal>,
    pub bid_size: Option<i64>,
    pub bid_timestamp: Option<i64>,
    pub last_updated: Option<i64>,
    pub timeframe: Option<String>,
}

/// Last-trade section of a futures snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesSnapshotTrade {
    pub last_updated: Option<i64>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub price: Option<Decimal>,
    pub size: Option<i64>,
    pub timeframe: Option<String>,
}

/// Session section of a futures snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FuturesSnapshotSession {
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub change: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub change_percent: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub close: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub high: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub low: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub open: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub previous_settlement: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub settlement_price: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub volume: Option<Decimal>,
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
