use serde::{Deserialize, Serialize};

/// A TMX corporate event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TmxCorporateEvent {
    pub company_name: Option<String>,
    pub date: Option<String>,
    pub isin: Option<String>,
    pub name: Option<String>,
    pub status: Option<String>,
    pub ticker: Option<String>,
    pub tmx_company_id: Option<i64>,
    pub tmx_record_id: Option<String>,
    pub trading_venue: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub url: Option<String>,
}
