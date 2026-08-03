use serde::{Deserialize, Serialize};

use super::tickers::Branding;

/// Session data for the summaries endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Session {
    pub change: Option<f64>,
    pub change_percent: Option<f64>,
    pub early_trading_change: Option<f64>,
    pub early_trading_change_percent: Option<f64>,
    pub late_trading_change: Option<f64>,
    pub late_trading_change_percent: Option<f64>,
    pub close: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub open: Option<f64>,
    pub previous_close: Option<f64>,
    pub volume: Option<f64>,
}

/// Options data for the summaries endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Options {
    pub contract_type: Option<String>,
    pub exercise_style: Option<String>,
    pub expiration_date: Option<String>,
    pub shares_per_contract: Option<f64>,
    pub strike_price: Option<f64>,
    pub underlying_ticker: Option<f64>,
}

/// Summary result data for a list of tickers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SummaryResult {
    pub price: Option<f64>,
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
