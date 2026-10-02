use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Quant analytics, reward and risk scores for an ETF composite ticker.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EtfGlobalAnalytics {
    pub composite_ticker: Option<String>,
    pub effective_date: Option<String>,
    pub processed_date: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_composite_behavioral: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_composite_fundamental: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_composite_global: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_composite_quality: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_composite_sentiment: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_composite_technical: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_fundamental_div: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_fundamental_pb: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_fundamental_pcf: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_fundamental_pe: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_global_country: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_global_sector: Option<Decimal>,
    pub quant_grade: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_quality_diversification: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_quality_firm: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_quality_liquidity: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_sentiment_iv: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_sentiment_pc: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_sentiment_si: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_technical_it: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_technical_lt: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_technical_st: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quant_total_score: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub reward_score: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub risk_country: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub risk_deviation: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub risk_efficiency: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub risk_liquidity: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub risk_structure: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub risk_total_score: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub risk_volatility: Option<Decimal>,
}

/// A single holding (constituent) of an ETF composite.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EtfGlobalConstituent {
    pub asset_class: Option<String>,
    pub composite_ticker: Option<String>,
    pub constituent_name: Option<String>,
    pub constituent_ticker: Option<String>,
    pub country_of_exchange: Option<String>,
    pub currency_traded: Option<String>,
    pub effective_date: Option<String>,
    pub exchange: Option<String>,
    pub figi: Option<String>,
    pub isin: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub market_value: Option<Decimal>,
    pub processed_date: Option<String>,
    pub security_type: Option<String>,
    pub sedol: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub shares_held: Option<Decimal>,
    pub us_code: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub weight: Option<Decimal>,
}

/// Daily fund flow, NAV and shares outstanding for an ETF composite.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EtfGlobalFundFlow {
    pub composite_ticker: Option<String>,
    pub effective_date: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub fund_flow: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub nav: Option<Decimal>,
    pub processed_date: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub shares_outstanding: Option<Decimal>,
}

/// Profile, fee and exposure data for an ETF composite.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EtfGlobalProfile {
    pub administrator: Option<String>,
    pub advisor: Option<String>,
    pub asset_class: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub aum: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub avg_daily_trading_volume: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub bid_ask_spread: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub call_volume: Option<Decimal>,
    pub category: Option<String>,
    pub composite_ticker: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_map_opt")]
    pub coupon_exposure: Option<std::collections::HashMap<String, Decimal>>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub creation_fee: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub creation_unit_size: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_map_opt")]
    pub currency_exposure: Option<std::collections::HashMap<String, Decimal>>,
    pub custodian: Option<String>,
    pub description: Option<String>,
    pub development_class: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub discount_premium: Option<Decimal>,
    pub distribution_frequency: Option<String>,
    pub distributor: Option<String>,
    pub effective_date: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub fee_waivers: Option<Decimal>,
    pub fiscal_year_end: Option<String>,
    pub focus: Option<String>,
    pub futures_commission_merchant: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_map_opt")]
    pub geographic_exposure: Option<std::collections::HashMap<String, Decimal>>,
    pub inception_date: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_map_opt")]
    pub industry_exposure: Option<std::collections::HashMap<String, Decimal>>,
    #[serde(default, deserialize_with = "crate::de::decimal_map_opt")]
    pub industry_group_exposure: Option<std::collections::HashMap<String, Decimal>>,
    pub issuer: Option<String>,
    pub lead_market_maker: Option<String>,
    pub leverage_style: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub levered_amount: Option<Decimal>,
    pub listing_exchange: Option<String>,
    pub management_classification: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub management_fee: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_map_opt")]
    pub maturity_exposure: Option<std::collections::HashMap<String, Decimal>>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub net_expenses: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub num_holdings: Option<Decimal>,
    pub options_available: Option<i64>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub options_volume: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub other_expenses: Option<Decimal>,
    pub portfolio_manager: Option<String>,
    pub primary_benchmark: Option<String>,
    pub processed_date: Option<String>,
    pub product_type: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub put_call_ratio: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub put_volume: Option<Decimal>,
    pub region: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_map_opt")]
    pub sector_exposure: Option<std::collections::HashMap<String, Decimal>>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub short_interest: Option<Decimal>,
    pub subadvisor: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_map_opt")]
    pub subindustry_exposure: Option<std::collections::HashMap<String, Decimal>>,
    pub tax_classification: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub total_expenses: Option<Decimal>,
    pub transfer_agent: Option<String>,
    pub trustee: Option<String>,
}

/// Classification and strategy taxonomy for an ETF composite.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EtfGlobalTaxonomy {
    pub asset_class: Option<String>,
    pub category: Option<String>,
    pub composite_ticker: Option<String>,
    pub country: Option<String>,
    pub credit_quality_rating: Option<String>,
    pub description: Option<String>,
    pub development_class: Option<String>,
    pub duration: Option<String>,
    pub effective_date: Option<String>,
    pub esg: Option<String>,
    pub exposure_mechanism: Option<String>,
    pub factor: Option<String>,
    pub focus: Option<String>,
    pub hedge_reset: Option<String>,
    pub holdings_disclosure_frequency: Option<String>,
    pub inception_date: Option<String>,
    pub isin: Option<String>,
    pub issuer: Option<String>,
    pub leverage_reset: Option<String>,
    pub leverage_style: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub levered_amount: Option<Decimal>,
    pub management_classification: Option<String>,
    pub management_style: Option<String>,
    pub maturity: Option<String>,
    pub objective: Option<String>,
    pub primary_benchmark: Option<String>,
    pub processed_date: Option<String>,
    pub product_type: Option<String>,
    pub rebalance_frequency: Option<String>,
    pub reconstitution_frequency: Option<String>,
    pub region: Option<String>,
    pub secondary_objective: Option<String>,
    pub selection_methodology: Option<String>,
    pub selection_universe: Option<String>,
    pub strategic_focus: Option<String>,
    pub targeted_focus: Option<String>,
    pub tax_classification: Option<String>,
    pub us_code: Option<String>,
    pub weighting_methodology: Option<String>,
}
