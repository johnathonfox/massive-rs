use super::BoxStream;
use crate::client::{Client, RequestOptions};
use crate::models::{
    BenzingaAnalyst, BenzingaAnalystInsight, BenzingaBullsBearsSay, BenzingaConsensusRating,
    BenzingaEarning, BenzingaFirm, BenzingaGuidance, BenzingaNews, BenzingaRating,
};

/// Benzinga API.
pub trait BenzingaApi {
    /// List Benzinga analyst insights.
    #[allow(clippy::too_many_arguments)]
    fn list_benzinga_analyst_insights<'a>(
        &'a self,
        date: Option<&'a str>,
        date_any_of: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        last_updated: Option<&'a str>,
        last_updated_any_of: Option<&'a str>,
        last_updated_gt: Option<&'a str>,
        last_updated_gte: Option<&'a str>,
        last_updated_lt: Option<&'a str>,
        last_updated_lte: Option<&'a str>,
        firm: Option<&'a str>,
        firm_any_of: Option<&'a str>,
        firm_gt: Option<&'a str>,
        firm_gte: Option<&'a str>,
        firm_lt: Option<&'a str>,
        firm_lte: Option<&'a str>,
        rating_action: Option<&'a str>,
        rating_action_any_of: Option<&'a str>,
        rating_action_gt: Option<&'a str>,
        rating_action_gte: Option<&'a str>,
        rating_action_lt: Option<&'a str>,
        rating_action_lte: Option<&'a str>,
        benzinga_firm_id: Option<&'a str>,
        benzinga_firm_id_any_of: Option<&'a str>,
        benzinga_firm_id_gt: Option<&'a str>,
        benzinga_firm_id_gte: Option<&'a str>,
        benzinga_firm_id_lt: Option<&'a str>,
        benzinga_firm_id_lte: Option<&'a str>,
        benzinga_rating_id: Option<&'a str>,
        benzinga_rating_id_any_of: Option<&'a str>,
        benzinga_rating_id_gt: Option<&'a str>,
        benzinga_rating_id_gte: Option<&'a str>,
        benzinga_rating_id_lt: Option<&'a str>,
        benzinga_rating_id_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaAnalystInsight>;

    /// Same as [`Self::list_benzinga_analyst_insights`], but takes the optional arguments as a
    /// chainable [`ListBenzingaAnalystInsightsParams`] struct.
    fn list_benzinga_analyst_insights_with_params<'a>(
        &'a self,
        params: ListBenzingaAnalystInsightsParams,
    ) -> BoxStream<'a, BenzingaAnalystInsight>;

    /// List Benzinga analysts.
    #[allow(clippy::too_many_arguments)]
    fn list_benzinga_analysts<'a>(
        &'a self,
        benzinga_id: Option<&'a str>,
        benzinga_id_any_of: Option<&'a str>,
        benzinga_id_gt: Option<&'a str>,
        benzinga_id_gte: Option<&'a str>,
        benzinga_id_lt: Option<&'a str>,
        benzinga_id_lte: Option<&'a str>,
        benzinga_firm_id: Option<&'a str>,
        benzinga_firm_id_any_of: Option<&'a str>,
        benzinga_firm_id_gt: Option<&'a str>,
        benzinga_firm_id_gte: Option<&'a str>,
        benzinga_firm_id_lt: Option<&'a str>,
        benzinga_firm_id_lte: Option<&'a str>,
        firm_name: Option<&'a str>,
        firm_name_any_of: Option<&'a str>,
        firm_name_gt: Option<&'a str>,
        firm_name_gte: Option<&'a str>,
        firm_name_lt: Option<&'a str>,
        firm_name_lte: Option<&'a str>,
        full_name: Option<&'a str>,
        full_name_any_of: Option<&'a str>,
        full_name_gt: Option<&'a str>,
        full_name_gte: Option<&'a str>,
        full_name_lt: Option<&'a str>,
        full_name_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaAnalyst>;

    /// Same as [`Self::list_benzinga_analysts`], but takes the optional arguments as a
    /// chainable [`ListBenzingaAnalystsParams`] struct.
    fn list_benzinga_analysts_with_params<'a>(
        &'a self,
        params: ListBenzingaAnalystsParams,
    ) -> BoxStream<'a, BenzingaAnalyst>;

    /// List Benzinga consensus ratings for a ticker.
    fn list_benzinga_consensus_ratings<'a>(
        &'a self,
        ticker: &'a str,
        date: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        limit: Option<i64>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaConsensusRating>;

    /// Same as [`Self::list_benzinga_consensus_ratings`], but takes the optional arguments as a
    /// chainable [`ListBenzingaConsensusRatingsParams`] struct.
    fn list_benzinga_consensus_ratings_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: ListBenzingaConsensusRatingsParams,
    ) -> BoxStream<'a, BenzingaConsensusRating>;

    /// List Benzinga earnings.
    #[allow(clippy::too_many_arguments)]
    fn list_benzinga_earnings<'a>(
        &'a self,
        date: Option<&'a str>,
        date_any_of: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        importance: Option<i64>,
        importance_any_of: Option<&'a str>,
        importance_gt: Option<i64>,
        importance_gte: Option<i64>,
        importance_lt: Option<i64>,
        importance_lte: Option<i64>,
        last_updated: Option<&'a str>,
        last_updated_any_of: Option<&'a str>,
        last_updated_gt: Option<&'a str>,
        last_updated_gte: Option<&'a str>,
        last_updated_lt: Option<&'a str>,
        last_updated_lte: Option<&'a str>,
        date_status: Option<&'a str>,
        date_status_any_of: Option<&'a str>,
        date_status_gt: Option<&'a str>,
        date_status_gte: Option<&'a str>,
        date_status_lt: Option<&'a str>,
        date_status_lte: Option<&'a str>,
        eps_surprise_percent: Option<f64>,
        eps_surprise_percent_any_of: Option<&'a str>,
        eps_surprise_percent_gt: Option<f64>,
        eps_surprise_percent_gte: Option<f64>,
        eps_surprise_percent_lt: Option<f64>,
        eps_surprise_percent_lte: Option<f64>,
        revenue_surprise_percent: Option<f64>,
        revenue_surprise_percent_any_of: Option<&'a str>,
        revenue_surprise_percent_gt: Option<f64>,
        revenue_surprise_percent_gte: Option<f64>,
        revenue_surprise_percent_lt: Option<f64>,
        revenue_surprise_percent_lte: Option<f64>,
        fiscal_year: Option<i64>,
        fiscal_year_any_of: Option<&'a str>,
        fiscal_year_gt: Option<i64>,
        fiscal_year_gte: Option<i64>,
        fiscal_year_lt: Option<i64>,
        fiscal_year_lte: Option<i64>,
        fiscal_period: Option<&'a str>,
        fiscal_period_any_of: Option<&'a str>,
        fiscal_period_gt: Option<&'a str>,
        fiscal_period_gte: Option<&'a str>,
        fiscal_period_lt: Option<&'a str>,
        fiscal_period_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaEarning>;

    /// Same as [`Self::list_benzinga_earnings`], but takes the optional arguments as a
    /// chainable [`ListBenzingaEarningsParams`] struct.
    fn list_benzinga_earnings_with_params<'a>(
        &'a self,
        params: ListBenzingaEarningsParams,
    ) -> BoxStream<'a, BenzingaEarning>;

    /// List Benzinga firms.
    fn list_benzinga_firms<'a>(
        &'a self,
        benzinga_id: Option<&'a str>,
        benzinga_id_any_of: Option<&'a str>,
        benzinga_id_gt: Option<&'a str>,
        benzinga_id_gte: Option<&'a str>,
        benzinga_id_lt: Option<&'a str>,
        benzinga_id_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaFirm>;

    /// Same as [`Self::list_benzinga_firms`], but takes the optional arguments as a
    /// chainable [`ListBenzingaFirmsParams`] struct.
    fn list_benzinga_firms_with_params<'a>(
        &'a self,
        params: ListBenzingaFirmsParams,
    ) -> BoxStream<'a, BenzingaFirm>;

    /// List Benzinga guidance.
    #[allow(clippy::too_many_arguments)]
    fn list_benzinga_guidance<'a>(
        &'a self,
        date: Option<&'a str>,
        date_any_of: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        positioning: Option<&'a str>,
        positioning_any_of: Option<&'a str>,
        positioning_gt: Option<&'a str>,
        positioning_gte: Option<&'a str>,
        positioning_lt: Option<&'a str>,
        positioning_lte: Option<&'a str>,
        importance: Option<i64>,
        importance_any_of: Option<&'a str>,
        importance_gt: Option<i64>,
        importance_gte: Option<i64>,
        importance_lt: Option<i64>,
        importance_lte: Option<i64>,
        last_updated: Option<&'a str>,
        last_updated_any_of: Option<&'a str>,
        last_updated_gt: Option<&'a str>,
        last_updated_gte: Option<&'a str>,
        last_updated_lt: Option<&'a str>,
        last_updated_lte: Option<&'a str>,
        fiscal_year: Option<i64>,
        fiscal_year_any_of: Option<&'a str>,
        fiscal_year_gt: Option<i64>,
        fiscal_year_gte: Option<i64>,
        fiscal_year_lt: Option<i64>,
        fiscal_year_lte: Option<i64>,
        fiscal_period: Option<&'a str>,
        fiscal_period_any_of: Option<&'a str>,
        fiscal_period_gt: Option<&'a str>,
        fiscal_period_gte: Option<&'a str>,
        fiscal_period_lt: Option<&'a str>,
        fiscal_period_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaGuidance>;

    /// Same as [`Self::list_benzinga_guidance`], but takes the optional arguments as a
    /// chainable [`ListBenzingaGuidanceParams`] struct.
    fn list_benzinga_guidance_with_params<'a>(
        &'a self,
        params: ListBenzingaGuidanceParams,
    ) -> BoxStream<'a, BenzingaGuidance>;

    /// List Benzinga news (v1).
    #[allow(clippy::too_many_arguments)]
    fn list_benzinga_news<'a>(
        &'a self,
        published: Option<&'a str>,
        published_any_of: Option<&'a str>,
        published_gt: Option<&'a str>,
        published_gte: Option<&'a str>,
        published_lt: Option<&'a str>,
        published_lte: Option<&'a str>,
        last_updated: Option<&'a str>,
        last_updated_any_of: Option<&'a str>,
        last_updated_gt: Option<&'a str>,
        last_updated_gte: Option<&'a str>,
        last_updated_lt: Option<&'a str>,
        last_updated_lte: Option<&'a str>,
        tickers: Option<&'a str>,
        tickers_all_of: Option<&'a str>,
        tickers_any_of: Option<&'a str>,
        channels: Option<&'a str>,
        channels_all_of: Option<&'a str>,
        channels_any_of: Option<&'a str>,
        tags: Option<&'a str>,
        tags_all_of: Option<&'a str>,
        tags_any_of: Option<&'a str>,
        author: Option<&'a str>,
        author_any_of: Option<&'a str>,
        author_gt: Option<&'a str>,
        author_gte: Option<&'a str>,
        author_lt: Option<&'a str>,
        author_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaNews>;

    /// Same as [`Self::list_benzinga_news`], but takes the optional arguments as a
    /// chainable [`ListBenzingaNewsParams`] struct.
    fn list_benzinga_news_with_params<'a>(
        &'a self,
        params: ListBenzingaNewsParams,
    ) -> BoxStream<'a, BenzingaNews>;

    /// List Benzinga news (v2).
    #[allow(clippy::too_many_arguments)]
    fn list_benzinga_news_v2<'a>(
        &'a self,
        published: Option<&'a str>,
        published_gt: Option<&'a str>,
        published_gte: Option<&'a str>,
        published_lt: Option<&'a str>,
        published_lte: Option<&'a str>,
        channels: Option<&'a str>,
        channels_all_of: Option<&'a str>,
        channels_any_of: Option<&'a str>,
        tags: Option<&'a str>,
        tags_all_of: Option<&'a str>,
        tags_any_of: Option<&'a str>,
        author: Option<&'a str>,
        author_any_of: Option<&'a str>,
        author_gt: Option<&'a str>,
        author_gte: Option<&'a str>,
        author_lt: Option<&'a str>,
        author_lte: Option<&'a str>,
        stocks: Option<&'a str>,
        stocks_all_of: Option<&'a str>,
        stocks_any_of: Option<&'a str>,
        tickers: Option<&'a str>,
        tickers_all_of: Option<&'a str>,
        tickers_any_of: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaNews>;

    /// Same as [`Self::list_benzinga_news_v2`], but takes the optional arguments as a
    /// chainable [`ListBenzingaNewsV2Params`] struct.
    fn list_benzinga_news_v2_with_params<'a>(
        &'a self,
        params: ListBenzingaNewsV2Params,
    ) -> BoxStream<'a, BenzingaNews>;

    /// List Benzinga ratings.
    #[allow(clippy::too_many_arguments)]
    fn list_benzinga_ratings<'a>(
        &'a self,
        date: Option<&'a str>,
        date_any_of: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        importance: Option<i64>,
        importance_any_of: Option<&'a str>,
        importance_gt: Option<i64>,
        importance_gte: Option<i64>,
        importance_lt: Option<i64>,
        importance_lte: Option<i64>,
        last_updated: Option<&'a str>,
        last_updated_any_of: Option<&'a str>,
        last_updated_gt: Option<&'a str>,
        last_updated_gte: Option<&'a str>,
        last_updated_lt: Option<&'a str>,
        last_updated_lte: Option<&'a str>,
        rating_action: Option<&'a str>,
        rating_action_any_of: Option<&'a str>,
        rating_action_gt: Option<&'a str>,
        rating_action_gte: Option<&'a str>,
        rating_action_lt: Option<&'a str>,
        rating_action_lte: Option<&'a str>,
        price_target_action: Option<&'a str>,
        price_target_action_any_of: Option<&'a str>,
        price_target_action_gt: Option<&'a str>,
        price_target_action_gte: Option<&'a str>,
        price_target_action_lt: Option<&'a str>,
        price_target_action_lte: Option<&'a str>,
        benzinga_id: Option<&'a str>,
        benzinga_id_any_of: Option<&'a str>,
        benzinga_id_gt: Option<&'a str>,
        benzinga_id_gte: Option<&'a str>,
        benzinga_id_lt: Option<&'a str>,
        benzinga_id_lte: Option<&'a str>,
        benzinga_analyst_id: Option<&'a str>,
        benzinga_analyst_id_any_of: Option<&'a str>,
        benzinga_analyst_id_gt: Option<&'a str>,
        benzinga_analyst_id_gte: Option<&'a str>,
        benzinga_analyst_id_lt: Option<&'a str>,
        benzinga_analyst_id_lte: Option<&'a str>,
        benzinga_firm_id: Option<&'a str>,
        benzinga_firm_id_any_of: Option<&'a str>,
        benzinga_firm_id_gt: Option<&'a str>,
        benzinga_firm_id_gte: Option<&'a str>,
        benzinga_firm_id_lt: Option<&'a str>,
        benzinga_firm_id_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaRating>;

    /// Same as [`Self::list_benzinga_ratings`], but takes the optional arguments as a
    /// chainable [`ListBenzingaRatingsParams`] struct.
    fn list_benzinga_ratings_with_params<'a>(
        &'a self,
        params: ListBenzingaRatingsParams,
    ) -> BoxStream<'a, BenzingaRating>;

    /// List Benzinga bulls/bears case summaries.
    #[allow(clippy::too_many_arguments)]
    fn list_benzinga_bulls_bears_say<'a>(
        &'a self,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        benzinga_id: Option<&'a str>,
        benzinga_id_any_of: Option<&'a str>,
        benzinga_id_gt: Option<&'a str>,
        benzinga_id_gte: Option<&'a str>,
        benzinga_id_lt: Option<&'a str>,
        benzinga_id_lte: Option<&'a str>,
        last_updated: Option<&'a str>,
        last_updated_gt: Option<&'a str>,
        last_updated_gte: Option<&'a str>,
        last_updated_lt: Option<&'a str>,
        last_updated_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaBullsBearsSay>;

    /// Same as [`Self::list_benzinga_bulls_bears_say`], but takes the optional arguments as a
    /// chainable [`ListBenzingaBullsBearsSayParams`] struct.
    fn list_benzinga_bulls_bears_say_with_params<'a>(
        &'a self,
        params: ListBenzingaBullsBearsSayParams,
    ) -> BoxStream<'a, BenzingaBullsBearsSay>;
}

impl BenzingaApi for Client {
    fn list_benzinga_analyst_insights<'a>(
        &'a self,
        date: Option<&'a str>,
        date_any_of: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        last_updated: Option<&'a str>,
        last_updated_any_of: Option<&'a str>,
        last_updated_gt: Option<&'a str>,
        last_updated_gte: Option<&'a str>,
        last_updated_lt: Option<&'a str>,
        last_updated_lte: Option<&'a str>,
        firm: Option<&'a str>,
        firm_any_of: Option<&'a str>,
        firm_gt: Option<&'a str>,
        firm_gte: Option<&'a str>,
        firm_lt: Option<&'a str>,
        firm_lte: Option<&'a str>,
        rating_action: Option<&'a str>,
        rating_action_any_of: Option<&'a str>,
        rating_action_gt: Option<&'a str>,
        rating_action_gte: Option<&'a str>,
        rating_action_lt: Option<&'a str>,
        rating_action_lte: Option<&'a str>,
        benzinga_firm_id: Option<&'a str>,
        benzinga_firm_id_any_of: Option<&'a str>,
        benzinga_firm_id_gt: Option<&'a str>,
        benzinga_firm_id_gte: Option<&'a str>,
        benzinga_firm_id_lt: Option<&'a str>,
        benzinga_firm_id_lte: Option<&'a str>,
        benzinga_rating_id: Option<&'a str>,
        benzinga_rating_id_any_of: Option<&'a str>,
        benzinga_rating_id_gt: Option<&'a str>,
        benzinga_rating_id_gte: Option<&'a str>,
        benzinga_rating_id_lt: Option<&'a str>,
        benzinga_rating_id_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaAnalystInsight> {
        self.list_benzinga_analyst_insights_with_params(ListBenzingaAnalystInsightsParams {
            date: date.map(String::from),
            date_any_of: date_any_of.map(String::from),
            date_gt: date_gt.map(String::from),
            date_gte: date_gte.map(String::from),
            date_lt: date_lt.map(String::from),
            date_lte: date_lte.map(String::from),
            ticker: ticker.map(String::from),
            ticker_any_of: ticker_any_of.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            last_updated: last_updated.map(String::from),
            last_updated_any_of: last_updated_any_of.map(String::from),
            last_updated_gt: last_updated_gt.map(String::from),
            last_updated_gte: last_updated_gte.map(String::from),
            last_updated_lt: last_updated_lt.map(String::from),
            last_updated_lte: last_updated_lte.map(String::from),
            firm: firm.map(String::from),
            firm_any_of: firm_any_of.map(String::from),
            firm_gt: firm_gt.map(String::from),
            firm_gte: firm_gte.map(String::from),
            firm_lt: firm_lt.map(String::from),
            firm_lte: firm_lte.map(String::from),
            rating_action: rating_action.map(String::from),
            rating_action_any_of: rating_action_any_of.map(String::from),
            rating_action_gt: rating_action_gt.map(String::from),
            rating_action_gte: rating_action_gte.map(String::from),
            rating_action_lt: rating_action_lt.map(String::from),
            rating_action_lte: rating_action_lte.map(String::from),
            benzinga_firm_id: benzinga_firm_id.map(String::from),
            benzinga_firm_id_any_of: benzinga_firm_id_any_of.map(String::from),
            benzinga_firm_id_gt: benzinga_firm_id_gt.map(String::from),
            benzinga_firm_id_gte: benzinga_firm_id_gte.map(String::from),
            benzinga_firm_id_lt: benzinga_firm_id_lt.map(String::from),
            benzinga_firm_id_lte: benzinga_firm_id_lte.map(String::from),
            benzinga_rating_id: benzinga_rating_id.map(String::from),
            benzinga_rating_id_any_of: benzinga_rating_id_any_of.map(String::from),
            benzinga_rating_id_gt: benzinga_rating_id_gt.map(String::from),
            benzinga_rating_id_gte: benzinga_rating_id_gte.map(String::from),
            benzinga_rating_id_lt: benzinga_rating_id_lt.map(String::from),
            benzinga_rating_id_lte: benzinga_rating_id_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_benzinga_analyst_insights_with_params<'a>(
        &'a self,
        params: ListBenzingaAnalystInsightsParams,
    ) -> BoxStream<'a, BenzingaAnalystInsight> {
        Box::pin({
            let ListBenzingaAnalystInsightsParams {
                date,
                date_any_of,
                date_gt,
                date_gte,
                date_lt,
                date_lte,
                ticker,
                ticker_any_of,
                ticker_gt,
                ticker_gte,
                ticker_lt,
                ticker_lte,
                last_updated,
                last_updated_any_of,
                last_updated_gt,
                last_updated_gte,
                last_updated_lt,
                last_updated_lte,
                firm,
                firm_any_of,
                firm_gt,
                firm_gte,
                firm_lt,
                firm_lte,
                rating_action,
                rating_action_any_of,
                rating_action_gt,
                rating_action_gte,
                rating_action_lt,
                rating_action_lte,
                benzinga_firm_id,
                benzinga_firm_id_any_of,
                benzinga_firm_id_gt,
                benzinga_firm_id_gte,
                benzinga_firm_id_lt,
                benzinga_firm_id_lte,
                benzinga_rating_id,
                benzinga_rating_id_any_of,
                benzinga_rating_id_gt,
                benzinga_rating_id_gte,
                benzinga_rating_id_lt,
                benzinga_rating_id_lte,
                limit,
                sort,
                options,
            } = params;
            let date = date.as_deref();
            let date_any_of = date_any_of.as_deref();
            let date_gt = date_gt.as_deref();
            let date_gte = date_gte.as_deref();
            let date_lt = date_lt.as_deref();
            let date_lte = date_lte.as_deref();
            let ticker = ticker.as_deref();
            let ticker_any_of = ticker_any_of.as_deref();
            let ticker_gt = ticker_gt.as_deref();
            let ticker_gte = ticker_gte.as_deref();
            let ticker_lt = ticker_lt.as_deref();
            let ticker_lte = ticker_lte.as_deref();
            let last_updated = last_updated.as_deref();
            let last_updated_any_of = last_updated_any_of.as_deref();
            let last_updated_gt = last_updated_gt.as_deref();
            let last_updated_gte = last_updated_gte.as_deref();
            let last_updated_lt = last_updated_lt.as_deref();
            let last_updated_lte = last_updated_lte.as_deref();
            let firm = firm.as_deref();
            let firm_any_of = firm_any_of.as_deref();
            let firm_gt = firm_gt.as_deref();
            let firm_gte = firm_gte.as_deref();
            let firm_lt = firm_lt.as_deref();
            let firm_lte = firm_lte.as_deref();
            let rating_action = rating_action.as_deref();
            let rating_action_any_of = rating_action_any_of.as_deref();
            let rating_action_gt = rating_action_gt.as_deref();
            let rating_action_gte = rating_action_gte.as_deref();
            let rating_action_lt = rating_action_lt.as_deref();
            let rating_action_lte = rating_action_lte.as_deref();
            let benzinga_firm_id = benzinga_firm_id.as_deref();
            let benzinga_firm_id_any_of = benzinga_firm_id_any_of.as_deref();
            let benzinga_firm_id_gt = benzinga_firm_id_gt.as_deref();
            let benzinga_firm_id_gte = benzinga_firm_id_gte.as_deref();
            let benzinga_firm_id_lt = benzinga_firm_id_lt.as_deref();
            let benzinga_firm_id_lte = benzinga_firm_id_lte.as_deref();
            let benzinga_rating_id = benzinga_rating_id.as_deref();
            let benzinga_rating_id_any_of = benzinga_rating_id_any_of.as_deref();
            let benzinga_rating_id_gt = benzinga_rating_id_gt.as_deref();
            let benzinga_rating_id_gte = benzinga_rating_id_gte.as_deref();
            let benzinga_rating_id_lt = benzinga_rating_id_lt.as_deref();
            let benzinga_rating_id_lte = benzinga_rating_id_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/benzinga/v1/analyst-insights";
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = date {
                query.push(("date", v.to_string()));
            }
            if let Some(v) = date_any_of {
                query.push(("date.any_of", v.to_string()));
            }
            if let Some(v) = date_gt {
                query.push(("date.gt", v.to_string()));
            }
            if let Some(v) = date_gte {
                query.push(("date.gte", v.to_string()));
            }
            if let Some(v) = date_lt {
                query.push(("date.lt", v.to_string()));
            }
            if let Some(v) = date_lte {
                query.push(("date.lte", v.to_string()));
            }
            if let Some(v) = ticker {
                query.push(("ticker", v.to_string()));
            }
            if let Some(v) = ticker_any_of {
                query.push(("ticker.any_of", v.to_string()));
            }
            if let Some(v) = ticker_gt {
                query.push(("ticker.gt", v.to_string()));
            }
            if let Some(v) = ticker_gte {
                query.push(("ticker.gte", v.to_string()));
            }
            if let Some(v) = ticker_lt {
                query.push(("ticker.lt", v.to_string()));
            }
            if let Some(v) = ticker_lte {
                query.push(("ticker.lte", v.to_string()));
            }
            if let Some(v) = last_updated {
                query.push(("last_updated", v.to_string()));
            }
            if let Some(v) = last_updated_any_of {
                query.push(("last_updated.any_of", v.to_string()));
            }
            if let Some(v) = last_updated_gt {
                query.push(("last_updated.gt", v.to_string()));
            }
            if let Some(v) = last_updated_gte {
                query.push(("last_updated.gte", v.to_string()));
            }
            if let Some(v) = last_updated_lt {
                query.push(("last_updated.lt", v.to_string()));
            }
            if let Some(v) = last_updated_lte {
                query.push(("last_updated.lte", v.to_string()));
            }
            if let Some(v) = firm {
                query.push(("firm", v.to_string()));
            }
            if let Some(v) = firm_any_of {
                query.push(("firm.any_of", v.to_string()));
            }
            if let Some(v) = firm_gt {
                query.push(("firm.gt", v.to_string()));
            }
            if let Some(v) = firm_gte {
                query.push(("firm.gte", v.to_string()));
            }
            if let Some(v) = firm_lt {
                query.push(("firm.lt", v.to_string()));
            }
            if let Some(v) = firm_lte {
                query.push(("firm.lte", v.to_string()));
            }
            if let Some(v) = rating_action {
                query.push(("rating_action", v.to_string()));
            }
            if let Some(v) = rating_action_any_of {
                query.push(("rating_action.any_of", v.to_string()));
            }
            if let Some(v) = rating_action_gt {
                query.push(("rating_action.gt", v.to_string()));
            }
            if let Some(v) = rating_action_gte {
                query.push(("rating_action.gte", v.to_string()));
            }
            if let Some(v) = rating_action_lt {
                query.push(("rating_action.lt", v.to_string()));
            }
            if let Some(v) = rating_action_lte {
                query.push(("rating_action.lte", v.to_string()));
            }
            if let Some(v) = benzinga_firm_id {
                query.push(("benzinga_firm_id", v.to_string()));
            }
            if let Some(v) = benzinga_firm_id_any_of {
                query.push(("benzinga_firm_id.any_of", v.to_string()));
            }
            if let Some(v) = benzinga_firm_id_gt {
                query.push(("benzinga_firm_id.gt", v.to_string()));
            }
            if let Some(v) = benzinga_firm_id_gte {
                query.push(("benzinga_firm_id.gte", v.to_string()));
            }
            if let Some(v) = benzinga_firm_id_lt {
                query.push(("benzinga_firm_id.lt", v.to_string()));
            }
            if let Some(v) = benzinga_firm_id_lte {
                query.push(("benzinga_firm_id.lte", v.to_string()));
            }
            if let Some(v) = benzinga_rating_id {
                query.push(("benzinga_rating_id", v.to_string()));
            }
            if let Some(v) = benzinga_rating_id_any_of {
                query.push(("benzinga_rating_id.any_of", v.to_string()));
            }
            if let Some(v) = benzinga_rating_id_gt {
                query.push(("benzinga_rating_id.gt", v.to_string()));
            }
            if let Some(v) = benzinga_rating_id_gte {
                query.push(("benzinga_rating_id.gte", v.to_string()));
            }
            if let Some(v) = benzinga_rating_id_lt {
                query.push(("benzinga_rating_id.lt", v.to_string()));
            }
            if let Some(v) = benzinga_rating_id_lte {
                query.push(("benzinga_rating_id.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<BenzingaAnalystInsight>(path, Some(&query), options)
        })
    }

    fn list_benzinga_analysts<'a>(
        &'a self,
        benzinga_id: Option<&'a str>,
        benzinga_id_any_of: Option<&'a str>,
        benzinga_id_gt: Option<&'a str>,
        benzinga_id_gte: Option<&'a str>,
        benzinga_id_lt: Option<&'a str>,
        benzinga_id_lte: Option<&'a str>,
        benzinga_firm_id: Option<&'a str>,
        benzinga_firm_id_any_of: Option<&'a str>,
        benzinga_firm_id_gt: Option<&'a str>,
        benzinga_firm_id_gte: Option<&'a str>,
        benzinga_firm_id_lt: Option<&'a str>,
        benzinga_firm_id_lte: Option<&'a str>,
        firm_name: Option<&'a str>,
        firm_name_any_of: Option<&'a str>,
        firm_name_gt: Option<&'a str>,
        firm_name_gte: Option<&'a str>,
        firm_name_lt: Option<&'a str>,
        firm_name_lte: Option<&'a str>,
        full_name: Option<&'a str>,
        full_name_any_of: Option<&'a str>,
        full_name_gt: Option<&'a str>,
        full_name_gte: Option<&'a str>,
        full_name_lt: Option<&'a str>,
        full_name_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaAnalyst> {
        self.list_benzinga_analysts_with_params(ListBenzingaAnalystsParams {
            benzinga_id: benzinga_id.map(String::from),
            benzinga_id_any_of: benzinga_id_any_of.map(String::from),
            benzinga_id_gt: benzinga_id_gt.map(String::from),
            benzinga_id_gte: benzinga_id_gte.map(String::from),
            benzinga_id_lt: benzinga_id_lt.map(String::from),
            benzinga_id_lte: benzinga_id_lte.map(String::from),
            benzinga_firm_id: benzinga_firm_id.map(String::from),
            benzinga_firm_id_any_of: benzinga_firm_id_any_of.map(String::from),
            benzinga_firm_id_gt: benzinga_firm_id_gt.map(String::from),
            benzinga_firm_id_gte: benzinga_firm_id_gte.map(String::from),
            benzinga_firm_id_lt: benzinga_firm_id_lt.map(String::from),
            benzinga_firm_id_lte: benzinga_firm_id_lte.map(String::from),
            firm_name: firm_name.map(String::from),
            firm_name_any_of: firm_name_any_of.map(String::from),
            firm_name_gt: firm_name_gt.map(String::from),
            firm_name_gte: firm_name_gte.map(String::from),
            firm_name_lt: firm_name_lt.map(String::from),
            firm_name_lte: firm_name_lte.map(String::from),
            full_name: full_name.map(String::from),
            full_name_any_of: full_name_any_of.map(String::from),
            full_name_gt: full_name_gt.map(String::from),
            full_name_gte: full_name_gte.map(String::from),
            full_name_lt: full_name_lt.map(String::from),
            full_name_lte: full_name_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_benzinga_analysts_with_params<'a>(
        &'a self,
        params: ListBenzingaAnalystsParams,
    ) -> BoxStream<'a, BenzingaAnalyst> {
        Box::pin({
            let ListBenzingaAnalystsParams {
                benzinga_id,
                benzinga_id_any_of,
                benzinga_id_gt,
                benzinga_id_gte,
                benzinga_id_lt,
                benzinga_id_lte,
                benzinga_firm_id,
                benzinga_firm_id_any_of,
                benzinga_firm_id_gt,
                benzinga_firm_id_gte,
                benzinga_firm_id_lt,
                benzinga_firm_id_lte,
                firm_name,
                firm_name_any_of,
                firm_name_gt,
                firm_name_gte,
                firm_name_lt,
                firm_name_lte,
                full_name,
                full_name_any_of,
                full_name_gt,
                full_name_gte,
                full_name_lt,
                full_name_lte,
                limit,
                sort,
                options,
            } = params;
            let benzinga_id = benzinga_id.as_deref();
            let benzinga_id_any_of = benzinga_id_any_of.as_deref();
            let benzinga_id_gt = benzinga_id_gt.as_deref();
            let benzinga_id_gte = benzinga_id_gte.as_deref();
            let benzinga_id_lt = benzinga_id_lt.as_deref();
            let benzinga_id_lte = benzinga_id_lte.as_deref();
            let benzinga_firm_id = benzinga_firm_id.as_deref();
            let benzinga_firm_id_any_of = benzinga_firm_id_any_of.as_deref();
            let benzinga_firm_id_gt = benzinga_firm_id_gt.as_deref();
            let benzinga_firm_id_gte = benzinga_firm_id_gte.as_deref();
            let benzinga_firm_id_lt = benzinga_firm_id_lt.as_deref();
            let benzinga_firm_id_lte = benzinga_firm_id_lte.as_deref();
            let firm_name = firm_name.as_deref();
            let firm_name_any_of = firm_name_any_of.as_deref();
            let firm_name_gt = firm_name_gt.as_deref();
            let firm_name_gte = firm_name_gte.as_deref();
            let firm_name_lt = firm_name_lt.as_deref();
            let firm_name_lte = firm_name_lte.as_deref();
            let full_name = full_name.as_deref();
            let full_name_any_of = full_name_any_of.as_deref();
            let full_name_gt = full_name_gt.as_deref();
            let full_name_gte = full_name_gte.as_deref();
            let full_name_lt = full_name_lt.as_deref();
            let full_name_lte = full_name_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/benzinga/v1/analysts";
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = benzinga_id {
                query.push(("benzinga_id", v.to_string()));
            }
            if let Some(v) = benzinga_id_any_of {
                query.push(("benzinga_id.any_of", v.to_string()));
            }
            if let Some(v) = benzinga_id_gt {
                query.push(("benzinga_id.gt", v.to_string()));
            }
            if let Some(v) = benzinga_id_gte {
                query.push(("benzinga_id.gte", v.to_string()));
            }
            if let Some(v) = benzinga_id_lt {
                query.push(("benzinga_id.lt", v.to_string()));
            }
            if let Some(v) = benzinga_id_lte {
                query.push(("benzinga_id.lte", v.to_string()));
            }
            if let Some(v) = benzinga_firm_id {
                query.push(("benzinga_firm_id", v.to_string()));
            }
            if let Some(v) = benzinga_firm_id_any_of {
                query.push(("benzinga_firm_id.any_of", v.to_string()));
            }
            if let Some(v) = benzinga_firm_id_gt {
                query.push(("benzinga_firm_id.gt", v.to_string()));
            }
            if let Some(v) = benzinga_firm_id_gte {
                query.push(("benzinga_firm_id.gte", v.to_string()));
            }
            if let Some(v) = benzinga_firm_id_lt {
                query.push(("benzinga_firm_id.lt", v.to_string()));
            }
            if let Some(v) = benzinga_firm_id_lte {
                query.push(("benzinga_firm_id.lte", v.to_string()));
            }
            if let Some(v) = firm_name {
                query.push(("firm_name", v.to_string()));
            }
            if let Some(v) = firm_name_any_of {
                query.push(("firm_name.any_of", v.to_string()));
            }
            if let Some(v) = firm_name_gt {
                query.push(("firm_name.gt", v.to_string()));
            }
            if let Some(v) = firm_name_gte {
                query.push(("firm_name.gte", v.to_string()));
            }
            if let Some(v) = firm_name_lt {
                query.push(("firm_name.lt", v.to_string()));
            }
            if let Some(v) = firm_name_lte {
                query.push(("firm_name.lte", v.to_string()));
            }
            if let Some(v) = full_name {
                query.push(("full_name", v.to_string()));
            }
            if let Some(v) = full_name_any_of {
                query.push(("full_name.any_of", v.to_string()));
            }
            if let Some(v) = full_name_gt {
                query.push(("full_name.gt", v.to_string()));
            }
            if let Some(v) = full_name_gte {
                query.push(("full_name.gte", v.to_string()));
            }
            if let Some(v) = full_name_lt {
                query.push(("full_name.lt", v.to_string()));
            }
            if let Some(v) = full_name_lte {
                query.push(("full_name.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<BenzingaAnalyst>(path, Some(&query), options)
        })
    }

    fn list_benzinga_consensus_ratings<'a>(
        &'a self,
        ticker: &'a str,
        date: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        limit: Option<i64>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaConsensusRating> {
        self.list_benzinga_consensus_ratings_with_params(
            ticker,
            ListBenzingaConsensusRatingsParams {
                date: date.map(String::from),
                date_gt: date_gt.map(String::from),
                date_gte: date_gte.map(String::from),
                date_lt: date_lt.map(String::from),
                date_lte: date_lte.map(String::from),
                limit,
                options: options.cloned(),
            },
        )
    }

    fn list_benzinga_consensus_ratings_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: ListBenzingaConsensusRatingsParams,
    ) -> BoxStream<'a, BenzingaConsensusRating> {
        Box::pin({
            let ListBenzingaConsensusRatingsParams {
                date,
                date_gt,
                date_gte,
                date_lt,
                date_lte,
                limit,
                options,
            } = params;
            let date = date.as_deref();
            let date_gt = date_gt.as_deref();
            let date_gte = date_gte.as_deref();
            let date_lt = date_lt.as_deref();
            let date_lte = date_lte.as_deref();
            let options = options.as_ref();
            let path = format!("/benzinga/v1/consensus-ratings/{}", ticker);
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = date {
                query.push(("date", v.to_string()));
            }
            if let Some(v) = date_gt {
                query.push(("date.gt", v.to_string()));
            }
            if let Some(v) = date_gte {
                query.push(("date.gte", v.to_string()));
            }
            if let Some(v) = date_lt {
                query.push(("date.lt", v.to_string()));
            }
            if let Some(v) = date_lte {
                query.push(("date.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            self.list::<BenzingaConsensusRating>(&path, Some(&query), options)
        })
    }

    fn list_benzinga_earnings<'a>(
        &'a self,
        date: Option<&'a str>,
        date_any_of: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        importance: Option<i64>,
        importance_any_of: Option<&'a str>,
        importance_gt: Option<i64>,
        importance_gte: Option<i64>,
        importance_lt: Option<i64>,
        importance_lte: Option<i64>,
        last_updated: Option<&'a str>,
        last_updated_any_of: Option<&'a str>,
        last_updated_gt: Option<&'a str>,
        last_updated_gte: Option<&'a str>,
        last_updated_lt: Option<&'a str>,
        last_updated_lte: Option<&'a str>,
        date_status: Option<&'a str>,
        date_status_any_of: Option<&'a str>,
        date_status_gt: Option<&'a str>,
        date_status_gte: Option<&'a str>,
        date_status_lt: Option<&'a str>,
        date_status_lte: Option<&'a str>,
        eps_surprise_percent: Option<f64>,
        eps_surprise_percent_any_of: Option<&'a str>,
        eps_surprise_percent_gt: Option<f64>,
        eps_surprise_percent_gte: Option<f64>,
        eps_surprise_percent_lt: Option<f64>,
        eps_surprise_percent_lte: Option<f64>,
        revenue_surprise_percent: Option<f64>,
        revenue_surprise_percent_any_of: Option<&'a str>,
        revenue_surprise_percent_gt: Option<f64>,
        revenue_surprise_percent_gte: Option<f64>,
        revenue_surprise_percent_lt: Option<f64>,
        revenue_surprise_percent_lte: Option<f64>,
        fiscal_year: Option<i64>,
        fiscal_year_any_of: Option<&'a str>,
        fiscal_year_gt: Option<i64>,
        fiscal_year_gte: Option<i64>,
        fiscal_year_lt: Option<i64>,
        fiscal_year_lte: Option<i64>,
        fiscal_period: Option<&'a str>,
        fiscal_period_any_of: Option<&'a str>,
        fiscal_period_gt: Option<&'a str>,
        fiscal_period_gte: Option<&'a str>,
        fiscal_period_lt: Option<&'a str>,
        fiscal_period_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaEarning> {
        self.list_benzinga_earnings_with_params(ListBenzingaEarningsParams {
            date: date.map(String::from),
            date_any_of: date_any_of.map(String::from),
            date_gt: date_gt.map(String::from),
            date_gte: date_gte.map(String::from),
            date_lt: date_lt.map(String::from),
            date_lte: date_lte.map(String::from),
            ticker: ticker.map(String::from),
            ticker_any_of: ticker_any_of.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            importance,
            importance_any_of: importance_any_of.map(String::from),
            importance_gt,
            importance_gte,
            importance_lt,
            importance_lte,
            last_updated: last_updated.map(String::from),
            last_updated_any_of: last_updated_any_of.map(String::from),
            last_updated_gt: last_updated_gt.map(String::from),
            last_updated_gte: last_updated_gte.map(String::from),
            last_updated_lt: last_updated_lt.map(String::from),
            last_updated_lte: last_updated_lte.map(String::from),
            date_status: date_status.map(String::from),
            date_status_any_of: date_status_any_of.map(String::from),
            date_status_gt: date_status_gt.map(String::from),
            date_status_gte: date_status_gte.map(String::from),
            date_status_lt: date_status_lt.map(String::from),
            date_status_lte: date_status_lte.map(String::from),
            eps_surprise_percent,
            eps_surprise_percent_any_of: eps_surprise_percent_any_of.map(String::from),
            eps_surprise_percent_gt,
            eps_surprise_percent_gte,
            eps_surprise_percent_lt,
            eps_surprise_percent_lte,
            revenue_surprise_percent,
            revenue_surprise_percent_any_of: revenue_surprise_percent_any_of.map(String::from),
            revenue_surprise_percent_gt,
            revenue_surprise_percent_gte,
            revenue_surprise_percent_lt,
            revenue_surprise_percent_lte,
            fiscal_year,
            fiscal_year_any_of: fiscal_year_any_of.map(String::from),
            fiscal_year_gt,
            fiscal_year_gte,
            fiscal_year_lt,
            fiscal_year_lte,
            fiscal_period: fiscal_period.map(String::from),
            fiscal_period_any_of: fiscal_period_any_of.map(String::from),
            fiscal_period_gt: fiscal_period_gt.map(String::from),
            fiscal_period_gte: fiscal_period_gte.map(String::from),
            fiscal_period_lt: fiscal_period_lt.map(String::from),
            fiscal_period_lte: fiscal_period_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_benzinga_earnings_with_params<'a>(
        &'a self,
        params: ListBenzingaEarningsParams,
    ) -> BoxStream<'a, BenzingaEarning> {
        Box::pin({
            let ListBenzingaEarningsParams {
                date,
                date_any_of,
                date_gt,
                date_gte,
                date_lt,
                date_lte,
                ticker,
                ticker_any_of,
                ticker_gt,
                ticker_gte,
                ticker_lt,
                ticker_lte,
                importance,
                importance_any_of,
                importance_gt,
                importance_gte,
                importance_lt,
                importance_lte,
                last_updated,
                last_updated_any_of,
                last_updated_gt,
                last_updated_gte,
                last_updated_lt,
                last_updated_lte,
                date_status,
                date_status_any_of,
                date_status_gt,
                date_status_gte,
                date_status_lt,
                date_status_lte,
                eps_surprise_percent,
                eps_surprise_percent_any_of,
                eps_surprise_percent_gt,
                eps_surprise_percent_gte,
                eps_surprise_percent_lt,
                eps_surprise_percent_lte,
                revenue_surprise_percent,
                revenue_surprise_percent_any_of,
                revenue_surprise_percent_gt,
                revenue_surprise_percent_gte,
                revenue_surprise_percent_lt,
                revenue_surprise_percent_lte,
                fiscal_year,
                fiscal_year_any_of,
                fiscal_year_gt,
                fiscal_year_gte,
                fiscal_year_lt,
                fiscal_year_lte,
                fiscal_period,
                fiscal_period_any_of,
                fiscal_period_gt,
                fiscal_period_gte,
                fiscal_period_lt,
                fiscal_period_lte,
                limit,
                sort,
                options,
            } = params;
            let date = date.as_deref();
            let date_any_of = date_any_of.as_deref();
            let date_gt = date_gt.as_deref();
            let date_gte = date_gte.as_deref();
            let date_lt = date_lt.as_deref();
            let date_lte = date_lte.as_deref();
            let ticker = ticker.as_deref();
            let ticker_any_of = ticker_any_of.as_deref();
            let ticker_gt = ticker_gt.as_deref();
            let ticker_gte = ticker_gte.as_deref();
            let ticker_lt = ticker_lt.as_deref();
            let ticker_lte = ticker_lte.as_deref();
            let importance_any_of = importance_any_of.as_deref();
            let last_updated = last_updated.as_deref();
            let last_updated_any_of = last_updated_any_of.as_deref();
            let last_updated_gt = last_updated_gt.as_deref();
            let last_updated_gte = last_updated_gte.as_deref();
            let last_updated_lt = last_updated_lt.as_deref();
            let last_updated_lte = last_updated_lte.as_deref();
            let date_status = date_status.as_deref();
            let date_status_any_of = date_status_any_of.as_deref();
            let date_status_gt = date_status_gt.as_deref();
            let date_status_gte = date_status_gte.as_deref();
            let date_status_lt = date_status_lt.as_deref();
            let date_status_lte = date_status_lte.as_deref();
            let eps_surprise_percent_any_of = eps_surprise_percent_any_of.as_deref();
            let revenue_surprise_percent_any_of = revenue_surprise_percent_any_of.as_deref();
            let fiscal_year_any_of = fiscal_year_any_of.as_deref();
            let fiscal_period = fiscal_period.as_deref();
            let fiscal_period_any_of = fiscal_period_any_of.as_deref();
            let fiscal_period_gt = fiscal_period_gt.as_deref();
            let fiscal_period_gte = fiscal_period_gte.as_deref();
            let fiscal_period_lt = fiscal_period_lt.as_deref();
            let fiscal_period_lte = fiscal_period_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/benzinga/v1/earnings";
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = date {
                query.push(("date", v.to_string()));
            }
            if let Some(v) = date_any_of {
                query.push(("date.any_of", v.to_string()));
            }
            if let Some(v) = date_gt {
                query.push(("date.gt", v.to_string()));
            }
            if let Some(v) = date_gte {
                query.push(("date.gte", v.to_string()));
            }
            if let Some(v) = date_lt {
                query.push(("date.lt", v.to_string()));
            }
            if let Some(v) = date_lte {
                query.push(("date.lte", v.to_string()));
            }
            if let Some(v) = ticker {
                query.push(("ticker", v.to_string()));
            }
            if let Some(v) = ticker_any_of {
                query.push(("ticker.any_of", v.to_string()));
            }
            if let Some(v) = ticker_gt {
                query.push(("ticker.gt", v.to_string()));
            }
            if let Some(v) = ticker_gte {
                query.push(("ticker.gte", v.to_string()));
            }
            if let Some(v) = ticker_lt {
                query.push(("ticker.lt", v.to_string()));
            }
            if let Some(v) = ticker_lte {
                query.push(("ticker.lte", v.to_string()));
            }
            if let Some(v) = importance {
                query.push(("importance", v.to_string()));
            }
            if let Some(v) = importance_any_of {
                query.push(("importance.any_of", v.to_string()));
            }
            if let Some(v) = importance_gt {
                query.push(("importance.gt", v.to_string()));
            }
            if let Some(v) = importance_gte {
                query.push(("importance.gte", v.to_string()));
            }
            if let Some(v) = importance_lt {
                query.push(("importance.lt", v.to_string()));
            }
            if let Some(v) = importance_lte {
                query.push(("importance.lte", v.to_string()));
            }
            if let Some(v) = last_updated {
                query.push(("last_updated", v.to_string()));
            }
            if let Some(v) = last_updated_any_of {
                query.push(("last_updated.any_of", v.to_string()));
            }
            if let Some(v) = last_updated_gt {
                query.push(("last_updated.gt", v.to_string()));
            }
            if let Some(v) = last_updated_gte {
                query.push(("last_updated.gte", v.to_string()));
            }
            if let Some(v) = last_updated_lt {
                query.push(("last_updated.lt", v.to_string()));
            }
            if let Some(v) = last_updated_lte {
                query.push(("last_updated.lte", v.to_string()));
            }
            if let Some(v) = date_status {
                query.push(("date_status", v.to_string()));
            }
            if let Some(v) = date_status_any_of {
                query.push(("date_status.any_of", v.to_string()));
            }
            if let Some(v) = date_status_gt {
                query.push(("date_status.gt", v.to_string()));
            }
            if let Some(v) = date_status_gte {
                query.push(("date_status.gte", v.to_string()));
            }
            if let Some(v) = date_status_lt {
                query.push(("date_status.lt", v.to_string()));
            }
            if let Some(v) = date_status_lte {
                query.push(("date_status.lte", v.to_string()));
            }
            if let Some(v) = eps_surprise_percent {
                query.push(("eps_surprise_percent", v.to_string()));
            }
            if let Some(v) = eps_surprise_percent_any_of {
                query.push(("eps_surprise_percent.any_of", v.to_string()));
            }
            if let Some(v) = eps_surprise_percent_gt {
                query.push(("eps_surprise_percent.gt", v.to_string()));
            }
            if let Some(v) = eps_surprise_percent_gte {
                query.push(("eps_surprise_percent.gte", v.to_string()));
            }
            if let Some(v) = eps_surprise_percent_lt {
                query.push(("eps_surprise_percent.lt", v.to_string()));
            }
            if let Some(v) = eps_surprise_percent_lte {
                query.push(("eps_surprise_percent.lte", v.to_string()));
            }
            if let Some(v) = revenue_surprise_percent {
                query.push(("revenue_surprise_percent", v.to_string()));
            }
            if let Some(v) = revenue_surprise_percent_any_of {
                query.push(("revenue_surprise_percent.any_of", v.to_string()));
            }
            if let Some(v) = revenue_surprise_percent_gt {
                query.push(("revenue_surprise_percent.gt", v.to_string()));
            }
            if let Some(v) = revenue_surprise_percent_gte {
                query.push(("revenue_surprise_percent.gte", v.to_string()));
            }
            if let Some(v) = revenue_surprise_percent_lt {
                query.push(("revenue_surprise_percent.lt", v.to_string()));
            }
            if let Some(v) = revenue_surprise_percent_lte {
                query.push(("revenue_surprise_percent.lte", v.to_string()));
            }
            if let Some(v) = fiscal_year {
                query.push(("fiscal_year", v.to_string()));
            }
            if let Some(v) = fiscal_year_any_of {
                query.push(("fiscal_year.any_of", v.to_string()));
            }
            if let Some(v) = fiscal_year_gt {
                query.push(("fiscal_year.gt", v.to_string()));
            }
            if let Some(v) = fiscal_year_gte {
                query.push(("fiscal_year.gte", v.to_string()));
            }
            if let Some(v) = fiscal_year_lt {
                query.push(("fiscal_year.lt", v.to_string()));
            }
            if let Some(v) = fiscal_year_lte {
                query.push(("fiscal_year.lte", v.to_string()));
            }
            if let Some(v) = fiscal_period {
                query.push(("fiscal_period", v.to_string()));
            }
            if let Some(v) = fiscal_period_any_of {
                query.push(("fiscal_period.any_of", v.to_string()));
            }
            if let Some(v) = fiscal_period_gt {
                query.push(("fiscal_period.gt", v.to_string()));
            }
            if let Some(v) = fiscal_period_gte {
                query.push(("fiscal_period.gte", v.to_string()));
            }
            if let Some(v) = fiscal_period_lt {
                query.push(("fiscal_period.lt", v.to_string()));
            }
            if let Some(v) = fiscal_period_lte {
                query.push(("fiscal_period.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<BenzingaEarning>(path, Some(&query), options)
        })
    }

    fn list_benzinga_firms<'a>(
        &'a self,
        benzinga_id: Option<&'a str>,
        benzinga_id_any_of: Option<&'a str>,
        benzinga_id_gt: Option<&'a str>,
        benzinga_id_gte: Option<&'a str>,
        benzinga_id_lt: Option<&'a str>,
        benzinga_id_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaFirm> {
        self.list_benzinga_firms_with_params(ListBenzingaFirmsParams {
            benzinga_id: benzinga_id.map(String::from),
            benzinga_id_any_of: benzinga_id_any_of.map(String::from),
            benzinga_id_gt: benzinga_id_gt.map(String::from),
            benzinga_id_gte: benzinga_id_gte.map(String::from),
            benzinga_id_lt: benzinga_id_lt.map(String::from),
            benzinga_id_lte: benzinga_id_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_benzinga_firms_with_params<'a>(
        &'a self,
        params: ListBenzingaFirmsParams,
    ) -> BoxStream<'a, BenzingaFirm> {
        Box::pin({
            let ListBenzingaFirmsParams {
                benzinga_id,
                benzinga_id_any_of,
                benzinga_id_gt,
                benzinga_id_gte,
                benzinga_id_lt,
                benzinga_id_lte,
                limit,
                sort,
                options,
            } = params;
            let benzinga_id = benzinga_id.as_deref();
            let benzinga_id_any_of = benzinga_id_any_of.as_deref();
            let benzinga_id_gt = benzinga_id_gt.as_deref();
            let benzinga_id_gte = benzinga_id_gte.as_deref();
            let benzinga_id_lt = benzinga_id_lt.as_deref();
            let benzinga_id_lte = benzinga_id_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/benzinga/v1/firms";
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = benzinga_id {
                query.push(("benzinga_id", v.to_string()));
            }
            if let Some(v) = benzinga_id_any_of {
                query.push(("benzinga_id.any_of", v.to_string()));
            }
            if let Some(v) = benzinga_id_gt {
                query.push(("benzinga_id.gt", v.to_string()));
            }
            if let Some(v) = benzinga_id_gte {
                query.push(("benzinga_id.gte", v.to_string()));
            }
            if let Some(v) = benzinga_id_lt {
                query.push(("benzinga_id.lt", v.to_string()));
            }
            if let Some(v) = benzinga_id_lte {
                query.push(("benzinga_id.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<BenzingaFirm>(path, Some(&query), options)
        })
    }

    fn list_benzinga_guidance<'a>(
        &'a self,
        date: Option<&'a str>,
        date_any_of: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        positioning: Option<&'a str>,
        positioning_any_of: Option<&'a str>,
        positioning_gt: Option<&'a str>,
        positioning_gte: Option<&'a str>,
        positioning_lt: Option<&'a str>,
        positioning_lte: Option<&'a str>,
        importance: Option<i64>,
        importance_any_of: Option<&'a str>,
        importance_gt: Option<i64>,
        importance_gte: Option<i64>,
        importance_lt: Option<i64>,
        importance_lte: Option<i64>,
        last_updated: Option<&'a str>,
        last_updated_any_of: Option<&'a str>,
        last_updated_gt: Option<&'a str>,
        last_updated_gte: Option<&'a str>,
        last_updated_lt: Option<&'a str>,
        last_updated_lte: Option<&'a str>,
        fiscal_year: Option<i64>,
        fiscal_year_any_of: Option<&'a str>,
        fiscal_year_gt: Option<i64>,
        fiscal_year_gte: Option<i64>,
        fiscal_year_lt: Option<i64>,
        fiscal_year_lte: Option<i64>,
        fiscal_period: Option<&'a str>,
        fiscal_period_any_of: Option<&'a str>,
        fiscal_period_gt: Option<&'a str>,
        fiscal_period_gte: Option<&'a str>,
        fiscal_period_lt: Option<&'a str>,
        fiscal_period_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaGuidance> {
        self.list_benzinga_guidance_with_params(ListBenzingaGuidanceParams {
            date: date.map(String::from),
            date_any_of: date_any_of.map(String::from),
            date_gt: date_gt.map(String::from),
            date_gte: date_gte.map(String::from),
            date_lt: date_lt.map(String::from),
            date_lte: date_lte.map(String::from),
            ticker: ticker.map(String::from),
            ticker_any_of: ticker_any_of.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            positioning: positioning.map(String::from),
            positioning_any_of: positioning_any_of.map(String::from),
            positioning_gt: positioning_gt.map(String::from),
            positioning_gte: positioning_gte.map(String::from),
            positioning_lt: positioning_lt.map(String::from),
            positioning_lte: positioning_lte.map(String::from),
            importance,
            importance_any_of: importance_any_of.map(String::from),
            importance_gt,
            importance_gte,
            importance_lt,
            importance_lte,
            last_updated: last_updated.map(String::from),
            last_updated_any_of: last_updated_any_of.map(String::from),
            last_updated_gt: last_updated_gt.map(String::from),
            last_updated_gte: last_updated_gte.map(String::from),
            last_updated_lt: last_updated_lt.map(String::from),
            last_updated_lte: last_updated_lte.map(String::from),
            fiscal_year,
            fiscal_year_any_of: fiscal_year_any_of.map(String::from),
            fiscal_year_gt,
            fiscal_year_gte,
            fiscal_year_lt,
            fiscal_year_lte,
            fiscal_period: fiscal_period.map(String::from),
            fiscal_period_any_of: fiscal_period_any_of.map(String::from),
            fiscal_period_gt: fiscal_period_gt.map(String::from),
            fiscal_period_gte: fiscal_period_gte.map(String::from),
            fiscal_period_lt: fiscal_period_lt.map(String::from),
            fiscal_period_lte: fiscal_period_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_benzinga_guidance_with_params<'a>(
        &'a self,
        params: ListBenzingaGuidanceParams,
    ) -> BoxStream<'a, BenzingaGuidance> {
        Box::pin({
            let ListBenzingaGuidanceParams {
                date,
                date_any_of,
                date_gt,
                date_gte,
                date_lt,
                date_lte,
                ticker,
                ticker_any_of,
                ticker_gt,
                ticker_gte,
                ticker_lt,
                ticker_lte,
                positioning,
                positioning_any_of,
                positioning_gt,
                positioning_gte,
                positioning_lt,
                positioning_lte,
                importance,
                importance_any_of,
                importance_gt,
                importance_gte,
                importance_lt,
                importance_lte,
                last_updated,
                last_updated_any_of,
                last_updated_gt,
                last_updated_gte,
                last_updated_lt,
                last_updated_lte,
                fiscal_year,
                fiscal_year_any_of,
                fiscal_year_gt,
                fiscal_year_gte,
                fiscal_year_lt,
                fiscal_year_lte,
                fiscal_period,
                fiscal_period_any_of,
                fiscal_period_gt,
                fiscal_period_gte,
                fiscal_period_lt,
                fiscal_period_lte,
                limit,
                sort,
                options,
            } = params;
            let date = date.as_deref();
            let date_any_of = date_any_of.as_deref();
            let date_gt = date_gt.as_deref();
            let date_gte = date_gte.as_deref();
            let date_lt = date_lt.as_deref();
            let date_lte = date_lte.as_deref();
            let ticker = ticker.as_deref();
            let ticker_any_of = ticker_any_of.as_deref();
            let ticker_gt = ticker_gt.as_deref();
            let ticker_gte = ticker_gte.as_deref();
            let ticker_lt = ticker_lt.as_deref();
            let ticker_lte = ticker_lte.as_deref();
            let positioning = positioning.as_deref();
            let positioning_any_of = positioning_any_of.as_deref();
            let positioning_gt = positioning_gt.as_deref();
            let positioning_gte = positioning_gte.as_deref();
            let positioning_lt = positioning_lt.as_deref();
            let positioning_lte = positioning_lte.as_deref();
            let importance_any_of = importance_any_of.as_deref();
            let last_updated = last_updated.as_deref();
            let last_updated_any_of = last_updated_any_of.as_deref();
            let last_updated_gt = last_updated_gt.as_deref();
            let last_updated_gte = last_updated_gte.as_deref();
            let last_updated_lt = last_updated_lt.as_deref();
            let last_updated_lte = last_updated_lte.as_deref();
            let fiscal_year_any_of = fiscal_year_any_of.as_deref();
            let fiscal_period = fiscal_period.as_deref();
            let fiscal_period_any_of = fiscal_period_any_of.as_deref();
            let fiscal_period_gt = fiscal_period_gt.as_deref();
            let fiscal_period_gte = fiscal_period_gte.as_deref();
            let fiscal_period_lt = fiscal_period_lt.as_deref();
            let fiscal_period_lte = fiscal_period_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/benzinga/v1/guidance";
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = date {
                query.push(("date", v.to_string()));
            }
            if let Some(v) = date_any_of {
                query.push(("date.any_of", v.to_string()));
            }
            if let Some(v) = date_gt {
                query.push(("date.gt", v.to_string()));
            }
            if let Some(v) = date_gte {
                query.push(("date.gte", v.to_string()));
            }
            if let Some(v) = date_lt {
                query.push(("date.lt", v.to_string()));
            }
            if let Some(v) = date_lte {
                query.push(("date.lte", v.to_string()));
            }
            if let Some(v) = ticker {
                query.push(("ticker", v.to_string()));
            }
            if let Some(v) = ticker_any_of {
                query.push(("ticker.any_of", v.to_string()));
            }
            if let Some(v) = ticker_gt {
                query.push(("ticker.gt", v.to_string()));
            }
            if let Some(v) = ticker_gte {
                query.push(("ticker.gte", v.to_string()));
            }
            if let Some(v) = ticker_lt {
                query.push(("ticker.lt", v.to_string()));
            }
            if let Some(v) = ticker_lte {
                query.push(("ticker.lte", v.to_string()));
            }
            if let Some(v) = positioning {
                query.push(("positioning", v.to_string()));
            }
            if let Some(v) = positioning_any_of {
                query.push(("positioning.any_of", v.to_string()));
            }
            if let Some(v) = positioning_gt {
                query.push(("positioning.gt", v.to_string()));
            }
            if let Some(v) = positioning_gte {
                query.push(("positioning.gte", v.to_string()));
            }
            if let Some(v) = positioning_lt {
                query.push(("positioning.lt", v.to_string()));
            }
            if let Some(v) = positioning_lte {
                query.push(("positioning.lte", v.to_string()));
            }
            if let Some(v) = importance {
                query.push(("importance", v.to_string()));
            }
            if let Some(v) = importance_any_of {
                query.push(("importance.any_of", v.to_string()));
            }
            if let Some(v) = importance_gt {
                query.push(("importance.gt", v.to_string()));
            }
            if let Some(v) = importance_gte {
                query.push(("importance.gte", v.to_string()));
            }
            if let Some(v) = importance_lt {
                query.push(("importance.lt", v.to_string()));
            }
            if let Some(v) = importance_lte {
                query.push(("importance.lte", v.to_string()));
            }
            if let Some(v) = last_updated {
                query.push(("last_updated", v.to_string()));
            }
            if let Some(v) = last_updated_any_of {
                query.push(("last_updated.any_of", v.to_string()));
            }
            if let Some(v) = last_updated_gt {
                query.push(("last_updated.gt", v.to_string()));
            }
            if let Some(v) = last_updated_gte {
                query.push(("last_updated.gte", v.to_string()));
            }
            if let Some(v) = last_updated_lt {
                query.push(("last_updated.lt", v.to_string()));
            }
            if let Some(v) = last_updated_lte {
                query.push(("last_updated.lte", v.to_string()));
            }
            if let Some(v) = fiscal_year {
                query.push(("fiscal_year", v.to_string()));
            }
            if let Some(v) = fiscal_year_any_of {
                query.push(("fiscal_year.any_of", v.to_string()));
            }
            if let Some(v) = fiscal_year_gt {
                query.push(("fiscal_year.gt", v.to_string()));
            }
            if let Some(v) = fiscal_year_gte {
                query.push(("fiscal_year.gte", v.to_string()));
            }
            if let Some(v) = fiscal_year_lt {
                query.push(("fiscal_year.lt", v.to_string()));
            }
            if let Some(v) = fiscal_year_lte {
                query.push(("fiscal_year.lte", v.to_string()));
            }
            if let Some(v) = fiscal_period {
                query.push(("fiscal_period", v.to_string()));
            }
            if let Some(v) = fiscal_period_any_of {
                query.push(("fiscal_period.any_of", v.to_string()));
            }
            if let Some(v) = fiscal_period_gt {
                query.push(("fiscal_period.gt", v.to_string()));
            }
            if let Some(v) = fiscal_period_gte {
                query.push(("fiscal_period.gte", v.to_string()));
            }
            if let Some(v) = fiscal_period_lt {
                query.push(("fiscal_period.lt", v.to_string()));
            }
            if let Some(v) = fiscal_period_lte {
                query.push(("fiscal_period.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<BenzingaGuidance>(path, Some(&query), options)
        })
    }

    fn list_benzinga_news<'a>(
        &'a self,
        published: Option<&'a str>,
        published_any_of: Option<&'a str>,
        published_gt: Option<&'a str>,
        published_gte: Option<&'a str>,
        published_lt: Option<&'a str>,
        published_lte: Option<&'a str>,
        last_updated: Option<&'a str>,
        last_updated_any_of: Option<&'a str>,
        last_updated_gt: Option<&'a str>,
        last_updated_gte: Option<&'a str>,
        last_updated_lt: Option<&'a str>,
        last_updated_lte: Option<&'a str>,
        tickers: Option<&'a str>,
        tickers_all_of: Option<&'a str>,
        tickers_any_of: Option<&'a str>,
        channels: Option<&'a str>,
        channels_all_of: Option<&'a str>,
        channels_any_of: Option<&'a str>,
        tags: Option<&'a str>,
        tags_all_of: Option<&'a str>,
        tags_any_of: Option<&'a str>,
        author: Option<&'a str>,
        author_any_of: Option<&'a str>,
        author_gt: Option<&'a str>,
        author_gte: Option<&'a str>,
        author_lt: Option<&'a str>,
        author_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaNews> {
        self.list_benzinga_news_with_params(ListBenzingaNewsParams {
            published: published.map(String::from),
            published_any_of: published_any_of.map(String::from),
            published_gt: published_gt.map(String::from),
            published_gte: published_gte.map(String::from),
            published_lt: published_lt.map(String::from),
            published_lte: published_lte.map(String::from),
            last_updated: last_updated.map(String::from),
            last_updated_any_of: last_updated_any_of.map(String::from),
            last_updated_gt: last_updated_gt.map(String::from),
            last_updated_gte: last_updated_gte.map(String::from),
            last_updated_lt: last_updated_lt.map(String::from),
            last_updated_lte: last_updated_lte.map(String::from),
            tickers: tickers.map(String::from),
            tickers_all_of: tickers_all_of.map(String::from),
            tickers_any_of: tickers_any_of.map(String::from),
            channels: channels.map(String::from),
            channels_all_of: channels_all_of.map(String::from),
            channels_any_of: channels_any_of.map(String::from),
            tags: tags.map(String::from),
            tags_all_of: tags_all_of.map(String::from),
            tags_any_of: tags_any_of.map(String::from),
            author: author.map(String::from),
            author_any_of: author_any_of.map(String::from),
            author_gt: author_gt.map(String::from),
            author_gte: author_gte.map(String::from),
            author_lt: author_lt.map(String::from),
            author_lte: author_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_benzinga_news_with_params<'a>(
        &'a self,
        params: ListBenzingaNewsParams,
    ) -> BoxStream<'a, BenzingaNews> {
        Box::pin({
            let ListBenzingaNewsParams {
                published,
                published_any_of,
                published_gt,
                published_gte,
                published_lt,
                published_lte,
                last_updated,
                last_updated_any_of,
                last_updated_gt,
                last_updated_gte,
                last_updated_lt,
                last_updated_lte,
                tickers,
                tickers_all_of,
                tickers_any_of,
                channels,
                channels_all_of,
                channels_any_of,
                tags,
                tags_all_of,
                tags_any_of,
                author,
                author_any_of,
                author_gt,
                author_gte,
                author_lt,
                author_lte,
                limit,
                sort,
                options,
            } = params;
            let published = published.as_deref();
            let published_any_of = published_any_of.as_deref();
            let published_gt = published_gt.as_deref();
            let published_gte = published_gte.as_deref();
            let published_lt = published_lt.as_deref();
            let published_lte = published_lte.as_deref();
            let last_updated = last_updated.as_deref();
            let last_updated_any_of = last_updated_any_of.as_deref();
            let last_updated_gt = last_updated_gt.as_deref();
            let last_updated_gte = last_updated_gte.as_deref();
            let last_updated_lt = last_updated_lt.as_deref();
            let last_updated_lte = last_updated_lte.as_deref();
            let tickers = tickers.as_deref();
            let tickers_all_of = tickers_all_of.as_deref();
            let tickers_any_of = tickers_any_of.as_deref();
            let channels = channels.as_deref();
            let channels_all_of = channels_all_of.as_deref();
            let channels_any_of = channels_any_of.as_deref();
            let tags = tags.as_deref();
            let tags_all_of = tags_all_of.as_deref();
            let tags_any_of = tags_any_of.as_deref();
            let author = author.as_deref();
            let author_any_of = author_any_of.as_deref();
            let author_gt = author_gt.as_deref();
            let author_gte = author_gte.as_deref();
            let author_lt = author_lt.as_deref();
            let author_lte = author_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/benzinga/v1/news";
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = published {
                query.push(("published", v.to_string()));
            }
            if let Some(v) = published_any_of {
                query.push(("published.any_of", v.to_string()));
            }
            if let Some(v) = published_gt {
                query.push(("published.gt", v.to_string()));
            }
            if let Some(v) = published_gte {
                query.push(("published.gte", v.to_string()));
            }
            if let Some(v) = published_lt {
                query.push(("published.lt", v.to_string()));
            }
            if let Some(v) = published_lte {
                query.push(("published.lte", v.to_string()));
            }
            if let Some(v) = last_updated {
                query.push(("last_updated", v.to_string()));
            }
            if let Some(v) = last_updated_any_of {
                query.push(("last_updated.any_of", v.to_string()));
            }
            if let Some(v) = last_updated_gt {
                query.push(("last_updated.gt", v.to_string()));
            }
            if let Some(v) = last_updated_gte {
                query.push(("last_updated.gte", v.to_string()));
            }
            if let Some(v) = last_updated_lt {
                query.push(("last_updated.lt", v.to_string()));
            }
            if let Some(v) = last_updated_lte {
                query.push(("last_updated.lte", v.to_string()));
            }
            if let Some(v) = tickers {
                query.push(("tickers", v.to_string()));
            }
            if let Some(v) = tickers_all_of {
                query.push(("tickers.all_of", v.to_string()));
            }
            if let Some(v) = tickers_any_of {
                query.push(("tickers.any_of", v.to_string()));
            }
            if let Some(v) = channels {
                query.push(("channels", v.to_string()));
            }
            if let Some(v) = channels_all_of {
                query.push(("channels.all_of", v.to_string()));
            }
            if let Some(v) = channels_any_of {
                query.push(("channels.any_of", v.to_string()));
            }
            if let Some(v) = tags {
                query.push(("tags", v.to_string()));
            }
            if let Some(v) = tags_all_of {
                query.push(("tags.all_of", v.to_string()));
            }
            if let Some(v) = tags_any_of {
                query.push(("tags.any_of", v.to_string()));
            }
            if let Some(v) = author {
                query.push(("author", v.to_string()));
            }
            if let Some(v) = author_any_of {
                query.push(("author.any_of", v.to_string()));
            }
            if let Some(v) = author_gt {
                query.push(("author.gt", v.to_string()));
            }
            if let Some(v) = author_gte {
                query.push(("author.gte", v.to_string()));
            }
            if let Some(v) = author_lt {
                query.push(("author.lt", v.to_string()));
            }
            if let Some(v) = author_lte {
                query.push(("author.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<BenzingaNews>(path, Some(&query), options)
        })
    }

    fn list_benzinga_news_v2<'a>(
        &'a self,
        published: Option<&'a str>,
        published_gt: Option<&'a str>,
        published_gte: Option<&'a str>,
        published_lt: Option<&'a str>,
        published_lte: Option<&'a str>,
        channels: Option<&'a str>,
        channels_all_of: Option<&'a str>,
        channels_any_of: Option<&'a str>,
        tags: Option<&'a str>,
        tags_all_of: Option<&'a str>,
        tags_any_of: Option<&'a str>,
        author: Option<&'a str>,
        author_any_of: Option<&'a str>,
        author_gt: Option<&'a str>,
        author_gte: Option<&'a str>,
        author_lt: Option<&'a str>,
        author_lte: Option<&'a str>,
        stocks: Option<&'a str>,
        stocks_all_of: Option<&'a str>,
        stocks_any_of: Option<&'a str>,
        tickers: Option<&'a str>,
        tickers_all_of: Option<&'a str>,
        tickers_any_of: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaNews> {
        self.list_benzinga_news_v2_with_params(ListBenzingaNewsV2Params {
            published: published.map(String::from),
            published_gt: published_gt.map(String::from),
            published_gte: published_gte.map(String::from),
            published_lt: published_lt.map(String::from),
            published_lte: published_lte.map(String::from),
            channels: channels.map(String::from),
            channels_all_of: channels_all_of.map(String::from),
            channels_any_of: channels_any_of.map(String::from),
            tags: tags.map(String::from),
            tags_all_of: tags_all_of.map(String::from),
            tags_any_of: tags_any_of.map(String::from),
            author: author.map(String::from),
            author_any_of: author_any_of.map(String::from),
            author_gt: author_gt.map(String::from),
            author_gte: author_gte.map(String::from),
            author_lt: author_lt.map(String::from),
            author_lte: author_lte.map(String::from),
            stocks: stocks.map(String::from),
            stocks_all_of: stocks_all_of.map(String::from),
            stocks_any_of: stocks_any_of.map(String::from),
            tickers: tickers.map(String::from),
            tickers_all_of: tickers_all_of.map(String::from),
            tickers_any_of: tickers_any_of.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_benzinga_news_v2_with_params<'a>(
        &'a self,
        params: ListBenzingaNewsV2Params,
    ) -> BoxStream<'a, BenzingaNews> {
        Box::pin({
            let ListBenzingaNewsV2Params {
                published,
                published_gt,
                published_gte,
                published_lt,
                published_lte,
                channels,
                channels_all_of,
                channels_any_of,
                tags,
                tags_all_of,
                tags_any_of,
                author,
                author_any_of,
                author_gt,
                author_gte,
                author_lt,
                author_lte,
                stocks,
                stocks_all_of,
                stocks_any_of,
                tickers,
                tickers_all_of,
                tickers_any_of,
                limit,
                sort,
                options,
            } = params;
            let published = published.as_deref();
            let published_gt = published_gt.as_deref();
            let published_gte = published_gte.as_deref();
            let published_lt = published_lt.as_deref();
            let published_lte = published_lte.as_deref();
            let channels = channels.as_deref();
            let channels_all_of = channels_all_of.as_deref();
            let channels_any_of = channels_any_of.as_deref();
            let tags = tags.as_deref();
            let tags_all_of = tags_all_of.as_deref();
            let tags_any_of = tags_any_of.as_deref();
            let author = author.as_deref();
            let author_any_of = author_any_of.as_deref();
            let author_gt = author_gt.as_deref();
            let author_gte = author_gte.as_deref();
            let author_lt = author_lt.as_deref();
            let author_lte = author_lte.as_deref();
            let stocks = stocks.as_deref();
            let stocks_all_of = stocks_all_of.as_deref();
            let stocks_any_of = stocks_any_of.as_deref();
            let tickers = tickers.as_deref();
            let tickers_all_of = tickers_all_of.as_deref();
            let tickers_any_of = tickers_any_of.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/benzinga/v2/news";
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = published {
                query.push(("published", v.to_string()));
            }
            if let Some(v) = published_gt {
                query.push(("published.gt", v.to_string()));
            }
            if let Some(v) = published_gte {
                query.push(("published.gte", v.to_string()));
            }
            if let Some(v) = published_lt {
                query.push(("published.lt", v.to_string()));
            }
            if let Some(v) = published_lte {
                query.push(("published.lte", v.to_string()));
            }
            if let Some(v) = channels {
                query.push(("channels", v.to_string()));
            }
            if let Some(v) = channels_all_of {
                query.push(("channels.all_of", v.to_string()));
            }
            if let Some(v) = channels_any_of {
                query.push(("channels.any_of", v.to_string()));
            }
            if let Some(v) = tags {
                query.push(("tags", v.to_string()));
            }
            if let Some(v) = tags_all_of {
                query.push(("tags.all_of", v.to_string()));
            }
            if let Some(v) = tags_any_of {
                query.push(("tags.any_of", v.to_string()));
            }
            if let Some(v) = author {
                query.push(("author", v.to_string()));
            }
            if let Some(v) = author_any_of {
                query.push(("author.any_of", v.to_string()));
            }
            if let Some(v) = author_gt {
                query.push(("author.gt", v.to_string()));
            }
            if let Some(v) = author_gte {
                query.push(("author.gte", v.to_string()));
            }
            if let Some(v) = author_lt {
                query.push(("author.lt", v.to_string()));
            }
            if let Some(v) = author_lte {
                query.push(("author.lte", v.to_string()));
            }
            if let Some(v) = stocks {
                query.push(("stocks", v.to_string()));
            }
            if let Some(v) = stocks_all_of {
                query.push(("stocks.all_of", v.to_string()));
            }
            if let Some(v) = stocks_any_of {
                query.push(("stocks.any_of", v.to_string()));
            }
            if let Some(v) = tickers {
                query.push(("tickers", v.to_string()));
            }
            if let Some(v) = tickers_all_of {
                query.push(("tickers.all_of", v.to_string()));
            }
            if let Some(v) = tickers_any_of {
                query.push(("tickers.any_of", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<BenzingaNews>(path, Some(&query), options)
        })
    }

    fn list_benzinga_ratings<'a>(
        &'a self,
        date: Option<&'a str>,
        date_any_of: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        importance: Option<i64>,
        importance_any_of: Option<&'a str>,
        importance_gt: Option<i64>,
        importance_gte: Option<i64>,
        importance_lt: Option<i64>,
        importance_lte: Option<i64>,
        last_updated: Option<&'a str>,
        last_updated_any_of: Option<&'a str>,
        last_updated_gt: Option<&'a str>,
        last_updated_gte: Option<&'a str>,
        last_updated_lt: Option<&'a str>,
        last_updated_lte: Option<&'a str>,
        rating_action: Option<&'a str>,
        rating_action_any_of: Option<&'a str>,
        rating_action_gt: Option<&'a str>,
        rating_action_gte: Option<&'a str>,
        rating_action_lt: Option<&'a str>,
        rating_action_lte: Option<&'a str>,
        price_target_action: Option<&'a str>,
        price_target_action_any_of: Option<&'a str>,
        price_target_action_gt: Option<&'a str>,
        price_target_action_gte: Option<&'a str>,
        price_target_action_lt: Option<&'a str>,
        price_target_action_lte: Option<&'a str>,
        benzinga_id: Option<&'a str>,
        benzinga_id_any_of: Option<&'a str>,
        benzinga_id_gt: Option<&'a str>,
        benzinga_id_gte: Option<&'a str>,
        benzinga_id_lt: Option<&'a str>,
        benzinga_id_lte: Option<&'a str>,
        benzinga_analyst_id: Option<&'a str>,
        benzinga_analyst_id_any_of: Option<&'a str>,
        benzinga_analyst_id_gt: Option<&'a str>,
        benzinga_analyst_id_gte: Option<&'a str>,
        benzinga_analyst_id_lt: Option<&'a str>,
        benzinga_analyst_id_lte: Option<&'a str>,
        benzinga_firm_id: Option<&'a str>,
        benzinga_firm_id_any_of: Option<&'a str>,
        benzinga_firm_id_gt: Option<&'a str>,
        benzinga_firm_id_gte: Option<&'a str>,
        benzinga_firm_id_lt: Option<&'a str>,
        benzinga_firm_id_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaRating> {
        self.list_benzinga_ratings_with_params(ListBenzingaRatingsParams {
            date: date.map(String::from),
            date_any_of: date_any_of.map(String::from),
            date_gt: date_gt.map(String::from),
            date_gte: date_gte.map(String::from),
            date_lt: date_lt.map(String::from),
            date_lte: date_lte.map(String::from),
            ticker: ticker.map(String::from),
            ticker_any_of: ticker_any_of.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            importance,
            importance_any_of: importance_any_of.map(String::from),
            importance_gt,
            importance_gte,
            importance_lt,
            importance_lte,
            last_updated: last_updated.map(String::from),
            last_updated_any_of: last_updated_any_of.map(String::from),
            last_updated_gt: last_updated_gt.map(String::from),
            last_updated_gte: last_updated_gte.map(String::from),
            last_updated_lt: last_updated_lt.map(String::from),
            last_updated_lte: last_updated_lte.map(String::from),
            rating_action: rating_action.map(String::from),
            rating_action_any_of: rating_action_any_of.map(String::from),
            rating_action_gt: rating_action_gt.map(String::from),
            rating_action_gte: rating_action_gte.map(String::from),
            rating_action_lt: rating_action_lt.map(String::from),
            rating_action_lte: rating_action_lte.map(String::from),
            price_target_action: price_target_action.map(String::from),
            price_target_action_any_of: price_target_action_any_of.map(String::from),
            price_target_action_gt: price_target_action_gt.map(String::from),
            price_target_action_gte: price_target_action_gte.map(String::from),
            price_target_action_lt: price_target_action_lt.map(String::from),
            price_target_action_lte: price_target_action_lte.map(String::from),
            benzinga_id: benzinga_id.map(String::from),
            benzinga_id_any_of: benzinga_id_any_of.map(String::from),
            benzinga_id_gt: benzinga_id_gt.map(String::from),
            benzinga_id_gte: benzinga_id_gte.map(String::from),
            benzinga_id_lt: benzinga_id_lt.map(String::from),
            benzinga_id_lte: benzinga_id_lte.map(String::from),
            benzinga_analyst_id: benzinga_analyst_id.map(String::from),
            benzinga_analyst_id_any_of: benzinga_analyst_id_any_of.map(String::from),
            benzinga_analyst_id_gt: benzinga_analyst_id_gt.map(String::from),
            benzinga_analyst_id_gte: benzinga_analyst_id_gte.map(String::from),
            benzinga_analyst_id_lt: benzinga_analyst_id_lt.map(String::from),
            benzinga_analyst_id_lte: benzinga_analyst_id_lte.map(String::from),
            benzinga_firm_id: benzinga_firm_id.map(String::from),
            benzinga_firm_id_any_of: benzinga_firm_id_any_of.map(String::from),
            benzinga_firm_id_gt: benzinga_firm_id_gt.map(String::from),
            benzinga_firm_id_gte: benzinga_firm_id_gte.map(String::from),
            benzinga_firm_id_lt: benzinga_firm_id_lt.map(String::from),
            benzinga_firm_id_lte: benzinga_firm_id_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_benzinga_ratings_with_params<'a>(
        &'a self,
        params: ListBenzingaRatingsParams,
    ) -> BoxStream<'a, BenzingaRating> {
        Box::pin({
            let ListBenzingaRatingsParams {
                date,
                date_any_of,
                date_gt,
                date_gte,
                date_lt,
                date_lte,
                ticker,
                ticker_any_of,
                ticker_gt,
                ticker_gte,
                ticker_lt,
                ticker_lte,
                importance,
                importance_any_of,
                importance_gt,
                importance_gte,
                importance_lt,
                importance_lte,
                last_updated,
                last_updated_any_of,
                last_updated_gt,
                last_updated_gte,
                last_updated_lt,
                last_updated_lte,
                rating_action,
                rating_action_any_of,
                rating_action_gt,
                rating_action_gte,
                rating_action_lt,
                rating_action_lte,
                price_target_action,
                price_target_action_any_of,
                price_target_action_gt,
                price_target_action_gte,
                price_target_action_lt,
                price_target_action_lte,
                benzinga_id,
                benzinga_id_any_of,
                benzinga_id_gt,
                benzinga_id_gte,
                benzinga_id_lt,
                benzinga_id_lte,
                benzinga_analyst_id,
                benzinga_analyst_id_any_of,
                benzinga_analyst_id_gt,
                benzinga_analyst_id_gte,
                benzinga_analyst_id_lt,
                benzinga_analyst_id_lte,
                benzinga_firm_id,
                benzinga_firm_id_any_of,
                benzinga_firm_id_gt,
                benzinga_firm_id_gte,
                benzinga_firm_id_lt,
                benzinga_firm_id_lte,
                limit,
                sort,
                options,
            } = params;
            let date = date.as_deref();
            let date_any_of = date_any_of.as_deref();
            let date_gt = date_gt.as_deref();
            let date_gte = date_gte.as_deref();
            let date_lt = date_lt.as_deref();
            let date_lte = date_lte.as_deref();
            let ticker = ticker.as_deref();
            let ticker_any_of = ticker_any_of.as_deref();
            let ticker_gt = ticker_gt.as_deref();
            let ticker_gte = ticker_gte.as_deref();
            let ticker_lt = ticker_lt.as_deref();
            let ticker_lte = ticker_lte.as_deref();
            let importance_any_of = importance_any_of.as_deref();
            let last_updated = last_updated.as_deref();
            let last_updated_any_of = last_updated_any_of.as_deref();
            let last_updated_gt = last_updated_gt.as_deref();
            let last_updated_gte = last_updated_gte.as_deref();
            let last_updated_lt = last_updated_lt.as_deref();
            let last_updated_lte = last_updated_lte.as_deref();
            let rating_action = rating_action.as_deref();
            let rating_action_any_of = rating_action_any_of.as_deref();
            let rating_action_gt = rating_action_gt.as_deref();
            let rating_action_gte = rating_action_gte.as_deref();
            let rating_action_lt = rating_action_lt.as_deref();
            let rating_action_lte = rating_action_lte.as_deref();
            let price_target_action = price_target_action.as_deref();
            let price_target_action_any_of = price_target_action_any_of.as_deref();
            let price_target_action_gt = price_target_action_gt.as_deref();
            let price_target_action_gte = price_target_action_gte.as_deref();
            let price_target_action_lt = price_target_action_lt.as_deref();
            let price_target_action_lte = price_target_action_lte.as_deref();
            let benzinga_id = benzinga_id.as_deref();
            let benzinga_id_any_of = benzinga_id_any_of.as_deref();
            let benzinga_id_gt = benzinga_id_gt.as_deref();
            let benzinga_id_gte = benzinga_id_gte.as_deref();
            let benzinga_id_lt = benzinga_id_lt.as_deref();
            let benzinga_id_lte = benzinga_id_lte.as_deref();
            let benzinga_analyst_id = benzinga_analyst_id.as_deref();
            let benzinga_analyst_id_any_of = benzinga_analyst_id_any_of.as_deref();
            let benzinga_analyst_id_gt = benzinga_analyst_id_gt.as_deref();
            let benzinga_analyst_id_gte = benzinga_analyst_id_gte.as_deref();
            let benzinga_analyst_id_lt = benzinga_analyst_id_lt.as_deref();
            let benzinga_analyst_id_lte = benzinga_analyst_id_lte.as_deref();
            let benzinga_firm_id = benzinga_firm_id.as_deref();
            let benzinga_firm_id_any_of = benzinga_firm_id_any_of.as_deref();
            let benzinga_firm_id_gt = benzinga_firm_id_gt.as_deref();
            let benzinga_firm_id_gte = benzinga_firm_id_gte.as_deref();
            let benzinga_firm_id_lt = benzinga_firm_id_lt.as_deref();
            let benzinga_firm_id_lte = benzinga_firm_id_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/benzinga/v1/ratings";
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = date {
                query.push(("date", v.to_string()));
            }
            if let Some(v) = date_any_of {
                query.push(("date.any_of", v.to_string()));
            }
            if let Some(v) = date_gt {
                query.push(("date.gt", v.to_string()));
            }
            if let Some(v) = date_gte {
                query.push(("date.gte", v.to_string()));
            }
            if let Some(v) = date_lt {
                query.push(("date.lt", v.to_string()));
            }
            if let Some(v) = date_lte {
                query.push(("date.lte", v.to_string()));
            }
            if let Some(v) = ticker {
                query.push(("ticker", v.to_string()));
            }
            if let Some(v) = ticker_any_of {
                query.push(("ticker.any_of", v.to_string()));
            }
            if let Some(v) = ticker_gt {
                query.push(("ticker.gt", v.to_string()));
            }
            if let Some(v) = ticker_gte {
                query.push(("ticker.gte", v.to_string()));
            }
            if let Some(v) = ticker_lt {
                query.push(("ticker.lt", v.to_string()));
            }
            if let Some(v) = ticker_lte {
                query.push(("ticker.lte", v.to_string()));
            }
            if let Some(v) = importance {
                query.push(("importance", v.to_string()));
            }
            if let Some(v) = importance_any_of {
                query.push(("importance.any_of", v.to_string()));
            }
            if let Some(v) = importance_gt {
                query.push(("importance.gt", v.to_string()));
            }
            if let Some(v) = importance_gte {
                query.push(("importance.gte", v.to_string()));
            }
            if let Some(v) = importance_lt {
                query.push(("importance.lt", v.to_string()));
            }
            if let Some(v) = importance_lte {
                query.push(("importance.lte", v.to_string()));
            }
            if let Some(v) = last_updated {
                query.push(("last_updated", v.to_string()));
            }
            if let Some(v) = last_updated_any_of {
                query.push(("last_updated.any_of", v.to_string()));
            }
            if let Some(v) = last_updated_gt {
                query.push(("last_updated.gt", v.to_string()));
            }
            if let Some(v) = last_updated_gte {
                query.push(("last_updated.gte", v.to_string()));
            }
            if let Some(v) = last_updated_lt {
                query.push(("last_updated.lt", v.to_string()));
            }
            if let Some(v) = last_updated_lte {
                query.push(("last_updated.lte", v.to_string()));
            }
            if let Some(v) = rating_action {
                query.push(("rating_action", v.to_string()));
            }
            if let Some(v) = rating_action_any_of {
                query.push(("rating_action.any_of", v.to_string()));
            }
            if let Some(v) = rating_action_gt {
                query.push(("rating_action.gt", v.to_string()));
            }
            if let Some(v) = rating_action_gte {
                query.push(("rating_action.gte", v.to_string()));
            }
            if let Some(v) = rating_action_lt {
                query.push(("rating_action.lt", v.to_string()));
            }
            if let Some(v) = rating_action_lte {
                query.push(("rating_action.lte", v.to_string()));
            }
            if let Some(v) = price_target_action {
                query.push(("price_target_action", v.to_string()));
            }
            if let Some(v) = price_target_action_any_of {
                query.push(("price_target_action.any_of", v.to_string()));
            }
            if let Some(v) = price_target_action_gt {
                query.push(("price_target_action.gt", v.to_string()));
            }
            if let Some(v) = price_target_action_gte {
                query.push(("price_target_action.gte", v.to_string()));
            }
            if let Some(v) = price_target_action_lt {
                query.push(("price_target_action.lt", v.to_string()));
            }
            if let Some(v) = price_target_action_lte {
                query.push(("price_target_action.lte", v.to_string()));
            }
            if let Some(v) = benzinga_id {
                query.push(("benzinga_id", v.to_string()));
            }
            if let Some(v) = benzinga_id_any_of {
                query.push(("benzinga_id.any_of", v.to_string()));
            }
            if let Some(v) = benzinga_id_gt {
                query.push(("benzinga_id.gt", v.to_string()));
            }
            if let Some(v) = benzinga_id_gte {
                query.push(("benzinga_id.gte", v.to_string()));
            }
            if let Some(v) = benzinga_id_lt {
                query.push(("benzinga_id.lt", v.to_string()));
            }
            if let Some(v) = benzinga_id_lte {
                query.push(("benzinga_id.lte", v.to_string()));
            }
            if let Some(v) = benzinga_analyst_id {
                query.push(("benzinga_analyst_id", v.to_string()));
            }
            if let Some(v) = benzinga_analyst_id_any_of {
                query.push(("benzinga_analyst_id.any_of", v.to_string()));
            }
            if let Some(v) = benzinga_analyst_id_gt {
                query.push(("benzinga_analyst_id.gt", v.to_string()));
            }
            if let Some(v) = benzinga_analyst_id_gte {
                query.push(("benzinga_analyst_id.gte", v.to_string()));
            }
            if let Some(v) = benzinga_analyst_id_lt {
                query.push(("benzinga_analyst_id.lt", v.to_string()));
            }
            if let Some(v) = benzinga_analyst_id_lte {
                query.push(("benzinga_analyst_id.lte", v.to_string()));
            }
            if let Some(v) = benzinga_firm_id {
                query.push(("benzinga_firm_id", v.to_string()));
            }
            if let Some(v) = benzinga_firm_id_any_of {
                query.push(("benzinga_firm_id.any_of", v.to_string()));
            }
            if let Some(v) = benzinga_firm_id_gt {
                query.push(("benzinga_firm_id.gt", v.to_string()));
            }
            if let Some(v) = benzinga_firm_id_gte {
                query.push(("benzinga_firm_id.gte", v.to_string()));
            }
            if let Some(v) = benzinga_firm_id_lt {
                query.push(("benzinga_firm_id.lt", v.to_string()));
            }
            if let Some(v) = benzinga_firm_id_lte {
                query.push(("benzinga_firm_id.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<BenzingaRating>(path, Some(&query), options)
        })
    }

    fn list_benzinga_bulls_bears_say<'a>(
        &'a self,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        benzinga_id: Option<&'a str>,
        benzinga_id_any_of: Option<&'a str>,
        benzinga_id_gt: Option<&'a str>,
        benzinga_id_gte: Option<&'a str>,
        benzinga_id_lt: Option<&'a str>,
        benzinga_id_lte: Option<&'a str>,
        last_updated: Option<&'a str>,
        last_updated_gt: Option<&'a str>,
        last_updated_gte: Option<&'a str>,
        last_updated_lt: Option<&'a str>,
        last_updated_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, BenzingaBullsBearsSay> {
        self.list_benzinga_bulls_bears_say_with_params(ListBenzingaBullsBearsSayParams {
            ticker: ticker.map(String::from),
            ticker_any_of: ticker_any_of.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            benzinga_id: benzinga_id.map(String::from),
            benzinga_id_any_of: benzinga_id_any_of.map(String::from),
            benzinga_id_gt: benzinga_id_gt.map(String::from),
            benzinga_id_gte: benzinga_id_gte.map(String::from),
            benzinga_id_lt: benzinga_id_lt.map(String::from),
            benzinga_id_lte: benzinga_id_lte.map(String::from),
            last_updated: last_updated.map(String::from),
            last_updated_gt: last_updated_gt.map(String::from),
            last_updated_gte: last_updated_gte.map(String::from),
            last_updated_lt: last_updated_lt.map(String::from),
            last_updated_lte: last_updated_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_benzinga_bulls_bears_say_with_params<'a>(
        &'a self,
        params: ListBenzingaBullsBearsSayParams,
    ) -> BoxStream<'a, BenzingaBullsBearsSay> {
        Box::pin({
            let ListBenzingaBullsBearsSayParams {
                ticker,
                ticker_any_of,
                ticker_gt,
                ticker_gte,
                ticker_lt,
                ticker_lte,
                benzinga_id,
                benzinga_id_any_of,
                benzinga_id_gt,
                benzinga_id_gte,
                benzinga_id_lt,
                benzinga_id_lte,
                last_updated,
                last_updated_gt,
                last_updated_gte,
                last_updated_lt,
                last_updated_lte,
                limit,
                sort,
                options,
            } = params;
            let ticker = ticker.as_deref();
            let ticker_any_of = ticker_any_of.as_deref();
            let ticker_gt = ticker_gt.as_deref();
            let ticker_gte = ticker_gte.as_deref();
            let ticker_lt = ticker_lt.as_deref();
            let ticker_lte = ticker_lte.as_deref();
            let benzinga_id = benzinga_id.as_deref();
            let benzinga_id_any_of = benzinga_id_any_of.as_deref();
            let benzinga_id_gt = benzinga_id_gt.as_deref();
            let benzinga_id_gte = benzinga_id_gte.as_deref();
            let benzinga_id_lt = benzinga_id_lt.as_deref();
            let benzinga_id_lte = benzinga_id_lte.as_deref();
            let last_updated = last_updated.as_deref();
            let last_updated_gt = last_updated_gt.as_deref();
            let last_updated_gte = last_updated_gte.as_deref();
            let last_updated_lt = last_updated_lt.as_deref();
            let last_updated_lte = last_updated_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/benzinga/v1/bulls-bears-say";
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = ticker {
                query.push(("ticker", v.to_string()));
            }
            if let Some(v) = ticker_any_of {
                query.push(("ticker.any_of", v.to_string()));
            }
            if let Some(v) = ticker_gt {
                query.push(("ticker.gt", v.to_string()));
            }
            if let Some(v) = ticker_gte {
                query.push(("ticker.gte", v.to_string()));
            }
            if let Some(v) = ticker_lt {
                query.push(("ticker.lt", v.to_string()));
            }
            if let Some(v) = ticker_lte {
                query.push(("ticker.lte", v.to_string()));
            }
            if let Some(v) = benzinga_id {
                query.push(("benzinga_id", v.to_string()));
            }
            if let Some(v) = benzinga_id_any_of {
                query.push(("benzinga_id.any_of", v.to_string()));
            }
            if let Some(v) = benzinga_id_gt {
                query.push(("benzinga_id.gt", v.to_string()));
            }
            if let Some(v) = benzinga_id_gte {
                query.push(("benzinga_id.gte", v.to_string()));
            }
            if let Some(v) = benzinga_id_lt {
                query.push(("benzinga_id.lt", v.to_string()));
            }
            if let Some(v) = benzinga_id_lte {
                query.push(("benzinga_id.lte", v.to_string()));
            }
            if let Some(v) = last_updated {
                query.push(("last_updated", v.to_string()));
            }
            if let Some(v) = last_updated_gt {
                query.push(("last_updated.gt", v.to_string()));
            }
            if let Some(v) = last_updated_gte {
                query.push(("last_updated.gte", v.to_string()));
            }
            if let Some(v) = last_updated_lt {
                query.push(("last_updated.lt", v.to_string()));
            }
            if let Some(v) = last_updated_lte {
                query.push(("last_updated.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<BenzingaBullsBearsSay>(path, Some(&query), options)
        })
    }
}

// --- Params structs (additive builder API) ---

/// Optional arguments for [`BenzingaApi::list_benzinga_analyst_insights`].
#[derive(Debug, Default, Clone)]
pub struct ListBenzingaAnalystInsightsParams {
    /// The `date` argument.
    pub date: Option<String>,
    /// The `date_any_of` argument.
    pub date_any_of: Option<String>,
    /// The `date_gt` argument.
    pub date_gt: Option<String>,
    /// The `date_gte` argument.
    pub date_gte: Option<String>,
    /// The `date_lt` argument.
    pub date_lt: Option<String>,
    /// The `date_lte` argument.
    pub date_lte: Option<String>,
    /// The `ticker` argument.
    pub ticker: Option<String>,
    /// The `ticker_any_of` argument.
    pub ticker_any_of: Option<String>,
    /// The `ticker_gt` argument.
    pub ticker_gt: Option<String>,
    /// The `ticker_gte` argument.
    pub ticker_gte: Option<String>,
    /// The `ticker_lt` argument.
    pub ticker_lt: Option<String>,
    /// The `ticker_lte` argument.
    pub ticker_lte: Option<String>,
    /// The `last_updated` argument.
    pub last_updated: Option<String>,
    /// The `last_updated_any_of` argument.
    pub last_updated_any_of: Option<String>,
    /// The `last_updated_gt` argument.
    pub last_updated_gt: Option<String>,
    /// The `last_updated_gte` argument.
    pub last_updated_gte: Option<String>,
    /// The `last_updated_lt` argument.
    pub last_updated_lt: Option<String>,
    /// The `last_updated_lte` argument.
    pub last_updated_lte: Option<String>,
    /// The `firm` argument.
    pub firm: Option<String>,
    /// The `firm_any_of` argument.
    pub firm_any_of: Option<String>,
    /// The `firm_gt` argument.
    pub firm_gt: Option<String>,
    /// The `firm_gte` argument.
    pub firm_gte: Option<String>,
    /// The `firm_lt` argument.
    pub firm_lt: Option<String>,
    /// The `firm_lte` argument.
    pub firm_lte: Option<String>,
    /// The `rating_action` argument.
    pub rating_action: Option<String>,
    /// The `rating_action_any_of` argument.
    pub rating_action_any_of: Option<String>,
    /// The `rating_action_gt` argument.
    pub rating_action_gt: Option<String>,
    /// The `rating_action_gte` argument.
    pub rating_action_gte: Option<String>,
    /// The `rating_action_lt` argument.
    pub rating_action_lt: Option<String>,
    /// The `rating_action_lte` argument.
    pub rating_action_lte: Option<String>,
    /// The `benzinga_firm_id` argument.
    pub benzinga_firm_id: Option<String>,
    /// The `benzinga_firm_id_any_of` argument.
    pub benzinga_firm_id_any_of: Option<String>,
    /// The `benzinga_firm_id_gt` argument.
    pub benzinga_firm_id_gt: Option<String>,
    /// The `benzinga_firm_id_gte` argument.
    pub benzinga_firm_id_gte: Option<String>,
    /// The `benzinga_firm_id_lt` argument.
    pub benzinga_firm_id_lt: Option<String>,
    /// The `benzinga_firm_id_lte` argument.
    pub benzinga_firm_id_lte: Option<String>,
    /// The `benzinga_rating_id` argument.
    pub benzinga_rating_id: Option<String>,
    /// The `benzinga_rating_id_any_of` argument.
    pub benzinga_rating_id_any_of: Option<String>,
    /// The `benzinga_rating_id_gt` argument.
    pub benzinga_rating_id_gt: Option<String>,
    /// The `benzinga_rating_id_gte` argument.
    pub benzinga_rating_id_gte: Option<String>,
    /// The `benzinga_rating_id_lt` argument.
    pub benzinga_rating_id_lt: Option<String>,
    /// The `benzinga_rating_id_lte` argument.
    pub benzinga_rating_id_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListBenzingaAnalystInsightsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `date` argument.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }

    /// Set the `date_any_of` argument.
    pub fn date_any_of(mut self, date_any_of: impl Into<String>) -> Self {
        self.date_any_of = Some(date_any_of.into());
        self
    }

    /// Set the `date_gt` argument.
    pub fn date_gt(mut self, date_gt: impl Into<String>) -> Self {
        self.date_gt = Some(date_gt.into());
        self
    }

    /// Set the `date_gte` argument.
    pub fn date_gte(mut self, date_gte: impl Into<String>) -> Self {
        self.date_gte = Some(date_gte.into());
        self
    }

    /// Set the `date_lt` argument.
    pub fn date_lt(mut self, date_lt: impl Into<String>) -> Self {
        self.date_lt = Some(date_lt.into());
        self
    }

    /// Set the `date_lte` argument.
    pub fn date_lte(mut self, date_lte: impl Into<String>) -> Self {
        self.date_lte = Some(date_lte.into());
        self
    }

    /// Set the `ticker` argument.
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    /// Set the `ticker_any_of` argument.
    pub fn ticker_any_of(mut self, ticker_any_of: impl Into<String>) -> Self {
        self.ticker_any_of = Some(ticker_any_of.into());
        self
    }

    /// Set the `ticker_gt` argument.
    pub fn ticker_gt(mut self, ticker_gt: impl Into<String>) -> Self {
        self.ticker_gt = Some(ticker_gt.into());
        self
    }

    /// Set the `ticker_gte` argument.
    pub fn ticker_gte(mut self, ticker_gte: impl Into<String>) -> Self {
        self.ticker_gte = Some(ticker_gte.into());
        self
    }

    /// Set the `ticker_lt` argument.
    pub fn ticker_lt(mut self, ticker_lt: impl Into<String>) -> Self {
        self.ticker_lt = Some(ticker_lt.into());
        self
    }

    /// Set the `ticker_lte` argument.
    pub fn ticker_lte(mut self, ticker_lte: impl Into<String>) -> Self {
        self.ticker_lte = Some(ticker_lte.into());
        self
    }

    /// Set the `last_updated` argument.
    pub fn last_updated(mut self, last_updated: impl Into<String>) -> Self {
        self.last_updated = Some(last_updated.into());
        self
    }

    /// Set the `last_updated_any_of` argument.
    pub fn last_updated_any_of(mut self, last_updated_any_of: impl Into<String>) -> Self {
        self.last_updated_any_of = Some(last_updated_any_of.into());
        self
    }

    /// Set the `last_updated_gt` argument.
    pub fn last_updated_gt(mut self, last_updated_gt: impl Into<String>) -> Self {
        self.last_updated_gt = Some(last_updated_gt.into());
        self
    }

    /// Set the `last_updated_gte` argument.
    pub fn last_updated_gte(mut self, last_updated_gte: impl Into<String>) -> Self {
        self.last_updated_gte = Some(last_updated_gte.into());
        self
    }

    /// Set the `last_updated_lt` argument.
    pub fn last_updated_lt(mut self, last_updated_lt: impl Into<String>) -> Self {
        self.last_updated_lt = Some(last_updated_lt.into());
        self
    }

    /// Set the `last_updated_lte` argument.
    pub fn last_updated_lte(mut self, last_updated_lte: impl Into<String>) -> Self {
        self.last_updated_lte = Some(last_updated_lte.into());
        self
    }

    /// Set the `firm` argument.
    pub fn firm(mut self, firm: impl Into<String>) -> Self {
        self.firm = Some(firm.into());
        self
    }

    /// Set the `firm_any_of` argument.
    pub fn firm_any_of(mut self, firm_any_of: impl Into<String>) -> Self {
        self.firm_any_of = Some(firm_any_of.into());
        self
    }

    /// Set the `firm_gt` argument.
    pub fn firm_gt(mut self, firm_gt: impl Into<String>) -> Self {
        self.firm_gt = Some(firm_gt.into());
        self
    }

    /// Set the `firm_gte` argument.
    pub fn firm_gte(mut self, firm_gte: impl Into<String>) -> Self {
        self.firm_gte = Some(firm_gte.into());
        self
    }

    /// Set the `firm_lt` argument.
    pub fn firm_lt(mut self, firm_lt: impl Into<String>) -> Self {
        self.firm_lt = Some(firm_lt.into());
        self
    }

    /// Set the `firm_lte` argument.
    pub fn firm_lte(mut self, firm_lte: impl Into<String>) -> Self {
        self.firm_lte = Some(firm_lte.into());
        self
    }

    /// Set the `rating_action` argument.
    pub fn rating_action(mut self, rating_action: impl Into<String>) -> Self {
        self.rating_action = Some(rating_action.into());
        self
    }

    /// Set the `rating_action_any_of` argument.
    pub fn rating_action_any_of(mut self, rating_action_any_of: impl Into<String>) -> Self {
        self.rating_action_any_of = Some(rating_action_any_of.into());
        self
    }

    /// Set the `rating_action_gt` argument.
    pub fn rating_action_gt(mut self, rating_action_gt: impl Into<String>) -> Self {
        self.rating_action_gt = Some(rating_action_gt.into());
        self
    }

    /// Set the `rating_action_gte` argument.
    pub fn rating_action_gte(mut self, rating_action_gte: impl Into<String>) -> Self {
        self.rating_action_gte = Some(rating_action_gte.into());
        self
    }

    /// Set the `rating_action_lt` argument.
    pub fn rating_action_lt(mut self, rating_action_lt: impl Into<String>) -> Self {
        self.rating_action_lt = Some(rating_action_lt.into());
        self
    }

    /// Set the `rating_action_lte` argument.
    pub fn rating_action_lte(mut self, rating_action_lte: impl Into<String>) -> Self {
        self.rating_action_lte = Some(rating_action_lte.into());
        self
    }

    /// Set the `benzinga_firm_id` argument.
    pub fn benzinga_firm_id(mut self, benzinga_firm_id: impl Into<String>) -> Self {
        self.benzinga_firm_id = Some(benzinga_firm_id.into());
        self
    }

    /// Set the `benzinga_firm_id_any_of` argument.
    pub fn benzinga_firm_id_any_of(mut self, benzinga_firm_id_any_of: impl Into<String>) -> Self {
        self.benzinga_firm_id_any_of = Some(benzinga_firm_id_any_of.into());
        self
    }

    /// Set the `benzinga_firm_id_gt` argument.
    pub fn benzinga_firm_id_gt(mut self, benzinga_firm_id_gt: impl Into<String>) -> Self {
        self.benzinga_firm_id_gt = Some(benzinga_firm_id_gt.into());
        self
    }

    /// Set the `benzinga_firm_id_gte` argument.
    pub fn benzinga_firm_id_gte(mut self, benzinga_firm_id_gte: impl Into<String>) -> Self {
        self.benzinga_firm_id_gte = Some(benzinga_firm_id_gte.into());
        self
    }

    /// Set the `benzinga_firm_id_lt` argument.
    pub fn benzinga_firm_id_lt(mut self, benzinga_firm_id_lt: impl Into<String>) -> Self {
        self.benzinga_firm_id_lt = Some(benzinga_firm_id_lt.into());
        self
    }

    /// Set the `benzinga_firm_id_lte` argument.
    pub fn benzinga_firm_id_lte(mut self, benzinga_firm_id_lte: impl Into<String>) -> Self {
        self.benzinga_firm_id_lte = Some(benzinga_firm_id_lte.into());
        self
    }

    /// Set the `benzinga_rating_id` argument.
    pub fn benzinga_rating_id(mut self, benzinga_rating_id: impl Into<String>) -> Self {
        self.benzinga_rating_id = Some(benzinga_rating_id.into());
        self
    }

    /// Set the `benzinga_rating_id_any_of` argument.
    pub fn benzinga_rating_id_any_of(
        mut self,
        benzinga_rating_id_any_of: impl Into<String>,
    ) -> Self {
        self.benzinga_rating_id_any_of = Some(benzinga_rating_id_any_of.into());
        self
    }

    /// Set the `benzinga_rating_id_gt` argument.
    pub fn benzinga_rating_id_gt(mut self, benzinga_rating_id_gt: impl Into<String>) -> Self {
        self.benzinga_rating_id_gt = Some(benzinga_rating_id_gt.into());
        self
    }

    /// Set the `benzinga_rating_id_gte` argument.
    pub fn benzinga_rating_id_gte(mut self, benzinga_rating_id_gte: impl Into<String>) -> Self {
        self.benzinga_rating_id_gte = Some(benzinga_rating_id_gte.into());
        self
    }

    /// Set the `benzinga_rating_id_lt` argument.
    pub fn benzinga_rating_id_lt(mut self, benzinga_rating_id_lt: impl Into<String>) -> Self {
        self.benzinga_rating_id_lt = Some(benzinga_rating_id_lt.into());
        self
    }

    /// Set the `benzinga_rating_id_lte` argument.
    pub fn benzinga_rating_id_lte(mut self, benzinga_rating_id_lte: impl Into<String>) -> Self {
        self.benzinga_rating_id_lte = Some(benzinga_rating_id_lte.into());
        self
    }

    /// Set the `limit` argument.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the `sort` argument.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`BenzingaApi::list_benzinga_analysts`].
#[derive(Debug, Default, Clone)]
pub struct ListBenzingaAnalystsParams {
    /// The `benzinga_id` argument.
    pub benzinga_id: Option<String>,
    /// The `benzinga_id_any_of` argument.
    pub benzinga_id_any_of: Option<String>,
    /// The `benzinga_id_gt` argument.
    pub benzinga_id_gt: Option<String>,
    /// The `benzinga_id_gte` argument.
    pub benzinga_id_gte: Option<String>,
    /// The `benzinga_id_lt` argument.
    pub benzinga_id_lt: Option<String>,
    /// The `benzinga_id_lte` argument.
    pub benzinga_id_lte: Option<String>,
    /// The `benzinga_firm_id` argument.
    pub benzinga_firm_id: Option<String>,
    /// The `benzinga_firm_id_any_of` argument.
    pub benzinga_firm_id_any_of: Option<String>,
    /// The `benzinga_firm_id_gt` argument.
    pub benzinga_firm_id_gt: Option<String>,
    /// The `benzinga_firm_id_gte` argument.
    pub benzinga_firm_id_gte: Option<String>,
    /// The `benzinga_firm_id_lt` argument.
    pub benzinga_firm_id_lt: Option<String>,
    /// The `benzinga_firm_id_lte` argument.
    pub benzinga_firm_id_lte: Option<String>,
    /// The `firm_name` argument.
    pub firm_name: Option<String>,
    /// The `firm_name_any_of` argument.
    pub firm_name_any_of: Option<String>,
    /// The `firm_name_gt` argument.
    pub firm_name_gt: Option<String>,
    /// The `firm_name_gte` argument.
    pub firm_name_gte: Option<String>,
    /// The `firm_name_lt` argument.
    pub firm_name_lt: Option<String>,
    /// The `firm_name_lte` argument.
    pub firm_name_lte: Option<String>,
    /// The `full_name` argument.
    pub full_name: Option<String>,
    /// The `full_name_any_of` argument.
    pub full_name_any_of: Option<String>,
    /// The `full_name_gt` argument.
    pub full_name_gt: Option<String>,
    /// The `full_name_gte` argument.
    pub full_name_gte: Option<String>,
    /// The `full_name_lt` argument.
    pub full_name_lt: Option<String>,
    /// The `full_name_lte` argument.
    pub full_name_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListBenzingaAnalystsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `benzinga_id` argument.
    pub fn benzinga_id(mut self, benzinga_id: impl Into<String>) -> Self {
        self.benzinga_id = Some(benzinga_id.into());
        self
    }

    /// Set the `benzinga_id_any_of` argument.
    pub fn benzinga_id_any_of(mut self, benzinga_id_any_of: impl Into<String>) -> Self {
        self.benzinga_id_any_of = Some(benzinga_id_any_of.into());
        self
    }

    /// Set the `benzinga_id_gt` argument.
    pub fn benzinga_id_gt(mut self, benzinga_id_gt: impl Into<String>) -> Self {
        self.benzinga_id_gt = Some(benzinga_id_gt.into());
        self
    }

    /// Set the `benzinga_id_gte` argument.
    pub fn benzinga_id_gte(mut self, benzinga_id_gte: impl Into<String>) -> Self {
        self.benzinga_id_gte = Some(benzinga_id_gte.into());
        self
    }

    /// Set the `benzinga_id_lt` argument.
    pub fn benzinga_id_lt(mut self, benzinga_id_lt: impl Into<String>) -> Self {
        self.benzinga_id_lt = Some(benzinga_id_lt.into());
        self
    }

    /// Set the `benzinga_id_lte` argument.
    pub fn benzinga_id_lte(mut self, benzinga_id_lte: impl Into<String>) -> Self {
        self.benzinga_id_lte = Some(benzinga_id_lte.into());
        self
    }

    /// Set the `benzinga_firm_id` argument.
    pub fn benzinga_firm_id(mut self, benzinga_firm_id: impl Into<String>) -> Self {
        self.benzinga_firm_id = Some(benzinga_firm_id.into());
        self
    }

    /// Set the `benzinga_firm_id_any_of` argument.
    pub fn benzinga_firm_id_any_of(mut self, benzinga_firm_id_any_of: impl Into<String>) -> Self {
        self.benzinga_firm_id_any_of = Some(benzinga_firm_id_any_of.into());
        self
    }

    /// Set the `benzinga_firm_id_gt` argument.
    pub fn benzinga_firm_id_gt(mut self, benzinga_firm_id_gt: impl Into<String>) -> Self {
        self.benzinga_firm_id_gt = Some(benzinga_firm_id_gt.into());
        self
    }

    /// Set the `benzinga_firm_id_gte` argument.
    pub fn benzinga_firm_id_gte(mut self, benzinga_firm_id_gte: impl Into<String>) -> Self {
        self.benzinga_firm_id_gte = Some(benzinga_firm_id_gte.into());
        self
    }

    /// Set the `benzinga_firm_id_lt` argument.
    pub fn benzinga_firm_id_lt(mut self, benzinga_firm_id_lt: impl Into<String>) -> Self {
        self.benzinga_firm_id_lt = Some(benzinga_firm_id_lt.into());
        self
    }

    /// Set the `benzinga_firm_id_lte` argument.
    pub fn benzinga_firm_id_lte(mut self, benzinga_firm_id_lte: impl Into<String>) -> Self {
        self.benzinga_firm_id_lte = Some(benzinga_firm_id_lte.into());
        self
    }

    /// Set the `firm_name` argument.
    pub fn firm_name(mut self, firm_name: impl Into<String>) -> Self {
        self.firm_name = Some(firm_name.into());
        self
    }

    /// Set the `firm_name_any_of` argument.
    pub fn firm_name_any_of(mut self, firm_name_any_of: impl Into<String>) -> Self {
        self.firm_name_any_of = Some(firm_name_any_of.into());
        self
    }

    /// Set the `firm_name_gt` argument.
    pub fn firm_name_gt(mut self, firm_name_gt: impl Into<String>) -> Self {
        self.firm_name_gt = Some(firm_name_gt.into());
        self
    }

    /// Set the `firm_name_gte` argument.
    pub fn firm_name_gte(mut self, firm_name_gte: impl Into<String>) -> Self {
        self.firm_name_gte = Some(firm_name_gte.into());
        self
    }

    /// Set the `firm_name_lt` argument.
    pub fn firm_name_lt(mut self, firm_name_lt: impl Into<String>) -> Self {
        self.firm_name_lt = Some(firm_name_lt.into());
        self
    }

    /// Set the `firm_name_lte` argument.
    pub fn firm_name_lte(mut self, firm_name_lte: impl Into<String>) -> Self {
        self.firm_name_lte = Some(firm_name_lte.into());
        self
    }

    /// Set the `full_name` argument.
    pub fn full_name(mut self, full_name: impl Into<String>) -> Self {
        self.full_name = Some(full_name.into());
        self
    }

    /// Set the `full_name_any_of` argument.
    pub fn full_name_any_of(mut self, full_name_any_of: impl Into<String>) -> Self {
        self.full_name_any_of = Some(full_name_any_of.into());
        self
    }

    /// Set the `full_name_gt` argument.
    pub fn full_name_gt(mut self, full_name_gt: impl Into<String>) -> Self {
        self.full_name_gt = Some(full_name_gt.into());
        self
    }

    /// Set the `full_name_gte` argument.
    pub fn full_name_gte(mut self, full_name_gte: impl Into<String>) -> Self {
        self.full_name_gte = Some(full_name_gte.into());
        self
    }

    /// Set the `full_name_lt` argument.
    pub fn full_name_lt(mut self, full_name_lt: impl Into<String>) -> Self {
        self.full_name_lt = Some(full_name_lt.into());
        self
    }

    /// Set the `full_name_lte` argument.
    pub fn full_name_lte(mut self, full_name_lte: impl Into<String>) -> Self {
        self.full_name_lte = Some(full_name_lte.into());
        self
    }

    /// Set the `limit` argument.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the `sort` argument.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`BenzingaApi::list_benzinga_consensus_ratings`].
#[derive(Debug, Default, Clone)]
pub struct ListBenzingaConsensusRatingsParams {
    /// The `date` argument.
    pub date: Option<String>,
    /// The `date_gt` argument.
    pub date_gt: Option<String>,
    /// The `date_gte` argument.
    pub date_gte: Option<String>,
    /// The `date_lt` argument.
    pub date_lt: Option<String>,
    /// The `date_lte` argument.
    pub date_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListBenzingaConsensusRatingsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `date` argument.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }

    /// Set the `date_gt` argument.
    pub fn date_gt(mut self, date_gt: impl Into<String>) -> Self {
        self.date_gt = Some(date_gt.into());
        self
    }

    /// Set the `date_gte` argument.
    pub fn date_gte(mut self, date_gte: impl Into<String>) -> Self {
        self.date_gte = Some(date_gte.into());
        self
    }

    /// Set the `date_lt` argument.
    pub fn date_lt(mut self, date_lt: impl Into<String>) -> Self {
        self.date_lt = Some(date_lt.into());
        self
    }

    /// Set the `date_lte` argument.
    pub fn date_lte(mut self, date_lte: impl Into<String>) -> Self {
        self.date_lte = Some(date_lte.into());
        self
    }

    /// Set the `limit` argument.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`BenzingaApi::list_benzinga_earnings`].
#[derive(Debug, Default, Clone)]
pub struct ListBenzingaEarningsParams {
    /// The `date` argument.
    pub date: Option<String>,
    /// The `date_any_of` argument.
    pub date_any_of: Option<String>,
    /// The `date_gt` argument.
    pub date_gt: Option<String>,
    /// The `date_gte` argument.
    pub date_gte: Option<String>,
    /// The `date_lt` argument.
    pub date_lt: Option<String>,
    /// The `date_lte` argument.
    pub date_lte: Option<String>,
    /// The `ticker` argument.
    pub ticker: Option<String>,
    /// The `ticker_any_of` argument.
    pub ticker_any_of: Option<String>,
    /// The `ticker_gt` argument.
    pub ticker_gt: Option<String>,
    /// The `ticker_gte` argument.
    pub ticker_gte: Option<String>,
    /// The `ticker_lt` argument.
    pub ticker_lt: Option<String>,
    /// The `ticker_lte` argument.
    pub ticker_lte: Option<String>,
    /// The `importance` argument.
    pub importance: Option<i64>,
    /// The `importance_any_of` argument.
    pub importance_any_of: Option<String>,
    /// The `importance_gt` argument.
    pub importance_gt: Option<i64>,
    /// The `importance_gte` argument.
    pub importance_gte: Option<i64>,
    /// The `importance_lt` argument.
    pub importance_lt: Option<i64>,
    /// The `importance_lte` argument.
    pub importance_lte: Option<i64>,
    /// The `last_updated` argument.
    pub last_updated: Option<String>,
    /// The `last_updated_any_of` argument.
    pub last_updated_any_of: Option<String>,
    /// The `last_updated_gt` argument.
    pub last_updated_gt: Option<String>,
    /// The `last_updated_gte` argument.
    pub last_updated_gte: Option<String>,
    /// The `last_updated_lt` argument.
    pub last_updated_lt: Option<String>,
    /// The `last_updated_lte` argument.
    pub last_updated_lte: Option<String>,
    /// The `date_status` argument.
    pub date_status: Option<String>,
    /// The `date_status_any_of` argument.
    pub date_status_any_of: Option<String>,
    /// The `date_status_gt` argument.
    pub date_status_gt: Option<String>,
    /// The `date_status_gte` argument.
    pub date_status_gte: Option<String>,
    /// The `date_status_lt` argument.
    pub date_status_lt: Option<String>,
    /// The `date_status_lte` argument.
    pub date_status_lte: Option<String>,
    /// The `eps_surprise_percent` argument.
    pub eps_surprise_percent: Option<f64>,
    /// The `eps_surprise_percent_any_of` argument.
    pub eps_surprise_percent_any_of: Option<String>,
    /// The `eps_surprise_percent_gt` argument.
    pub eps_surprise_percent_gt: Option<f64>,
    /// The `eps_surprise_percent_gte` argument.
    pub eps_surprise_percent_gte: Option<f64>,
    /// The `eps_surprise_percent_lt` argument.
    pub eps_surprise_percent_lt: Option<f64>,
    /// The `eps_surprise_percent_lte` argument.
    pub eps_surprise_percent_lte: Option<f64>,
    /// The `revenue_surprise_percent` argument.
    pub revenue_surprise_percent: Option<f64>,
    /// The `revenue_surprise_percent_any_of` argument.
    pub revenue_surprise_percent_any_of: Option<String>,
    /// The `revenue_surprise_percent_gt` argument.
    pub revenue_surprise_percent_gt: Option<f64>,
    /// The `revenue_surprise_percent_gte` argument.
    pub revenue_surprise_percent_gte: Option<f64>,
    /// The `revenue_surprise_percent_lt` argument.
    pub revenue_surprise_percent_lt: Option<f64>,
    /// The `revenue_surprise_percent_lte` argument.
    pub revenue_surprise_percent_lte: Option<f64>,
    /// The `fiscal_year` argument.
    pub fiscal_year: Option<i64>,
    /// The `fiscal_year_any_of` argument.
    pub fiscal_year_any_of: Option<String>,
    /// The `fiscal_year_gt` argument.
    pub fiscal_year_gt: Option<i64>,
    /// The `fiscal_year_gte` argument.
    pub fiscal_year_gte: Option<i64>,
    /// The `fiscal_year_lt` argument.
    pub fiscal_year_lt: Option<i64>,
    /// The `fiscal_year_lte` argument.
    pub fiscal_year_lte: Option<i64>,
    /// The `fiscal_period` argument.
    pub fiscal_period: Option<String>,
    /// The `fiscal_period_any_of` argument.
    pub fiscal_period_any_of: Option<String>,
    /// The `fiscal_period_gt` argument.
    pub fiscal_period_gt: Option<String>,
    /// The `fiscal_period_gte` argument.
    pub fiscal_period_gte: Option<String>,
    /// The `fiscal_period_lt` argument.
    pub fiscal_period_lt: Option<String>,
    /// The `fiscal_period_lte` argument.
    pub fiscal_period_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListBenzingaEarningsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `date` argument.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }

    /// Set the `date_any_of` argument.
    pub fn date_any_of(mut self, date_any_of: impl Into<String>) -> Self {
        self.date_any_of = Some(date_any_of.into());
        self
    }

    /// Set the `date_gt` argument.
    pub fn date_gt(mut self, date_gt: impl Into<String>) -> Self {
        self.date_gt = Some(date_gt.into());
        self
    }

    /// Set the `date_gte` argument.
    pub fn date_gte(mut self, date_gte: impl Into<String>) -> Self {
        self.date_gte = Some(date_gte.into());
        self
    }

    /// Set the `date_lt` argument.
    pub fn date_lt(mut self, date_lt: impl Into<String>) -> Self {
        self.date_lt = Some(date_lt.into());
        self
    }

    /// Set the `date_lte` argument.
    pub fn date_lte(mut self, date_lte: impl Into<String>) -> Self {
        self.date_lte = Some(date_lte.into());
        self
    }

    /// Set the `ticker` argument.
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    /// Set the `ticker_any_of` argument.
    pub fn ticker_any_of(mut self, ticker_any_of: impl Into<String>) -> Self {
        self.ticker_any_of = Some(ticker_any_of.into());
        self
    }

    /// Set the `ticker_gt` argument.
    pub fn ticker_gt(mut self, ticker_gt: impl Into<String>) -> Self {
        self.ticker_gt = Some(ticker_gt.into());
        self
    }

    /// Set the `ticker_gte` argument.
    pub fn ticker_gte(mut self, ticker_gte: impl Into<String>) -> Self {
        self.ticker_gte = Some(ticker_gte.into());
        self
    }

    /// Set the `ticker_lt` argument.
    pub fn ticker_lt(mut self, ticker_lt: impl Into<String>) -> Self {
        self.ticker_lt = Some(ticker_lt.into());
        self
    }

    /// Set the `ticker_lte` argument.
    pub fn ticker_lte(mut self, ticker_lte: impl Into<String>) -> Self {
        self.ticker_lte = Some(ticker_lte.into());
        self
    }

    /// Set the `importance` argument.
    pub fn importance(mut self, importance: i64) -> Self {
        self.importance = Some(importance);
        self
    }

    /// Set the `importance_any_of` argument.
    pub fn importance_any_of(mut self, importance_any_of: impl Into<String>) -> Self {
        self.importance_any_of = Some(importance_any_of.into());
        self
    }

    /// Set the `importance_gt` argument.
    pub fn importance_gt(mut self, importance_gt: i64) -> Self {
        self.importance_gt = Some(importance_gt);
        self
    }

    /// Set the `importance_gte` argument.
    pub fn importance_gte(mut self, importance_gte: i64) -> Self {
        self.importance_gte = Some(importance_gte);
        self
    }

    /// Set the `importance_lt` argument.
    pub fn importance_lt(mut self, importance_lt: i64) -> Self {
        self.importance_lt = Some(importance_lt);
        self
    }

    /// Set the `importance_lte` argument.
    pub fn importance_lte(mut self, importance_lte: i64) -> Self {
        self.importance_lte = Some(importance_lte);
        self
    }

    /// Set the `last_updated` argument.
    pub fn last_updated(mut self, last_updated: impl Into<String>) -> Self {
        self.last_updated = Some(last_updated.into());
        self
    }

    /// Set the `last_updated_any_of` argument.
    pub fn last_updated_any_of(mut self, last_updated_any_of: impl Into<String>) -> Self {
        self.last_updated_any_of = Some(last_updated_any_of.into());
        self
    }

    /// Set the `last_updated_gt` argument.
    pub fn last_updated_gt(mut self, last_updated_gt: impl Into<String>) -> Self {
        self.last_updated_gt = Some(last_updated_gt.into());
        self
    }

    /// Set the `last_updated_gte` argument.
    pub fn last_updated_gte(mut self, last_updated_gte: impl Into<String>) -> Self {
        self.last_updated_gte = Some(last_updated_gte.into());
        self
    }

    /// Set the `last_updated_lt` argument.
    pub fn last_updated_lt(mut self, last_updated_lt: impl Into<String>) -> Self {
        self.last_updated_lt = Some(last_updated_lt.into());
        self
    }

    /// Set the `last_updated_lte` argument.
    pub fn last_updated_lte(mut self, last_updated_lte: impl Into<String>) -> Self {
        self.last_updated_lte = Some(last_updated_lte.into());
        self
    }

    /// Set the `date_status` argument.
    pub fn date_status(mut self, date_status: impl Into<String>) -> Self {
        self.date_status = Some(date_status.into());
        self
    }

    /// Set the `date_status_any_of` argument.
    pub fn date_status_any_of(mut self, date_status_any_of: impl Into<String>) -> Self {
        self.date_status_any_of = Some(date_status_any_of.into());
        self
    }

    /// Set the `date_status_gt` argument.
    pub fn date_status_gt(mut self, date_status_gt: impl Into<String>) -> Self {
        self.date_status_gt = Some(date_status_gt.into());
        self
    }

    /// Set the `date_status_gte` argument.
    pub fn date_status_gte(mut self, date_status_gte: impl Into<String>) -> Self {
        self.date_status_gte = Some(date_status_gte.into());
        self
    }

    /// Set the `date_status_lt` argument.
    pub fn date_status_lt(mut self, date_status_lt: impl Into<String>) -> Self {
        self.date_status_lt = Some(date_status_lt.into());
        self
    }

    /// Set the `date_status_lte` argument.
    pub fn date_status_lte(mut self, date_status_lte: impl Into<String>) -> Self {
        self.date_status_lte = Some(date_status_lte.into());
        self
    }

    /// Set the `eps_surprise_percent` argument.
    pub fn eps_surprise_percent(mut self, eps_surprise_percent: f64) -> Self {
        self.eps_surprise_percent = Some(eps_surprise_percent);
        self
    }

    /// Set the `eps_surprise_percent_any_of` argument.
    pub fn eps_surprise_percent_any_of(
        mut self,
        eps_surprise_percent_any_of: impl Into<String>,
    ) -> Self {
        self.eps_surprise_percent_any_of = Some(eps_surprise_percent_any_of.into());
        self
    }

    /// Set the `eps_surprise_percent_gt` argument.
    pub fn eps_surprise_percent_gt(mut self, eps_surprise_percent_gt: f64) -> Self {
        self.eps_surprise_percent_gt = Some(eps_surprise_percent_gt);
        self
    }

    /// Set the `eps_surprise_percent_gte` argument.
    pub fn eps_surprise_percent_gte(mut self, eps_surprise_percent_gte: f64) -> Self {
        self.eps_surprise_percent_gte = Some(eps_surprise_percent_gte);
        self
    }

    /// Set the `eps_surprise_percent_lt` argument.
    pub fn eps_surprise_percent_lt(mut self, eps_surprise_percent_lt: f64) -> Self {
        self.eps_surprise_percent_lt = Some(eps_surprise_percent_lt);
        self
    }

    /// Set the `eps_surprise_percent_lte` argument.
    pub fn eps_surprise_percent_lte(mut self, eps_surprise_percent_lte: f64) -> Self {
        self.eps_surprise_percent_lte = Some(eps_surprise_percent_lte);
        self
    }

    /// Set the `revenue_surprise_percent` argument.
    pub fn revenue_surprise_percent(mut self, revenue_surprise_percent: f64) -> Self {
        self.revenue_surprise_percent = Some(revenue_surprise_percent);
        self
    }

    /// Set the `revenue_surprise_percent_any_of` argument.
    pub fn revenue_surprise_percent_any_of(
        mut self,
        revenue_surprise_percent_any_of: impl Into<String>,
    ) -> Self {
        self.revenue_surprise_percent_any_of = Some(revenue_surprise_percent_any_of.into());
        self
    }

    /// Set the `revenue_surprise_percent_gt` argument.
    pub fn revenue_surprise_percent_gt(mut self, revenue_surprise_percent_gt: f64) -> Self {
        self.revenue_surprise_percent_gt = Some(revenue_surprise_percent_gt);
        self
    }

    /// Set the `revenue_surprise_percent_gte` argument.
    pub fn revenue_surprise_percent_gte(mut self, revenue_surprise_percent_gte: f64) -> Self {
        self.revenue_surprise_percent_gte = Some(revenue_surprise_percent_gte);
        self
    }

    /// Set the `revenue_surprise_percent_lt` argument.
    pub fn revenue_surprise_percent_lt(mut self, revenue_surprise_percent_lt: f64) -> Self {
        self.revenue_surprise_percent_lt = Some(revenue_surprise_percent_lt);
        self
    }

    /// Set the `revenue_surprise_percent_lte` argument.
    pub fn revenue_surprise_percent_lte(mut self, revenue_surprise_percent_lte: f64) -> Self {
        self.revenue_surprise_percent_lte = Some(revenue_surprise_percent_lte);
        self
    }

    /// Set the `fiscal_year` argument.
    pub fn fiscal_year(mut self, fiscal_year: i64) -> Self {
        self.fiscal_year = Some(fiscal_year);
        self
    }

    /// Set the `fiscal_year_any_of` argument.
    pub fn fiscal_year_any_of(mut self, fiscal_year_any_of: impl Into<String>) -> Self {
        self.fiscal_year_any_of = Some(fiscal_year_any_of.into());
        self
    }

    /// Set the `fiscal_year_gt` argument.
    pub fn fiscal_year_gt(mut self, fiscal_year_gt: i64) -> Self {
        self.fiscal_year_gt = Some(fiscal_year_gt);
        self
    }

    /// Set the `fiscal_year_gte` argument.
    pub fn fiscal_year_gte(mut self, fiscal_year_gte: i64) -> Self {
        self.fiscal_year_gte = Some(fiscal_year_gte);
        self
    }

    /// Set the `fiscal_year_lt` argument.
    pub fn fiscal_year_lt(mut self, fiscal_year_lt: i64) -> Self {
        self.fiscal_year_lt = Some(fiscal_year_lt);
        self
    }

    /// Set the `fiscal_year_lte` argument.
    pub fn fiscal_year_lte(mut self, fiscal_year_lte: i64) -> Self {
        self.fiscal_year_lte = Some(fiscal_year_lte);
        self
    }

    /// Set the `fiscal_period` argument.
    pub fn fiscal_period(mut self, fiscal_period: impl Into<String>) -> Self {
        self.fiscal_period = Some(fiscal_period.into());
        self
    }

    /// Set the `fiscal_period_any_of` argument.
    pub fn fiscal_period_any_of(mut self, fiscal_period_any_of: impl Into<String>) -> Self {
        self.fiscal_period_any_of = Some(fiscal_period_any_of.into());
        self
    }

    /// Set the `fiscal_period_gt` argument.
    pub fn fiscal_period_gt(mut self, fiscal_period_gt: impl Into<String>) -> Self {
        self.fiscal_period_gt = Some(fiscal_period_gt.into());
        self
    }

    /// Set the `fiscal_period_gte` argument.
    pub fn fiscal_period_gte(mut self, fiscal_period_gte: impl Into<String>) -> Self {
        self.fiscal_period_gte = Some(fiscal_period_gte.into());
        self
    }

    /// Set the `fiscal_period_lt` argument.
    pub fn fiscal_period_lt(mut self, fiscal_period_lt: impl Into<String>) -> Self {
        self.fiscal_period_lt = Some(fiscal_period_lt.into());
        self
    }

    /// Set the `fiscal_period_lte` argument.
    pub fn fiscal_period_lte(mut self, fiscal_period_lte: impl Into<String>) -> Self {
        self.fiscal_period_lte = Some(fiscal_period_lte.into());
        self
    }

    /// Set the `limit` argument.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the `sort` argument.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`BenzingaApi::list_benzinga_firms`].
#[derive(Debug, Default, Clone)]
pub struct ListBenzingaFirmsParams {
    /// The `benzinga_id` argument.
    pub benzinga_id: Option<String>,
    /// The `benzinga_id_any_of` argument.
    pub benzinga_id_any_of: Option<String>,
    /// The `benzinga_id_gt` argument.
    pub benzinga_id_gt: Option<String>,
    /// The `benzinga_id_gte` argument.
    pub benzinga_id_gte: Option<String>,
    /// The `benzinga_id_lt` argument.
    pub benzinga_id_lt: Option<String>,
    /// The `benzinga_id_lte` argument.
    pub benzinga_id_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListBenzingaFirmsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `benzinga_id` argument.
    pub fn benzinga_id(mut self, benzinga_id: impl Into<String>) -> Self {
        self.benzinga_id = Some(benzinga_id.into());
        self
    }

    /// Set the `benzinga_id_any_of` argument.
    pub fn benzinga_id_any_of(mut self, benzinga_id_any_of: impl Into<String>) -> Self {
        self.benzinga_id_any_of = Some(benzinga_id_any_of.into());
        self
    }

    /// Set the `benzinga_id_gt` argument.
    pub fn benzinga_id_gt(mut self, benzinga_id_gt: impl Into<String>) -> Self {
        self.benzinga_id_gt = Some(benzinga_id_gt.into());
        self
    }

    /// Set the `benzinga_id_gte` argument.
    pub fn benzinga_id_gte(mut self, benzinga_id_gte: impl Into<String>) -> Self {
        self.benzinga_id_gte = Some(benzinga_id_gte.into());
        self
    }

    /// Set the `benzinga_id_lt` argument.
    pub fn benzinga_id_lt(mut self, benzinga_id_lt: impl Into<String>) -> Self {
        self.benzinga_id_lt = Some(benzinga_id_lt.into());
        self
    }

    /// Set the `benzinga_id_lte` argument.
    pub fn benzinga_id_lte(mut self, benzinga_id_lte: impl Into<String>) -> Self {
        self.benzinga_id_lte = Some(benzinga_id_lte.into());
        self
    }

    /// Set the `limit` argument.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the `sort` argument.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`BenzingaApi::list_benzinga_guidance`].
#[derive(Debug, Default, Clone)]
pub struct ListBenzingaGuidanceParams {
    /// The `date` argument.
    pub date: Option<String>,
    /// The `date_any_of` argument.
    pub date_any_of: Option<String>,
    /// The `date_gt` argument.
    pub date_gt: Option<String>,
    /// The `date_gte` argument.
    pub date_gte: Option<String>,
    /// The `date_lt` argument.
    pub date_lt: Option<String>,
    /// The `date_lte` argument.
    pub date_lte: Option<String>,
    /// The `ticker` argument.
    pub ticker: Option<String>,
    /// The `ticker_any_of` argument.
    pub ticker_any_of: Option<String>,
    /// The `ticker_gt` argument.
    pub ticker_gt: Option<String>,
    /// The `ticker_gte` argument.
    pub ticker_gte: Option<String>,
    /// The `ticker_lt` argument.
    pub ticker_lt: Option<String>,
    /// The `ticker_lte` argument.
    pub ticker_lte: Option<String>,
    /// The `positioning` argument.
    pub positioning: Option<String>,
    /// The `positioning_any_of` argument.
    pub positioning_any_of: Option<String>,
    /// The `positioning_gt` argument.
    pub positioning_gt: Option<String>,
    /// The `positioning_gte` argument.
    pub positioning_gte: Option<String>,
    /// The `positioning_lt` argument.
    pub positioning_lt: Option<String>,
    /// The `positioning_lte` argument.
    pub positioning_lte: Option<String>,
    /// The `importance` argument.
    pub importance: Option<i64>,
    /// The `importance_any_of` argument.
    pub importance_any_of: Option<String>,
    /// The `importance_gt` argument.
    pub importance_gt: Option<i64>,
    /// The `importance_gte` argument.
    pub importance_gte: Option<i64>,
    /// The `importance_lt` argument.
    pub importance_lt: Option<i64>,
    /// The `importance_lte` argument.
    pub importance_lte: Option<i64>,
    /// The `last_updated` argument.
    pub last_updated: Option<String>,
    /// The `last_updated_any_of` argument.
    pub last_updated_any_of: Option<String>,
    /// The `last_updated_gt` argument.
    pub last_updated_gt: Option<String>,
    /// The `last_updated_gte` argument.
    pub last_updated_gte: Option<String>,
    /// The `last_updated_lt` argument.
    pub last_updated_lt: Option<String>,
    /// The `last_updated_lte` argument.
    pub last_updated_lte: Option<String>,
    /// The `fiscal_year` argument.
    pub fiscal_year: Option<i64>,
    /// The `fiscal_year_any_of` argument.
    pub fiscal_year_any_of: Option<String>,
    /// The `fiscal_year_gt` argument.
    pub fiscal_year_gt: Option<i64>,
    /// The `fiscal_year_gte` argument.
    pub fiscal_year_gte: Option<i64>,
    /// The `fiscal_year_lt` argument.
    pub fiscal_year_lt: Option<i64>,
    /// The `fiscal_year_lte` argument.
    pub fiscal_year_lte: Option<i64>,
    /// The `fiscal_period` argument.
    pub fiscal_period: Option<String>,
    /// The `fiscal_period_any_of` argument.
    pub fiscal_period_any_of: Option<String>,
    /// The `fiscal_period_gt` argument.
    pub fiscal_period_gt: Option<String>,
    /// The `fiscal_period_gte` argument.
    pub fiscal_period_gte: Option<String>,
    /// The `fiscal_period_lt` argument.
    pub fiscal_period_lt: Option<String>,
    /// The `fiscal_period_lte` argument.
    pub fiscal_period_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListBenzingaGuidanceParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `date` argument.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }

    /// Set the `date_any_of` argument.
    pub fn date_any_of(mut self, date_any_of: impl Into<String>) -> Self {
        self.date_any_of = Some(date_any_of.into());
        self
    }

    /// Set the `date_gt` argument.
    pub fn date_gt(mut self, date_gt: impl Into<String>) -> Self {
        self.date_gt = Some(date_gt.into());
        self
    }

    /// Set the `date_gte` argument.
    pub fn date_gte(mut self, date_gte: impl Into<String>) -> Self {
        self.date_gte = Some(date_gte.into());
        self
    }

    /// Set the `date_lt` argument.
    pub fn date_lt(mut self, date_lt: impl Into<String>) -> Self {
        self.date_lt = Some(date_lt.into());
        self
    }

    /// Set the `date_lte` argument.
    pub fn date_lte(mut self, date_lte: impl Into<String>) -> Self {
        self.date_lte = Some(date_lte.into());
        self
    }

    /// Set the `ticker` argument.
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    /// Set the `ticker_any_of` argument.
    pub fn ticker_any_of(mut self, ticker_any_of: impl Into<String>) -> Self {
        self.ticker_any_of = Some(ticker_any_of.into());
        self
    }

    /// Set the `ticker_gt` argument.
    pub fn ticker_gt(mut self, ticker_gt: impl Into<String>) -> Self {
        self.ticker_gt = Some(ticker_gt.into());
        self
    }

    /// Set the `ticker_gte` argument.
    pub fn ticker_gte(mut self, ticker_gte: impl Into<String>) -> Self {
        self.ticker_gte = Some(ticker_gte.into());
        self
    }

    /// Set the `ticker_lt` argument.
    pub fn ticker_lt(mut self, ticker_lt: impl Into<String>) -> Self {
        self.ticker_lt = Some(ticker_lt.into());
        self
    }

    /// Set the `ticker_lte` argument.
    pub fn ticker_lte(mut self, ticker_lte: impl Into<String>) -> Self {
        self.ticker_lte = Some(ticker_lte.into());
        self
    }

    /// Set the `positioning` argument.
    pub fn positioning(mut self, positioning: impl Into<String>) -> Self {
        self.positioning = Some(positioning.into());
        self
    }

    /// Set the `positioning_any_of` argument.
    pub fn positioning_any_of(mut self, positioning_any_of: impl Into<String>) -> Self {
        self.positioning_any_of = Some(positioning_any_of.into());
        self
    }

    /// Set the `positioning_gt` argument.
    pub fn positioning_gt(mut self, positioning_gt: impl Into<String>) -> Self {
        self.positioning_gt = Some(positioning_gt.into());
        self
    }

    /// Set the `positioning_gte` argument.
    pub fn positioning_gte(mut self, positioning_gte: impl Into<String>) -> Self {
        self.positioning_gte = Some(positioning_gte.into());
        self
    }

    /// Set the `positioning_lt` argument.
    pub fn positioning_lt(mut self, positioning_lt: impl Into<String>) -> Self {
        self.positioning_lt = Some(positioning_lt.into());
        self
    }

    /// Set the `positioning_lte` argument.
    pub fn positioning_lte(mut self, positioning_lte: impl Into<String>) -> Self {
        self.positioning_lte = Some(positioning_lte.into());
        self
    }

    /// Set the `importance` argument.
    pub fn importance(mut self, importance: i64) -> Self {
        self.importance = Some(importance);
        self
    }

    /// Set the `importance_any_of` argument.
    pub fn importance_any_of(mut self, importance_any_of: impl Into<String>) -> Self {
        self.importance_any_of = Some(importance_any_of.into());
        self
    }

    /// Set the `importance_gt` argument.
    pub fn importance_gt(mut self, importance_gt: i64) -> Self {
        self.importance_gt = Some(importance_gt);
        self
    }

    /// Set the `importance_gte` argument.
    pub fn importance_gte(mut self, importance_gte: i64) -> Self {
        self.importance_gte = Some(importance_gte);
        self
    }

    /// Set the `importance_lt` argument.
    pub fn importance_lt(mut self, importance_lt: i64) -> Self {
        self.importance_lt = Some(importance_lt);
        self
    }

    /// Set the `importance_lte` argument.
    pub fn importance_lte(mut self, importance_lte: i64) -> Self {
        self.importance_lte = Some(importance_lte);
        self
    }

    /// Set the `last_updated` argument.
    pub fn last_updated(mut self, last_updated: impl Into<String>) -> Self {
        self.last_updated = Some(last_updated.into());
        self
    }

    /// Set the `last_updated_any_of` argument.
    pub fn last_updated_any_of(mut self, last_updated_any_of: impl Into<String>) -> Self {
        self.last_updated_any_of = Some(last_updated_any_of.into());
        self
    }

    /// Set the `last_updated_gt` argument.
    pub fn last_updated_gt(mut self, last_updated_gt: impl Into<String>) -> Self {
        self.last_updated_gt = Some(last_updated_gt.into());
        self
    }

    /// Set the `last_updated_gte` argument.
    pub fn last_updated_gte(mut self, last_updated_gte: impl Into<String>) -> Self {
        self.last_updated_gte = Some(last_updated_gte.into());
        self
    }

    /// Set the `last_updated_lt` argument.
    pub fn last_updated_lt(mut self, last_updated_lt: impl Into<String>) -> Self {
        self.last_updated_lt = Some(last_updated_lt.into());
        self
    }

    /// Set the `last_updated_lte` argument.
    pub fn last_updated_lte(mut self, last_updated_lte: impl Into<String>) -> Self {
        self.last_updated_lte = Some(last_updated_lte.into());
        self
    }

    /// Set the `fiscal_year` argument.
    pub fn fiscal_year(mut self, fiscal_year: i64) -> Self {
        self.fiscal_year = Some(fiscal_year);
        self
    }

    /// Set the `fiscal_year_any_of` argument.
    pub fn fiscal_year_any_of(mut self, fiscal_year_any_of: impl Into<String>) -> Self {
        self.fiscal_year_any_of = Some(fiscal_year_any_of.into());
        self
    }

    /// Set the `fiscal_year_gt` argument.
    pub fn fiscal_year_gt(mut self, fiscal_year_gt: i64) -> Self {
        self.fiscal_year_gt = Some(fiscal_year_gt);
        self
    }

    /// Set the `fiscal_year_gte` argument.
    pub fn fiscal_year_gte(mut self, fiscal_year_gte: i64) -> Self {
        self.fiscal_year_gte = Some(fiscal_year_gte);
        self
    }

    /// Set the `fiscal_year_lt` argument.
    pub fn fiscal_year_lt(mut self, fiscal_year_lt: i64) -> Self {
        self.fiscal_year_lt = Some(fiscal_year_lt);
        self
    }

    /// Set the `fiscal_year_lte` argument.
    pub fn fiscal_year_lte(mut self, fiscal_year_lte: i64) -> Self {
        self.fiscal_year_lte = Some(fiscal_year_lte);
        self
    }

    /// Set the `fiscal_period` argument.
    pub fn fiscal_period(mut self, fiscal_period: impl Into<String>) -> Self {
        self.fiscal_period = Some(fiscal_period.into());
        self
    }

    /// Set the `fiscal_period_any_of` argument.
    pub fn fiscal_period_any_of(mut self, fiscal_period_any_of: impl Into<String>) -> Self {
        self.fiscal_period_any_of = Some(fiscal_period_any_of.into());
        self
    }

    /// Set the `fiscal_period_gt` argument.
    pub fn fiscal_period_gt(mut self, fiscal_period_gt: impl Into<String>) -> Self {
        self.fiscal_period_gt = Some(fiscal_period_gt.into());
        self
    }

    /// Set the `fiscal_period_gte` argument.
    pub fn fiscal_period_gte(mut self, fiscal_period_gte: impl Into<String>) -> Self {
        self.fiscal_period_gte = Some(fiscal_period_gte.into());
        self
    }

    /// Set the `fiscal_period_lt` argument.
    pub fn fiscal_period_lt(mut self, fiscal_period_lt: impl Into<String>) -> Self {
        self.fiscal_period_lt = Some(fiscal_period_lt.into());
        self
    }

    /// Set the `fiscal_period_lte` argument.
    pub fn fiscal_period_lte(mut self, fiscal_period_lte: impl Into<String>) -> Self {
        self.fiscal_period_lte = Some(fiscal_period_lte.into());
        self
    }

    /// Set the `limit` argument.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the `sort` argument.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`BenzingaApi::list_benzinga_news`].
#[derive(Debug, Default, Clone)]
pub struct ListBenzingaNewsParams {
    /// The `published` argument.
    pub published: Option<String>,
    /// The `published_any_of` argument.
    pub published_any_of: Option<String>,
    /// The `published_gt` argument.
    pub published_gt: Option<String>,
    /// The `published_gte` argument.
    pub published_gte: Option<String>,
    /// The `published_lt` argument.
    pub published_lt: Option<String>,
    /// The `published_lte` argument.
    pub published_lte: Option<String>,
    /// The `last_updated` argument.
    pub last_updated: Option<String>,
    /// The `last_updated_any_of` argument.
    pub last_updated_any_of: Option<String>,
    /// The `last_updated_gt` argument.
    pub last_updated_gt: Option<String>,
    /// The `last_updated_gte` argument.
    pub last_updated_gte: Option<String>,
    /// The `last_updated_lt` argument.
    pub last_updated_lt: Option<String>,
    /// The `last_updated_lte` argument.
    pub last_updated_lte: Option<String>,
    /// The `tickers` argument.
    pub tickers: Option<String>,
    /// The `tickers_all_of` argument.
    pub tickers_all_of: Option<String>,
    /// The `tickers_any_of` argument.
    pub tickers_any_of: Option<String>,
    /// The `channels` argument.
    pub channels: Option<String>,
    /// The `channels_all_of` argument.
    pub channels_all_of: Option<String>,
    /// The `channels_any_of` argument.
    pub channels_any_of: Option<String>,
    /// The `tags` argument.
    pub tags: Option<String>,
    /// The `tags_all_of` argument.
    pub tags_all_of: Option<String>,
    /// The `tags_any_of` argument.
    pub tags_any_of: Option<String>,
    /// The `author` argument.
    pub author: Option<String>,
    /// The `author_any_of` argument.
    pub author_any_of: Option<String>,
    /// The `author_gt` argument.
    pub author_gt: Option<String>,
    /// The `author_gte` argument.
    pub author_gte: Option<String>,
    /// The `author_lt` argument.
    pub author_lt: Option<String>,
    /// The `author_lte` argument.
    pub author_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListBenzingaNewsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `published` argument.
    pub fn published(mut self, published: impl Into<String>) -> Self {
        self.published = Some(published.into());
        self
    }

    /// Set the `published_any_of` argument.
    pub fn published_any_of(mut self, published_any_of: impl Into<String>) -> Self {
        self.published_any_of = Some(published_any_of.into());
        self
    }

    /// Set the `published_gt` argument.
    pub fn published_gt(mut self, published_gt: impl Into<String>) -> Self {
        self.published_gt = Some(published_gt.into());
        self
    }

    /// Set the `published_gte` argument.
    pub fn published_gte(mut self, published_gte: impl Into<String>) -> Self {
        self.published_gte = Some(published_gte.into());
        self
    }

    /// Set the `published_lt` argument.
    pub fn published_lt(mut self, published_lt: impl Into<String>) -> Self {
        self.published_lt = Some(published_lt.into());
        self
    }

    /// Set the `published_lte` argument.
    pub fn published_lte(mut self, published_lte: impl Into<String>) -> Self {
        self.published_lte = Some(published_lte.into());
        self
    }

    /// Set the `last_updated` argument.
    pub fn last_updated(mut self, last_updated: impl Into<String>) -> Self {
        self.last_updated = Some(last_updated.into());
        self
    }

    /// Set the `last_updated_any_of` argument.
    pub fn last_updated_any_of(mut self, last_updated_any_of: impl Into<String>) -> Self {
        self.last_updated_any_of = Some(last_updated_any_of.into());
        self
    }

    /// Set the `last_updated_gt` argument.
    pub fn last_updated_gt(mut self, last_updated_gt: impl Into<String>) -> Self {
        self.last_updated_gt = Some(last_updated_gt.into());
        self
    }

    /// Set the `last_updated_gte` argument.
    pub fn last_updated_gte(mut self, last_updated_gte: impl Into<String>) -> Self {
        self.last_updated_gte = Some(last_updated_gte.into());
        self
    }

    /// Set the `last_updated_lt` argument.
    pub fn last_updated_lt(mut self, last_updated_lt: impl Into<String>) -> Self {
        self.last_updated_lt = Some(last_updated_lt.into());
        self
    }

    /// Set the `last_updated_lte` argument.
    pub fn last_updated_lte(mut self, last_updated_lte: impl Into<String>) -> Self {
        self.last_updated_lte = Some(last_updated_lte.into());
        self
    }

    /// Set the `tickers` argument.
    pub fn tickers(mut self, tickers: impl Into<String>) -> Self {
        self.tickers = Some(tickers.into());
        self
    }

    /// Set the `tickers_all_of` argument.
    pub fn tickers_all_of(mut self, tickers_all_of: impl Into<String>) -> Self {
        self.tickers_all_of = Some(tickers_all_of.into());
        self
    }

    /// Set the `tickers_any_of` argument.
    pub fn tickers_any_of(mut self, tickers_any_of: impl Into<String>) -> Self {
        self.tickers_any_of = Some(tickers_any_of.into());
        self
    }

    /// Set the `channels` argument.
    pub fn channels(mut self, channels: impl Into<String>) -> Self {
        self.channels = Some(channels.into());
        self
    }

    /// Set the `channels_all_of` argument.
    pub fn channels_all_of(mut self, channels_all_of: impl Into<String>) -> Self {
        self.channels_all_of = Some(channels_all_of.into());
        self
    }

    /// Set the `channels_any_of` argument.
    pub fn channels_any_of(mut self, channels_any_of: impl Into<String>) -> Self {
        self.channels_any_of = Some(channels_any_of.into());
        self
    }

    /// Set the `tags` argument.
    pub fn tags(mut self, tags: impl Into<String>) -> Self {
        self.tags = Some(tags.into());
        self
    }

    /// Set the `tags_all_of` argument.
    pub fn tags_all_of(mut self, tags_all_of: impl Into<String>) -> Self {
        self.tags_all_of = Some(tags_all_of.into());
        self
    }

    /// Set the `tags_any_of` argument.
    pub fn tags_any_of(mut self, tags_any_of: impl Into<String>) -> Self {
        self.tags_any_of = Some(tags_any_of.into());
        self
    }

    /// Set the `author` argument.
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Set the `author_any_of` argument.
    pub fn author_any_of(mut self, author_any_of: impl Into<String>) -> Self {
        self.author_any_of = Some(author_any_of.into());
        self
    }

    /// Set the `author_gt` argument.
    pub fn author_gt(mut self, author_gt: impl Into<String>) -> Self {
        self.author_gt = Some(author_gt.into());
        self
    }

    /// Set the `author_gte` argument.
    pub fn author_gte(mut self, author_gte: impl Into<String>) -> Self {
        self.author_gte = Some(author_gte.into());
        self
    }

    /// Set the `author_lt` argument.
    pub fn author_lt(mut self, author_lt: impl Into<String>) -> Self {
        self.author_lt = Some(author_lt.into());
        self
    }

    /// Set the `author_lte` argument.
    pub fn author_lte(mut self, author_lte: impl Into<String>) -> Self {
        self.author_lte = Some(author_lte.into());
        self
    }

    /// Set the `limit` argument.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the `sort` argument.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`BenzingaApi::list_benzinga_news_v2`].
#[derive(Debug, Default, Clone)]
pub struct ListBenzingaNewsV2Params {
    /// The `published` argument.
    pub published: Option<String>,
    /// The `published_gt` argument.
    pub published_gt: Option<String>,
    /// The `published_gte` argument.
    pub published_gte: Option<String>,
    /// The `published_lt` argument.
    pub published_lt: Option<String>,
    /// The `published_lte` argument.
    pub published_lte: Option<String>,
    /// The `channels` argument.
    pub channels: Option<String>,
    /// The `channels_all_of` argument.
    pub channels_all_of: Option<String>,
    /// The `channels_any_of` argument.
    pub channels_any_of: Option<String>,
    /// The `tags` argument.
    pub tags: Option<String>,
    /// The `tags_all_of` argument.
    pub tags_all_of: Option<String>,
    /// The `tags_any_of` argument.
    pub tags_any_of: Option<String>,
    /// The `author` argument.
    pub author: Option<String>,
    /// The `author_any_of` argument.
    pub author_any_of: Option<String>,
    /// The `author_gt` argument.
    pub author_gt: Option<String>,
    /// The `author_gte` argument.
    pub author_gte: Option<String>,
    /// The `author_lt` argument.
    pub author_lt: Option<String>,
    /// The `author_lte` argument.
    pub author_lte: Option<String>,
    /// The `stocks` argument.
    pub stocks: Option<String>,
    /// The `stocks_all_of` argument.
    pub stocks_all_of: Option<String>,
    /// The `stocks_any_of` argument.
    pub stocks_any_of: Option<String>,
    /// The `tickers` argument.
    pub tickers: Option<String>,
    /// The `tickers_all_of` argument.
    pub tickers_all_of: Option<String>,
    /// The `tickers_any_of` argument.
    pub tickers_any_of: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListBenzingaNewsV2Params {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `published` argument.
    pub fn published(mut self, published: impl Into<String>) -> Self {
        self.published = Some(published.into());
        self
    }

    /// Set the `published_gt` argument.
    pub fn published_gt(mut self, published_gt: impl Into<String>) -> Self {
        self.published_gt = Some(published_gt.into());
        self
    }

    /// Set the `published_gte` argument.
    pub fn published_gte(mut self, published_gte: impl Into<String>) -> Self {
        self.published_gte = Some(published_gte.into());
        self
    }

    /// Set the `published_lt` argument.
    pub fn published_lt(mut self, published_lt: impl Into<String>) -> Self {
        self.published_lt = Some(published_lt.into());
        self
    }

    /// Set the `published_lte` argument.
    pub fn published_lte(mut self, published_lte: impl Into<String>) -> Self {
        self.published_lte = Some(published_lte.into());
        self
    }

    /// Set the `channels` argument.
    pub fn channels(mut self, channels: impl Into<String>) -> Self {
        self.channels = Some(channels.into());
        self
    }

    /// Set the `channels_all_of` argument.
    pub fn channels_all_of(mut self, channels_all_of: impl Into<String>) -> Self {
        self.channels_all_of = Some(channels_all_of.into());
        self
    }

    /// Set the `channels_any_of` argument.
    pub fn channels_any_of(mut self, channels_any_of: impl Into<String>) -> Self {
        self.channels_any_of = Some(channels_any_of.into());
        self
    }

    /// Set the `tags` argument.
    pub fn tags(mut self, tags: impl Into<String>) -> Self {
        self.tags = Some(tags.into());
        self
    }

    /// Set the `tags_all_of` argument.
    pub fn tags_all_of(mut self, tags_all_of: impl Into<String>) -> Self {
        self.tags_all_of = Some(tags_all_of.into());
        self
    }

    /// Set the `tags_any_of` argument.
    pub fn tags_any_of(mut self, tags_any_of: impl Into<String>) -> Self {
        self.tags_any_of = Some(tags_any_of.into());
        self
    }

    /// Set the `author` argument.
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Set the `author_any_of` argument.
    pub fn author_any_of(mut self, author_any_of: impl Into<String>) -> Self {
        self.author_any_of = Some(author_any_of.into());
        self
    }

    /// Set the `author_gt` argument.
    pub fn author_gt(mut self, author_gt: impl Into<String>) -> Self {
        self.author_gt = Some(author_gt.into());
        self
    }

    /// Set the `author_gte` argument.
    pub fn author_gte(mut self, author_gte: impl Into<String>) -> Self {
        self.author_gte = Some(author_gte.into());
        self
    }

    /// Set the `author_lt` argument.
    pub fn author_lt(mut self, author_lt: impl Into<String>) -> Self {
        self.author_lt = Some(author_lt.into());
        self
    }

    /// Set the `author_lte` argument.
    pub fn author_lte(mut self, author_lte: impl Into<String>) -> Self {
        self.author_lte = Some(author_lte.into());
        self
    }

    /// Set the `stocks` argument.
    pub fn stocks(mut self, stocks: impl Into<String>) -> Self {
        self.stocks = Some(stocks.into());
        self
    }

    /// Set the `stocks_all_of` argument.
    pub fn stocks_all_of(mut self, stocks_all_of: impl Into<String>) -> Self {
        self.stocks_all_of = Some(stocks_all_of.into());
        self
    }

    /// Set the `stocks_any_of` argument.
    pub fn stocks_any_of(mut self, stocks_any_of: impl Into<String>) -> Self {
        self.stocks_any_of = Some(stocks_any_of.into());
        self
    }

    /// Set the `tickers` argument.
    pub fn tickers(mut self, tickers: impl Into<String>) -> Self {
        self.tickers = Some(tickers.into());
        self
    }

    /// Set the `tickers_all_of` argument.
    pub fn tickers_all_of(mut self, tickers_all_of: impl Into<String>) -> Self {
        self.tickers_all_of = Some(tickers_all_of.into());
        self
    }

    /// Set the `tickers_any_of` argument.
    pub fn tickers_any_of(mut self, tickers_any_of: impl Into<String>) -> Self {
        self.tickers_any_of = Some(tickers_any_of.into());
        self
    }

    /// Set the `limit` argument.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the `sort` argument.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`BenzingaApi::list_benzinga_ratings`].
#[derive(Debug, Default, Clone)]
pub struct ListBenzingaRatingsParams {
    /// The `date` argument.
    pub date: Option<String>,
    /// The `date_any_of` argument.
    pub date_any_of: Option<String>,
    /// The `date_gt` argument.
    pub date_gt: Option<String>,
    /// The `date_gte` argument.
    pub date_gte: Option<String>,
    /// The `date_lt` argument.
    pub date_lt: Option<String>,
    /// The `date_lte` argument.
    pub date_lte: Option<String>,
    /// The `ticker` argument.
    pub ticker: Option<String>,
    /// The `ticker_any_of` argument.
    pub ticker_any_of: Option<String>,
    /// The `ticker_gt` argument.
    pub ticker_gt: Option<String>,
    /// The `ticker_gte` argument.
    pub ticker_gte: Option<String>,
    /// The `ticker_lt` argument.
    pub ticker_lt: Option<String>,
    /// The `ticker_lte` argument.
    pub ticker_lte: Option<String>,
    /// The `importance` argument.
    pub importance: Option<i64>,
    /// The `importance_any_of` argument.
    pub importance_any_of: Option<String>,
    /// The `importance_gt` argument.
    pub importance_gt: Option<i64>,
    /// The `importance_gte` argument.
    pub importance_gte: Option<i64>,
    /// The `importance_lt` argument.
    pub importance_lt: Option<i64>,
    /// The `importance_lte` argument.
    pub importance_lte: Option<i64>,
    /// The `last_updated` argument.
    pub last_updated: Option<String>,
    /// The `last_updated_any_of` argument.
    pub last_updated_any_of: Option<String>,
    /// The `last_updated_gt` argument.
    pub last_updated_gt: Option<String>,
    /// The `last_updated_gte` argument.
    pub last_updated_gte: Option<String>,
    /// The `last_updated_lt` argument.
    pub last_updated_lt: Option<String>,
    /// The `last_updated_lte` argument.
    pub last_updated_lte: Option<String>,
    /// The `rating_action` argument.
    pub rating_action: Option<String>,
    /// The `rating_action_any_of` argument.
    pub rating_action_any_of: Option<String>,
    /// The `rating_action_gt` argument.
    pub rating_action_gt: Option<String>,
    /// The `rating_action_gte` argument.
    pub rating_action_gte: Option<String>,
    /// The `rating_action_lt` argument.
    pub rating_action_lt: Option<String>,
    /// The `rating_action_lte` argument.
    pub rating_action_lte: Option<String>,
    /// The `price_target_action` argument.
    pub price_target_action: Option<String>,
    /// The `price_target_action_any_of` argument.
    pub price_target_action_any_of: Option<String>,
    /// The `price_target_action_gt` argument.
    pub price_target_action_gt: Option<String>,
    /// The `price_target_action_gte` argument.
    pub price_target_action_gte: Option<String>,
    /// The `price_target_action_lt` argument.
    pub price_target_action_lt: Option<String>,
    /// The `price_target_action_lte` argument.
    pub price_target_action_lte: Option<String>,
    /// The `benzinga_id` argument.
    pub benzinga_id: Option<String>,
    /// The `benzinga_id_any_of` argument.
    pub benzinga_id_any_of: Option<String>,
    /// The `benzinga_id_gt` argument.
    pub benzinga_id_gt: Option<String>,
    /// The `benzinga_id_gte` argument.
    pub benzinga_id_gte: Option<String>,
    /// The `benzinga_id_lt` argument.
    pub benzinga_id_lt: Option<String>,
    /// The `benzinga_id_lte` argument.
    pub benzinga_id_lte: Option<String>,
    /// The `benzinga_analyst_id` argument.
    pub benzinga_analyst_id: Option<String>,
    /// The `benzinga_analyst_id_any_of` argument.
    pub benzinga_analyst_id_any_of: Option<String>,
    /// The `benzinga_analyst_id_gt` argument.
    pub benzinga_analyst_id_gt: Option<String>,
    /// The `benzinga_analyst_id_gte` argument.
    pub benzinga_analyst_id_gte: Option<String>,
    /// The `benzinga_analyst_id_lt` argument.
    pub benzinga_analyst_id_lt: Option<String>,
    /// The `benzinga_analyst_id_lte` argument.
    pub benzinga_analyst_id_lte: Option<String>,
    /// The `benzinga_firm_id` argument.
    pub benzinga_firm_id: Option<String>,
    /// The `benzinga_firm_id_any_of` argument.
    pub benzinga_firm_id_any_of: Option<String>,
    /// The `benzinga_firm_id_gt` argument.
    pub benzinga_firm_id_gt: Option<String>,
    /// The `benzinga_firm_id_gte` argument.
    pub benzinga_firm_id_gte: Option<String>,
    /// The `benzinga_firm_id_lt` argument.
    pub benzinga_firm_id_lt: Option<String>,
    /// The `benzinga_firm_id_lte` argument.
    pub benzinga_firm_id_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListBenzingaRatingsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `date` argument.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }

    /// Set the `date_any_of` argument.
    pub fn date_any_of(mut self, date_any_of: impl Into<String>) -> Self {
        self.date_any_of = Some(date_any_of.into());
        self
    }

    /// Set the `date_gt` argument.
    pub fn date_gt(mut self, date_gt: impl Into<String>) -> Self {
        self.date_gt = Some(date_gt.into());
        self
    }

    /// Set the `date_gte` argument.
    pub fn date_gte(mut self, date_gte: impl Into<String>) -> Self {
        self.date_gte = Some(date_gte.into());
        self
    }

    /// Set the `date_lt` argument.
    pub fn date_lt(mut self, date_lt: impl Into<String>) -> Self {
        self.date_lt = Some(date_lt.into());
        self
    }

    /// Set the `date_lte` argument.
    pub fn date_lte(mut self, date_lte: impl Into<String>) -> Self {
        self.date_lte = Some(date_lte.into());
        self
    }

    /// Set the `ticker` argument.
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    /// Set the `ticker_any_of` argument.
    pub fn ticker_any_of(mut self, ticker_any_of: impl Into<String>) -> Self {
        self.ticker_any_of = Some(ticker_any_of.into());
        self
    }

    /// Set the `ticker_gt` argument.
    pub fn ticker_gt(mut self, ticker_gt: impl Into<String>) -> Self {
        self.ticker_gt = Some(ticker_gt.into());
        self
    }

    /// Set the `ticker_gte` argument.
    pub fn ticker_gte(mut self, ticker_gte: impl Into<String>) -> Self {
        self.ticker_gte = Some(ticker_gte.into());
        self
    }

    /// Set the `ticker_lt` argument.
    pub fn ticker_lt(mut self, ticker_lt: impl Into<String>) -> Self {
        self.ticker_lt = Some(ticker_lt.into());
        self
    }

    /// Set the `ticker_lte` argument.
    pub fn ticker_lte(mut self, ticker_lte: impl Into<String>) -> Self {
        self.ticker_lte = Some(ticker_lte.into());
        self
    }

    /// Set the `importance` argument.
    pub fn importance(mut self, importance: i64) -> Self {
        self.importance = Some(importance);
        self
    }

    /// Set the `importance_any_of` argument.
    pub fn importance_any_of(mut self, importance_any_of: impl Into<String>) -> Self {
        self.importance_any_of = Some(importance_any_of.into());
        self
    }

    /// Set the `importance_gt` argument.
    pub fn importance_gt(mut self, importance_gt: i64) -> Self {
        self.importance_gt = Some(importance_gt);
        self
    }

    /// Set the `importance_gte` argument.
    pub fn importance_gte(mut self, importance_gte: i64) -> Self {
        self.importance_gte = Some(importance_gte);
        self
    }

    /// Set the `importance_lt` argument.
    pub fn importance_lt(mut self, importance_lt: i64) -> Self {
        self.importance_lt = Some(importance_lt);
        self
    }

    /// Set the `importance_lte` argument.
    pub fn importance_lte(mut self, importance_lte: i64) -> Self {
        self.importance_lte = Some(importance_lte);
        self
    }

    /// Set the `last_updated` argument.
    pub fn last_updated(mut self, last_updated: impl Into<String>) -> Self {
        self.last_updated = Some(last_updated.into());
        self
    }

    /// Set the `last_updated_any_of` argument.
    pub fn last_updated_any_of(mut self, last_updated_any_of: impl Into<String>) -> Self {
        self.last_updated_any_of = Some(last_updated_any_of.into());
        self
    }

    /// Set the `last_updated_gt` argument.
    pub fn last_updated_gt(mut self, last_updated_gt: impl Into<String>) -> Self {
        self.last_updated_gt = Some(last_updated_gt.into());
        self
    }

    /// Set the `last_updated_gte` argument.
    pub fn last_updated_gte(mut self, last_updated_gte: impl Into<String>) -> Self {
        self.last_updated_gte = Some(last_updated_gte.into());
        self
    }

    /// Set the `last_updated_lt` argument.
    pub fn last_updated_lt(mut self, last_updated_lt: impl Into<String>) -> Self {
        self.last_updated_lt = Some(last_updated_lt.into());
        self
    }

    /// Set the `last_updated_lte` argument.
    pub fn last_updated_lte(mut self, last_updated_lte: impl Into<String>) -> Self {
        self.last_updated_lte = Some(last_updated_lte.into());
        self
    }

    /// Set the `rating_action` argument.
    pub fn rating_action(mut self, rating_action: impl Into<String>) -> Self {
        self.rating_action = Some(rating_action.into());
        self
    }

    /// Set the `rating_action_any_of` argument.
    pub fn rating_action_any_of(mut self, rating_action_any_of: impl Into<String>) -> Self {
        self.rating_action_any_of = Some(rating_action_any_of.into());
        self
    }

    /// Set the `rating_action_gt` argument.
    pub fn rating_action_gt(mut self, rating_action_gt: impl Into<String>) -> Self {
        self.rating_action_gt = Some(rating_action_gt.into());
        self
    }

    /// Set the `rating_action_gte` argument.
    pub fn rating_action_gte(mut self, rating_action_gte: impl Into<String>) -> Self {
        self.rating_action_gte = Some(rating_action_gte.into());
        self
    }

    /// Set the `rating_action_lt` argument.
    pub fn rating_action_lt(mut self, rating_action_lt: impl Into<String>) -> Self {
        self.rating_action_lt = Some(rating_action_lt.into());
        self
    }

    /// Set the `rating_action_lte` argument.
    pub fn rating_action_lte(mut self, rating_action_lte: impl Into<String>) -> Self {
        self.rating_action_lte = Some(rating_action_lte.into());
        self
    }

    /// Set the `price_target_action` argument.
    pub fn price_target_action(mut self, price_target_action: impl Into<String>) -> Self {
        self.price_target_action = Some(price_target_action.into());
        self
    }

    /// Set the `price_target_action_any_of` argument.
    pub fn price_target_action_any_of(
        mut self,
        price_target_action_any_of: impl Into<String>,
    ) -> Self {
        self.price_target_action_any_of = Some(price_target_action_any_of.into());
        self
    }

    /// Set the `price_target_action_gt` argument.
    pub fn price_target_action_gt(mut self, price_target_action_gt: impl Into<String>) -> Self {
        self.price_target_action_gt = Some(price_target_action_gt.into());
        self
    }

    /// Set the `price_target_action_gte` argument.
    pub fn price_target_action_gte(mut self, price_target_action_gte: impl Into<String>) -> Self {
        self.price_target_action_gte = Some(price_target_action_gte.into());
        self
    }

    /// Set the `price_target_action_lt` argument.
    pub fn price_target_action_lt(mut self, price_target_action_lt: impl Into<String>) -> Self {
        self.price_target_action_lt = Some(price_target_action_lt.into());
        self
    }

    /// Set the `price_target_action_lte` argument.
    pub fn price_target_action_lte(mut self, price_target_action_lte: impl Into<String>) -> Self {
        self.price_target_action_lte = Some(price_target_action_lte.into());
        self
    }

    /// Set the `benzinga_id` argument.
    pub fn benzinga_id(mut self, benzinga_id: impl Into<String>) -> Self {
        self.benzinga_id = Some(benzinga_id.into());
        self
    }

    /// Set the `benzinga_id_any_of` argument.
    pub fn benzinga_id_any_of(mut self, benzinga_id_any_of: impl Into<String>) -> Self {
        self.benzinga_id_any_of = Some(benzinga_id_any_of.into());
        self
    }

    /// Set the `benzinga_id_gt` argument.
    pub fn benzinga_id_gt(mut self, benzinga_id_gt: impl Into<String>) -> Self {
        self.benzinga_id_gt = Some(benzinga_id_gt.into());
        self
    }

    /// Set the `benzinga_id_gte` argument.
    pub fn benzinga_id_gte(mut self, benzinga_id_gte: impl Into<String>) -> Self {
        self.benzinga_id_gte = Some(benzinga_id_gte.into());
        self
    }

    /// Set the `benzinga_id_lt` argument.
    pub fn benzinga_id_lt(mut self, benzinga_id_lt: impl Into<String>) -> Self {
        self.benzinga_id_lt = Some(benzinga_id_lt.into());
        self
    }

    /// Set the `benzinga_id_lte` argument.
    pub fn benzinga_id_lte(mut self, benzinga_id_lte: impl Into<String>) -> Self {
        self.benzinga_id_lte = Some(benzinga_id_lte.into());
        self
    }

    /// Set the `benzinga_analyst_id` argument.
    pub fn benzinga_analyst_id(mut self, benzinga_analyst_id: impl Into<String>) -> Self {
        self.benzinga_analyst_id = Some(benzinga_analyst_id.into());
        self
    }

    /// Set the `benzinga_analyst_id_any_of` argument.
    pub fn benzinga_analyst_id_any_of(
        mut self,
        benzinga_analyst_id_any_of: impl Into<String>,
    ) -> Self {
        self.benzinga_analyst_id_any_of = Some(benzinga_analyst_id_any_of.into());
        self
    }

    /// Set the `benzinga_analyst_id_gt` argument.
    pub fn benzinga_analyst_id_gt(mut self, benzinga_analyst_id_gt: impl Into<String>) -> Self {
        self.benzinga_analyst_id_gt = Some(benzinga_analyst_id_gt.into());
        self
    }

    /// Set the `benzinga_analyst_id_gte` argument.
    pub fn benzinga_analyst_id_gte(mut self, benzinga_analyst_id_gte: impl Into<String>) -> Self {
        self.benzinga_analyst_id_gte = Some(benzinga_analyst_id_gte.into());
        self
    }

    /// Set the `benzinga_analyst_id_lt` argument.
    pub fn benzinga_analyst_id_lt(mut self, benzinga_analyst_id_lt: impl Into<String>) -> Self {
        self.benzinga_analyst_id_lt = Some(benzinga_analyst_id_lt.into());
        self
    }

    /// Set the `benzinga_analyst_id_lte` argument.
    pub fn benzinga_analyst_id_lte(mut self, benzinga_analyst_id_lte: impl Into<String>) -> Self {
        self.benzinga_analyst_id_lte = Some(benzinga_analyst_id_lte.into());
        self
    }

    /// Set the `benzinga_firm_id` argument.
    pub fn benzinga_firm_id(mut self, benzinga_firm_id: impl Into<String>) -> Self {
        self.benzinga_firm_id = Some(benzinga_firm_id.into());
        self
    }

    /// Set the `benzinga_firm_id_any_of` argument.
    pub fn benzinga_firm_id_any_of(mut self, benzinga_firm_id_any_of: impl Into<String>) -> Self {
        self.benzinga_firm_id_any_of = Some(benzinga_firm_id_any_of.into());
        self
    }

    /// Set the `benzinga_firm_id_gt` argument.
    pub fn benzinga_firm_id_gt(mut self, benzinga_firm_id_gt: impl Into<String>) -> Self {
        self.benzinga_firm_id_gt = Some(benzinga_firm_id_gt.into());
        self
    }

    /// Set the `benzinga_firm_id_gte` argument.
    pub fn benzinga_firm_id_gte(mut self, benzinga_firm_id_gte: impl Into<String>) -> Self {
        self.benzinga_firm_id_gte = Some(benzinga_firm_id_gte.into());
        self
    }

    /// Set the `benzinga_firm_id_lt` argument.
    pub fn benzinga_firm_id_lt(mut self, benzinga_firm_id_lt: impl Into<String>) -> Self {
        self.benzinga_firm_id_lt = Some(benzinga_firm_id_lt.into());
        self
    }

    /// Set the `benzinga_firm_id_lte` argument.
    pub fn benzinga_firm_id_lte(mut self, benzinga_firm_id_lte: impl Into<String>) -> Self {
        self.benzinga_firm_id_lte = Some(benzinga_firm_id_lte.into());
        self
    }

    /// Set the `limit` argument.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the `sort` argument.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`BenzingaApi::list_benzinga_bulls_bears_say`].
#[derive(Debug, Default, Clone)]
pub struct ListBenzingaBullsBearsSayParams {
    /// The `ticker` argument.
    pub ticker: Option<String>,
    /// The `ticker_any_of` argument.
    pub ticker_any_of: Option<String>,
    /// The `ticker_gt` argument.
    pub ticker_gt: Option<String>,
    /// The `ticker_gte` argument.
    pub ticker_gte: Option<String>,
    /// The `ticker_lt` argument.
    pub ticker_lt: Option<String>,
    /// The `ticker_lte` argument.
    pub ticker_lte: Option<String>,
    /// The `benzinga_id` argument.
    pub benzinga_id: Option<String>,
    /// The `benzinga_id_any_of` argument.
    pub benzinga_id_any_of: Option<String>,
    /// The `benzinga_id_gt` argument.
    pub benzinga_id_gt: Option<String>,
    /// The `benzinga_id_gte` argument.
    pub benzinga_id_gte: Option<String>,
    /// The `benzinga_id_lt` argument.
    pub benzinga_id_lt: Option<String>,
    /// The `benzinga_id_lte` argument.
    pub benzinga_id_lte: Option<String>,
    /// The `last_updated` argument.
    pub last_updated: Option<String>,
    /// The `last_updated_gt` argument.
    pub last_updated_gt: Option<String>,
    /// The `last_updated_gte` argument.
    pub last_updated_gte: Option<String>,
    /// The `last_updated_lt` argument.
    pub last_updated_lt: Option<String>,
    /// The `last_updated_lte` argument.
    pub last_updated_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListBenzingaBullsBearsSayParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `ticker` argument.
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    /// Set the `ticker_any_of` argument.
    pub fn ticker_any_of(mut self, ticker_any_of: impl Into<String>) -> Self {
        self.ticker_any_of = Some(ticker_any_of.into());
        self
    }

    /// Set the `ticker_gt` argument.
    pub fn ticker_gt(mut self, ticker_gt: impl Into<String>) -> Self {
        self.ticker_gt = Some(ticker_gt.into());
        self
    }

    /// Set the `ticker_gte` argument.
    pub fn ticker_gte(mut self, ticker_gte: impl Into<String>) -> Self {
        self.ticker_gte = Some(ticker_gte.into());
        self
    }

    /// Set the `ticker_lt` argument.
    pub fn ticker_lt(mut self, ticker_lt: impl Into<String>) -> Self {
        self.ticker_lt = Some(ticker_lt.into());
        self
    }

    /// Set the `ticker_lte` argument.
    pub fn ticker_lte(mut self, ticker_lte: impl Into<String>) -> Self {
        self.ticker_lte = Some(ticker_lte.into());
        self
    }

    /// Set the `benzinga_id` argument.
    pub fn benzinga_id(mut self, benzinga_id: impl Into<String>) -> Self {
        self.benzinga_id = Some(benzinga_id.into());
        self
    }

    /// Set the `benzinga_id_any_of` argument.
    pub fn benzinga_id_any_of(mut self, benzinga_id_any_of: impl Into<String>) -> Self {
        self.benzinga_id_any_of = Some(benzinga_id_any_of.into());
        self
    }

    /// Set the `benzinga_id_gt` argument.
    pub fn benzinga_id_gt(mut self, benzinga_id_gt: impl Into<String>) -> Self {
        self.benzinga_id_gt = Some(benzinga_id_gt.into());
        self
    }

    /// Set the `benzinga_id_gte` argument.
    pub fn benzinga_id_gte(mut self, benzinga_id_gte: impl Into<String>) -> Self {
        self.benzinga_id_gte = Some(benzinga_id_gte.into());
        self
    }

    /// Set the `benzinga_id_lt` argument.
    pub fn benzinga_id_lt(mut self, benzinga_id_lt: impl Into<String>) -> Self {
        self.benzinga_id_lt = Some(benzinga_id_lt.into());
        self
    }

    /// Set the `benzinga_id_lte` argument.
    pub fn benzinga_id_lte(mut self, benzinga_id_lte: impl Into<String>) -> Self {
        self.benzinga_id_lte = Some(benzinga_id_lte.into());
        self
    }

    /// Set the `last_updated` argument.
    pub fn last_updated(mut self, last_updated: impl Into<String>) -> Self {
        self.last_updated = Some(last_updated.into());
        self
    }

    /// Set the `last_updated_gt` argument.
    pub fn last_updated_gt(mut self, last_updated_gt: impl Into<String>) -> Self {
        self.last_updated_gt = Some(last_updated_gt.into());
        self
    }

    /// Set the `last_updated_gte` argument.
    pub fn last_updated_gte(mut self, last_updated_gte: impl Into<String>) -> Self {
        self.last_updated_gte = Some(last_updated_gte.into());
        self
    }

    /// Set the `last_updated_lt` argument.
    pub fn last_updated_lt(mut self, last_updated_lt: impl Into<String>) -> Self {
        self.last_updated_lt = Some(last_updated_lt.into());
        self
    }

    /// Set the `last_updated_lte` argument.
    pub fn last_updated_lte(mut self, last_updated_lte: impl Into<String>) -> Self {
        self.last_updated_lte = Some(last_updated_lte.into());
        self
    }

    /// Set the `limit` argument.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the `sort` argument.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
