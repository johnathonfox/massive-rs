use serde::{Deserialize, Serialize};

/// Exchange contains data for a condition that Massive.com uses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Exchange {
    pub acronym: Option<String>,
    pub asset_class: Option<String>,
    pub id: Option<i64>,
    pub locale: Option<String>,
    pub mic: Option<String>,
    pub name: Option<String>,
    pub operating_mic: Option<String>,
    pub participant_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub url: Option<String>,
}
