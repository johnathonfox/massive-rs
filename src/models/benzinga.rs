use serde::{Deserialize, Serialize};

/// A Benzinga analyst insight for a ticker.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BenzingaAnalystInsight {
    pub benzinga_firm_id: Option<String>,
    pub benzinga_id: Option<String>,
    pub benzinga_rating_id: Option<String>,
    pub company_name: Option<String>,
    pub date: Option<String>,
    pub firm: Option<String>,
    pub insight: Option<String>,
    pub last_updated: Option<String>,
    pub price_target: Option<f64>,
    pub rating: Option<String>,
    pub rating_action: Option<String>,
    pub ticker: Option<String>,
}

/// A Benzinga analyst with performance stats.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BenzingaAnalyst {
    pub benzinga_firm_id: Option<String>,
    pub benzinga_id: Option<String>,
    pub firm_name: Option<String>,
    pub full_name: Option<String>,
    pub last_updated: Option<String>,
    pub overall_avg_return: Option<f64>,
    pub overall_avg_return_percentile: Option<f64>,
    pub overall_success_rate: Option<f64>,
    pub smart_score: Option<f64>,
    pub total_ratings: Option<f64>,
    pub total_ratings_percentile: Option<f64>,
}

/// A Benzinga consensus rating for a ticker.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BenzingaConsensusRating {
    pub buy_ratings: Option<i64>,
    pub consensus_price_target: Option<f64>,
    pub consensus_rating: Option<String>,
    pub consensus_rating_value: Option<f64>,
    pub high_price_target: Option<f64>,
    pub hold_ratings: Option<i64>,
    pub low_price_target: Option<f64>,
    pub price_target_contributors: Option<i64>,
    pub ratings_contributors: Option<i64>,
    pub sell_ratings: Option<i64>,
    pub strong_buy_ratings: Option<i64>,
    pub strong_sell_ratings: Option<i64>,
    pub ticker: Option<String>,
}

/// A Benzinga earnings report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BenzingaEarning {
    pub actual_eps: Option<f64>,
    pub actual_revenue: Option<f64>,
    pub benzinga_id: Option<String>,
    pub company_name: Option<String>,
    pub currency: Option<String>,
    pub date: Option<String>,
    pub date_status: Option<String>,
    pub eps_method: Option<String>,
    pub eps_surprise: Option<f64>,
    pub eps_surprise_percent: Option<f64>,
    pub estimated_eps: Option<f64>,
    pub estimated_revenue: Option<f64>,
    pub fiscal_period: Option<String>,
    pub fiscal_year: Option<i64>,
    pub importance: Option<i64>,
    pub last_updated: Option<String>,
    pub notes: Option<String>,
    pub previous_eps: Option<f64>,
    pub previous_revenue: Option<f64>,
    pub revenue_method: Option<String>,
    pub revenue_surprise: Option<f64>,
    pub revenue_surprise_percent: Option<f64>,
    pub ticker: Option<String>,
    pub time: Option<String>,
}

/// A Benzinga analyst firm.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BenzingaFirm {
    pub benzinga_id: Option<String>,
    pub currency: Option<String>,
    pub last_updated: Option<String>,
    pub name: Option<String>,
}

/// A Benzinga earnings guidance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BenzingaGuidance {
    pub benzinga_id: Option<String>,
    pub company_name: Option<String>,
    pub currency: Option<String>,
    pub date: Option<String>,
    pub eps_method: Option<String>,
    pub estimated_eps_guidance: Option<f64>,
    pub estimated_revenue_guidance: Option<f64>,
    pub fiscal_period: Option<String>,
    pub fiscal_year: Option<i64>,
    pub importance: Option<i64>,
    pub last_updated: Option<String>,
    pub max_eps_guidance: Option<f64>,
    pub max_revenue_guidance: Option<f64>,
    pub min_eps_guidance: Option<f64>,
    pub min_revenue_guidance: Option<f64>,
    pub notes: Option<String>,
    pub positioning: Option<String>,
    pub previous_max_eps_guidance: Option<f64>,
    pub previous_max_revenue_guidance: Option<f64>,
    pub previous_min_eps_guidance: Option<f64>,
    pub previous_min_revenue_guidance: Option<f64>,
    pub release_type: Option<String>,
    pub revenue_method: Option<String>,
    pub ticker: Option<String>,
    pub time: Option<String>,
}

/// A Benzinga news article.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BenzingaNews {
    pub author: Option<String>,
    pub benzinga_id: Option<i64>,
    pub body: Option<String>,
    pub channels: Option<Vec<String>>,
    pub images: Option<Vec<String>>,
    pub last_updated: Option<String>,
    pub published: Option<String>,
    pub tags: Option<Vec<String>>,
    pub teaser: Option<String>,
    pub tickers: Option<Vec<String>>,
    pub title: Option<String>,
    pub url: Option<String>,
}

/// A Benzinga analyst rating change.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BenzingaRating {
    pub adjusted_price_target: Option<f64>,
    pub analyst: Option<String>,
    pub benzinga_analyst_id: Option<String>,
    pub benzinga_calendar_url: Option<String>,
    pub benzinga_firm_id: Option<String>,
    pub benzinga_id: Option<String>,
    pub benzinga_news_url: Option<String>,
    pub company_name: Option<String>,
    pub currency: Option<String>,
    pub date: Option<String>,
    pub firm: Option<String>,
    pub importance: Option<i64>,
    pub last_updated: Option<String>,
    pub notes: Option<String>,
    pub previous_adjusted_price_target: Option<f64>,
    pub previous_price_target: Option<f64>,
    pub previous_rating: Option<String>,
    pub price_percent_change: Option<f64>,
    pub price_target: Option<f64>,
    pub price_target_action: Option<String>,
    pub rating: Option<String>,
    pub rating_action: Option<String>,
    pub ticker: Option<String>,
    pub time: Option<String>,
}

/// A Benzinga "bulls and bears say" analysis for a ticker.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BenzingaBullsBearsSay {
    pub bear_case: Option<String>,
    pub benzinga_id: Option<String>,
    pub bull_case: Option<String>,
    pub last_updated: Option<String>,
    pub ticker: Option<String>,
}
