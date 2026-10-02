use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::aggs::Agg;

/// One datum for indicators with a single value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct IndicatorValue {
    pub timestamp: Option<i64>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub value: Option<Decimal>,
}

/// One datum for all MACD values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MacdIndicatorValue {
    pub timestamp: Option<i64>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub value: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub signal: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub histogram: Option<Decimal>,
}

/// URL to call to get the aggs used for building the indicator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct IndicatorUnderlying {
    pub url: Option<String>,
    pub aggregates: Option<Vec<Agg>>,
}

/// Indicator values and underlying data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SingleIndicatorResults {
    pub values: Option<Vec<IndicatorValue>>,
    pub underlying: Option<IndicatorUnderlying>,
}

/// Indicator values and underlying data for SMA.
pub type SmaIndicatorResults = SingleIndicatorResults;

/// Indicator values and underlying data for EMA.
pub type EmaIndicatorResults = SingleIndicatorResults;

/// Indicator values and underlying data for RSI.
pub type RsiIndicatorResults = SingleIndicatorResults;

/// MACD indicator values and underlying data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MacdIndicatorResults {
    pub values: Option<Vec<MacdIndicatorValue>>,
    pub underlying: Option<IndicatorUnderlying>,
}
