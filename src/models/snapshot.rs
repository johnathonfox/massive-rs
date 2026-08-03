use serde::{Deserialize, Serialize};
use super::aggs::Agg;
use super::quotes::LastQuote;
use super::trades::LastTrade;

/// Most recent minute bar.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MinuteSnapshot {
    #[serde(rename = "av")]
    pub accumulated_volume: Option<f64>,
    #[serde(rename = "o")]
    pub open: Option<f64>,
    #[serde(rename = "h")]
    pub high: Option<f64>,
    #[serde(rename = "l")]
    pub low: Option<f64>,
    #[serde(rename = "c")]
    pub close: Option<f64>,
    #[serde(rename = "v")]
    pub volume: Option<f64>,
    #[serde(rename = "vw")]
    pub vwap: Option<f64>,
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
    pub change: Option<f64>,
    #[serde(rename = "change_percent")]
    pub change_percent: Option<f64>,
    pub close: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub open: Option<f64>,
    #[serde(rename = "previous_close")]
    pub previous_close: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct IndicesSnapshot {
    pub value: Option<f64>,
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
    pub todays_change: Option<f64>,
    #[serde(rename = "todaysChangePerc")]
    pub todays_change_percent: Option<f64>,
    pub updated: Option<i64>,
    #[serde(rename = "fmv")]
    pub fair_market_value: Option<f64>,
}

/// Data for the most recent daily bar in an options contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DayOptionContractSnapshot {
    pub change: Option<f64>,
    #[serde(rename = "change_percent")]
    pub change_percent: Option<f64>,
    pub close: Option<f64>,
    pub high: Option<f64>,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<i64>,
    pub low: Option<f64>,
    pub open: Option<f64>,
    #[serde(rename = "previous_close")]
    pub previous_close: Option<f64>,
    pub volume: Option<f64>,
    pub vwap: Option<f64>,
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
    pub shares_per_contract: Option<f64>,
    #[serde(rename = "strike_price")]
    pub strike_price: Option<f64>,
    pub ticker: Option<String>,
}

/// Data for the most recent quote in an options contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LastQuoteOptionContractSnapshot {
    pub ask: Option<f64>,
    #[serde(rename = "ask_size")]
    pub ask_size: Option<f64>,
    pub bid: Option<f64>,
    #[serde(rename = "bid_size")]
    pub bid_size: Option<f64>,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<i64>,
    pub midpoint: Option<f64>,
    pub timeframe: Option<String>,
}

/// Data for the most recent trade for an options contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LastTradeOptionContractSnapshot {
    pub price: Option<f64>,
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
    pub delta: Option<f64>,
    pub gamma: Option<f64>,
    pub theta: Option<f64>,
    pub vega: Option<f64>,
}

/// Data for the underlying stock in an options contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UnderlyingAsset {
    #[serde(rename = "change_to_break_even")]
    pub change_to_break_even: Option<f64>,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<i64>,
    pub price: Option<f64>,
    pub value: Option<f64>,
    pub ticker: Option<String>,
    pub timeframe: Option<String>,
}

/// Snapshot data of an option contract of a stock equity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OptionContractSnapshot {
    #[serde(rename = "break_even_price")]
    pub break_even_price: Option<f64>,
    pub day: Option<DayOptionContractSnapshot>,
    pub details: Option<OptionDetails>,
    pub greeks: Option<Greeks>,
    #[serde(rename = "implied_volatility")]
    pub implied_volatility: Option<f64>,
    #[serde(rename = "last_quote")]
    pub last_quote: Option<LastQuoteOptionContractSnapshot>,
    #[serde(rename = "last_trade")]
    pub last_trade: Option<LastTradeOptionContractSnapshot>,
    #[serde(rename = "open_interest")]
    pub open_interest: Option<f64>,
    #[serde(rename = "underlying_asset")]
    pub underlying_asset: Option<UnderlyingAsset>,
    #[serde(rename = "fmv")]
    pub fair_market_value: Option<f64>,
}

/// Data for a book bid or ask.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OrderBookQuote {
    #[serde(rename = "p")]
    pub price: Option<f64>,
    #[serde(rename = "x")]
    pub exchange_shares: Option<std::collections::HashMap<String, f64>>,
}

/// Current level 2 book of a single ticker, combined from all exchanges.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotTickerFullBook {
    pub ticker: Option<String>,
    pub bids: Option<Vec<OrderBookQuote>>,
    pub asks: Option<Vec<OrderBookQuote>>,
    #[serde(rename = "bidCount")]
    pub bid_count: Option<f64>,
    #[serde(rename = "askCount")]
    pub ask_count: Option<f64>,
    pub spread: Option<f64>,
    pub updated: Option<i64>,
}

/// Data about the most recent trading session for an asset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UniversalSnapshotSession {
    pub price: Option<f64>,
    pub change: Option<f64>,
    #[serde(rename = "change_percent")]
    pub change_percent: Option<f64>,
    #[serde(rename = "early_trading_change")]
    pub early_trading_change: Option<f64>,
    #[serde(rename = "early_trading_change_percent")]
    pub early_trading_change_percent: Option<f64>,
    #[serde(rename = "regular_trading_change")]
    pub regular_trading_change: Option<f64>,
    #[serde(rename = "regular_trading_change_percent")]
    pub regular_trading_change_percent: Option<f64>,
    #[serde(rename = "late_trading_change")]
    pub late_trading_change: Option<f64>,
    #[serde(rename = "late_trading_change_percent")]
    pub late_trading_change_percent: Option<f64>,
    pub open: Option<f64>,
    pub close: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    #[serde(rename = "previous_close")]
    pub previous_close: Option<f64>,
    pub volume: Option<f64>,
    pub vwap: Option<f64>,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<i64>,
    #[serde(rename = "decimal_volume")]
    pub fractional_volume: Option<String>,
}

/// The most recent quote for an asset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UniversalSnapshotLastQuote {
    pub ask: Option<f64>,
    #[serde(rename = "ask_size")]
    pub ask_size: Option<f64>,
    #[serde(rename = "ask_exchange")]
    pub ask_exchange: Option<i64>,
    pub bid: Option<f64>,
    #[serde(rename = "bid_size")]
    pub bid_size: Option<f64>,
    #[serde(rename = "bid_exchange")]
    pub bid_exchange: Option<i64>,
    pub midpoint: Option<f64>,
    pub exchange: Option<i64>,
    pub timeframe: Option<String>,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<i64>,
}

/// The most recent trade for an asset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UniversalSnapshotLastTrade {
    pub id: Option<i64>,
    pub price: Option<f64>,
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
    pub open: Option<f64>,
    pub close: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub volume: Option<f64>,
    pub vwap: Option<f64>,
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
    pub price: Option<f64>,
    pub value: Option<f64>,
    #[serde(rename = "change_to_break_even")]
    pub change_to_break_even: Option<f64>,
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
    pub shares_per_contract: Option<f64>,
    #[serde(rename = "strike_price")]
    pub strike_price: Option<f64>,
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
    pub break_even_price: Option<f64>,
    #[serde(rename = "implied_volatility")]
    pub implied_volatility: Option<f64>,
    #[serde(rename = "open_interest")]
    pub open_interest: Option<f64>,
    #[serde(rename = "market_status")]
    pub market_status: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "fmv")]
    pub fair_market_value: Option<f64>,
    pub error: Option<String>,
    pub message: Option<String>,
    pub value: Option<f64>,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<i64>,
    pub timeframe: Option<String>,
}
