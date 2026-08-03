use serde::{Deserialize, Serialize};

/// Contains currency market status data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketCurrencies {
    pub crypto: Option<String>,
    pub fx: Option<String>,
}

/// Contains exchange market status data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketExchanges {
    pub nasdaq: Option<String>,
    pub nyse: Option<String>,
    pub otc: Option<String>,
}

/// Contains indices market status data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketIndices {
    pub s_and_p: Option<String>,
    pub societe_generale: Option<String>,
    pub cgi: Option<String>,
    pub msci: Option<String>,
    pub ftse_russell: Option<String>,
    pub mstar: Option<String>,
    pub mstarc: Option<String>,
    pub cccy: Option<String>,
    pub nasdaq: Option<String>,
    pub dow_jones: Option<String>,
}

/// MarketHoliday contains data for upcoming market holidays and their open/close times.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketHoliday {
    pub close: Option<String>,
    pub date: Option<String>,
    pub exchange: Option<String>,
    pub name: Option<String>,
    pub open: Option<String>,
    pub status: Option<String>,
}

/// MarketStatus contains data for the current trading status of the exchanges and overall financial markets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketStatus {
    #[serde(rename = "afterHours")]
    pub after_hours: Option<bool>,
    pub currencies: Option<MarketCurrencies>,
    #[serde(rename = "earlyHours")]
    pub early_hours: Option<bool>,
    pub exchanges: Option<MarketExchanges>,
    #[serde(rename = "indicesGroups")]
    pub indices_groups: Option<MarketIndices>,
    pub market: Option<String>,
    #[serde(rename = "serverTime")]
    pub server_time: Option<String>,
}
