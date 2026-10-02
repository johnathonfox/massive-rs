use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Quote data for a specified ticker symbol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Quote {
    pub ask_exchange: Option<i64>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub ask_price: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub ask_size: Option<Decimal>,
    pub bid_exchange: Option<i64>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub bid_price: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub bid_size: Option<Decimal>,
    pub conditions: Option<Vec<i64>>,
    pub indicators: Option<Vec<i64>>,
    pub participant_timestamp: Option<i64>,
    pub sequence_number: Option<i64>,
    pub sip_timestamp: Option<i64>,
    pub tape: Option<i64>,
    pub trf_timestamp: Option<i64>,
}

/// The most recent NBBO (Quote) tick for a given stock.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LastQuote {
    #[serde(rename = "T")]
    pub ticker: Option<String>,
    #[serde(rename = "f")]
    pub trf_timestamp: Option<i64>,
    #[serde(rename = "q")]
    pub sequence_number: Option<i64>,
    #[serde(rename = "t")]
    pub sip_timestamp: Option<i64>,
    #[serde(rename = "y")]
    pub participant_timestamp: Option<i64>,
    #[serde(rename = "P")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub ask_price: Option<Decimal>,
    #[serde(rename = "S")]
    pub ask_size: Option<i64>,
    #[serde(rename = "X")]
    pub ask_exchange: Option<i64>,
    #[serde(rename = "c")]
    pub conditions: Option<Vec<i64>>,
    #[serde(rename = "i")]
    pub indicators: Option<Vec<i64>>,
    #[serde(rename = "p")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub bid_price: Option<Decimal>,
    #[serde(rename = "s")]
    pub bid_size: Option<i64>,
    #[serde(rename = "x")]
    pub bid_exchange: Option<i64>,
    #[serde(rename = "z")]
    pub tape: Option<i64>,
}

/// Data for a forex quote.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ForexQuote {
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub ask: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub bid: Option<Decimal>,
    pub exchange: Option<i64>,
    pub timestamp: Option<i64>,
}

/// The last quote tick for a forex currency pair.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LastForexQuote {
    pub last: Option<ForexQuote>,
    pub symbol: Option<String>,
}

/// Currency conversion using the latest market conversion rates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RealTimeCurrencyConversion {
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub converted: Option<Decimal>,
    #[serde(rename = "from_")]
    pub from_: Option<String>,
    #[serde(rename = "initialAmount")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub initial_amount: Option<Decimal>,
    pub last: Option<ForexQuote>,
    pub to: Option<String>,
}
