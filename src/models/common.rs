use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Sort {
    #[serde(rename = "asc")]
    #[default]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Order {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Locale {
    #[serde(rename = "us")]
    Us,
    #[serde(rename = "global")]
    Global,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Market {
    #[serde(rename = "stocks")]
    Stocks,
    #[serde(rename = "crypto")]
    Crypto,
    #[serde(rename = "fx")]
    Fx,
    #[serde(rename = "otc")]
    Otc,
    #[serde(rename = "indices")]
    Indices,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetClass {
    #[serde(rename = "stocks")]
    Stocks,
    #[serde(rename = "options")]
    Options,
    #[serde(rename = "crypto")]
    Crypto,
    #[serde(rename = "fx")]
    Fx,
    #[serde(rename = "indices")]
    Indices,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DividendType {
    #[serde(rename = "CD")]
    Cd,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "ST")]
    St,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Frequency {
    OneTime = 0,
    Annually = 1,
    Biannually = 2,
    Quarterly = 4,
    Monthly = 12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataType {
    #[serde(rename = "trade")]
    Trade,
    #[serde(rename = "bbo")]
    Bbo,
    #[serde(rename = "nbbo")]
    Nbbo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sip {
    #[serde(rename = "CTA")]
    Cta,
    #[serde(rename = "UTP")]
    Utp,
    #[serde(rename = "OPRA")]
    Opra,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExchangeType {
    #[serde(rename = "exchange")]
    Exchange,
    #[serde(rename = "TRF")]
    Trf,
    #[serde(rename = "SIP")]
    Sip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "gainers")]
    Gainers,
    #[serde(rename = "losers")]
    Losers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotMarketType {
    #[serde(rename = "stocks")]
    Stocks,
    #[serde(rename = "forex")]
    Forex,
    #[serde(rename = "crypto")]
    Crypto,
    #[serde(rename = "indices")]
    Indices,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Timeframe {
    #[serde(rename = "annual")]
    Annual,
    #[serde(rename = "quarterly")]
    Quarterly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Precision {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeriesType {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "close")]
    Close,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "low")]
    Low,
}

/// Massive Edge launchpad header options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LaunchPadOptions {
    #[serde(rename = "X-Massive-Edge-ID")]
    XMassiveEdgeId,
    #[serde(rename = "X-Massive-Edge-IP-Address")]
    XMassiveIpAddress,
    #[serde(rename = "X-Massive-Edge-User-Agent")]
    XMassiveEdgeUserAgent,
}
