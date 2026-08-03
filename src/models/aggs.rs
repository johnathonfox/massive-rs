use serde::{Deserialize, Serialize};

/// Aggregate data for a given ticker symbol over a date range in a custom time window size.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Agg {
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
    #[serde(rename = "t")]
    pub timestamp: Option<i64>,
    #[serde(rename = "n")]
    pub transactions: Option<i64>,
    #[serde(rename = "otc")]
    pub otc: Option<bool>,
}

/// Daily open, high, low, and close (OHLC) data for a given date.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupedDailyAgg {
    #[serde(rename = "T")]
    pub ticker: Option<String>,
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
    #[serde(rename = "t")]
    pub timestamp: Option<i64>,
    #[serde(rename = "n")]
    pub transactions: Option<i64>,
    #[serde(rename = "otc")]
    pub otc: Option<bool>,
}

/// Open, close and afterhours prices of a ticker symbol on a specified date.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DailyOpenCloseAgg {
    #[serde(rename = "afterHours")]
    pub after_hours: Option<f64>,
    pub close: Option<f64>,
    #[serde(rename = "from")]
    pub from_: Option<String>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub open: Option<f64>,
    #[serde(rename = "preMarket")]
    pub pre_market: Option<f64>,
    pub status: Option<String>,
    pub symbol: Option<String>,
    pub volume: Option<f64>,
    pub otc: Option<bool>,
}

/// Previous day's open, high, low, and close (OHLC) of the specified stock ticker.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PreviousCloseAgg {
    #[serde(rename = "T")]
    pub ticker: Option<String>,
    #[serde(rename = "c")]
    pub close: Option<f64>,
    #[serde(rename = "h")]
    pub high: Option<f64>,
    #[serde(rename = "l")]
    pub low: Option<f64>,
    #[serde(rename = "o")]
    pub open: Option<f64>,
    #[serde(rename = "t")]
    pub timestamp: Option<f64>,
    #[serde(rename = "v")]
    pub volume: Option<f64>,
    #[serde(rename = "vw")]
    pub vwap: Option<f64>,
}
