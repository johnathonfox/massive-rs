use serde::{Deserialize, Serialize};

/// Dividend contains data for a historical cash dividend, including the ticker symbol, declaration date, ex-dividend date, record date, pay date, frequency, and amount.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Dividend {
    pub id: Option<i64>,
    pub cash_amount: Option<f64>,
    pub currency: Option<String>,
    pub declaration_date: Option<String>,
    pub dividend_type: Option<String>,
    pub ex_dividend_date: Option<String>,
    pub frequency: Option<i64>,
    pub pay_date: Option<String>,
    pub record_date: Option<String>,
    pub ticker: Option<String>,
}

/// StockDividend contains data for a historical stock dividend (v3 reference API).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct StockDividend {
    pub cash_amount: Option<f64>,
    pub currency: Option<String>,
    pub declaration_date: Option<String>,
    pub distribution_type: Option<String>,
    pub ex_dividend_date: Option<String>,
    pub frequency: Option<i64>,
    pub historical_adjustment_factor: Option<f64>,
    pub id: Option<String>,
    pub pay_date: Option<String>,
    pub record_date: Option<String>,
    pub split_adjusted_cash_amount: Option<f64>,
    pub ticker: Option<String>,
}
