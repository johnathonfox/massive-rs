use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Aggregate data for a given ticker symbol over a date range in a custom time window size.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Agg {
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
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub after_hours: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub close: Option<Decimal>,
    #[serde(rename = "from")]
    pub from_: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub high: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub low: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub open: Option<Decimal>,
    #[serde(rename = "preMarket")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub pre_market: Option<Decimal>,
    pub status: Option<String>,
    pub symbol: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub volume: Option<Decimal>,
    pub otc: Option<bool>,
}

/// Previous day's open, high, low, and close (OHLC) of the specified stock ticker.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PreviousCloseAgg {
    #[serde(rename = "T")]
    pub ticker: Option<String>,
    #[serde(rename = "c")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub close: Option<Decimal>,
    #[serde(rename = "h")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub high: Option<Decimal>,
    #[serde(rename = "l")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub low: Option<Decimal>,
    #[serde(rename = "o")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub open: Option<Decimal>,
    #[serde(rename = "t")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub timestamp: Option<Decimal>,
    #[serde(rename = "v")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub volume: Option<Decimal>,
    #[serde(rename = "vw")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub vwap: Option<Decimal>,
}
