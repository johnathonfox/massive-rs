use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::tickers::Branding;

/// Session data for the summaries endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Session {
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub change: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub change_percent: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub early_trading_change: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub early_trading_change_percent: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub late_trading_change: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub late_trading_change_percent: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub close: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub high: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub low: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub open: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub previous_close: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub volume: Option<Decimal>,
}

/// Options data for the summaries endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Options {
    pub contract_type: Option<String>,
    pub exercise_style: Option<String>,
    pub expiration_date: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub shares_per_contract: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub strike_price: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub underlying_ticker: Option<Decimal>,
}

/// Summary result data for a list of tickers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SummaryResult {
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub price: Option<Decimal>,
    pub name: Option<String>,
    pub ticker: Option<String>,
    pub branding: Option<Branding>,
    pub market_status: Option<String>,
    pub last_updated: Option<i64>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub session: Option<Session>,
    pub options: Option<Options>,
    pub error: Option<String>,
    pub message: Option<String>,
}
