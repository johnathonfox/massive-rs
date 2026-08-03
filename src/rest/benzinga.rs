use crate::client::{Client, RequestOptions};
use crate::error::Result;
use crate::models::{
    BenzingaAnalyst, BenzingaAnalystInsight, BenzingaBullsBearsSay, BenzingaConsensusRating,
    BenzingaEarning, BenzingaFirm, BenzingaGuidance, BenzingaNews, BenzingaRating,
};
use futures::Stream;

/// Benzinga API.
pub trait BenzingaApi {
    /// List Benzinga analyst insights.
    #[allow(clippy::too_many_arguments)]
    fn list_benzinga_analyst_insights(
        &self,
        date: Option<&str>,
        date_any_of: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        last_updated: Option<&str>,
        last_updated_any_of: Option<&str>,
        last_updated_gt: Option<&str>,
        last_updated_gte: Option<&str>,
        last_updated_lt: Option<&str>,
        last_updated_lte: Option<&str>,
        firm: Option<&str>,
        firm_any_of: Option<&str>,
        firm_gt: Option<&str>,
        firm_gte: Option<&str>,
        firm_lt: Option<&str>,
        firm_lte: Option<&str>,
        rating_action: Option<&str>,
        rating_action_any_of: Option<&str>,
        rating_action_gt: Option<&str>,
        rating_action_gte: Option<&str>,
        rating_action_lt: Option<&str>,
        rating_action_lte: Option<&str>,
        benzinga_firm_id: Option<&str>,
        benzinga_firm_id_any_of: Option<&str>,
        benzinga_firm_id_gt: Option<&str>,
        benzinga_firm_id_gte: Option<&str>,
        benzinga_firm_id_lt: Option<&str>,
        benzinga_firm_id_lte: Option<&str>,
        benzinga_rating_id: Option<&str>,
        benzinga_rating_id_any_of: Option<&str>,
        benzinga_rating_id_gt: Option<&str>,
        benzinga_rating_id_gte: Option<&str>,
        benzinga_rating_id_lt: Option<&str>,
        benzinga_rating_id_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaAnalystInsight>>;

    /// List Benzinga analysts.
    #[allow(clippy::too_many_arguments)]
    fn list_benzinga_analysts(
        &self,
        benzinga_id: Option<&str>,
        benzinga_id_any_of: Option<&str>,
        benzinga_id_gt: Option<&str>,
        benzinga_id_gte: Option<&str>,
        benzinga_id_lt: Option<&str>,
        benzinga_id_lte: Option<&str>,
        benzinga_firm_id: Option<&str>,
        benzinga_firm_id_any_of: Option<&str>,
        benzinga_firm_id_gt: Option<&str>,
        benzinga_firm_id_gte: Option<&str>,
        benzinga_firm_id_lt: Option<&str>,
        benzinga_firm_id_lte: Option<&str>,
        firm_name: Option<&str>,
        firm_name_any_of: Option<&str>,
        firm_name_gt: Option<&str>,
        firm_name_gte: Option<&str>,
        firm_name_lt: Option<&str>,
        firm_name_lte: Option<&str>,
        full_name: Option<&str>,
        full_name_any_of: Option<&str>,
        full_name_gt: Option<&str>,
        full_name_gte: Option<&str>,
        full_name_lt: Option<&str>,
        full_name_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaAnalyst>>;

    /// List Benzinga consensus ratings for a ticker.
    fn list_benzinga_consensus_ratings(
        &self,
        ticker: &str,
        date: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        limit: Option<i64>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaConsensusRating>>;

    /// List Benzinga earnings.
    #[allow(clippy::too_many_arguments)]
    fn list_benzinga_earnings(
        &self,
        date: Option<&str>,
        date_any_of: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        importance: Option<i64>,
        importance_any_of: Option<&str>,
        importance_gt: Option<i64>,
        importance_gte: Option<i64>,
        importance_lt: Option<i64>,
        importance_lte: Option<i64>,
        last_updated: Option<&str>,
        last_updated_any_of: Option<&str>,
        last_updated_gt: Option<&str>,
        last_updated_gte: Option<&str>,
        last_updated_lt: Option<&str>,
        last_updated_lte: Option<&str>,
        date_status: Option<&str>,
        date_status_any_of: Option<&str>,
        date_status_gt: Option<&str>,
        date_status_gte: Option<&str>,
        date_status_lt: Option<&str>,
        date_status_lte: Option<&str>,
        eps_surprise_percent: Option<f64>,
        eps_surprise_percent_any_of: Option<&str>,
        eps_surprise_percent_gt: Option<f64>,
        eps_surprise_percent_gte: Option<f64>,
        eps_surprise_percent_lt: Option<f64>,
        eps_surprise_percent_lte: Option<f64>,
        revenue_surprise_percent: Option<f64>,
        revenue_surprise_percent_any_of: Option<&str>,
        revenue_surprise_percent_gt: Option<f64>,
        revenue_surprise_percent_gte: Option<f64>,
        revenue_surprise_percent_lt: Option<f64>,
        revenue_surprise_percent_lte: Option<f64>,
        fiscal_year: Option<i64>,
        fiscal_year_any_of: Option<&str>,
        fiscal_year_gt: Option<i64>,
        fiscal_year_gte: Option<i64>,
        fiscal_year_lt: Option<i64>,
        fiscal_year_lte: Option<i64>,
        fiscal_period: Option<&str>,
        fiscal_period_any_of: Option<&str>,
        fiscal_period_gt: Option<&str>,
        fiscal_period_gte: Option<&str>,
        fiscal_period_lt: Option<&str>,
        fiscal_period_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaEarning>>;

    /// List Benzinga firms.
    fn list_benzinga_firms(
        &self,
        benzinga_id: Option<&str>,
        benzinga_id_any_of: Option<&str>,
        benzinga_id_gt: Option<&str>,
        benzinga_id_gte: Option<&str>,
        benzinga_id_lt: Option<&str>,
        benzinga_id_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaFirm>>;

    /// List Benzinga guidance.
    #[allow(clippy::too_many_arguments)]
    fn list_benzinga_guidance(
        &self,
        date: Option<&str>,
        date_any_of: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        positioning: Option<&str>,
        positioning_any_of: Option<&str>,
        positioning_gt: Option<&str>,
        positioning_gte: Option<&str>,
        positioning_lt: Option<&str>,
        positioning_lte: Option<&str>,
        importance: Option<i64>,
        importance_any_of: Option<&str>,
        importance_gt: Option<i64>,
        importance_gte: Option<i64>,
        importance_lt: Option<i64>,
        importance_lte: Option<i64>,
        last_updated: Option<&str>,
        last_updated_any_of: Option<&str>,
        last_updated_gt: Option<&str>,
        last_updated_gte: Option<&str>,
        last_updated_lt: Option<&str>,
        last_updated_lte: Option<&str>,
        fiscal_year: Option<i64>,
        fiscal_year_any_of: Option<&str>,
        fiscal_year_gt: Option<i64>,
        fiscal_year_gte: Option<i64>,
        fiscal_year_lt: Option<i64>,
        fiscal_year_lte: Option<i64>,
        fiscal_period: Option<&str>,
        fiscal_period_any_of: Option<&str>,
        fiscal_period_gt: Option<&str>,
        fiscal_period_gte: Option<&str>,
        fiscal_period_lt: Option<&str>,
        fiscal_period_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaGuidance>>;

    /// List Benzinga news (v1).
    #[allow(clippy::too_many_arguments)]
    fn list_benzinga_news(
        &self,
        published: Option<&str>,
        published_any_of: Option<&str>,
        published_gt: Option<&str>,
        published_gte: Option<&str>,
        published_lt: Option<&str>,
        published_lte: Option<&str>,
        last_updated: Option<&str>,
        last_updated_any_of: Option<&str>,
        last_updated_gt: Option<&str>,
        last_updated_gte: Option<&str>,
        last_updated_lt: Option<&str>,
        last_updated_lte: Option<&str>,
        tickers: Option<&str>,
        tickers_all_of: Option<&str>,
        tickers_any_of: Option<&str>,
        channels: Option<&str>,
        channels_all_of: Option<&str>,
        channels_any_of: Option<&str>,
        tags: Option<&str>,
        tags_all_of: Option<&str>,
        tags_any_of: Option<&str>,
        author: Option<&str>,
        author_any_of: Option<&str>,
        author_gt: Option<&str>,
        author_gte: Option<&str>,
        author_lt: Option<&str>,
        author_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaNews>>;

    /// List Benzinga news (v2).
    #[allow(clippy::too_many_arguments)]
    fn list_benzinga_news_v2(
        &self,
        published: Option<&str>,
        published_gt: Option<&str>,
        published_gte: Option<&str>,
        published_lt: Option<&str>,
        published_lte: Option<&str>,
        channels: Option<&str>,
        channels_all_of: Option<&str>,
        channels_any_of: Option<&str>,
        tags: Option<&str>,
        tags_all_of: Option<&str>,
        tags_any_of: Option<&str>,
        author: Option<&str>,
        author_any_of: Option<&str>,
        author_gt: Option<&str>,
        author_gte: Option<&str>,
        author_lt: Option<&str>,
        author_lte: Option<&str>,
        stocks: Option<&str>,
        stocks_all_of: Option<&str>,
        stocks_any_of: Option<&str>,
        tickers: Option<&str>,
        tickers_all_of: Option<&str>,
        tickers_any_of: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaNews>>;

    /// List Benzinga ratings.
    #[allow(clippy::too_many_arguments)]
    fn list_benzinga_ratings(
        &self,
        date: Option<&str>,
        date_any_of: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        importance: Option<i64>,
        importance_any_of: Option<&str>,
        importance_gt: Option<i64>,
        importance_gte: Option<i64>,
        importance_lt: Option<i64>,
        importance_lte: Option<i64>,
        last_updated: Option<&str>,
        last_updated_any_of: Option<&str>,
        last_updated_gt: Option<&str>,
        last_updated_gte: Option<&str>,
        last_updated_lt: Option<&str>,
        last_updated_lte: Option<&str>,
        rating_action: Option<&str>,
        rating_action_any_of: Option<&str>,
        rating_action_gt: Option<&str>,
        rating_action_gte: Option<&str>,
        rating_action_lt: Option<&str>,
        rating_action_lte: Option<&str>,
        price_target_action: Option<&str>,
        price_target_action_any_of: Option<&str>,
        price_target_action_gt: Option<&str>,
        price_target_action_gte: Option<&str>,
        price_target_action_lt: Option<&str>,
        price_target_action_lte: Option<&str>,
        benzinga_id: Option<&str>,
        benzinga_id_any_of: Option<&str>,
        benzinga_id_gt: Option<&str>,
        benzinga_id_gte: Option<&str>,
        benzinga_id_lt: Option<&str>,
        benzinga_id_lte: Option<&str>,
        benzinga_analyst_id: Option<&str>,
        benzinga_analyst_id_any_of: Option<&str>,
        benzinga_analyst_id_gt: Option<&str>,
        benzinga_analyst_id_gte: Option<&str>,
        benzinga_analyst_id_lt: Option<&str>,
        benzinga_analyst_id_lte: Option<&str>,
        benzinga_firm_id: Option<&str>,
        benzinga_firm_id_any_of: Option<&str>,
        benzinga_firm_id_gt: Option<&str>,
        benzinga_firm_id_gte: Option<&str>,
        benzinga_firm_id_lt: Option<&str>,
        benzinga_firm_id_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaRating>>;

    /// List Benzinga bulls/bears case summaries.
    #[allow(clippy::too_many_arguments)]
    fn list_benzinga_bulls_bears_say(
        &self,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        benzinga_id: Option<&str>,
        benzinga_id_any_of: Option<&str>,
        benzinga_id_gt: Option<&str>,
        benzinga_id_gte: Option<&str>,
        benzinga_id_lt: Option<&str>,
        benzinga_id_lte: Option<&str>,
        last_updated: Option<&str>,
        last_updated_gt: Option<&str>,
        last_updated_gte: Option<&str>,
        last_updated_lt: Option<&str>,
        last_updated_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaBullsBearsSay>>;
}

impl BenzingaApi for Client {
    fn list_benzinga_analyst_insights(
        &self,
        date: Option<&str>,
        date_any_of: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        last_updated: Option<&str>,
        last_updated_any_of: Option<&str>,
        last_updated_gt: Option<&str>,
        last_updated_gte: Option<&str>,
        last_updated_lt: Option<&str>,
        last_updated_lte: Option<&str>,
        firm: Option<&str>,
        firm_any_of: Option<&str>,
        firm_gt: Option<&str>,
        firm_gte: Option<&str>,
        firm_lt: Option<&str>,
        firm_lte: Option<&str>,
        rating_action: Option<&str>,
        rating_action_any_of: Option<&str>,
        rating_action_gt: Option<&str>,
        rating_action_gte: Option<&str>,
        rating_action_lt: Option<&str>,
        rating_action_lte: Option<&str>,
        benzinga_firm_id: Option<&str>,
        benzinga_firm_id_any_of: Option<&str>,
        benzinga_firm_id_gt: Option<&str>,
        benzinga_firm_id_gte: Option<&str>,
        benzinga_firm_id_lt: Option<&str>,
        benzinga_firm_id_lte: Option<&str>,
        benzinga_rating_id: Option<&str>,
        benzinga_rating_id_any_of: Option<&str>,
        benzinga_rating_id_gt: Option<&str>,
        benzinga_rating_id_gte: Option<&str>,
        benzinga_rating_id_lt: Option<&str>,
        benzinga_rating_id_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaAnalystInsight>> {
        let path = "/benzinga/v1/analyst-insights";
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = date {
            params.push(("date", v.to_string()));
        }
        if let Some(v) = date_any_of {
            params.push(("date.any_of", v.to_string()));
        }
        if let Some(v) = date_gt {
            params.push(("date.gt", v.to_string()));
        }
        if let Some(v) = date_gte {
            params.push(("date.gte", v.to_string()));
        }
        if let Some(v) = date_lt {
            params.push(("date.lt", v.to_string()));
        }
        if let Some(v) = date_lte {
            params.push(("date.lte", v.to_string()));
        }
        if let Some(v) = ticker {
            params.push(("ticker", v.to_string()));
        }
        if let Some(v) = ticker_any_of {
            params.push(("ticker.any_of", v.to_string()));
        }
        if let Some(v) = ticker_gt {
            params.push(("ticker.gt", v.to_string()));
        }
        if let Some(v) = ticker_gte {
            params.push(("ticker.gte", v.to_string()));
        }
        if let Some(v) = ticker_lt {
            params.push(("ticker.lt", v.to_string()));
        }
        if let Some(v) = ticker_lte {
            params.push(("ticker.lte", v.to_string()));
        }
        if let Some(v) = last_updated {
            params.push(("last_updated", v.to_string()));
        }
        if let Some(v) = last_updated_any_of {
            params.push(("last_updated.any_of", v.to_string()));
        }
        if let Some(v) = last_updated_gt {
            params.push(("last_updated.gt", v.to_string()));
        }
        if let Some(v) = last_updated_gte {
            params.push(("last_updated.gte", v.to_string()));
        }
        if let Some(v) = last_updated_lt {
            params.push(("last_updated.lt", v.to_string()));
        }
        if let Some(v) = last_updated_lte {
            params.push(("last_updated.lte", v.to_string()));
        }
        if let Some(v) = firm {
            params.push(("firm", v.to_string()));
        }
        if let Some(v) = firm_any_of {
            params.push(("firm.any_of", v.to_string()));
        }
        if let Some(v) = firm_gt {
            params.push(("firm.gt", v.to_string()));
        }
        if let Some(v) = firm_gte {
            params.push(("firm.gte", v.to_string()));
        }
        if let Some(v) = firm_lt {
            params.push(("firm.lt", v.to_string()));
        }
        if let Some(v) = firm_lte {
            params.push(("firm.lte", v.to_string()));
        }
        if let Some(v) = rating_action {
            params.push(("rating_action", v.to_string()));
        }
        if let Some(v) = rating_action_any_of {
            params.push(("rating_action.any_of", v.to_string()));
        }
        if let Some(v) = rating_action_gt {
            params.push(("rating_action.gt", v.to_string()));
        }
        if let Some(v) = rating_action_gte {
            params.push(("rating_action.gte", v.to_string()));
        }
        if let Some(v) = rating_action_lt {
            params.push(("rating_action.lt", v.to_string()));
        }
        if let Some(v) = rating_action_lte {
            params.push(("rating_action.lte", v.to_string()));
        }
        if let Some(v) = benzinga_firm_id {
            params.push(("benzinga_firm_id", v.to_string()));
        }
        if let Some(v) = benzinga_firm_id_any_of {
            params.push(("benzinga_firm_id.any_of", v.to_string()));
        }
        if let Some(v) = benzinga_firm_id_gt {
            params.push(("benzinga_firm_id.gt", v.to_string()));
        }
        if let Some(v) = benzinga_firm_id_gte {
            params.push(("benzinga_firm_id.gte", v.to_string()));
        }
        if let Some(v) = benzinga_firm_id_lt {
            params.push(("benzinga_firm_id.lt", v.to_string()));
        }
        if let Some(v) = benzinga_firm_id_lte {
            params.push(("benzinga_firm_id.lte", v.to_string()));
        }
        if let Some(v) = benzinga_rating_id {
            params.push(("benzinga_rating_id", v.to_string()));
        }
        if let Some(v) = benzinga_rating_id_any_of {
            params.push(("benzinga_rating_id.any_of", v.to_string()));
        }
        if let Some(v) = benzinga_rating_id_gt {
            params.push(("benzinga_rating_id.gt", v.to_string()));
        }
        if let Some(v) = benzinga_rating_id_gte {
            params.push(("benzinga_rating_id.gte", v.to_string()));
        }
        if let Some(v) = benzinga_rating_id_lt {
            params.push(("benzinga_rating_id.lt", v.to_string()));
        }
        if let Some(v) = benzinga_rating_id_lte {
            params.push(("benzinga_rating_id.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<BenzingaAnalystInsight>(path, Some(&params), options)
        } else {
            self.single_page::<BenzingaAnalystInsight>(path, Some(&params), options)
        }
    }

    fn list_benzinga_analysts(
        &self,
        benzinga_id: Option<&str>,
        benzinga_id_any_of: Option<&str>,
        benzinga_id_gt: Option<&str>,
        benzinga_id_gte: Option<&str>,
        benzinga_id_lt: Option<&str>,
        benzinga_id_lte: Option<&str>,
        benzinga_firm_id: Option<&str>,
        benzinga_firm_id_any_of: Option<&str>,
        benzinga_firm_id_gt: Option<&str>,
        benzinga_firm_id_gte: Option<&str>,
        benzinga_firm_id_lt: Option<&str>,
        benzinga_firm_id_lte: Option<&str>,
        firm_name: Option<&str>,
        firm_name_any_of: Option<&str>,
        firm_name_gt: Option<&str>,
        firm_name_gte: Option<&str>,
        firm_name_lt: Option<&str>,
        firm_name_lte: Option<&str>,
        full_name: Option<&str>,
        full_name_any_of: Option<&str>,
        full_name_gt: Option<&str>,
        full_name_gte: Option<&str>,
        full_name_lt: Option<&str>,
        full_name_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaAnalyst>> {
        let path = "/benzinga/v1/analysts";
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = benzinga_id {
            params.push(("benzinga_id", v.to_string()));
        }
        if let Some(v) = benzinga_id_any_of {
            params.push(("benzinga_id.any_of", v.to_string()));
        }
        if let Some(v) = benzinga_id_gt {
            params.push(("benzinga_id.gt", v.to_string()));
        }
        if let Some(v) = benzinga_id_gte {
            params.push(("benzinga_id.gte", v.to_string()));
        }
        if let Some(v) = benzinga_id_lt {
            params.push(("benzinga_id.lt", v.to_string()));
        }
        if let Some(v) = benzinga_id_lte {
            params.push(("benzinga_id.lte", v.to_string()));
        }
        if let Some(v) = benzinga_firm_id {
            params.push(("benzinga_firm_id", v.to_string()));
        }
        if let Some(v) = benzinga_firm_id_any_of {
            params.push(("benzinga_firm_id.any_of", v.to_string()));
        }
        if let Some(v) = benzinga_firm_id_gt {
            params.push(("benzinga_firm_id.gt", v.to_string()));
        }
        if let Some(v) = benzinga_firm_id_gte {
            params.push(("benzinga_firm_id.gte", v.to_string()));
        }
        if let Some(v) = benzinga_firm_id_lt {
            params.push(("benzinga_firm_id.lt", v.to_string()));
        }
        if let Some(v) = benzinga_firm_id_lte {
            params.push(("benzinga_firm_id.lte", v.to_string()));
        }
        if let Some(v) = firm_name {
            params.push(("firm_name", v.to_string()));
        }
        if let Some(v) = firm_name_any_of {
            params.push(("firm_name.any_of", v.to_string()));
        }
        if let Some(v) = firm_name_gt {
            params.push(("firm_name.gt", v.to_string()));
        }
        if let Some(v) = firm_name_gte {
            params.push(("firm_name.gte", v.to_string()));
        }
        if let Some(v) = firm_name_lt {
            params.push(("firm_name.lt", v.to_string()));
        }
        if let Some(v) = firm_name_lte {
            params.push(("firm_name.lte", v.to_string()));
        }
        if let Some(v) = full_name {
            params.push(("full_name", v.to_string()));
        }
        if let Some(v) = full_name_any_of {
            params.push(("full_name.any_of", v.to_string()));
        }
        if let Some(v) = full_name_gt {
            params.push(("full_name.gt", v.to_string()));
        }
        if let Some(v) = full_name_gte {
            params.push(("full_name.gte", v.to_string()));
        }
        if let Some(v) = full_name_lt {
            params.push(("full_name.lt", v.to_string()));
        }
        if let Some(v) = full_name_lte {
            params.push(("full_name.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<BenzingaAnalyst>(path, Some(&params), options)
        } else {
            self.single_page::<BenzingaAnalyst>(path, Some(&params), options)
        }
    }

    fn list_benzinga_consensus_ratings(
        &self,
        ticker: &str,
        date: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        limit: Option<i64>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaConsensusRating>> {
        let path = format!("/benzinga/v1/consensus-ratings/{}", ticker);
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = date {
            params.push(("date", v.to_string()));
        }
        if let Some(v) = date_gt {
            params.push(("date.gt", v.to_string()));
        }
        if let Some(v) = date_gte {
            params.push(("date.gte", v.to_string()));
        }
        if let Some(v) = date_lt {
            params.push(("date.lt", v.to_string()));
        }
        if let Some(v) = date_lte {
            params.push(("date.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if self.pagination {
            self.paginate::<BenzingaConsensusRating>(&path, Some(&params), options)
        } else {
            self.single_page::<BenzingaConsensusRating>(&path, Some(&params), options)
        }
    }

    fn list_benzinga_earnings(
        &self,
        date: Option<&str>,
        date_any_of: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        importance: Option<i64>,
        importance_any_of: Option<&str>,
        importance_gt: Option<i64>,
        importance_gte: Option<i64>,
        importance_lt: Option<i64>,
        importance_lte: Option<i64>,
        last_updated: Option<&str>,
        last_updated_any_of: Option<&str>,
        last_updated_gt: Option<&str>,
        last_updated_gte: Option<&str>,
        last_updated_lt: Option<&str>,
        last_updated_lte: Option<&str>,
        date_status: Option<&str>,
        date_status_any_of: Option<&str>,
        date_status_gt: Option<&str>,
        date_status_gte: Option<&str>,
        date_status_lt: Option<&str>,
        date_status_lte: Option<&str>,
        eps_surprise_percent: Option<f64>,
        eps_surprise_percent_any_of: Option<&str>,
        eps_surprise_percent_gt: Option<f64>,
        eps_surprise_percent_gte: Option<f64>,
        eps_surprise_percent_lt: Option<f64>,
        eps_surprise_percent_lte: Option<f64>,
        revenue_surprise_percent: Option<f64>,
        revenue_surprise_percent_any_of: Option<&str>,
        revenue_surprise_percent_gt: Option<f64>,
        revenue_surprise_percent_gte: Option<f64>,
        revenue_surprise_percent_lt: Option<f64>,
        revenue_surprise_percent_lte: Option<f64>,
        fiscal_year: Option<i64>,
        fiscal_year_any_of: Option<&str>,
        fiscal_year_gt: Option<i64>,
        fiscal_year_gte: Option<i64>,
        fiscal_year_lt: Option<i64>,
        fiscal_year_lte: Option<i64>,
        fiscal_period: Option<&str>,
        fiscal_period_any_of: Option<&str>,
        fiscal_period_gt: Option<&str>,
        fiscal_period_gte: Option<&str>,
        fiscal_period_lt: Option<&str>,
        fiscal_period_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaEarning>> {
        let path = "/benzinga/v1/earnings";
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = date {
            params.push(("date", v.to_string()));
        }
        if let Some(v) = date_any_of {
            params.push(("date.any_of", v.to_string()));
        }
        if let Some(v) = date_gt {
            params.push(("date.gt", v.to_string()));
        }
        if let Some(v) = date_gte {
            params.push(("date.gte", v.to_string()));
        }
        if let Some(v) = date_lt {
            params.push(("date.lt", v.to_string()));
        }
        if let Some(v) = date_lte {
            params.push(("date.lte", v.to_string()));
        }
        if let Some(v) = ticker {
            params.push(("ticker", v.to_string()));
        }
        if let Some(v) = ticker_any_of {
            params.push(("ticker.any_of", v.to_string()));
        }
        if let Some(v) = ticker_gt {
            params.push(("ticker.gt", v.to_string()));
        }
        if let Some(v) = ticker_gte {
            params.push(("ticker.gte", v.to_string()));
        }
        if let Some(v) = ticker_lt {
            params.push(("ticker.lt", v.to_string()));
        }
        if let Some(v) = ticker_lte {
            params.push(("ticker.lte", v.to_string()));
        }
        if let Some(v) = importance {
            params.push(("importance", v.to_string()));
        }
        if let Some(v) = importance_any_of {
            params.push(("importance.any_of", v.to_string()));
        }
        if let Some(v) = importance_gt {
            params.push(("importance.gt", v.to_string()));
        }
        if let Some(v) = importance_gte {
            params.push(("importance.gte", v.to_string()));
        }
        if let Some(v) = importance_lt {
            params.push(("importance.lt", v.to_string()));
        }
        if let Some(v) = importance_lte {
            params.push(("importance.lte", v.to_string()));
        }
        if let Some(v) = last_updated {
            params.push(("last_updated", v.to_string()));
        }
        if let Some(v) = last_updated_any_of {
            params.push(("last_updated.any_of", v.to_string()));
        }
        if let Some(v) = last_updated_gt {
            params.push(("last_updated.gt", v.to_string()));
        }
        if let Some(v) = last_updated_gte {
            params.push(("last_updated.gte", v.to_string()));
        }
        if let Some(v) = last_updated_lt {
            params.push(("last_updated.lt", v.to_string()));
        }
        if let Some(v) = last_updated_lte {
            params.push(("last_updated.lte", v.to_string()));
        }
        if let Some(v) = date_status {
            params.push(("date_status", v.to_string()));
        }
        if let Some(v) = date_status_any_of {
            params.push(("date_status.any_of", v.to_string()));
        }
        if let Some(v) = date_status_gt {
            params.push(("date_status.gt", v.to_string()));
        }
        if let Some(v) = date_status_gte {
            params.push(("date_status.gte", v.to_string()));
        }
        if let Some(v) = date_status_lt {
            params.push(("date_status.lt", v.to_string()));
        }
        if let Some(v) = date_status_lte {
            params.push(("date_status.lte", v.to_string()));
        }
        if let Some(v) = eps_surprise_percent {
            params.push(("eps_surprise_percent", v.to_string()));
        }
        if let Some(v) = eps_surprise_percent_any_of {
            params.push(("eps_surprise_percent.any_of", v.to_string()));
        }
        if let Some(v) = eps_surprise_percent_gt {
            params.push(("eps_surprise_percent.gt", v.to_string()));
        }
        if let Some(v) = eps_surprise_percent_gte {
            params.push(("eps_surprise_percent.gte", v.to_string()));
        }
        if let Some(v) = eps_surprise_percent_lt {
            params.push(("eps_surprise_percent.lt", v.to_string()));
        }
        if let Some(v) = eps_surprise_percent_lte {
            params.push(("eps_surprise_percent.lte", v.to_string()));
        }
        if let Some(v) = revenue_surprise_percent {
            params.push(("revenue_surprise_percent", v.to_string()));
        }
        if let Some(v) = revenue_surprise_percent_any_of {
            params.push(("revenue_surprise_percent.any_of", v.to_string()));
        }
        if let Some(v) = revenue_surprise_percent_gt {
            params.push(("revenue_surprise_percent.gt", v.to_string()));
        }
        if let Some(v) = revenue_surprise_percent_gte {
            params.push(("revenue_surprise_percent.gte", v.to_string()));
        }
        if let Some(v) = revenue_surprise_percent_lt {
            params.push(("revenue_surprise_percent.lt", v.to_string()));
        }
        if let Some(v) = revenue_surprise_percent_lte {
            params.push(("revenue_surprise_percent.lte", v.to_string()));
        }
        if let Some(v) = fiscal_year {
            params.push(("fiscal_year", v.to_string()));
        }
        if let Some(v) = fiscal_year_any_of {
            params.push(("fiscal_year.any_of", v.to_string()));
        }
        if let Some(v) = fiscal_year_gt {
            params.push(("fiscal_year.gt", v.to_string()));
        }
        if let Some(v) = fiscal_year_gte {
            params.push(("fiscal_year.gte", v.to_string()));
        }
        if let Some(v) = fiscal_year_lt {
            params.push(("fiscal_year.lt", v.to_string()));
        }
        if let Some(v) = fiscal_year_lte {
            params.push(("fiscal_year.lte", v.to_string()));
        }
        if let Some(v) = fiscal_period {
            params.push(("fiscal_period", v.to_string()));
        }
        if let Some(v) = fiscal_period_any_of {
            params.push(("fiscal_period.any_of", v.to_string()));
        }
        if let Some(v) = fiscal_period_gt {
            params.push(("fiscal_period.gt", v.to_string()));
        }
        if let Some(v) = fiscal_period_gte {
            params.push(("fiscal_period.gte", v.to_string()));
        }
        if let Some(v) = fiscal_period_lt {
            params.push(("fiscal_period.lt", v.to_string()));
        }
        if let Some(v) = fiscal_period_lte {
            params.push(("fiscal_period.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<BenzingaEarning>(path, Some(&params), options)
        } else {
            self.single_page::<BenzingaEarning>(path, Some(&params), options)
        }
    }

    fn list_benzinga_firms(
        &self,
        benzinga_id: Option<&str>,
        benzinga_id_any_of: Option<&str>,
        benzinga_id_gt: Option<&str>,
        benzinga_id_gte: Option<&str>,
        benzinga_id_lt: Option<&str>,
        benzinga_id_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaFirm>> {
        let path = "/benzinga/v1/firms";
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = benzinga_id {
            params.push(("benzinga_id", v.to_string()));
        }
        if let Some(v) = benzinga_id_any_of {
            params.push(("benzinga_id.any_of", v.to_string()));
        }
        if let Some(v) = benzinga_id_gt {
            params.push(("benzinga_id.gt", v.to_string()));
        }
        if let Some(v) = benzinga_id_gte {
            params.push(("benzinga_id.gte", v.to_string()));
        }
        if let Some(v) = benzinga_id_lt {
            params.push(("benzinga_id.lt", v.to_string()));
        }
        if let Some(v) = benzinga_id_lte {
            params.push(("benzinga_id.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<BenzingaFirm>(path, Some(&params), options)
        } else {
            self.single_page::<BenzingaFirm>(path, Some(&params), options)
        }
    }

    fn list_benzinga_guidance(
        &self,
        date: Option<&str>,
        date_any_of: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        positioning: Option<&str>,
        positioning_any_of: Option<&str>,
        positioning_gt: Option<&str>,
        positioning_gte: Option<&str>,
        positioning_lt: Option<&str>,
        positioning_lte: Option<&str>,
        importance: Option<i64>,
        importance_any_of: Option<&str>,
        importance_gt: Option<i64>,
        importance_gte: Option<i64>,
        importance_lt: Option<i64>,
        importance_lte: Option<i64>,
        last_updated: Option<&str>,
        last_updated_any_of: Option<&str>,
        last_updated_gt: Option<&str>,
        last_updated_gte: Option<&str>,
        last_updated_lt: Option<&str>,
        last_updated_lte: Option<&str>,
        fiscal_year: Option<i64>,
        fiscal_year_any_of: Option<&str>,
        fiscal_year_gt: Option<i64>,
        fiscal_year_gte: Option<i64>,
        fiscal_year_lt: Option<i64>,
        fiscal_year_lte: Option<i64>,
        fiscal_period: Option<&str>,
        fiscal_period_any_of: Option<&str>,
        fiscal_period_gt: Option<&str>,
        fiscal_period_gte: Option<&str>,
        fiscal_period_lt: Option<&str>,
        fiscal_period_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaGuidance>> {
        let path = "/benzinga/v1/guidance";
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = date {
            params.push(("date", v.to_string()));
        }
        if let Some(v) = date_any_of {
            params.push(("date.any_of", v.to_string()));
        }
        if let Some(v) = date_gt {
            params.push(("date.gt", v.to_string()));
        }
        if let Some(v) = date_gte {
            params.push(("date.gte", v.to_string()));
        }
        if let Some(v) = date_lt {
            params.push(("date.lt", v.to_string()));
        }
        if let Some(v) = date_lte {
            params.push(("date.lte", v.to_string()));
        }
        if let Some(v) = ticker {
            params.push(("ticker", v.to_string()));
        }
        if let Some(v) = ticker_any_of {
            params.push(("ticker.any_of", v.to_string()));
        }
        if let Some(v) = ticker_gt {
            params.push(("ticker.gt", v.to_string()));
        }
        if let Some(v) = ticker_gte {
            params.push(("ticker.gte", v.to_string()));
        }
        if let Some(v) = ticker_lt {
            params.push(("ticker.lt", v.to_string()));
        }
        if let Some(v) = ticker_lte {
            params.push(("ticker.lte", v.to_string()));
        }
        if let Some(v) = positioning {
            params.push(("positioning", v.to_string()));
        }
        if let Some(v) = positioning_any_of {
            params.push(("positioning.any_of", v.to_string()));
        }
        if let Some(v) = positioning_gt {
            params.push(("positioning.gt", v.to_string()));
        }
        if let Some(v) = positioning_gte {
            params.push(("positioning.gte", v.to_string()));
        }
        if let Some(v) = positioning_lt {
            params.push(("positioning.lt", v.to_string()));
        }
        if let Some(v) = positioning_lte {
            params.push(("positioning.lte", v.to_string()));
        }
        if let Some(v) = importance {
            params.push(("importance", v.to_string()));
        }
        if let Some(v) = importance_any_of {
            params.push(("importance.any_of", v.to_string()));
        }
        if let Some(v) = importance_gt {
            params.push(("importance.gt", v.to_string()));
        }
        if let Some(v) = importance_gte {
            params.push(("importance.gte", v.to_string()));
        }
        if let Some(v) = importance_lt {
            params.push(("importance.lt", v.to_string()));
        }
        if let Some(v) = importance_lte {
            params.push(("importance.lte", v.to_string()));
        }
        if let Some(v) = last_updated {
            params.push(("last_updated", v.to_string()));
        }
        if let Some(v) = last_updated_any_of {
            params.push(("last_updated.any_of", v.to_string()));
        }
        if let Some(v) = last_updated_gt {
            params.push(("last_updated.gt", v.to_string()));
        }
        if let Some(v) = last_updated_gte {
            params.push(("last_updated.gte", v.to_string()));
        }
        if let Some(v) = last_updated_lt {
            params.push(("last_updated.lt", v.to_string()));
        }
        if let Some(v) = last_updated_lte {
            params.push(("last_updated.lte", v.to_string()));
        }
        if let Some(v) = fiscal_year {
            params.push(("fiscal_year", v.to_string()));
        }
        if let Some(v) = fiscal_year_any_of {
            params.push(("fiscal_year.any_of", v.to_string()));
        }
        if let Some(v) = fiscal_year_gt {
            params.push(("fiscal_year.gt", v.to_string()));
        }
        if let Some(v) = fiscal_year_gte {
            params.push(("fiscal_year.gte", v.to_string()));
        }
        if let Some(v) = fiscal_year_lt {
            params.push(("fiscal_year.lt", v.to_string()));
        }
        if let Some(v) = fiscal_year_lte {
            params.push(("fiscal_year.lte", v.to_string()));
        }
        if let Some(v) = fiscal_period {
            params.push(("fiscal_period", v.to_string()));
        }
        if let Some(v) = fiscal_period_any_of {
            params.push(("fiscal_period.any_of", v.to_string()));
        }
        if let Some(v) = fiscal_period_gt {
            params.push(("fiscal_period.gt", v.to_string()));
        }
        if let Some(v) = fiscal_period_gte {
            params.push(("fiscal_period.gte", v.to_string()));
        }
        if let Some(v) = fiscal_period_lt {
            params.push(("fiscal_period.lt", v.to_string()));
        }
        if let Some(v) = fiscal_period_lte {
            params.push(("fiscal_period.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<BenzingaGuidance>(path, Some(&params), options)
        } else {
            self.single_page::<BenzingaGuidance>(path, Some(&params), options)
        }
    }

    fn list_benzinga_news(
        &self,
        published: Option<&str>,
        published_any_of: Option<&str>,
        published_gt: Option<&str>,
        published_gte: Option<&str>,
        published_lt: Option<&str>,
        published_lte: Option<&str>,
        last_updated: Option<&str>,
        last_updated_any_of: Option<&str>,
        last_updated_gt: Option<&str>,
        last_updated_gte: Option<&str>,
        last_updated_lt: Option<&str>,
        last_updated_lte: Option<&str>,
        tickers: Option<&str>,
        tickers_all_of: Option<&str>,
        tickers_any_of: Option<&str>,
        channels: Option<&str>,
        channels_all_of: Option<&str>,
        channels_any_of: Option<&str>,
        tags: Option<&str>,
        tags_all_of: Option<&str>,
        tags_any_of: Option<&str>,
        author: Option<&str>,
        author_any_of: Option<&str>,
        author_gt: Option<&str>,
        author_gte: Option<&str>,
        author_lt: Option<&str>,
        author_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaNews>> {
        let path = "/benzinga/v1/news";
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = published {
            params.push(("published", v.to_string()));
        }
        if let Some(v) = published_any_of {
            params.push(("published.any_of", v.to_string()));
        }
        if let Some(v) = published_gt {
            params.push(("published.gt", v.to_string()));
        }
        if let Some(v) = published_gte {
            params.push(("published.gte", v.to_string()));
        }
        if let Some(v) = published_lt {
            params.push(("published.lt", v.to_string()));
        }
        if let Some(v) = published_lte {
            params.push(("published.lte", v.to_string()));
        }
        if let Some(v) = last_updated {
            params.push(("last_updated", v.to_string()));
        }
        if let Some(v) = last_updated_any_of {
            params.push(("last_updated.any_of", v.to_string()));
        }
        if let Some(v) = last_updated_gt {
            params.push(("last_updated.gt", v.to_string()));
        }
        if let Some(v) = last_updated_gte {
            params.push(("last_updated.gte", v.to_string()));
        }
        if let Some(v) = last_updated_lt {
            params.push(("last_updated.lt", v.to_string()));
        }
        if let Some(v) = last_updated_lte {
            params.push(("last_updated.lte", v.to_string()));
        }
        if let Some(v) = tickers {
            params.push(("tickers", v.to_string()));
        }
        if let Some(v) = tickers_all_of {
            params.push(("tickers.all_of", v.to_string()));
        }
        if let Some(v) = tickers_any_of {
            params.push(("tickers.any_of", v.to_string()));
        }
        if let Some(v) = channels {
            params.push(("channels", v.to_string()));
        }
        if let Some(v) = channels_all_of {
            params.push(("channels.all_of", v.to_string()));
        }
        if let Some(v) = channels_any_of {
            params.push(("channels.any_of", v.to_string()));
        }
        if let Some(v) = tags {
            params.push(("tags", v.to_string()));
        }
        if let Some(v) = tags_all_of {
            params.push(("tags.all_of", v.to_string()));
        }
        if let Some(v) = tags_any_of {
            params.push(("tags.any_of", v.to_string()));
        }
        if let Some(v) = author {
            params.push(("author", v.to_string()));
        }
        if let Some(v) = author_any_of {
            params.push(("author.any_of", v.to_string()));
        }
        if let Some(v) = author_gt {
            params.push(("author.gt", v.to_string()));
        }
        if let Some(v) = author_gte {
            params.push(("author.gte", v.to_string()));
        }
        if let Some(v) = author_lt {
            params.push(("author.lt", v.to_string()));
        }
        if let Some(v) = author_lte {
            params.push(("author.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<BenzingaNews>(path, Some(&params), options)
        } else {
            self.single_page::<BenzingaNews>(path, Some(&params), options)
        }
    }

    fn list_benzinga_news_v2(
        &self,
        published: Option<&str>,
        published_gt: Option<&str>,
        published_gte: Option<&str>,
        published_lt: Option<&str>,
        published_lte: Option<&str>,
        channels: Option<&str>,
        channels_all_of: Option<&str>,
        channels_any_of: Option<&str>,
        tags: Option<&str>,
        tags_all_of: Option<&str>,
        tags_any_of: Option<&str>,
        author: Option<&str>,
        author_any_of: Option<&str>,
        author_gt: Option<&str>,
        author_gte: Option<&str>,
        author_lt: Option<&str>,
        author_lte: Option<&str>,
        stocks: Option<&str>,
        stocks_all_of: Option<&str>,
        stocks_any_of: Option<&str>,
        tickers: Option<&str>,
        tickers_all_of: Option<&str>,
        tickers_any_of: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaNews>> {
        let path = "/benzinga/v2/news";
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = published {
            params.push(("published", v.to_string()));
        }
        if let Some(v) = published_gt {
            params.push(("published.gt", v.to_string()));
        }
        if let Some(v) = published_gte {
            params.push(("published.gte", v.to_string()));
        }
        if let Some(v) = published_lt {
            params.push(("published.lt", v.to_string()));
        }
        if let Some(v) = published_lte {
            params.push(("published.lte", v.to_string()));
        }
        if let Some(v) = channels {
            params.push(("channels", v.to_string()));
        }
        if let Some(v) = channels_all_of {
            params.push(("channels.all_of", v.to_string()));
        }
        if let Some(v) = channels_any_of {
            params.push(("channels.any_of", v.to_string()));
        }
        if let Some(v) = tags {
            params.push(("tags", v.to_string()));
        }
        if let Some(v) = tags_all_of {
            params.push(("tags.all_of", v.to_string()));
        }
        if let Some(v) = tags_any_of {
            params.push(("tags.any_of", v.to_string()));
        }
        if let Some(v) = author {
            params.push(("author", v.to_string()));
        }
        if let Some(v) = author_any_of {
            params.push(("author.any_of", v.to_string()));
        }
        if let Some(v) = author_gt {
            params.push(("author.gt", v.to_string()));
        }
        if let Some(v) = author_gte {
            params.push(("author.gte", v.to_string()));
        }
        if let Some(v) = author_lt {
            params.push(("author.lt", v.to_string()));
        }
        if let Some(v) = author_lte {
            params.push(("author.lte", v.to_string()));
        }
        if let Some(v) = stocks {
            params.push(("stocks", v.to_string()));
        }
        if let Some(v) = stocks_all_of {
            params.push(("stocks.all_of", v.to_string()));
        }
        if let Some(v) = stocks_any_of {
            params.push(("stocks.any_of", v.to_string()));
        }
        if let Some(v) = tickers {
            params.push(("tickers", v.to_string()));
        }
        if let Some(v) = tickers_all_of {
            params.push(("tickers.all_of", v.to_string()));
        }
        if let Some(v) = tickers_any_of {
            params.push(("tickers.any_of", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<BenzingaNews>(path, Some(&params), options)
        } else {
            self.single_page::<BenzingaNews>(path, Some(&params), options)
        }
    }

    fn list_benzinga_ratings(
        &self,
        date: Option<&str>,
        date_any_of: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        importance: Option<i64>,
        importance_any_of: Option<&str>,
        importance_gt: Option<i64>,
        importance_gte: Option<i64>,
        importance_lt: Option<i64>,
        importance_lte: Option<i64>,
        last_updated: Option<&str>,
        last_updated_any_of: Option<&str>,
        last_updated_gt: Option<&str>,
        last_updated_gte: Option<&str>,
        last_updated_lt: Option<&str>,
        last_updated_lte: Option<&str>,
        rating_action: Option<&str>,
        rating_action_any_of: Option<&str>,
        rating_action_gt: Option<&str>,
        rating_action_gte: Option<&str>,
        rating_action_lt: Option<&str>,
        rating_action_lte: Option<&str>,
        price_target_action: Option<&str>,
        price_target_action_any_of: Option<&str>,
        price_target_action_gt: Option<&str>,
        price_target_action_gte: Option<&str>,
        price_target_action_lt: Option<&str>,
        price_target_action_lte: Option<&str>,
        benzinga_id: Option<&str>,
        benzinga_id_any_of: Option<&str>,
        benzinga_id_gt: Option<&str>,
        benzinga_id_gte: Option<&str>,
        benzinga_id_lt: Option<&str>,
        benzinga_id_lte: Option<&str>,
        benzinga_analyst_id: Option<&str>,
        benzinga_analyst_id_any_of: Option<&str>,
        benzinga_analyst_id_gt: Option<&str>,
        benzinga_analyst_id_gte: Option<&str>,
        benzinga_analyst_id_lt: Option<&str>,
        benzinga_analyst_id_lte: Option<&str>,
        benzinga_firm_id: Option<&str>,
        benzinga_firm_id_any_of: Option<&str>,
        benzinga_firm_id_gt: Option<&str>,
        benzinga_firm_id_gte: Option<&str>,
        benzinga_firm_id_lt: Option<&str>,
        benzinga_firm_id_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaRating>> {
        let path = "/benzinga/v1/ratings";
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = date {
            params.push(("date", v.to_string()));
        }
        if let Some(v) = date_any_of {
            params.push(("date.any_of", v.to_string()));
        }
        if let Some(v) = date_gt {
            params.push(("date.gt", v.to_string()));
        }
        if let Some(v) = date_gte {
            params.push(("date.gte", v.to_string()));
        }
        if let Some(v) = date_lt {
            params.push(("date.lt", v.to_string()));
        }
        if let Some(v) = date_lte {
            params.push(("date.lte", v.to_string()));
        }
        if let Some(v) = ticker {
            params.push(("ticker", v.to_string()));
        }
        if let Some(v) = ticker_any_of {
            params.push(("ticker.any_of", v.to_string()));
        }
        if let Some(v) = ticker_gt {
            params.push(("ticker.gt", v.to_string()));
        }
        if let Some(v) = ticker_gte {
            params.push(("ticker.gte", v.to_string()));
        }
        if let Some(v) = ticker_lt {
            params.push(("ticker.lt", v.to_string()));
        }
        if let Some(v) = ticker_lte {
            params.push(("ticker.lte", v.to_string()));
        }
        if let Some(v) = importance {
            params.push(("importance", v.to_string()));
        }
        if let Some(v) = importance_any_of {
            params.push(("importance.any_of", v.to_string()));
        }
        if let Some(v) = importance_gt {
            params.push(("importance.gt", v.to_string()));
        }
        if let Some(v) = importance_gte {
            params.push(("importance.gte", v.to_string()));
        }
        if let Some(v) = importance_lt {
            params.push(("importance.lt", v.to_string()));
        }
        if let Some(v) = importance_lte {
            params.push(("importance.lte", v.to_string()));
        }
        if let Some(v) = last_updated {
            params.push(("last_updated", v.to_string()));
        }
        if let Some(v) = last_updated_any_of {
            params.push(("last_updated.any_of", v.to_string()));
        }
        if let Some(v) = last_updated_gt {
            params.push(("last_updated.gt", v.to_string()));
        }
        if let Some(v) = last_updated_gte {
            params.push(("last_updated.gte", v.to_string()));
        }
        if let Some(v) = last_updated_lt {
            params.push(("last_updated.lt", v.to_string()));
        }
        if let Some(v) = last_updated_lte {
            params.push(("last_updated.lte", v.to_string()));
        }
        if let Some(v) = rating_action {
            params.push(("rating_action", v.to_string()));
        }
        if let Some(v) = rating_action_any_of {
            params.push(("rating_action.any_of", v.to_string()));
        }
        if let Some(v) = rating_action_gt {
            params.push(("rating_action.gt", v.to_string()));
        }
        if let Some(v) = rating_action_gte {
            params.push(("rating_action.gte", v.to_string()));
        }
        if let Some(v) = rating_action_lt {
            params.push(("rating_action.lt", v.to_string()));
        }
        if let Some(v) = rating_action_lte {
            params.push(("rating_action.lte", v.to_string()));
        }
        if let Some(v) = price_target_action {
            params.push(("price_target_action", v.to_string()));
        }
        if let Some(v) = price_target_action_any_of {
            params.push(("price_target_action.any_of", v.to_string()));
        }
        if let Some(v) = price_target_action_gt {
            params.push(("price_target_action.gt", v.to_string()));
        }
        if let Some(v) = price_target_action_gte {
            params.push(("price_target_action.gte", v.to_string()));
        }
        if let Some(v) = price_target_action_lt {
            params.push(("price_target_action.lt", v.to_string()));
        }
        if let Some(v) = price_target_action_lte {
            params.push(("price_target_action.lte", v.to_string()));
        }
        if let Some(v) = benzinga_id {
            params.push(("benzinga_id", v.to_string()));
        }
        if let Some(v) = benzinga_id_any_of {
            params.push(("benzinga_id.any_of", v.to_string()));
        }
        if let Some(v) = benzinga_id_gt {
            params.push(("benzinga_id.gt", v.to_string()));
        }
        if let Some(v) = benzinga_id_gte {
            params.push(("benzinga_id.gte", v.to_string()));
        }
        if let Some(v) = benzinga_id_lt {
            params.push(("benzinga_id.lt", v.to_string()));
        }
        if let Some(v) = benzinga_id_lte {
            params.push(("benzinga_id.lte", v.to_string()));
        }
        if let Some(v) = benzinga_analyst_id {
            params.push(("benzinga_analyst_id", v.to_string()));
        }
        if let Some(v) = benzinga_analyst_id_any_of {
            params.push(("benzinga_analyst_id.any_of", v.to_string()));
        }
        if let Some(v) = benzinga_analyst_id_gt {
            params.push(("benzinga_analyst_id.gt", v.to_string()));
        }
        if let Some(v) = benzinga_analyst_id_gte {
            params.push(("benzinga_analyst_id.gte", v.to_string()));
        }
        if let Some(v) = benzinga_analyst_id_lt {
            params.push(("benzinga_analyst_id.lt", v.to_string()));
        }
        if let Some(v) = benzinga_analyst_id_lte {
            params.push(("benzinga_analyst_id.lte", v.to_string()));
        }
        if let Some(v) = benzinga_firm_id {
            params.push(("benzinga_firm_id", v.to_string()));
        }
        if let Some(v) = benzinga_firm_id_any_of {
            params.push(("benzinga_firm_id.any_of", v.to_string()));
        }
        if let Some(v) = benzinga_firm_id_gt {
            params.push(("benzinga_firm_id.gt", v.to_string()));
        }
        if let Some(v) = benzinga_firm_id_gte {
            params.push(("benzinga_firm_id.gte", v.to_string()));
        }
        if let Some(v) = benzinga_firm_id_lt {
            params.push(("benzinga_firm_id.lt", v.to_string()));
        }
        if let Some(v) = benzinga_firm_id_lte {
            params.push(("benzinga_firm_id.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<BenzingaRating>(path, Some(&params), options)
        } else {
            self.single_page::<BenzingaRating>(path, Some(&params), options)
        }
    }

    fn list_benzinga_bulls_bears_say(
        &self,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        benzinga_id: Option<&str>,
        benzinga_id_any_of: Option<&str>,
        benzinga_id_gt: Option<&str>,
        benzinga_id_gte: Option<&str>,
        benzinga_id_lt: Option<&str>,
        benzinga_id_lte: Option<&str>,
        last_updated: Option<&str>,
        last_updated_gt: Option<&str>,
        last_updated_gte: Option<&str>,
        last_updated_lt: Option<&str>,
        last_updated_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<BenzingaBullsBearsSay>> {
        let path = "/benzinga/v1/bulls-bears-say";
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = ticker {
            params.push(("ticker", v.to_string()));
        }
        if let Some(v) = ticker_any_of {
            params.push(("ticker.any_of", v.to_string()));
        }
        if let Some(v) = ticker_gt {
            params.push(("ticker.gt", v.to_string()));
        }
        if let Some(v) = ticker_gte {
            params.push(("ticker.gte", v.to_string()));
        }
        if let Some(v) = ticker_lt {
            params.push(("ticker.lt", v.to_string()));
        }
        if let Some(v) = ticker_lte {
            params.push(("ticker.lte", v.to_string()));
        }
        if let Some(v) = benzinga_id {
            params.push(("benzinga_id", v.to_string()));
        }
        if let Some(v) = benzinga_id_any_of {
            params.push(("benzinga_id.any_of", v.to_string()));
        }
        if let Some(v) = benzinga_id_gt {
            params.push(("benzinga_id.gt", v.to_string()));
        }
        if let Some(v) = benzinga_id_gte {
            params.push(("benzinga_id.gte", v.to_string()));
        }
        if let Some(v) = benzinga_id_lt {
            params.push(("benzinga_id.lt", v.to_string()));
        }
        if let Some(v) = benzinga_id_lte {
            params.push(("benzinga_id.lte", v.to_string()));
        }
        if let Some(v) = last_updated {
            params.push(("last_updated", v.to_string()));
        }
        if let Some(v) = last_updated_gt {
            params.push(("last_updated.gt", v.to_string()));
        }
        if let Some(v) = last_updated_gte {
            params.push(("last_updated.gte", v.to_string()));
        }
        if let Some(v) = last_updated_lt {
            params.push(("last_updated.lt", v.to_string()));
        }
        if let Some(v) = last_updated_lte {
            params.push(("last_updated.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<BenzingaBullsBearsSay>(path, Some(&params), options)
        } else {
            self.single_page::<BenzingaBullsBearsSay>(path, Some(&params), options)
        }
    }
}
