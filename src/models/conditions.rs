use serde::{Deserialize, Serialize};

/// Contains data for a mapping to a symbol for each SIP that has a given condition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SipMapping {
    #[serde(rename = "CTA")]
    pub cta: Option<String>,
    #[serde(rename = "OPRA")]
    pub opra: Option<String>,
    #[serde(rename = "UTP")]
    pub utp: Option<String>,
}

/// Contains data for aggregation rules on a consolidated (all exchanges) basis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Consolidated {
    pub updates_high_low: Option<bool>,
    pub updates_open_close: Option<bool>,
    pub updates_volume: Option<bool>,
}

/// Contains data for aggregation rules on a per-market-center basis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketCenter {
    pub updates_high_low: Option<bool>,
    pub updates_open_close: Option<bool>,
    pub updates_volume: Option<bool>,
}

/// Contains data for a list of aggregation rules.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateRules {
    pub consolidated: Option<Consolidated>,
    pub market_center: Option<MarketCenter>,
}

/// Contains data for a condition that Massive.com uses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Condition {
    pub abbreviation: Option<String>,
    pub asset_class: Option<String>,
    pub data_types: Option<Vec<String>>,
    pub description: Option<String>,
    pub exchange: Option<i64>,
    pub id: Option<i64>,
    pub legacy: Option<bool>,
    pub name: Option<String>,
    pub sip_mapping: Option<SipMapping>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub update_rules: Option<UpdateRules>,
}
