use serde::{Deserialize, Serialize};

/// Trade data for a specified ticker symbol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Trade {
    pub conditions: Option<Vec<i64>>,
    pub correction: Option<i64>,
    pub exchange: Option<i64>,
    pub id: Option<String>,
    pub participant_timestamp: Option<i64>,
    pub price: Option<f64>,
    pub sequence_number: Option<i64>,
    pub sip_timestamp: Option<i64>,
    pub size: Option<f64>,
    pub tape: Option<i64>,
    pub trf_id: Option<i64>,
    pub trf_timestamp: Option<i64>,
    pub decimal_size: Option<String>,
}

/// The most recent trade for a given ticker symbol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LastTrade {
    #[serde(rename = "T")]
    pub ticker: Option<String>,
    #[serde(rename = "f")]
    pub trf_timestamp: Option<i64>,
    #[serde(rename = "q")]
    pub sequence_number: Option<f64>,
    #[serde(rename = "t")]
    pub sip_timestamp: Option<i64>,
    #[serde(rename = "y")]
    pub participant_timestamp: Option<i64>,
    #[serde(rename = "c")]
    pub conditions: Option<Vec<i64>>,
    #[serde(rename = "e")]
    pub correction: Option<i64>,
    #[serde(rename = "i")]
    pub id: Option<String>,
    #[serde(rename = "p")]
    pub price: Option<f64>,
    #[serde(rename = "r")]
    pub trf_id: Option<i64>,
    #[serde(rename = "s")]
    pub size: Option<f64>,
    #[serde(rename = "x")]
    pub exchange: Option<i64>,
    #[serde(rename = "z")]
    pub tape: Option<i64>,
    #[serde(rename = "ds")]
    pub fractional_size: Option<String>,
}

/// A crypto trade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CryptoTrade {
    pub conditions: Option<Vec<i64>>,
    pub exchange: Option<i64>,
    pub price: Option<f64>,
    pub size: Option<f64>,
    pub timestamp: Option<i64>,
}
