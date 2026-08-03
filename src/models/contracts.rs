use serde::{Deserialize, Serialize};

/// Underlying contains data for an underlying or deliverable associated with an option contract.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Underlying {
    pub amount: Option<f64>,
    #[serde(rename = "type")]
    pub underlying_type: Option<String>,
    pub underlying: Option<String>,
}

/// OptionsContract contains data for a specified ticker symbol.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OptionsContract {
    pub additional_underlyings: Option<Vec<Underlying>>,
    pub cfi: Option<String>,
    pub contract_type: Option<String>,
    pub correction: Option<String>,
    pub exercise_style: Option<String>,
    pub expiration_date: Option<String>,
    pub primary_exchange: Option<String>,
    pub shares_per_contract: Option<f64>,
    pub strike_price: Option<f64>,
    pub ticker: Option<String>,
    pub underlying_ticker: Option<String>,
}
