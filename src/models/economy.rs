use serde::{Deserialize, Serialize};

/// Treasury yield data for a specific date.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TreasuryYield {
    pub date: Option<String>,
    pub yield_1_month: Option<f64>,
    pub yield_3_month: Option<f64>,
    pub yield_6_month: Option<f64>,
    pub yield_1_year: Option<f64>,
    pub yield_2_year: Option<f64>,
    pub yield_3_year: Option<f64>,
    pub yield_5_year: Option<f64>,
    pub yield_7_year: Option<f64>,
    pub yield_10_year: Option<f64>,
    pub yield_20_year: Option<f64>,
    pub yield_30_year: Option<f64>,
}

/// Fed inflation data (CPI/PCE) for a specific date.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FedInflation {
    pub cpi: Option<f64>,
    pub cpi_core: Option<f64>,
    pub cpi_year_over_year: Option<f64>,
    pub date: Option<String>,
    pub pce: Option<f64>,
    pub pce_core: Option<f64>,
    pub pce_spending: Option<f64>,
}

/// Fed inflation expectations for various horizons.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FedInflationExpectations {
    pub date: Option<String>,
    pub forward_years_5_to_10: Option<f64>,
    pub market_10_year: Option<f64>,
    pub market_5_year: Option<f64>,
    pub model_10_year: Option<f64>,
    pub model_1_year: Option<f64>,
    pub model_30_year: Option<f64>,
    pub model_5_year: Option<f64>,
}

/// Fed labor market indicators for a specific date.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FedLaborMarket {
    pub avg_hourly_earnings: Option<f64>,
    pub date: Option<String>,
    pub job_openings: Option<f64>,
    pub labor_force_participation_rate: Option<f64>,
    pub unemployment_rate: Option<f64>,
}

/// Aggregated consumer transactions from European credit card panels.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EUMerchantAggregate {
    pub channel: Option<String>,
    pub consumer_type: Option<String>,
    pub eight_day_rolling_category_accounts: Option<i64>,
    pub eight_day_rolling_total_accounts: Option<i64>,
    pub mcc_group: Option<String>,
    pub merchant_industry: Option<String>,
    pub merchant_ticker: Option<String>,
    pub name: Option<String>,
    pub parent_name: Option<String>,
    pub published_date: Option<String>,
    pub spend_in_distinct_account_key_count: Option<i64>,
    pub spend_in_spend: Option<f64>,
    pub spend_in_transaction_count: Option<i64>,
    pub spend_out_distinct_account_key_count: Option<i64>,
    pub spend_out_spend: Option<f64>,
    pub spend_out_transaction_count: Option<i64>,
    pub total_accounts: Option<i64>,
    pub total_spend: Option<f64>,
    pub total_transactions: Option<i64>,
    pub transaction_currency: Option<String>,
    pub transaction_date: Option<String>,
    pub twenty_eight_day_rolling_category_accounts: Option<i64>,
    pub twenty_eight_day_rolling_total_accounts: Option<i64>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub user_country: Option<String>,
}

/// Reference data mapping merchants to parent companies, tickers, sectors, and industries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EUMerchantHierarchy {
    pub active_from: Option<String>,
    pub active_to: Option<String>,
    pub category: Option<String>,
    pub grandparent_name: Option<String>,
    pub grandparent_ticker: Option<String>,
    pub great_grandparent_name: Option<String>,
    pub great_grandparent_ticker: Option<String>,
    pub industry: Option<String>,
    pub industry_group: Option<String>,
    pub listing_status: Option<String>,
    pub lookup_name: Option<String>,
    pub normalized_name: Option<String>,
    pub parent_name: Option<String>,
    pub parent_ticker: Option<String>,
    pub sector: Option<String>,
    pub sub_industry: Option<String>,
    pub ticker: Option<String>,
}
