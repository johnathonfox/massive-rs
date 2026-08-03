use serde::{Deserialize, Serialize};

/// Contains address data for a ticker detail.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CompanyAddress {
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "postal_code")]
    pub postal_code: Option<String>,
}

/// Contains branding data for a ticker detail.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Branding {
    #[serde(rename = "icon_url")]
    pub icon_url: Option<String>,
    #[serde(rename = "logo_url")]
    pub logo_url: Option<String>,
    #[serde(rename = "accent_color")]
    pub accent_color: Option<String>,
    #[serde(rename = "light_color")]
    pub light_color: Option<String>,
    #[serde(rename = "dark_color")]
    pub dark_color: Option<String>,
}

/// Contains the insights related to the article.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Insight {
    pub sentiment: Option<String>,
    #[serde(rename = "sentiment_reasoning")]
    pub sentiment_reasoning: Option<String>,
    pub ticker: Option<String>,
}

/// Contains publisher data for ticker news.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Publisher {
    #[serde(rename = "favicon_url")]
    pub favicon_url: Option<String>,
    #[serde(rename = "homepage_url")]
    pub homepage_url: Option<String>,
    #[serde(rename = "logo_url")]
    pub logo_url: Option<String>,
    pub name: Option<String>,
}

/// Data for a specified ticker symbol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Ticker {
    pub active: Option<bool>,
    pub cik: Option<String>,
    #[serde(rename = "composite_figi")]
    pub composite_figi: Option<String>,
    #[serde(rename = "currency_name")]
    pub currency_name: Option<String>,
    #[serde(rename = "currency_symbol")]
    pub currency_symbol: Option<String>,
    #[serde(rename = "base_currency_symbol")]
    pub base_currency_symbol: Option<String>,
    #[serde(rename = "base_currency_name")]
    pub base_currency_name: Option<String>,
    #[serde(rename = "delisted_utc")]
    pub delisted_utc: Option<String>,
    #[serde(rename = "last_updated_utc")]
    pub last_updated_utc: Option<String>,
    pub locale: Option<String>,
    pub market: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "primary_exchange")]
    pub primary_exchange: Option<String>,
    #[serde(rename = "share_class_figi")]
    pub share_class_figi: Option<String>,
    pub ticker: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "source_feed")]
    pub source_feed: Option<String>,
}

/// Detailed data for a specified ticker symbol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TickerDetails {
    pub active: Option<bool>,
    pub address: Option<CompanyAddress>,
    pub branding: Option<Branding>,
    pub cik: Option<String>,
    #[serde(rename = "composite_figi")]
    pub composite_figi: Option<String>,
    #[serde(rename = "currency_name")]
    pub currency_name: Option<String>,
    #[serde(rename = "currency_symbol")]
    pub currency_symbol: Option<String>,
    #[serde(rename = "base_currency_name")]
    pub base_currency_name: Option<String>,
    #[serde(rename = "base_currency_symbol")]
    pub base_currency_symbol: Option<String>,
    #[serde(rename = "delisted_utc")]
    pub delisted_utc: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "ticker_root")]
    pub ticker_root: Option<String>,
    #[serde(rename = "ticker_suffix")]
    pub ticker_suffix: Option<String>,
    #[serde(rename = "homepage_url")]
    pub homepage_url: Option<String>,
    #[serde(rename = "list_date")]
    pub list_date: Option<String>,
    pub locale: Option<String>,
    pub market: Option<String>,
    #[serde(rename = "market_cap")]
    pub market_cap: Option<f64>,
    pub name: Option<String>,
    #[serde(rename = "phone_number")]
    pub phone_number: Option<String>,
    #[serde(rename = "primary_exchange")]
    pub primary_exchange: Option<String>,
    #[serde(rename = "share_class_figi")]
    pub share_class_figi: Option<String>,
    #[serde(rename = "share_class_shares_outstanding")]
    pub share_class_shares_outstanding: Option<i64>,
    #[serde(rename = "sic_code")]
    pub sic_code: Option<String>,
    #[serde(rename = "sic_description")]
    pub sic_description: Option<String>,
    pub ticker: Option<String>,
    #[serde(rename = "total_employees")]
    pub total_employees: Option<i64>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "weighted_shares_outstanding")]
    pub weighted_shares_outstanding: Option<i64>,
}

/// News article data relating to a stock ticker symbol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TickerNews {
    #[serde(rename = "amp_url")]
    pub amp_url: Option<String>,
    #[serde(rename = "article_url")]
    pub article_url: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    pub insights: Option<Vec<Insight>>,
    pub keywords: Option<Vec<String>>,
    #[serde(rename = "published_utc")]
    pub published_utc: Option<String>,
    pub publisher: Option<Publisher>,
    pub tickers: Option<Vec<String>>,
    pub title: Option<String>,
}

/// Data for ticker types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TickerTypes {
    #[serde(rename = "asset_class")]
    pub asset_class: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub locale: Option<String>,
}

/// A ticker related to the queried ticker based on News and Returns data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RelatedCompany {
    pub ticker: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TickerChange {
    pub ticker: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TickerChangeEvent {
    #[serde(rename = "type")]
    pub type_: String,
    pub date: String,
    #[serde(rename = "ticker_change")]
    pub ticker_change: TickerChange,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TickerChangeResults {
    pub name: String,
    #[serde(rename = "composite_figi")]
    pub composite_figi: String,
    pub cik: String,
    pub events: Option<Vec<TickerChangeEvent>>,
}

/// IPO Listing data as returned by the /vX/reference/ipos endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct IPOListing {
    #[serde(rename = "announced_date")]
    pub announced_date: Option<String>,
    #[serde(rename = "currency_code")]
    pub currency_code: Option<String>,
    #[serde(rename = "final_issue_price")]
    pub final_issue_price: Option<f64>,
    #[serde(rename = "highest_offer_price")]
    pub highest_offer_price: Option<f64>,
    #[serde(rename = "ipo_status")]
    pub ipo_status: Option<String>,
    pub isin: Option<String>,
    #[serde(rename = "issuer_name")]
    pub issuer_name: Option<String>,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<String>,
    #[serde(rename = "listing_date")]
    pub listing_date: Option<String>,
    #[serde(rename = "lot_size")]
    pub lot_size: Option<i64>,
    #[serde(rename = "lowest_offer_price")]
    pub lowest_offer_price: Option<f64>,
    #[serde(rename = "max_shares_offered")]
    pub max_shares_offered: Option<i64>,
    #[serde(rename = "min_shares_offered")]
    pub min_shares_offered: Option<i64>,
    #[serde(rename = "primary_exchange")]
    pub primary_exchange: Option<String>,
    #[serde(rename = "security_description")]
    pub security_description: Option<String>,
    #[serde(rename = "security_type")]
    pub security_type: Option<String>,
    #[serde(rename = "shares_outstanding")]
    pub shares_outstanding: Option<i64>,
    pub ticker: Option<String>,
    #[serde(rename = "total_offer_size")]
    pub total_offer_size: Option<f64>,
    #[serde(rename = "us_code")]
    pub us_code: Option<String>,
}

/// Short Interest data for a specific identifier.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ShortInterest {
    #[serde(rename = "avg_daily_volume")]
    pub avg_daily_volume: Option<i64>,
    #[serde(rename = "days_to_cover")]
    pub days_to_cover: Option<f64>,
    #[serde(rename = "settlement_date")]
    pub settlement_date: Option<String>,
    #[serde(rename = "short_interest")]
    pub short_interest: Option<i64>,
    pub ticker: Option<String>,
}

/// Short Volume data for a specific identifier on a given date.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ShortVolume {
    #[serde(rename = "adf_short_volume")]
    pub adf_short_volume: Option<i64>,
    #[serde(rename = "adf_short_volume_exempt")]
    pub adf_short_volume_exempt: Option<i64>,
    pub date: Option<String>,
    #[serde(rename = "exempt_volume")]
    pub exempt_volume: Option<i64>,
    #[serde(rename = "nasdaq_carteret_short_volume")]
    pub nasdaq_carteret_short_volume: Option<i64>,
    #[serde(rename = "nasdaq_carteret_short_volume_exempt")]
    pub nasdaq_carteret_short_volume_exempt: Option<i64>,
    #[serde(rename = "nasdaq_chicago_short_volume")]
    pub nasdaq_chicago_short_volume: Option<i64>,
    #[serde(rename = "nasdaq_chicago_short_volume_exempt")]
    pub nasdaq_chicago_short_volume_exempt: Option<i64>,
    #[serde(rename = "non_exempt_volume")]
    pub non_exempt_volume: Option<i64>,
    #[serde(rename = "nyse_short_volume")]
    pub nyse_short_volume: Option<i64>,
    #[serde(rename = "nyse_short_volume_exempt")]
    pub nyse_short_volume_exempt: Option<i64>,
    #[serde(rename = "short_volume")]
    pub short_volume: Option<i64>,
    #[serde(rename = "short_volume_ratio")]
    pub short_volume_ratio: Option<f64>,
    pub ticker: Option<String>,
    #[serde(rename = "total_volume")]
    pub total_volume: Option<i64>,
}
