use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Split contains data for a historical stock split, including the ticker symbol, the execution date, and the factors of the split ratio.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Split {
    pub id: Option<i64>,
    pub execution_date: Option<String>,
    pub split_from: Option<i64>,
    pub split_to: Option<i64>,
    pub ticker: Option<String>,
}

/// StockSplit contains data for a historical stock split (v3 reference API).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct StockSplit {
    pub adjustment_type: Option<String>,
    pub execution_date: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub historical_adjustment_factor: Option<Decimal>,
    pub id: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub split_from: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub split_to: Option<Decimal>,
    pub ticker: Option<String>,
}
