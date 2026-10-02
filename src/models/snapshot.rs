use super::aggs::Agg;
use super::quotes::LastQuote;
use super::trades::LastTrade;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Most recent minute bar.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MinuteSnapshot {
    #[serde(rename = "av")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub accumulated_volume: Option<Decimal>,
    #[serde(rename = "o")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub open: Option<Decimal>,
    #[serde(rename = "h")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub high: Option<Decimal>,
    #[serde(rename = "l")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub low: Option<Decimal>,
    #[serde(rename = "c")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub close: Option<Decimal>,
    #[serde(rename = "v")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub volume: Option<Decimal>,
    #[serde(rename = "vw")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub vwap: Option<Decimal>,
    pub otc: Option<bool>,
    #[serde(rename = "t")]
    pub timestamp: Option<i64>,
    #[serde(rename = "n")]
    pub transactions: Option<i64>,
    #[serde(rename = "dv")]
    pub fractional_volume: Option<String>,
    #[serde(rename = "dav")]
    pub fractional_accumulated_volume: Option<String>,
}

/// Data for the most recent daily bar in an index snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct IndicesSession {
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub change: Option<Decimal>,
    #[serde(rename = "change_percent")]
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
    #[serde(rename = "previous_close")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub previous_close: Option<Decimal>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct IndicesSnapshot {
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub value: Option<Decimal>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub ticker: Option<String>,
    #[serde(rename = "market_status")]
    pub market_status: Option<String>,
    pub session: Option<IndicesSession>,
    pub error: Option<String>,
    pub message: Option<String>,
}

/// The most up-to-date market data for a traded ticker symbol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TickerSnapshot {
    pub day: Option<Agg>,
    #[serde(rename = "lastQuote")]
    pub last_quote: Option<LastQuote>,
    #[serde(rename = "lastTrade")]
    pub last_trade: Option<LastTrade>,
    #[serde(rename = "min")]
    pub min: Option<MinuteSnapshot>,
    #[serde(rename = "prevDay")]
    pub prev_day: Option<Agg>,
    pub ticker: Option<String>,
    #[serde(rename = "todaysChange")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub todays_change: Option<Decimal>,
    #[serde(rename = "todaysChangePerc")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub todays_change_percent: Option<Decimal>,
    pub updated: Option<i64>,
    #[serde(rename = "fmv")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub fair_market_value: Option<Decimal>,
}

/// Data for the most recent daily bar in an options contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DayOptionContractSnapshot {
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub change: Option<Decimal>,
    #[serde(rename = "change_percent")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub change_percent: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub close: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub high: Option<Decimal>,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<i64>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub low: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub open: Option<Decimal>,
    #[serde(rename = "previous_close")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub previous_close: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub volume: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub vwap: Option<Decimal>,
}

/// Details for an options contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OptionDetails {
    #[serde(rename = "contract_type")]
    pub contract_type: Option<String>,
    #[serde(rename = "exercise_style")]
    pub exercise_style: Option<String>,
    #[serde(rename = "expiration_date")]
    pub expiration_date: Option<String>,
    #[serde(rename = "shares_per_contract")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub shares_per_contract: Option<Decimal>,
    #[serde(rename = "strike_price")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub strike_price: Option<Decimal>,
    pub ticker: Option<String>,
}

/// Data for the most recent quote in an options contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LastQuoteOptionContractSnapshot {
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub ask: Option<Decimal>,
    #[serde(rename = "ask_size")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub ask_size: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub bid: Option<Decimal>,
    #[serde(rename = "bid_size")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub bid_size: Option<Decimal>,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<i64>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub midpoint: Option<Decimal>,
    pub timeframe: Option<String>,
}

/// Data for the most recent trade for an options contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LastTradeOptionContractSnapshot {
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub price: Option<Decimal>,
    #[serde(rename = "sip_timestamp")]
    pub sip_timestamp: Option<i64>,
    pub size: Option<i64>,
    pub conditions: Option<Vec<i64>>,
    pub exchange: Option<i64>,
    pub timeframe: Option<String>,
}

/// Greeks data for an options contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Greeks {
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub delta: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub gamma: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub theta: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub vega: Option<Decimal>,
}

/// Data for the underlying stock in an options contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UnderlyingAsset {
    #[serde(rename = "change_to_break_even")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub change_to_break_even: Option<Decimal>,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<i64>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub price: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub value: Option<Decimal>,
    pub ticker: Option<String>,
    pub timeframe: Option<String>,
}

/// Snapshot data of an option contract of a stock equity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OptionContractSnapshot {
    #[serde(rename = "break_even_price")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub break_even_price: Option<Decimal>,
    pub day: Option<DayOptionContractSnapshot>,
    pub details: Option<OptionDetails>,
    pub greeks: Option<Greeks>,
    #[serde(rename = "implied_volatility")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub implied_volatility: Option<Decimal>,
    #[serde(rename = "last_quote")]
    pub last_quote: Option<LastQuoteOptionContractSnapshot>,
    #[serde(rename = "last_trade")]
    pub last_trade: Option<LastTradeOptionContractSnapshot>,
    #[serde(rename = "open_interest")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub open_interest: Option<Decimal>,
    #[serde(rename = "underlying_asset")]
    pub underlying_asset: Option<UnderlyingAsset>,
    #[serde(rename = "fmv")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub fair_market_value: Option<Decimal>,
}

/// Data for a book bid or ask.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OrderBookQuote {
    #[serde(rename = "p")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub price: Option<Decimal>,
    #[serde(rename = "x")]
    #[serde(default, deserialize_with = "crate::de::decimal_map_opt")]
    pub exchange_shares: Option<std::collections::HashMap<String, Decimal>>,
}

/// Current level 2 book of a single ticker, combined from all exchanges.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotTickerFullBook {
    pub ticker: Option<String>,
    pub bids: Option<Vec<OrderBookQuote>>,
    pub asks: Option<Vec<OrderBookQuote>>,
    #[serde(rename = "bidCount")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub bid_count: Option<Decimal>,
    #[serde(rename = "askCount")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub ask_count: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub spread: Option<Decimal>,
    pub updated: Option<i64>,
}

/// Data about the most recent trading session for an asset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UniversalSnapshotSession {
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub price: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub change: Option<Decimal>,
    #[serde(rename = "change_percent")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub change_percent: Option<Decimal>,
    #[serde(rename = "early_trading_change")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub early_trading_change: Option<Decimal>,
    #[serde(rename = "early_trading_change_percent")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub early_trading_change_percent: Option<Decimal>,
    #[serde(rename = "regular_trading_change")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub regular_trading_change: Option<Decimal>,
    #[serde(rename = "regular_trading_change_percent")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub regular_trading_change_percent: Option<Decimal>,
    #[serde(rename = "late_trading_change")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub late_trading_change: Option<Decimal>,
    #[serde(rename = "late_trading_change_percent")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub late_trading_change_percent: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub open: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub close: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub high: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub low: Option<Decimal>,
    #[serde(rename = "previous_close")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub previous_close: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub volume: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub vwap: Option<Decimal>,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<i64>,
    #[serde(rename = "decimal_volume")]
    pub fractional_volume: Option<String>,
}

/// The most recent quote for an asset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UniversalSnapshotLastQuote {
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub ask: Option<Decimal>,
    #[serde(rename = "ask_size")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub ask_size: Option<Decimal>,
    #[serde(rename = "ask_exchange")]
    pub ask_exchange: Option<i64>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub bid: Option<Decimal>,
    #[serde(rename = "bid_size")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub bid_size: Option<Decimal>,
    #[serde(rename = "bid_exchange")]
    pub bid_exchange: Option<i64>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub midpoint: Option<Decimal>,
    pub exchange: Option<i64>,
    pub timeframe: Option<String>,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<i64>,
}

/// The most recent trade for an asset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UniversalSnapshotLastTrade {
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub price: Option<Decimal>,
    pub size: Option<i64>,
    pub exchange: Option<i64>,
    pub conditions: Option<Vec<i64>>,
    pub timeframe: Option<String>,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<i64>,
    #[serde(rename = "participant_timestamp")]
    pub participant_timestamp: Option<i64>,
    #[serde(rename = "sip_timestamp")]
    pub sip_timestamp: Option<i64>,
    #[serde(rename = "decimal_size")]
    pub fractional_size: Option<String>,
}

/// The most recent minute-level aggregate for the asset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UniversalSnapshotLastMinute {
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub open: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub close: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub high: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub low: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub volume: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub vwap: Option<Decimal>,
    pub transactions: Option<i64>,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<i64>,
    #[serde(rename = "decimal_volume")]
    pub fractional_volume: Option<String>,
}

/// Data for the underlying stock in an options contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UniversalSnapshotUnderlyingAsset {
    pub ticker: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub price: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub value: Option<Decimal>,
    #[serde(rename = "change_to_break_even")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub change_to_break_even: Option<Decimal>,
    pub timeframe: Option<String>,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<i64>,
}

/// Details for an options contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UniversalSnapshotDetails {
    #[serde(rename = "contract_type")]
    pub contract_type: Option<String>,
    #[serde(rename = "exercise_style")]
    pub exercise_style: Option<String>,
    #[serde(rename = "expiration_date")]
    pub expiration_date: Option<String>,
    #[serde(rename = "shares_per_contract")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub shares_per_contract: Option<Decimal>,
    #[serde(rename = "strike_price")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub strike_price: Option<Decimal>,
}

/// Snapshot data for an asset (stocks, options, indices, fx, crypto).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UniversalSnapshot {
    pub ticker: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub session: Option<UniversalSnapshotSession>,
    #[serde(rename = "last_quote")]
    pub last_quote: Option<UniversalSnapshotLastQuote>,
    #[serde(rename = "last_trade")]
    pub last_trade: Option<UniversalSnapshotLastTrade>,
    #[serde(rename = "last_minute")]
    pub last_minute: Option<UniversalSnapshotLastMinute>,
    pub greeks: Option<Greeks>,
    #[serde(rename = "underlying_asset")]
    pub underlying_asset: Option<UniversalSnapshotUnderlyingAsset>,
    pub details: Option<UniversalSnapshotDetails>,
    #[serde(rename = "break_even_price")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub break_even_price: Option<Decimal>,
    #[serde(rename = "implied_volatility")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub implied_volatility: Option<Decimal>,
    #[serde(rename = "open_interest")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub open_interest: Option<Decimal>,
    #[serde(rename = "market_status")]
    pub market_status: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "fmv")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub fair_market_value: Option<Decimal>,
    pub error: Option<String>,
    pub message: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub value: Option<Decimal>,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<i64>,
    pub timeframe: Option<String>,
}
