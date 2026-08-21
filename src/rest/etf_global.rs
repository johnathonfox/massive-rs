use super::BoxStream;
use crate::client::{Client, RequestOptions};
use crate::models::{
    EtfGlobalAnalytics, EtfGlobalConstituent, EtfGlobalFundFlow, EtfGlobalProfile,
    EtfGlobalTaxonomy,
};

/// Push a query param when the optional value is present, using the literal
/// (possibly dotted) wire key.
fn push_param<T: ToString>(
    params: &mut Vec<(&'static str, String)>,
    key: &'static str,
    value: Option<T>,
) {
    if let Some(v) = value {
        params.push((key, v.to_string()));
    }
}

/// ETF Global API.
pub trait EtfGlobalApi {
    /// Get ETF Global analytics (paginated stream). Endpoint: GET /etf-global/v1/analytics.
    fn get_etf_global_analytics<'a>(
        &'a self,
        composite_ticker: Option<&'a str>,
        composite_ticker_any_of: Option<&'a str>,
        composite_ticker_gt: Option<&'a str>,
        composite_ticker_gte: Option<&'a str>,
        composite_ticker_lt: Option<&'a str>,
        composite_ticker_lte: Option<&'a str>,
        processed_date: Option<&'a str>,
        processed_date_gt: Option<&'a str>,
        processed_date_gte: Option<&'a str>,
        processed_date_lt: Option<&'a str>,
        processed_date_lte: Option<&'a str>,
        effective_date: Option<&'a str>,
        effective_date_gt: Option<&'a str>,
        effective_date_gte: Option<&'a str>,
        effective_date_lt: Option<&'a str>,
        effective_date_lte: Option<&'a str>,
        risk_total_score: Option<f64>,
        risk_total_score_gt: Option<f64>,
        risk_total_score_gte: Option<f64>,
        risk_total_score_lt: Option<f64>,
        risk_total_score_lte: Option<f64>,
        reward_score: Option<f64>,
        reward_score_gt: Option<f64>,
        reward_score_gte: Option<f64>,
        reward_score_lt: Option<f64>,
        reward_score_lte: Option<f64>,
        quant_total_score: Option<f64>,
        quant_total_score_gt: Option<f64>,
        quant_total_score_gte: Option<f64>,
        quant_total_score_lt: Option<f64>,
        quant_total_score_lte: Option<f64>,
        quant_grade: Option<&'a str>,
        quant_grade_any_of: Option<&'a str>,
        quant_grade_gt: Option<&'a str>,
        quant_grade_gte: Option<&'a str>,
        quant_grade_lt: Option<&'a str>,
        quant_grade_lte: Option<&'a str>,
        quant_composite_technical: Option<f64>,
        quant_composite_technical_gt: Option<f64>,
        quant_composite_technical_gte: Option<f64>,
        quant_composite_technical_lt: Option<f64>,
        quant_composite_technical_lte: Option<f64>,
        quant_composite_sentiment: Option<f64>,
        quant_composite_sentiment_gt: Option<f64>,
        quant_composite_sentiment_gte: Option<f64>,
        quant_composite_sentiment_lt: Option<f64>,
        quant_composite_sentiment_lte: Option<f64>,
        quant_composite_behavioral: Option<f64>,
        quant_composite_behavioral_gt: Option<f64>,
        quant_composite_behavioral_gte: Option<f64>,
        quant_composite_behavioral_lt: Option<f64>,
        quant_composite_behavioral_lte: Option<f64>,
        quant_composite_fundamental: Option<f64>,
        quant_composite_fundamental_gt: Option<f64>,
        quant_composite_fundamental_gte: Option<f64>,
        quant_composite_fundamental_lt: Option<f64>,
        quant_composite_fundamental_lte: Option<f64>,
        quant_composite_global: Option<f64>,
        quant_composite_global_gt: Option<f64>,
        quant_composite_global_gte: Option<f64>,
        quant_composite_global_lt: Option<f64>,
        quant_composite_global_lte: Option<f64>,
        quant_composite_quality: Option<f64>,
        quant_composite_quality_gt: Option<f64>,
        quant_composite_quality_gte: Option<f64>,
        quant_composite_quality_lt: Option<f64>,
        quant_composite_quality_lte: Option<f64>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, EtfGlobalAnalytics>;

    /// Same as [`Self::get_etf_global_analytics`], but takes the optional arguments as a
    /// chainable [`GetEtfGlobalAnalyticsParams`] struct.
    fn get_etf_global_analytics_with_params<'a>(
        &'a self,
        params: GetEtfGlobalAnalyticsParams,
    ) -> BoxStream<'a, EtfGlobalAnalytics>;

    /// Get ETF Global constituents (paginated stream). Endpoint: GET /etf-global/v1/constituents.
    fn get_etf_global_constituents<'a>(
        &'a self,
        composite_ticker: Option<&'a str>,
        composite_ticker_any_of: Option<&'a str>,
        composite_ticker_gt: Option<&'a str>,
        composite_ticker_gte: Option<&'a str>,
        composite_ticker_lt: Option<&'a str>,
        composite_ticker_lte: Option<&'a str>,
        constituent_ticker: Option<&'a str>,
        constituent_ticker_any_of: Option<&'a str>,
        constituent_ticker_gt: Option<&'a str>,
        constituent_ticker_gte: Option<&'a str>,
        constituent_ticker_lt: Option<&'a str>,
        constituent_ticker_lte: Option<&'a str>,
        effective_date: Option<&'a str>,
        effective_date_gt: Option<&'a str>,
        effective_date_gte: Option<&'a str>,
        effective_date_lt: Option<&'a str>,
        effective_date_lte: Option<&'a str>,
        processed_date: Option<&'a str>,
        processed_date_gt: Option<&'a str>,
        processed_date_gte: Option<&'a str>,
        processed_date_lt: Option<&'a str>,
        processed_date_lte: Option<&'a str>,
        us_code: Option<&'a str>,
        us_code_any_of: Option<&'a str>,
        us_code_gt: Option<&'a str>,
        us_code_gte: Option<&'a str>,
        us_code_lt: Option<&'a str>,
        us_code_lte: Option<&'a str>,
        isin: Option<&'a str>,
        isin_any_of: Option<&'a str>,
        isin_gt: Option<&'a str>,
        isin_gte: Option<&'a str>,
        isin_lt: Option<&'a str>,
        isin_lte: Option<&'a str>,
        figi: Option<&'a str>,
        figi_any_of: Option<&'a str>,
        figi_gt: Option<&'a str>,
        figi_gte: Option<&'a str>,
        figi_lt: Option<&'a str>,
        figi_lte: Option<&'a str>,
        sedol: Option<&'a str>,
        sedol_any_of: Option<&'a str>,
        sedol_gt: Option<&'a str>,
        sedol_gte: Option<&'a str>,
        sedol_lt: Option<&'a str>,
        sedol_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, EtfGlobalConstituent>;

    /// Same as [`Self::get_etf_global_constituents`], but takes the optional arguments as a
    /// chainable [`GetEtfGlobalConstituentsParams`] struct.
    fn get_etf_global_constituents_with_params<'a>(
        &'a self,
        params: GetEtfGlobalConstituentsParams,
    ) -> BoxStream<'a, EtfGlobalConstituent>;

    /// Get ETF Global fund flows (paginated stream). Endpoint: GET /etf-global/v1/fund-flows.
    fn get_etf_global_fund_flows<'a>(
        &'a self,
        processed_date: Option<&'a str>,
        processed_date_gt: Option<&'a str>,
        processed_date_gte: Option<&'a str>,
        processed_date_lt: Option<&'a str>,
        processed_date_lte: Option<&'a str>,
        effective_date: Option<&'a str>,
        effective_date_gt: Option<&'a str>,
        effective_date_gte: Option<&'a str>,
        effective_date_lt: Option<&'a str>,
        effective_date_lte: Option<&'a str>,
        composite_ticker: Option<&'a str>,
        composite_ticker_any_of: Option<&'a str>,
        composite_ticker_gt: Option<&'a str>,
        composite_ticker_gte: Option<&'a str>,
        composite_ticker_lt: Option<&'a str>,
        composite_ticker_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, EtfGlobalFundFlow>;

    /// Same as [`Self::get_etf_global_fund_flows`], but takes the optional arguments as a
    /// chainable [`GetEtfGlobalFundFlowsParams`] struct.
    fn get_etf_global_fund_flows_with_params<'a>(
        &'a self,
        params: GetEtfGlobalFundFlowsParams,
    ) -> BoxStream<'a, EtfGlobalFundFlow>;

    /// Get ETF Global profiles (paginated stream). Endpoint: GET /etf-global/v1/profiles.
    fn get_etf_global_profiles<'a>(
        &'a self,
        processed_date: Option<&'a str>,
        processed_date_gt: Option<&'a str>,
        processed_date_gte: Option<&'a str>,
        processed_date_lt: Option<&'a str>,
        processed_date_lte: Option<&'a str>,
        effective_date: Option<&'a str>,
        effective_date_gt: Option<&'a str>,
        effective_date_gte: Option<&'a str>,
        effective_date_lt: Option<&'a str>,
        effective_date_lte: Option<&'a str>,
        composite_ticker: Option<&'a str>,
        composite_ticker_any_of: Option<&'a str>,
        composite_ticker_gt: Option<&'a str>,
        composite_ticker_gte: Option<&'a str>,
        composite_ticker_lt: Option<&'a str>,
        composite_ticker_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, EtfGlobalProfile>;

    /// Same as [`Self::get_etf_global_profiles`], but takes the optional arguments as a
    /// chainable [`GetEtfGlobalProfilesParams`] struct.
    fn get_etf_global_profiles_with_params<'a>(
        &'a self,
        params: GetEtfGlobalProfilesParams,
    ) -> BoxStream<'a, EtfGlobalProfile>;

    /// Get ETF Global taxonomies (paginated stream). Endpoint: GET /etf-global/v1/taxonomies.
    fn get_etf_global_taxonomies<'a>(
        &'a self,
        processed_date: Option<&'a str>,
        processed_date_gt: Option<&'a str>,
        processed_date_gte: Option<&'a str>,
        processed_date_lt: Option<&'a str>,
        processed_date_lte: Option<&'a str>,
        effective_date: Option<&'a str>,
        effective_date_gt: Option<&'a str>,
        effective_date_gte: Option<&'a str>,
        effective_date_lt: Option<&'a str>,
        effective_date_lte: Option<&'a str>,
        composite_ticker: Option<&'a str>,
        composite_ticker_any_of: Option<&'a str>,
        composite_ticker_gt: Option<&'a str>,
        composite_ticker_gte: Option<&'a str>,
        composite_ticker_lt: Option<&'a str>,
        composite_ticker_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, EtfGlobalTaxonomy>;

    /// Same as [`Self::get_etf_global_taxonomies`], but takes the optional arguments as a
    /// chainable [`GetEtfGlobalTaxonomiesParams`] struct.
    fn get_etf_global_taxonomies_with_params<'a>(
        &'a self,
        params: GetEtfGlobalTaxonomiesParams,
    ) -> BoxStream<'a, EtfGlobalTaxonomy>;
}

impl EtfGlobalApi for Client {
    fn get_etf_global_analytics<'a>(
        &'a self,
        composite_ticker: Option<&'a str>,
        composite_ticker_any_of: Option<&'a str>,
        composite_ticker_gt: Option<&'a str>,
        composite_ticker_gte: Option<&'a str>,
        composite_ticker_lt: Option<&'a str>,
        composite_ticker_lte: Option<&'a str>,
        processed_date: Option<&'a str>,
        processed_date_gt: Option<&'a str>,
        processed_date_gte: Option<&'a str>,
        processed_date_lt: Option<&'a str>,
        processed_date_lte: Option<&'a str>,
        effective_date: Option<&'a str>,
        effective_date_gt: Option<&'a str>,
        effective_date_gte: Option<&'a str>,
        effective_date_lt: Option<&'a str>,
        effective_date_lte: Option<&'a str>,
        risk_total_score: Option<f64>,
        risk_total_score_gt: Option<f64>,
        risk_total_score_gte: Option<f64>,
        risk_total_score_lt: Option<f64>,
        risk_total_score_lte: Option<f64>,
        reward_score: Option<f64>,
        reward_score_gt: Option<f64>,
        reward_score_gte: Option<f64>,
        reward_score_lt: Option<f64>,
        reward_score_lte: Option<f64>,
        quant_total_score: Option<f64>,
        quant_total_score_gt: Option<f64>,
        quant_total_score_gte: Option<f64>,
        quant_total_score_lt: Option<f64>,
        quant_total_score_lte: Option<f64>,
        quant_grade: Option<&'a str>,
        quant_grade_any_of: Option<&'a str>,
        quant_grade_gt: Option<&'a str>,
        quant_grade_gte: Option<&'a str>,
        quant_grade_lt: Option<&'a str>,
        quant_grade_lte: Option<&'a str>,
        quant_composite_technical: Option<f64>,
        quant_composite_technical_gt: Option<f64>,
        quant_composite_technical_gte: Option<f64>,
        quant_composite_technical_lt: Option<f64>,
        quant_composite_technical_lte: Option<f64>,
        quant_composite_sentiment: Option<f64>,
        quant_composite_sentiment_gt: Option<f64>,
        quant_composite_sentiment_gte: Option<f64>,
        quant_composite_sentiment_lt: Option<f64>,
        quant_composite_sentiment_lte: Option<f64>,
        quant_composite_behavioral: Option<f64>,
        quant_composite_behavioral_gt: Option<f64>,
        quant_composite_behavioral_gte: Option<f64>,
        quant_composite_behavioral_lt: Option<f64>,
        quant_composite_behavioral_lte: Option<f64>,
        quant_composite_fundamental: Option<f64>,
        quant_composite_fundamental_gt: Option<f64>,
        quant_composite_fundamental_gte: Option<f64>,
        quant_composite_fundamental_lt: Option<f64>,
        quant_composite_fundamental_lte: Option<f64>,
        quant_composite_global: Option<f64>,
        quant_composite_global_gt: Option<f64>,
        quant_composite_global_gte: Option<f64>,
        quant_composite_global_lt: Option<f64>,
        quant_composite_global_lte: Option<f64>,
        quant_composite_quality: Option<f64>,
        quant_composite_quality_gt: Option<f64>,
        quant_composite_quality_gte: Option<f64>,
        quant_composite_quality_lt: Option<f64>,
        quant_composite_quality_lte: Option<f64>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, EtfGlobalAnalytics> {
        self.get_etf_global_analytics_with_params(GetEtfGlobalAnalyticsParams {
            composite_ticker: composite_ticker.map(String::from),
            composite_ticker_any_of: composite_ticker_any_of.map(String::from),
            composite_ticker_gt: composite_ticker_gt.map(String::from),
            composite_ticker_gte: composite_ticker_gte.map(String::from),
            composite_ticker_lt: composite_ticker_lt.map(String::from),
            composite_ticker_lte: composite_ticker_lte.map(String::from),
            processed_date: processed_date.map(String::from),
            processed_date_gt: processed_date_gt.map(String::from),
            processed_date_gte: processed_date_gte.map(String::from),
            processed_date_lt: processed_date_lt.map(String::from),
            processed_date_lte: processed_date_lte.map(String::from),
            effective_date: effective_date.map(String::from),
            effective_date_gt: effective_date_gt.map(String::from),
            effective_date_gte: effective_date_gte.map(String::from),
            effective_date_lt: effective_date_lt.map(String::from),
            effective_date_lte: effective_date_lte.map(String::from),
            risk_total_score,
            risk_total_score_gt,
            risk_total_score_gte,
            risk_total_score_lt,
            risk_total_score_lte,
            reward_score,
            reward_score_gt,
            reward_score_gte,
            reward_score_lt,
            reward_score_lte,
            quant_total_score,
            quant_total_score_gt,
            quant_total_score_gte,
            quant_total_score_lt,
            quant_total_score_lte,
            quant_grade: quant_grade.map(String::from),
            quant_grade_any_of: quant_grade_any_of.map(String::from),
            quant_grade_gt: quant_grade_gt.map(String::from),
            quant_grade_gte: quant_grade_gte.map(String::from),
            quant_grade_lt: quant_grade_lt.map(String::from),
            quant_grade_lte: quant_grade_lte.map(String::from),
            quant_composite_technical,
            quant_composite_technical_gt,
            quant_composite_technical_gte,
            quant_composite_technical_lt,
            quant_composite_technical_lte,
            quant_composite_sentiment,
            quant_composite_sentiment_gt,
            quant_composite_sentiment_gte,
            quant_composite_sentiment_lt,
            quant_composite_sentiment_lte,
            quant_composite_behavioral,
            quant_composite_behavioral_gt,
            quant_composite_behavioral_gte,
            quant_composite_behavioral_lt,
            quant_composite_behavioral_lte,
            quant_composite_fundamental,
            quant_composite_fundamental_gt,
            quant_composite_fundamental_gte,
            quant_composite_fundamental_lt,
            quant_composite_fundamental_lte,
            quant_composite_global,
            quant_composite_global_gt,
            quant_composite_global_gte,
            quant_composite_global_lt,
            quant_composite_global_lte,
            quant_composite_quality,
            quant_composite_quality_gt,
            quant_composite_quality_gte,
            quant_composite_quality_lt,
            quant_composite_quality_lte,
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn get_etf_global_analytics_with_params<'a>(
        &'a self,
        params: GetEtfGlobalAnalyticsParams,
    ) -> BoxStream<'a, EtfGlobalAnalytics> {
        Box::pin({
            let GetEtfGlobalAnalyticsParams {
                composite_ticker,
                composite_ticker_any_of,
                composite_ticker_gt,
                composite_ticker_gte,
                composite_ticker_lt,
                composite_ticker_lte,
                processed_date,
                processed_date_gt,
                processed_date_gte,
                processed_date_lt,
                processed_date_lte,
                effective_date,
                effective_date_gt,
                effective_date_gte,
                effective_date_lt,
                effective_date_lte,
                risk_total_score,
                risk_total_score_gt,
                risk_total_score_gte,
                risk_total_score_lt,
                risk_total_score_lte,
                reward_score,
                reward_score_gt,
                reward_score_gte,
                reward_score_lt,
                reward_score_lte,
                quant_total_score,
                quant_total_score_gt,
                quant_total_score_gte,
                quant_total_score_lt,
                quant_total_score_lte,
                quant_grade,
                quant_grade_any_of,
                quant_grade_gt,
                quant_grade_gte,
                quant_grade_lt,
                quant_grade_lte,
                quant_composite_technical,
                quant_composite_technical_gt,
                quant_composite_technical_gte,
                quant_composite_technical_lt,
                quant_composite_technical_lte,
                quant_composite_sentiment,
                quant_composite_sentiment_gt,
                quant_composite_sentiment_gte,
                quant_composite_sentiment_lt,
                quant_composite_sentiment_lte,
                quant_composite_behavioral,
                quant_composite_behavioral_gt,
                quant_composite_behavioral_gte,
                quant_composite_behavioral_lt,
                quant_composite_behavioral_lte,
                quant_composite_fundamental,
                quant_composite_fundamental_gt,
                quant_composite_fundamental_gte,
                quant_composite_fundamental_lt,
                quant_composite_fundamental_lte,
                quant_composite_global,
                quant_composite_global_gt,
                quant_composite_global_gte,
                quant_composite_global_lt,
                quant_composite_global_lte,
                quant_composite_quality,
                quant_composite_quality_gt,
                quant_composite_quality_gte,
                quant_composite_quality_lt,
                quant_composite_quality_lte,
                limit,
                sort,
                options,
            } = params;
            let composite_ticker = composite_ticker.as_deref();
            let composite_ticker_any_of = composite_ticker_any_of.as_deref();
            let composite_ticker_gt = composite_ticker_gt.as_deref();
            let composite_ticker_gte = composite_ticker_gte.as_deref();
            let composite_ticker_lt = composite_ticker_lt.as_deref();
            let composite_ticker_lte = composite_ticker_lte.as_deref();
            let processed_date = processed_date.as_deref();
            let processed_date_gt = processed_date_gt.as_deref();
            let processed_date_gte = processed_date_gte.as_deref();
            let processed_date_lt = processed_date_lt.as_deref();
            let processed_date_lte = processed_date_lte.as_deref();
            let effective_date = effective_date.as_deref();
            let effective_date_gt = effective_date_gt.as_deref();
            let effective_date_gte = effective_date_gte.as_deref();
            let effective_date_lt = effective_date_lt.as_deref();
            let effective_date_lte = effective_date_lte.as_deref();
            let quant_grade = quant_grade.as_deref();
            let quant_grade_any_of = quant_grade_any_of.as_deref();
            let quant_grade_gt = quant_grade_gt.as_deref();
            let quant_grade_gte = quant_grade_gte.as_deref();
            let quant_grade_lt = quant_grade_lt.as_deref();
            let quant_grade_lte = quant_grade_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/etf-global/v1/analytics".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            push_param(&mut query, "composite_ticker", composite_ticker);
            push_param(
                &mut query,
                "composite_ticker.any_of",
                composite_ticker_any_of,
            );
            push_param(&mut query, "composite_ticker.gt", composite_ticker_gt);
            push_param(&mut query, "composite_ticker.gte", composite_ticker_gte);
            push_param(&mut query, "composite_ticker.lt", composite_ticker_lt);
            push_param(&mut query, "composite_ticker.lte", composite_ticker_lte);
            push_param(&mut query, "processed_date", processed_date);
            push_param(&mut query, "processed_date.gt", processed_date_gt);
            push_param(&mut query, "processed_date.gte", processed_date_gte);
            push_param(&mut query, "processed_date.lt", processed_date_lt);
            push_param(&mut query, "processed_date.lte", processed_date_lte);
            push_param(&mut query, "effective_date", effective_date);
            push_param(&mut query, "effective_date.gt", effective_date_gt);
            push_param(&mut query, "effective_date.gte", effective_date_gte);
            push_param(&mut query, "effective_date.lt", effective_date_lt);
            push_param(&mut query, "effective_date.lte", effective_date_lte);
            push_param(&mut query, "risk_total_score", risk_total_score);
            push_param(&mut query, "risk_total_score.gt", risk_total_score_gt);
            push_param(&mut query, "risk_total_score.gte", risk_total_score_gte);
            push_param(&mut query, "risk_total_score.lt", risk_total_score_lt);
            push_param(&mut query, "risk_total_score.lte", risk_total_score_lte);
            push_param(&mut query, "reward_score", reward_score);
            push_param(&mut query, "reward_score.gt", reward_score_gt);
            push_param(&mut query, "reward_score.gte", reward_score_gte);
            push_param(&mut query, "reward_score.lt", reward_score_lt);
            push_param(&mut query, "reward_score.lte", reward_score_lte);
            push_param(&mut query, "quant_total_score", quant_total_score);
            push_param(&mut query, "quant_total_score.gt", quant_total_score_gt);
            push_param(&mut query, "quant_total_score.gte", quant_total_score_gte);
            push_param(&mut query, "quant_total_score.lt", quant_total_score_lt);
            push_param(&mut query, "quant_total_score.lte", quant_total_score_lte);
            push_param(&mut query, "quant_grade", quant_grade);
            push_param(&mut query, "quant_grade.any_of", quant_grade_any_of);
            push_param(&mut query, "quant_grade.gt", quant_grade_gt);
            push_param(&mut query, "quant_grade.gte", quant_grade_gte);
            push_param(&mut query, "quant_grade.lt", quant_grade_lt);
            push_param(&mut query, "quant_grade.lte", quant_grade_lte);
            push_param(
                &mut query,
                "quant_composite_technical",
                quant_composite_technical,
            );
            push_param(
                &mut query,
                "quant_composite_technical.gt",
                quant_composite_technical_gt,
            );
            push_param(
                &mut query,
                "quant_composite_technical.gte",
                quant_composite_technical_gte,
            );
            push_param(
                &mut query,
                "quant_composite_technical.lt",
                quant_composite_technical_lt,
            );
            push_param(
                &mut query,
                "quant_composite_technical.lte",
                quant_composite_technical_lte,
            );
            push_param(
                &mut query,
                "quant_composite_sentiment",
                quant_composite_sentiment,
            );
            push_param(
                &mut query,
                "quant_composite_sentiment.gt",
                quant_composite_sentiment_gt,
            );
            push_param(
                &mut query,
                "quant_composite_sentiment.gte",
                quant_composite_sentiment_gte,
            );
            push_param(
                &mut query,
                "quant_composite_sentiment.lt",
                quant_composite_sentiment_lt,
            );
            push_param(
                &mut query,
                "quant_composite_sentiment.lte",
                quant_composite_sentiment_lte,
            );
            push_param(
                &mut query,
                "quant_composite_behavioral",
                quant_composite_behavioral,
            );
            push_param(
                &mut query,
                "quant_composite_behavioral.gt",
                quant_composite_behavioral_gt,
            );
            push_param(
                &mut query,
                "quant_composite_behavioral.gte",
                quant_composite_behavioral_gte,
            );
            push_param(
                &mut query,
                "quant_composite_behavioral.lt",
                quant_composite_behavioral_lt,
            );
            push_param(
                &mut query,
                "quant_composite_behavioral.lte",
                quant_composite_behavioral_lte,
            );
            push_param(
                &mut query,
                "quant_composite_fundamental",
                quant_composite_fundamental,
            );
            push_param(
                &mut query,
                "quant_composite_fundamental.gt",
                quant_composite_fundamental_gt,
            );
            push_param(
                &mut query,
                "quant_composite_fundamental.gte",
                quant_composite_fundamental_gte,
            );
            push_param(
                &mut query,
                "quant_composite_fundamental.lt",
                quant_composite_fundamental_lt,
            );
            push_param(
                &mut query,
                "quant_composite_fundamental.lte",
                quant_composite_fundamental_lte,
            );
            push_param(&mut query, "quant_composite_global", quant_composite_global);
            push_param(
                &mut query,
                "quant_composite_global.gt",
                quant_composite_global_gt,
            );
            push_param(
                &mut query,
                "quant_composite_global.gte",
                quant_composite_global_gte,
            );
            push_param(
                &mut query,
                "quant_composite_global.lt",
                quant_composite_global_lt,
            );
            push_param(
                &mut query,
                "quant_composite_global.lte",
                quant_composite_global_lte,
            );
            push_param(
                &mut query,
                "quant_composite_quality",
                quant_composite_quality,
            );
            push_param(
                &mut query,
                "quant_composite_quality.gt",
                quant_composite_quality_gt,
            );
            push_param(
                &mut query,
                "quant_composite_quality.gte",
                quant_composite_quality_gte,
            );
            push_param(
                &mut query,
                "quant_composite_quality.lt",
                quant_composite_quality_lt,
            );
            push_param(
                &mut query,
                "quant_composite_quality.lte",
                quant_composite_quality_lte,
            );
            push_param(&mut query, "limit", limit);
            push_param(&mut query, "sort", sort);
            if self.pagination {
                self.paginate::<EtfGlobalAnalytics>(&path, Some(&query), options)
            } else {
                self.single_page::<EtfGlobalAnalytics>(&path, Some(&query), options)
            }
        })
    }

    fn get_etf_global_constituents<'a>(
        &'a self,
        composite_ticker: Option<&'a str>,
        composite_ticker_any_of: Option<&'a str>,
        composite_ticker_gt: Option<&'a str>,
        composite_ticker_gte: Option<&'a str>,
        composite_ticker_lt: Option<&'a str>,
        composite_ticker_lte: Option<&'a str>,
        constituent_ticker: Option<&'a str>,
        constituent_ticker_any_of: Option<&'a str>,
        constituent_ticker_gt: Option<&'a str>,
        constituent_ticker_gte: Option<&'a str>,
        constituent_ticker_lt: Option<&'a str>,
        constituent_ticker_lte: Option<&'a str>,
        effective_date: Option<&'a str>,
        effective_date_gt: Option<&'a str>,
        effective_date_gte: Option<&'a str>,
        effective_date_lt: Option<&'a str>,
        effective_date_lte: Option<&'a str>,
        processed_date: Option<&'a str>,
        processed_date_gt: Option<&'a str>,
        processed_date_gte: Option<&'a str>,
        processed_date_lt: Option<&'a str>,
        processed_date_lte: Option<&'a str>,
        us_code: Option<&'a str>,
        us_code_any_of: Option<&'a str>,
        us_code_gt: Option<&'a str>,
        us_code_gte: Option<&'a str>,
        us_code_lt: Option<&'a str>,
        us_code_lte: Option<&'a str>,
        isin: Option<&'a str>,
        isin_any_of: Option<&'a str>,
        isin_gt: Option<&'a str>,
        isin_gte: Option<&'a str>,
        isin_lt: Option<&'a str>,
        isin_lte: Option<&'a str>,
        figi: Option<&'a str>,
        figi_any_of: Option<&'a str>,
        figi_gt: Option<&'a str>,
        figi_gte: Option<&'a str>,
        figi_lt: Option<&'a str>,
        figi_lte: Option<&'a str>,
        sedol: Option<&'a str>,
        sedol_any_of: Option<&'a str>,
        sedol_gt: Option<&'a str>,
        sedol_gte: Option<&'a str>,
        sedol_lt: Option<&'a str>,
        sedol_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, EtfGlobalConstituent> {
        self.get_etf_global_constituents_with_params(GetEtfGlobalConstituentsParams {
            composite_ticker: composite_ticker.map(String::from),
            composite_ticker_any_of: composite_ticker_any_of.map(String::from),
            composite_ticker_gt: composite_ticker_gt.map(String::from),
            composite_ticker_gte: composite_ticker_gte.map(String::from),
            composite_ticker_lt: composite_ticker_lt.map(String::from),
            composite_ticker_lte: composite_ticker_lte.map(String::from),
            constituent_ticker: constituent_ticker.map(String::from),
            constituent_ticker_any_of: constituent_ticker_any_of.map(String::from),
            constituent_ticker_gt: constituent_ticker_gt.map(String::from),
            constituent_ticker_gte: constituent_ticker_gte.map(String::from),
            constituent_ticker_lt: constituent_ticker_lt.map(String::from),
            constituent_ticker_lte: constituent_ticker_lte.map(String::from),
            effective_date: effective_date.map(String::from),
            effective_date_gt: effective_date_gt.map(String::from),
            effective_date_gte: effective_date_gte.map(String::from),
            effective_date_lt: effective_date_lt.map(String::from),
            effective_date_lte: effective_date_lte.map(String::from),
            processed_date: processed_date.map(String::from),
            processed_date_gt: processed_date_gt.map(String::from),
            processed_date_gte: processed_date_gte.map(String::from),
            processed_date_lt: processed_date_lt.map(String::from),
            processed_date_lte: processed_date_lte.map(String::from),
            us_code: us_code.map(String::from),
            us_code_any_of: us_code_any_of.map(String::from),
            us_code_gt: us_code_gt.map(String::from),
            us_code_gte: us_code_gte.map(String::from),
            us_code_lt: us_code_lt.map(String::from),
            us_code_lte: us_code_lte.map(String::from),
            isin: isin.map(String::from),
            isin_any_of: isin_any_of.map(String::from),
            isin_gt: isin_gt.map(String::from),
            isin_gte: isin_gte.map(String::from),
            isin_lt: isin_lt.map(String::from),
            isin_lte: isin_lte.map(String::from),
            figi: figi.map(String::from),
            figi_any_of: figi_any_of.map(String::from),
            figi_gt: figi_gt.map(String::from),
            figi_gte: figi_gte.map(String::from),
            figi_lt: figi_lt.map(String::from),
            figi_lte: figi_lte.map(String::from),
            sedol: sedol.map(String::from),
            sedol_any_of: sedol_any_of.map(String::from),
            sedol_gt: sedol_gt.map(String::from),
            sedol_gte: sedol_gte.map(String::from),
            sedol_lt: sedol_lt.map(String::from),
            sedol_lte: sedol_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn get_etf_global_constituents_with_params<'a>(
        &'a self,
        params: GetEtfGlobalConstituentsParams,
    ) -> BoxStream<'a, EtfGlobalConstituent> {
        Box::pin({
            let GetEtfGlobalConstituentsParams {
                composite_ticker,
                composite_ticker_any_of,
                composite_ticker_gt,
                composite_ticker_gte,
                composite_ticker_lt,
                composite_ticker_lte,
                constituent_ticker,
                constituent_ticker_any_of,
                constituent_ticker_gt,
                constituent_ticker_gte,
                constituent_ticker_lt,
                constituent_ticker_lte,
                effective_date,
                effective_date_gt,
                effective_date_gte,
                effective_date_lt,
                effective_date_lte,
                processed_date,
                processed_date_gt,
                processed_date_gte,
                processed_date_lt,
                processed_date_lte,
                us_code,
                us_code_any_of,
                us_code_gt,
                us_code_gte,
                us_code_lt,
                us_code_lte,
                isin,
                isin_any_of,
                isin_gt,
                isin_gte,
                isin_lt,
                isin_lte,
                figi,
                figi_any_of,
                figi_gt,
                figi_gte,
                figi_lt,
                figi_lte,
                sedol,
                sedol_any_of,
                sedol_gt,
                sedol_gte,
                sedol_lt,
                sedol_lte,
                limit,
                sort,
                options,
            } = params;
            let composite_ticker = composite_ticker.as_deref();
            let composite_ticker_any_of = composite_ticker_any_of.as_deref();
            let composite_ticker_gt = composite_ticker_gt.as_deref();
            let composite_ticker_gte = composite_ticker_gte.as_deref();
            let composite_ticker_lt = composite_ticker_lt.as_deref();
            let composite_ticker_lte = composite_ticker_lte.as_deref();
            let constituent_ticker = constituent_ticker.as_deref();
            let constituent_ticker_any_of = constituent_ticker_any_of.as_deref();
            let constituent_ticker_gt = constituent_ticker_gt.as_deref();
            let constituent_ticker_gte = constituent_ticker_gte.as_deref();
            let constituent_ticker_lt = constituent_ticker_lt.as_deref();
            let constituent_ticker_lte = constituent_ticker_lte.as_deref();
            let effective_date = effective_date.as_deref();
            let effective_date_gt = effective_date_gt.as_deref();
            let effective_date_gte = effective_date_gte.as_deref();
            let effective_date_lt = effective_date_lt.as_deref();
            let effective_date_lte = effective_date_lte.as_deref();
            let processed_date = processed_date.as_deref();
            let processed_date_gt = processed_date_gt.as_deref();
            let processed_date_gte = processed_date_gte.as_deref();
            let processed_date_lt = processed_date_lt.as_deref();
            let processed_date_lte = processed_date_lte.as_deref();
            let us_code = us_code.as_deref();
            let us_code_any_of = us_code_any_of.as_deref();
            let us_code_gt = us_code_gt.as_deref();
            let us_code_gte = us_code_gte.as_deref();
            let us_code_lt = us_code_lt.as_deref();
            let us_code_lte = us_code_lte.as_deref();
            let isin = isin.as_deref();
            let isin_any_of = isin_any_of.as_deref();
            let isin_gt = isin_gt.as_deref();
            let isin_gte = isin_gte.as_deref();
            let isin_lt = isin_lt.as_deref();
            let isin_lte = isin_lte.as_deref();
            let figi = figi.as_deref();
            let figi_any_of = figi_any_of.as_deref();
            let figi_gt = figi_gt.as_deref();
            let figi_gte = figi_gte.as_deref();
            let figi_lt = figi_lt.as_deref();
            let figi_lte = figi_lte.as_deref();
            let sedol = sedol.as_deref();
            let sedol_any_of = sedol_any_of.as_deref();
            let sedol_gt = sedol_gt.as_deref();
            let sedol_gte = sedol_gte.as_deref();
            let sedol_lt = sedol_lt.as_deref();
            let sedol_lte = sedol_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/etf-global/v1/constituents".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            push_param(&mut query, "composite_ticker", composite_ticker);
            push_param(
                &mut query,
                "composite_ticker.any_of",
                composite_ticker_any_of,
            );
            push_param(&mut query, "composite_ticker.gt", composite_ticker_gt);
            push_param(&mut query, "composite_ticker.gte", composite_ticker_gte);
            push_param(&mut query, "composite_ticker.lt", composite_ticker_lt);
            push_param(&mut query, "composite_ticker.lte", composite_ticker_lte);
            push_param(&mut query, "constituent_ticker", constituent_ticker);
            push_param(
                &mut query,
                "constituent_ticker.any_of",
                constituent_ticker_any_of,
            );
            push_param(&mut query, "constituent_ticker.gt", constituent_ticker_gt);
            push_param(&mut query, "constituent_ticker.gte", constituent_ticker_gte);
            push_param(&mut query, "constituent_ticker.lt", constituent_ticker_lt);
            push_param(&mut query, "constituent_ticker.lte", constituent_ticker_lte);
            push_param(&mut query, "effective_date", effective_date);
            push_param(&mut query, "effective_date.gt", effective_date_gt);
            push_param(&mut query, "effective_date.gte", effective_date_gte);
            push_param(&mut query, "effective_date.lt", effective_date_lt);
            push_param(&mut query, "effective_date.lte", effective_date_lte);
            push_param(&mut query, "processed_date", processed_date);
            push_param(&mut query, "processed_date.gt", processed_date_gt);
            push_param(&mut query, "processed_date.gte", processed_date_gte);
            push_param(&mut query, "processed_date.lt", processed_date_lt);
            push_param(&mut query, "processed_date.lte", processed_date_lte);
            push_param(&mut query, "us_code", us_code);
            push_param(&mut query, "us_code.any_of", us_code_any_of);
            push_param(&mut query, "us_code.gt", us_code_gt);
            push_param(&mut query, "us_code.gte", us_code_gte);
            push_param(&mut query, "us_code.lt", us_code_lt);
            push_param(&mut query, "us_code.lte", us_code_lte);
            push_param(&mut query, "isin", isin);
            push_param(&mut query, "isin.any_of", isin_any_of);
            push_param(&mut query, "isin.gt", isin_gt);
            push_param(&mut query, "isin.gte", isin_gte);
            push_param(&mut query, "isin.lt", isin_lt);
            push_param(&mut query, "isin.lte", isin_lte);
            push_param(&mut query, "figi", figi);
            push_param(&mut query, "figi.any_of", figi_any_of);
            push_param(&mut query, "figi.gt", figi_gt);
            push_param(&mut query, "figi.gte", figi_gte);
            push_param(&mut query, "figi.lt", figi_lt);
            push_param(&mut query, "figi.lte", figi_lte);
            push_param(&mut query, "sedol", sedol);
            push_param(&mut query, "sedol.any_of", sedol_any_of);
            push_param(&mut query, "sedol.gt", sedol_gt);
            push_param(&mut query, "sedol.gte", sedol_gte);
            push_param(&mut query, "sedol.lt", sedol_lt);
            push_param(&mut query, "sedol.lte", sedol_lte);
            push_param(&mut query, "limit", limit);
            push_param(&mut query, "sort", sort);
            if self.pagination {
                self.paginate::<EtfGlobalConstituent>(&path, Some(&query), options)
            } else {
                self.single_page::<EtfGlobalConstituent>(&path, Some(&query), options)
            }
        })
    }

    fn get_etf_global_fund_flows<'a>(
        &'a self,
        processed_date: Option<&'a str>,
        processed_date_gt: Option<&'a str>,
        processed_date_gte: Option<&'a str>,
        processed_date_lt: Option<&'a str>,
        processed_date_lte: Option<&'a str>,
        effective_date: Option<&'a str>,
        effective_date_gt: Option<&'a str>,
        effective_date_gte: Option<&'a str>,
        effective_date_lt: Option<&'a str>,
        effective_date_lte: Option<&'a str>,
        composite_ticker: Option<&'a str>,
        composite_ticker_any_of: Option<&'a str>,
        composite_ticker_gt: Option<&'a str>,
        composite_ticker_gte: Option<&'a str>,
        composite_ticker_lt: Option<&'a str>,
        composite_ticker_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, EtfGlobalFundFlow> {
        self.get_etf_global_fund_flows_with_params(GetEtfGlobalFundFlowsParams {
            processed_date: processed_date.map(String::from),
            processed_date_gt: processed_date_gt.map(String::from),
            processed_date_gte: processed_date_gte.map(String::from),
            processed_date_lt: processed_date_lt.map(String::from),
            processed_date_lte: processed_date_lte.map(String::from),
            effective_date: effective_date.map(String::from),
            effective_date_gt: effective_date_gt.map(String::from),
            effective_date_gte: effective_date_gte.map(String::from),
            effective_date_lt: effective_date_lt.map(String::from),
            effective_date_lte: effective_date_lte.map(String::from),
            composite_ticker: composite_ticker.map(String::from),
            composite_ticker_any_of: composite_ticker_any_of.map(String::from),
            composite_ticker_gt: composite_ticker_gt.map(String::from),
            composite_ticker_gte: composite_ticker_gte.map(String::from),
            composite_ticker_lt: composite_ticker_lt.map(String::from),
            composite_ticker_lte: composite_ticker_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn get_etf_global_fund_flows_with_params<'a>(
        &'a self,
        params: GetEtfGlobalFundFlowsParams,
    ) -> BoxStream<'a, EtfGlobalFundFlow> {
        Box::pin({
            let GetEtfGlobalFundFlowsParams {
                processed_date,
                processed_date_gt,
                processed_date_gte,
                processed_date_lt,
                processed_date_lte,
                effective_date,
                effective_date_gt,
                effective_date_gte,
                effective_date_lt,
                effective_date_lte,
                composite_ticker,
                composite_ticker_any_of,
                composite_ticker_gt,
                composite_ticker_gte,
                composite_ticker_lt,
                composite_ticker_lte,
                limit,
                sort,
                options,
            } = params;
            let processed_date = processed_date.as_deref();
            let processed_date_gt = processed_date_gt.as_deref();
            let processed_date_gte = processed_date_gte.as_deref();
            let processed_date_lt = processed_date_lt.as_deref();
            let processed_date_lte = processed_date_lte.as_deref();
            let effective_date = effective_date.as_deref();
            let effective_date_gt = effective_date_gt.as_deref();
            let effective_date_gte = effective_date_gte.as_deref();
            let effective_date_lt = effective_date_lt.as_deref();
            let effective_date_lte = effective_date_lte.as_deref();
            let composite_ticker = composite_ticker.as_deref();
            let composite_ticker_any_of = composite_ticker_any_of.as_deref();
            let composite_ticker_gt = composite_ticker_gt.as_deref();
            let composite_ticker_gte = composite_ticker_gte.as_deref();
            let composite_ticker_lt = composite_ticker_lt.as_deref();
            let composite_ticker_lte = composite_ticker_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/etf-global/v1/fund-flows".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            push_param(&mut query, "processed_date", processed_date);
            push_param(&mut query, "processed_date.gt", processed_date_gt);
            push_param(&mut query, "processed_date.gte", processed_date_gte);
            push_param(&mut query, "processed_date.lt", processed_date_lt);
            push_param(&mut query, "processed_date.lte", processed_date_lte);
            push_param(&mut query, "effective_date", effective_date);
            push_param(&mut query, "effective_date.gt", effective_date_gt);
            push_param(&mut query, "effective_date.gte", effective_date_gte);
            push_param(&mut query, "effective_date.lt", effective_date_lt);
            push_param(&mut query, "effective_date.lte", effective_date_lte);
            push_param(&mut query, "composite_ticker", composite_ticker);
            push_param(
                &mut query,
                "composite_ticker.any_of",
                composite_ticker_any_of,
            );
            push_param(&mut query, "composite_ticker.gt", composite_ticker_gt);
            push_param(&mut query, "composite_ticker.gte", composite_ticker_gte);
            push_param(&mut query, "composite_ticker.lt", composite_ticker_lt);
            push_param(&mut query, "composite_ticker.lte", composite_ticker_lte);
            push_param(&mut query, "limit", limit);
            push_param(&mut query, "sort", sort);
            if self.pagination {
                self.paginate::<EtfGlobalFundFlow>(&path, Some(&query), options)
            } else {
                self.single_page::<EtfGlobalFundFlow>(&path, Some(&query), options)
            }
        })
    }

    fn get_etf_global_profiles<'a>(
        &'a self,
        processed_date: Option<&'a str>,
        processed_date_gt: Option<&'a str>,
        processed_date_gte: Option<&'a str>,
        processed_date_lt: Option<&'a str>,
        processed_date_lte: Option<&'a str>,
        effective_date: Option<&'a str>,
        effective_date_gt: Option<&'a str>,
        effective_date_gte: Option<&'a str>,
        effective_date_lt: Option<&'a str>,
        effective_date_lte: Option<&'a str>,
        composite_ticker: Option<&'a str>,
        composite_ticker_any_of: Option<&'a str>,
        composite_ticker_gt: Option<&'a str>,
        composite_ticker_gte: Option<&'a str>,
        composite_ticker_lt: Option<&'a str>,
        composite_ticker_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, EtfGlobalProfile> {
        self.get_etf_global_profiles_with_params(GetEtfGlobalProfilesParams {
            processed_date: processed_date.map(String::from),
            processed_date_gt: processed_date_gt.map(String::from),
            processed_date_gte: processed_date_gte.map(String::from),
            processed_date_lt: processed_date_lt.map(String::from),
            processed_date_lte: processed_date_lte.map(String::from),
            effective_date: effective_date.map(String::from),
            effective_date_gt: effective_date_gt.map(String::from),
            effective_date_gte: effective_date_gte.map(String::from),
            effective_date_lt: effective_date_lt.map(String::from),
            effective_date_lte: effective_date_lte.map(String::from),
            composite_ticker: composite_ticker.map(String::from),
            composite_ticker_any_of: composite_ticker_any_of.map(String::from),
            composite_ticker_gt: composite_ticker_gt.map(String::from),
            composite_ticker_gte: composite_ticker_gte.map(String::from),
            composite_ticker_lt: composite_ticker_lt.map(String::from),
            composite_ticker_lte: composite_ticker_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn get_etf_global_profiles_with_params<'a>(
        &'a self,
        params: GetEtfGlobalProfilesParams,
    ) -> BoxStream<'a, EtfGlobalProfile> {
        Box::pin({
            let GetEtfGlobalProfilesParams {
                processed_date,
                processed_date_gt,
                processed_date_gte,
                processed_date_lt,
                processed_date_lte,
                effective_date,
                effective_date_gt,
                effective_date_gte,
                effective_date_lt,
                effective_date_lte,
                composite_ticker,
                composite_ticker_any_of,
                composite_ticker_gt,
                composite_ticker_gte,
                composite_ticker_lt,
                composite_ticker_lte,
                limit,
                sort,
                options,
            } = params;
            let processed_date = processed_date.as_deref();
            let processed_date_gt = processed_date_gt.as_deref();
            let processed_date_gte = processed_date_gte.as_deref();
            let processed_date_lt = processed_date_lt.as_deref();
            let processed_date_lte = processed_date_lte.as_deref();
            let effective_date = effective_date.as_deref();
            let effective_date_gt = effective_date_gt.as_deref();
            let effective_date_gte = effective_date_gte.as_deref();
            let effective_date_lt = effective_date_lt.as_deref();
            let effective_date_lte = effective_date_lte.as_deref();
            let composite_ticker = composite_ticker.as_deref();
            let composite_ticker_any_of = composite_ticker_any_of.as_deref();
            let composite_ticker_gt = composite_ticker_gt.as_deref();
            let composite_ticker_gte = composite_ticker_gte.as_deref();
            let composite_ticker_lt = composite_ticker_lt.as_deref();
            let composite_ticker_lte = composite_ticker_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/etf-global/v1/profiles".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            push_param(&mut query, "processed_date", processed_date);
            push_param(&mut query, "processed_date.gt", processed_date_gt);
            push_param(&mut query, "processed_date.gte", processed_date_gte);
            push_param(&mut query, "processed_date.lt", processed_date_lt);
            push_param(&mut query, "processed_date.lte", processed_date_lte);
            push_param(&mut query, "effective_date", effective_date);
            push_param(&mut query, "effective_date.gt", effective_date_gt);
            push_param(&mut query, "effective_date.gte", effective_date_gte);
            push_param(&mut query, "effective_date.lt", effective_date_lt);
            push_param(&mut query, "effective_date.lte", effective_date_lte);
            push_param(&mut query, "composite_ticker", composite_ticker);
            push_param(
                &mut query,
                "composite_ticker.any_of",
                composite_ticker_any_of,
            );
            push_param(&mut query, "composite_ticker.gt", composite_ticker_gt);
            push_param(&mut query, "composite_ticker.gte", composite_ticker_gte);
            push_param(&mut query, "composite_ticker.lt", composite_ticker_lt);
            push_param(&mut query, "composite_ticker.lte", composite_ticker_lte);
            push_param(&mut query, "limit", limit);
            push_param(&mut query, "sort", sort);
            if self.pagination {
                self.paginate::<EtfGlobalProfile>(&path, Some(&query), options)
            } else {
                self.single_page::<EtfGlobalProfile>(&path, Some(&query), options)
            }
        })
    }

    fn get_etf_global_taxonomies<'a>(
        &'a self,
        processed_date: Option<&'a str>,
        processed_date_gt: Option<&'a str>,
        processed_date_gte: Option<&'a str>,
        processed_date_lt: Option<&'a str>,
        processed_date_lte: Option<&'a str>,
        effective_date: Option<&'a str>,
        effective_date_gt: Option<&'a str>,
        effective_date_gte: Option<&'a str>,
        effective_date_lt: Option<&'a str>,
        effective_date_lte: Option<&'a str>,
        composite_ticker: Option<&'a str>,
        composite_ticker_any_of: Option<&'a str>,
        composite_ticker_gt: Option<&'a str>,
        composite_ticker_gte: Option<&'a str>,
        composite_ticker_lt: Option<&'a str>,
        composite_ticker_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, EtfGlobalTaxonomy> {
        self.get_etf_global_taxonomies_with_params(GetEtfGlobalTaxonomiesParams {
            processed_date: processed_date.map(String::from),
            processed_date_gt: processed_date_gt.map(String::from),
            processed_date_gte: processed_date_gte.map(String::from),
            processed_date_lt: processed_date_lt.map(String::from),
            processed_date_lte: processed_date_lte.map(String::from),
            effective_date: effective_date.map(String::from),
            effective_date_gt: effective_date_gt.map(String::from),
            effective_date_gte: effective_date_gte.map(String::from),
            effective_date_lt: effective_date_lt.map(String::from),
            effective_date_lte: effective_date_lte.map(String::from),
            composite_ticker: composite_ticker.map(String::from),
            composite_ticker_any_of: composite_ticker_any_of.map(String::from),
            composite_ticker_gt: composite_ticker_gt.map(String::from),
            composite_ticker_gte: composite_ticker_gte.map(String::from),
            composite_ticker_lt: composite_ticker_lt.map(String::from),
            composite_ticker_lte: composite_ticker_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn get_etf_global_taxonomies_with_params<'a>(
        &'a self,
        params: GetEtfGlobalTaxonomiesParams,
    ) -> BoxStream<'a, EtfGlobalTaxonomy> {
        Box::pin({
            let GetEtfGlobalTaxonomiesParams {
                processed_date,
                processed_date_gt,
                processed_date_gte,
                processed_date_lt,
                processed_date_lte,
                effective_date,
                effective_date_gt,
                effective_date_gte,
                effective_date_lt,
                effective_date_lte,
                composite_ticker,
                composite_ticker_any_of,
                composite_ticker_gt,
                composite_ticker_gte,
                composite_ticker_lt,
                composite_ticker_lte,
                limit,
                sort,
                options,
            } = params;
            let processed_date = processed_date.as_deref();
            let processed_date_gt = processed_date_gt.as_deref();
            let processed_date_gte = processed_date_gte.as_deref();
            let processed_date_lt = processed_date_lt.as_deref();
            let processed_date_lte = processed_date_lte.as_deref();
            let effective_date = effective_date.as_deref();
            let effective_date_gt = effective_date_gt.as_deref();
            let effective_date_gte = effective_date_gte.as_deref();
            let effective_date_lt = effective_date_lt.as_deref();
            let effective_date_lte = effective_date_lte.as_deref();
            let composite_ticker = composite_ticker.as_deref();
            let composite_ticker_any_of = composite_ticker_any_of.as_deref();
            let composite_ticker_gt = composite_ticker_gt.as_deref();
            let composite_ticker_gte = composite_ticker_gte.as_deref();
            let composite_ticker_lt = composite_ticker_lt.as_deref();
            let composite_ticker_lte = composite_ticker_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/etf-global/v1/taxonomies".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            push_param(&mut query, "processed_date", processed_date);
            push_param(&mut query, "processed_date.gt", processed_date_gt);
            push_param(&mut query, "processed_date.gte", processed_date_gte);
            push_param(&mut query, "processed_date.lt", processed_date_lt);
            push_param(&mut query, "processed_date.lte", processed_date_lte);
            push_param(&mut query, "effective_date", effective_date);
            push_param(&mut query, "effective_date.gt", effective_date_gt);
            push_param(&mut query, "effective_date.gte", effective_date_gte);
            push_param(&mut query, "effective_date.lt", effective_date_lt);
            push_param(&mut query, "effective_date.lte", effective_date_lte);
            push_param(&mut query, "composite_ticker", composite_ticker);
            push_param(
                &mut query,
                "composite_ticker.any_of",
                composite_ticker_any_of,
            );
            push_param(&mut query, "composite_ticker.gt", composite_ticker_gt);
            push_param(&mut query, "composite_ticker.gte", composite_ticker_gte);
            push_param(&mut query, "composite_ticker.lt", composite_ticker_lt);
            push_param(&mut query, "composite_ticker.lte", composite_ticker_lte);
            push_param(&mut query, "limit", limit);
            push_param(&mut query, "sort", sort);
            if self.pagination {
                self.paginate::<EtfGlobalTaxonomy>(&path, Some(&query), options)
            } else {
                self.single_page::<EtfGlobalTaxonomy>(&path, Some(&query), options)
            }
        })
    }
}

// --- Params structs (additive builder API) ---

/// Optional arguments for [`EtfGlobalApi::get_etf_global_analytics`].
#[derive(Debug, Default, Clone)]
pub struct GetEtfGlobalAnalyticsParams {
    /// The `composite_ticker` argument.
    pub composite_ticker: Option<String>,
    /// The `composite_ticker_any_of` argument.
    pub composite_ticker_any_of: Option<String>,
    /// The `composite_ticker_gt` argument.
    pub composite_ticker_gt: Option<String>,
    /// The `composite_ticker_gte` argument.
    pub composite_ticker_gte: Option<String>,
    /// The `composite_ticker_lt` argument.
    pub composite_ticker_lt: Option<String>,
    /// The `composite_ticker_lte` argument.
    pub composite_ticker_lte: Option<String>,
    /// The `processed_date` argument.
    pub processed_date: Option<String>,
    /// The `processed_date_gt` argument.
    pub processed_date_gt: Option<String>,
    /// The `processed_date_gte` argument.
    pub processed_date_gte: Option<String>,
    /// The `processed_date_lt` argument.
    pub processed_date_lt: Option<String>,
    /// The `processed_date_lte` argument.
    pub processed_date_lte: Option<String>,
    /// The `effective_date` argument.
    pub effective_date: Option<String>,
    /// The `effective_date_gt` argument.
    pub effective_date_gt: Option<String>,
    /// The `effective_date_gte` argument.
    pub effective_date_gte: Option<String>,
    /// The `effective_date_lt` argument.
    pub effective_date_lt: Option<String>,
    /// The `effective_date_lte` argument.
    pub effective_date_lte: Option<String>,
    /// The `risk_total_score` argument.
    pub risk_total_score: Option<f64>,
    /// The `risk_total_score_gt` argument.
    pub risk_total_score_gt: Option<f64>,
    /// The `risk_total_score_gte` argument.
    pub risk_total_score_gte: Option<f64>,
    /// The `risk_total_score_lt` argument.
    pub risk_total_score_lt: Option<f64>,
    /// The `risk_total_score_lte` argument.
    pub risk_total_score_lte: Option<f64>,
    /// The `reward_score` argument.
    pub reward_score: Option<f64>,
    /// The `reward_score_gt` argument.
    pub reward_score_gt: Option<f64>,
    /// The `reward_score_gte` argument.
    pub reward_score_gte: Option<f64>,
    /// The `reward_score_lt` argument.
    pub reward_score_lt: Option<f64>,
    /// The `reward_score_lte` argument.
    pub reward_score_lte: Option<f64>,
    /// The `quant_total_score` argument.
    pub quant_total_score: Option<f64>,
    /// The `quant_total_score_gt` argument.
    pub quant_total_score_gt: Option<f64>,
    /// The `quant_total_score_gte` argument.
    pub quant_total_score_gte: Option<f64>,
    /// The `quant_total_score_lt` argument.
    pub quant_total_score_lt: Option<f64>,
    /// The `quant_total_score_lte` argument.
    pub quant_total_score_lte: Option<f64>,
    /// The `quant_grade` argument.
    pub quant_grade: Option<String>,
    /// The `quant_grade_any_of` argument.
    pub quant_grade_any_of: Option<String>,
    /// The `quant_grade_gt` argument.
    pub quant_grade_gt: Option<String>,
    /// The `quant_grade_gte` argument.
    pub quant_grade_gte: Option<String>,
    /// The `quant_grade_lt` argument.
    pub quant_grade_lt: Option<String>,
    /// The `quant_grade_lte` argument.
    pub quant_grade_lte: Option<String>,
    /// The `quant_composite_technical` argument.
    pub quant_composite_technical: Option<f64>,
    /// The `quant_composite_technical_gt` argument.
    pub quant_composite_technical_gt: Option<f64>,
    /// The `quant_composite_technical_gte` argument.
    pub quant_composite_technical_gte: Option<f64>,
    /// The `quant_composite_technical_lt` argument.
    pub quant_composite_technical_lt: Option<f64>,
    /// The `quant_composite_technical_lte` argument.
    pub quant_composite_technical_lte: Option<f64>,
    /// The `quant_composite_sentiment` argument.
    pub quant_composite_sentiment: Option<f64>,
    /// The `quant_composite_sentiment_gt` argument.
    pub quant_composite_sentiment_gt: Option<f64>,
    /// The `quant_composite_sentiment_gte` argument.
    pub quant_composite_sentiment_gte: Option<f64>,
    /// The `quant_composite_sentiment_lt` argument.
    pub quant_composite_sentiment_lt: Option<f64>,
    /// The `quant_composite_sentiment_lte` argument.
    pub quant_composite_sentiment_lte: Option<f64>,
    /// The `quant_composite_behavioral` argument.
    pub quant_composite_behavioral: Option<f64>,
    /// The `quant_composite_behavioral_gt` argument.
    pub quant_composite_behavioral_gt: Option<f64>,
    /// The `quant_composite_behavioral_gte` argument.
    pub quant_composite_behavioral_gte: Option<f64>,
    /// The `quant_composite_behavioral_lt` argument.
    pub quant_composite_behavioral_lt: Option<f64>,
    /// The `quant_composite_behavioral_lte` argument.
    pub quant_composite_behavioral_lte: Option<f64>,
    /// The `quant_composite_fundamental` argument.
    pub quant_composite_fundamental: Option<f64>,
    /// The `quant_composite_fundamental_gt` argument.
    pub quant_composite_fundamental_gt: Option<f64>,
    /// The `quant_composite_fundamental_gte` argument.
    pub quant_composite_fundamental_gte: Option<f64>,
    /// The `quant_composite_fundamental_lt` argument.
    pub quant_composite_fundamental_lt: Option<f64>,
    /// The `quant_composite_fundamental_lte` argument.
    pub quant_composite_fundamental_lte: Option<f64>,
    /// The `quant_composite_global` argument.
    pub quant_composite_global: Option<f64>,
    /// The `quant_composite_global_gt` argument.
    pub quant_composite_global_gt: Option<f64>,
    /// The `quant_composite_global_gte` argument.
    pub quant_composite_global_gte: Option<f64>,
    /// The `quant_composite_global_lt` argument.
    pub quant_composite_global_lt: Option<f64>,
    /// The `quant_composite_global_lte` argument.
    pub quant_composite_global_lte: Option<f64>,
    /// The `quant_composite_quality` argument.
    pub quant_composite_quality: Option<f64>,
    /// The `quant_composite_quality_gt` argument.
    pub quant_composite_quality_gt: Option<f64>,
    /// The `quant_composite_quality_gte` argument.
    pub quant_composite_quality_gte: Option<f64>,
    /// The `quant_composite_quality_lt` argument.
    pub quant_composite_quality_lt: Option<f64>,
    /// The `quant_composite_quality_lte` argument.
    pub quant_composite_quality_lte: Option<f64>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetEtfGlobalAnalyticsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `composite_ticker` argument.
    pub fn composite_ticker(mut self, composite_ticker: impl Into<String>) -> Self {
        self.composite_ticker = Some(composite_ticker.into());
        self
    }

    /// Set the `composite_ticker_any_of` argument.
    pub fn composite_ticker_any_of(mut self, composite_ticker_any_of: impl Into<String>) -> Self {
        self.composite_ticker_any_of = Some(composite_ticker_any_of.into());
        self
    }

    /// Set the `composite_ticker_gt` argument.
    pub fn composite_ticker_gt(mut self, composite_ticker_gt: impl Into<String>) -> Self {
        self.composite_ticker_gt = Some(composite_ticker_gt.into());
        self
    }

    /// Set the `composite_ticker_gte` argument.
    pub fn composite_ticker_gte(mut self, composite_ticker_gte: impl Into<String>) -> Self {
        self.composite_ticker_gte = Some(composite_ticker_gte.into());
        self
    }

    /// Set the `composite_ticker_lt` argument.
    pub fn composite_ticker_lt(mut self, composite_ticker_lt: impl Into<String>) -> Self {
        self.composite_ticker_lt = Some(composite_ticker_lt.into());
        self
    }

    /// Set the `composite_ticker_lte` argument.
    pub fn composite_ticker_lte(mut self, composite_ticker_lte: impl Into<String>) -> Self {
        self.composite_ticker_lte = Some(composite_ticker_lte.into());
        self
    }

    /// Set the `processed_date` argument.
    pub fn processed_date(mut self, processed_date: impl Into<String>) -> Self {
        self.processed_date = Some(processed_date.into());
        self
    }

    /// Set the `processed_date_gt` argument.
    pub fn processed_date_gt(mut self, processed_date_gt: impl Into<String>) -> Self {
        self.processed_date_gt = Some(processed_date_gt.into());
        self
    }

    /// Set the `processed_date_gte` argument.
    pub fn processed_date_gte(mut self, processed_date_gte: impl Into<String>) -> Self {
        self.processed_date_gte = Some(processed_date_gte.into());
        self
    }

    /// Set the `processed_date_lt` argument.
    pub fn processed_date_lt(mut self, processed_date_lt: impl Into<String>) -> Self {
        self.processed_date_lt = Some(processed_date_lt.into());
        self
    }

    /// Set the `processed_date_lte` argument.
    pub fn processed_date_lte(mut self, processed_date_lte: impl Into<String>) -> Self {
        self.processed_date_lte = Some(processed_date_lte.into());
        self
    }

    /// Set the `effective_date` argument.
    pub fn effective_date(mut self, effective_date: impl Into<String>) -> Self {
        self.effective_date = Some(effective_date.into());
        self
    }

    /// Set the `effective_date_gt` argument.
    pub fn effective_date_gt(mut self, effective_date_gt: impl Into<String>) -> Self {
        self.effective_date_gt = Some(effective_date_gt.into());
        self
    }

    /// Set the `effective_date_gte` argument.
    pub fn effective_date_gte(mut self, effective_date_gte: impl Into<String>) -> Self {
        self.effective_date_gte = Some(effective_date_gte.into());
        self
    }

    /// Set the `effective_date_lt` argument.
    pub fn effective_date_lt(mut self, effective_date_lt: impl Into<String>) -> Self {
        self.effective_date_lt = Some(effective_date_lt.into());
        self
    }

    /// Set the `effective_date_lte` argument.
    pub fn effective_date_lte(mut self, effective_date_lte: impl Into<String>) -> Self {
        self.effective_date_lte = Some(effective_date_lte.into());
        self
    }

    /// Set the `risk_total_score` argument.
    pub fn risk_total_score(mut self, risk_total_score: f64) -> Self {
        self.risk_total_score = Some(risk_total_score);
        self
    }

    /// Set the `risk_total_score_gt` argument.
    pub fn risk_total_score_gt(mut self, risk_total_score_gt: f64) -> Self {
        self.risk_total_score_gt = Some(risk_total_score_gt);
        self
    }

    /// Set the `risk_total_score_gte` argument.
    pub fn risk_total_score_gte(mut self, risk_total_score_gte: f64) -> Self {
        self.risk_total_score_gte = Some(risk_total_score_gte);
        self
    }

    /// Set the `risk_total_score_lt` argument.
    pub fn risk_total_score_lt(mut self, risk_total_score_lt: f64) -> Self {
        self.risk_total_score_lt = Some(risk_total_score_lt);
        self
    }

    /// Set the `risk_total_score_lte` argument.
    pub fn risk_total_score_lte(mut self, risk_total_score_lte: f64) -> Self {
        self.risk_total_score_lte = Some(risk_total_score_lte);
        self
    }

    /// Set the `reward_score` argument.
    pub fn reward_score(mut self, reward_score: f64) -> Self {
        self.reward_score = Some(reward_score);
        self
    }

    /// Set the `reward_score_gt` argument.
    pub fn reward_score_gt(mut self, reward_score_gt: f64) -> Self {
        self.reward_score_gt = Some(reward_score_gt);
        self
    }

    /// Set the `reward_score_gte` argument.
    pub fn reward_score_gte(mut self, reward_score_gte: f64) -> Self {
        self.reward_score_gte = Some(reward_score_gte);
        self
    }

    /// Set the `reward_score_lt` argument.
    pub fn reward_score_lt(mut self, reward_score_lt: f64) -> Self {
        self.reward_score_lt = Some(reward_score_lt);
        self
    }

    /// Set the `reward_score_lte` argument.
    pub fn reward_score_lte(mut self, reward_score_lte: f64) -> Self {
        self.reward_score_lte = Some(reward_score_lte);
        self
    }

    /// Set the `quant_total_score` argument.
    pub fn quant_total_score(mut self, quant_total_score: f64) -> Self {
        self.quant_total_score = Some(quant_total_score);
        self
    }

    /// Set the `quant_total_score_gt` argument.
    pub fn quant_total_score_gt(mut self, quant_total_score_gt: f64) -> Self {
        self.quant_total_score_gt = Some(quant_total_score_gt);
        self
    }

    /// Set the `quant_total_score_gte` argument.
    pub fn quant_total_score_gte(mut self, quant_total_score_gte: f64) -> Self {
        self.quant_total_score_gte = Some(quant_total_score_gte);
        self
    }

    /// Set the `quant_total_score_lt` argument.
    pub fn quant_total_score_lt(mut self, quant_total_score_lt: f64) -> Self {
        self.quant_total_score_lt = Some(quant_total_score_lt);
        self
    }

    /// Set the `quant_total_score_lte` argument.
    pub fn quant_total_score_lte(mut self, quant_total_score_lte: f64) -> Self {
        self.quant_total_score_lte = Some(quant_total_score_lte);
        self
    }

    /// Set the `quant_grade` argument.
    pub fn quant_grade(mut self, quant_grade: impl Into<String>) -> Self {
        self.quant_grade = Some(quant_grade.into());
        self
    }

    /// Set the `quant_grade_any_of` argument.
    pub fn quant_grade_any_of(mut self, quant_grade_any_of: impl Into<String>) -> Self {
        self.quant_grade_any_of = Some(quant_grade_any_of.into());
        self
    }

    /// Set the `quant_grade_gt` argument.
    pub fn quant_grade_gt(mut self, quant_grade_gt: impl Into<String>) -> Self {
        self.quant_grade_gt = Some(quant_grade_gt.into());
        self
    }

    /// Set the `quant_grade_gte` argument.
    pub fn quant_grade_gte(mut self, quant_grade_gte: impl Into<String>) -> Self {
        self.quant_grade_gte = Some(quant_grade_gte.into());
        self
    }

    /// Set the `quant_grade_lt` argument.
    pub fn quant_grade_lt(mut self, quant_grade_lt: impl Into<String>) -> Self {
        self.quant_grade_lt = Some(quant_grade_lt.into());
        self
    }

    /// Set the `quant_grade_lte` argument.
    pub fn quant_grade_lte(mut self, quant_grade_lte: impl Into<String>) -> Self {
        self.quant_grade_lte = Some(quant_grade_lte.into());
        self
    }

    /// Set the `quant_composite_technical` argument.
    pub fn quant_composite_technical(mut self, quant_composite_technical: f64) -> Self {
        self.quant_composite_technical = Some(quant_composite_technical);
        self
    }

    /// Set the `quant_composite_technical_gt` argument.
    pub fn quant_composite_technical_gt(mut self, quant_composite_technical_gt: f64) -> Self {
        self.quant_composite_technical_gt = Some(quant_composite_technical_gt);
        self
    }

    /// Set the `quant_composite_technical_gte` argument.
    pub fn quant_composite_technical_gte(mut self, quant_composite_technical_gte: f64) -> Self {
        self.quant_composite_technical_gte = Some(quant_composite_technical_gte);
        self
    }

    /// Set the `quant_composite_technical_lt` argument.
    pub fn quant_composite_technical_lt(mut self, quant_composite_technical_lt: f64) -> Self {
        self.quant_composite_technical_lt = Some(quant_composite_technical_lt);
        self
    }

    /// Set the `quant_composite_technical_lte` argument.
    pub fn quant_composite_technical_lte(mut self, quant_composite_technical_lte: f64) -> Self {
        self.quant_composite_technical_lte = Some(quant_composite_technical_lte);
        self
    }

    /// Set the `quant_composite_sentiment` argument.
    pub fn quant_composite_sentiment(mut self, quant_composite_sentiment: f64) -> Self {
        self.quant_composite_sentiment = Some(quant_composite_sentiment);
        self
    }

    /// Set the `quant_composite_sentiment_gt` argument.
    pub fn quant_composite_sentiment_gt(mut self, quant_composite_sentiment_gt: f64) -> Self {
        self.quant_composite_sentiment_gt = Some(quant_composite_sentiment_gt);
        self
    }

    /// Set the `quant_composite_sentiment_gte` argument.
    pub fn quant_composite_sentiment_gte(mut self, quant_composite_sentiment_gte: f64) -> Self {
        self.quant_composite_sentiment_gte = Some(quant_composite_sentiment_gte);
        self
    }

    /// Set the `quant_composite_sentiment_lt` argument.
    pub fn quant_composite_sentiment_lt(mut self, quant_composite_sentiment_lt: f64) -> Self {
        self.quant_composite_sentiment_lt = Some(quant_composite_sentiment_lt);
        self
    }

    /// Set the `quant_composite_sentiment_lte` argument.
    pub fn quant_composite_sentiment_lte(mut self, quant_composite_sentiment_lte: f64) -> Self {
        self.quant_composite_sentiment_lte = Some(quant_composite_sentiment_lte);
        self
    }

    /// Set the `quant_composite_behavioral` argument.
    pub fn quant_composite_behavioral(mut self, quant_composite_behavioral: f64) -> Self {
        self.quant_composite_behavioral = Some(quant_composite_behavioral);
        self
    }

    /// Set the `quant_composite_behavioral_gt` argument.
    pub fn quant_composite_behavioral_gt(mut self, quant_composite_behavioral_gt: f64) -> Self {
        self.quant_composite_behavioral_gt = Some(quant_composite_behavioral_gt);
        self
    }

    /// Set the `quant_composite_behavioral_gte` argument.
    pub fn quant_composite_behavioral_gte(mut self, quant_composite_behavioral_gte: f64) -> Self {
        self.quant_composite_behavioral_gte = Some(quant_composite_behavioral_gte);
        self
    }

    /// Set the `quant_composite_behavioral_lt` argument.
    pub fn quant_composite_behavioral_lt(mut self, quant_composite_behavioral_lt: f64) -> Self {
        self.quant_composite_behavioral_lt = Some(quant_composite_behavioral_lt);
        self
    }

    /// Set the `quant_composite_behavioral_lte` argument.
    pub fn quant_composite_behavioral_lte(mut self, quant_composite_behavioral_lte: f64) -> Self {
        self.quant_composite_behavioral_lte = Some(quant_composite_behavioral_lte);
        self
    }

    /// Set the `quant_composite_fundamental` argument.
    pub fn quant_composite_fundamental(mut self, quant_composite_fundamental: f64) -> Self {
        self.quant_composite_fundamental = Some(quant_composite_fundamental);
        self
    }

    /// Set the `quant_composite_fundamental_gt` argument.
    pub fn quant_composite_fundamental_gt(mut self, quant_composite_fundamental_gt: f64) -> Self {
        self.quant_composite_fundamental_gt = Some(quant_composite_fundamental_gt);
        self
    }

    /// Set the `quant_composite_fundamental_gte` argument.
    pub fn quant_composite_fundamental_gte(mut self, quant_composite_fundamental_gte: f64) -> Self {
        self.quant_composite_fundamental_gte = Some(quant_composite_fundamental_gte);
        self
    }

    /// Set the `quant_composite_fundamental_lt` argument.
    pub fn quant_composite_fundamental_lt(mut self, quant_composite_fundamental_lt: f64) -> Self {
        self.quant_composite_fundamental_lt = Some(quant_composite_fundamental_lt);
        self
    }

    /// Set the `quant_composite_fundamental_lte` argument.
    pub fn quant_composite_fundamental_lte(mut self, quant_composite_fundamental_lte: f64) -> Self {
        self.quant_composite_fundamental_lte = Some(quant_composite_fundamental_lte);
        self
    }

    /// Set the `quant_composite_global` argument.
    pub fn quant_composite_global(mut self, quant_composite_global: f64) -> Self {
        self.quant_composite_global = Some(quant_composite_global);
        self
    }

    /// Set the `quant_composite_global_gt` argument.
    pub fn quant_composite_global_gt(mut self, quant_composite_global_gt: f64) -> Self {
        self.quant_composite_global_gt = Some(quant_composite_global_gt);
        self
    }

    /// Set the `quant_composite_global_gte` argument.
    pub fn quant_composite_global_gte(mut self, quant_composite_global_gte: f64) -> Self {
        self.quant_composite_global_gte = Some(quant_composite_global_gte);
        self
    }

    /// Set the `quant_composite_global_lt` argument.
    pub fn quant_composite_global_lt(mut self, quant_composite_global_lt: f64) -> Self {
        self.quant_composite_global_lt = Some(quant_composite_global_lt);
        self
    }

    /// Set the `quant_composite_global_lte` argument.
    pub fn quant_composite_global_lte(mut self, quant_composite_global_lte: f64) -> Self {
        self.quant_composite_global_lte = Some(quant_composite_global_lte);
        self
    }

    /// Set the `quant_composite_quality` argument.
    pub fn quant_composite_quality(mut self, quant_composite_quality: f64) -> Self {
        self.quant_composite_quality = Some(quant_composite_quality);
        self
    }

    /// Set the `quant_composite_quality_gt` argument.
    pub fn quant_composite_quality_gt(mut self, quant_composite_quality_gt: f64) -> Self {
        self.quant_composite_quality_gt = Some(quant_composite_quality_gt);
        self
    }

    /// Set the `quant_composite_quality_gte` argument.
    pub fn quant_composite_quality_gte(mut self, quant_composite_quality_gte: f64) -> Self {
        self.quant_composite_quality_gte = Some(quant_composite_quality_gte);
        self
    }

    /// Set the `quant_composite_quality_lt` argument.
    pub fn quant_composite_quality_lt(mut self, quant_composite_quality_lt: f64) -> Self {
        self.quant_composite_quality_lt = Some(quant_composite_quality_lt);
        self
    }

    /// Set the `quant_composite_quality_lte` argument.
    pub fn quant_composite_quality_lte(mut self, quant_composite_quality_lte: f64) -> Self {
        self.quant_composite_quality_lte = Some(quant_composite_quality_lte);
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

/// Optional arguments for [`EtfGlobalApi::get_etf_global_constituents`].
#[derive(Debug, Default, Clone)]
pub struct GetEtfGlobalConstituentsParams {
    /// The `composite_ticker` argument.
    pub composite_ticker: Option<String>,
    /// The `composite_ticker_any_of` argument.
    pub composite_ticker_any_of: Option<String>,
    /// The `composite_ticker_gt` argument.
    pub composite_ticker_gt: Option<String>,
    /// The `composite_ticker_gte` argument.
    pub composite_ticker_gte: Option<String>,
    /// The `composite_ticker_lt` argument.
    pub composite_ticker_lt: Option<String>,
    /// The `composite_ticker_lte` argument.
    pub composite_ticker_lte: Option<String>,
    /// The `constituent_ticker` argument.
    pub constituent_ticker: Option<String>,
    /// The `constituent_ticker_any_of` argument.
    pub constituent_ticker_any_of: Option<String>,
    /// The `constituent_ticker_gt` argument.
    pub constituent_ticker_gt: Option<String>,
    /// The `constituent_ticker_gte` argument.
    pub constituent_ticker_gte: Option<String>,
    /// The `constituent_ticker_lt` argument.
    pub constituent_ticker_lt: Option<String>,
    /// The `constituent_ticker_lte` argument.
    pub constituent_ticker_lte: Option<String>,
    /// The `effective_date` argument.
    pub effective_date: Option<String>,
    /// The `effective_date_gt` argument.
    pub effective_date_gt: Option<String>,
    /// The `effective_date_gte` argument.
    pub effective_date_gte: Option<String>,
    /// The `effective_date_lt` argument.
    pub effective_date_lt: Option<String>,
    /// The `effective_date_lte` argument.
    pub effective_date_lte: Option<String>,
    /// The `processed_date` argument.
    pub processed_date: Option<String>,
    /// The `processed_date_gt` argument.
    pub processed_date_gt: Option<String>,
    /// The `processed_date_gte` argument.
    pub processed_date_gte: Option<String>,
    /// The `processed_date_lt` argument.
    pub processed_date_lt: Option<String>,
    /// The `processed_date_lte` argument.
    pub processed_date_lte: Option<String>,
    /// The `us_code` argument.
    pub us_code: Option<String>,
    /// The `us_code_any_of` argument.
    pub us_code_any_of: Option<String>,
    /// The `us_code_gt` argument.
    pub us_code_gt: Option<String>,
    /// The `us_code_gte` argument.
    pub us_code_gte: Option<String>,
    /// The `us_code_lt` argument.
    pub us_code_lt: Option<String>,
    /// The `us_code_lte` argument.
    pub us_code_lte: Option<String>,
    /// The `isin` argument.
    pub isin: Option<String>,
    /// The `isin_any_of` argument.
    pub isin_any_of: Option<String>,
    /// The `isin_gt` argument.
    pub isin_gt: Option<String>,
    /// The `isin_gte` argument.
    pub isin_gte: Option<String>,
    /// The `isin_lt` argument.
    pub isin_lt: Option<String>,
    /// The `isin_lte` argument.
    pub isin_lte: Option<String>,
    /// The `figi` argument.
    pub figi: Option<String>,
    /// The `figi_any_of` argument.
    pub figi_any_of: Option<String>,
    /// The `figi_gt` argument.
    pub figi_gt: Option<String>,
    /// The `figi_gte` argument.
    pub figi_gte: Option<String>,
    /// The `figi_lt` argument.
    pub figi_lt: Option<String>,
    /// The `figi_lte` argument.
    pub figi_lte: Option<String>,
    /// The `sedol` argument.
    pub sedol: Option<String>,
    /// The `sedol_any_of` argument.
    pub sedol_any_of: Option<String>,
    /// The `sedol_gt` argument.
    pub sedol_gt: Option<String>,
    /// The `sedol_gte` argument.
    pub sedol_gte: Option<String>,
    /// The `sedol_lt` argument.
    pub sedol_lt: Option<String>,
    /// The `sedol_lte` argument.
    pub sedol_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetEtfGlobalConstituentsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `composite_ticker` argument.
    pub fn composite_ticker(mut self, composite_ticker: impl Into<String>) -> Self {
        self.composite_ticker = Some(composite_ticker.into());
        self
    }

    /// Set the `composite_ticker_any_of` argument.
    pub fn composite_ticker_any_of(mut self, composite_ticker_any_of: impl Into<String>) -> Self {
        self.composite_ticker_any_of = Some(composite_ticker_any_of.into());
        self
    }

    /// Set the `composite_ticker_gt` argument.
    pub fn composite_ticker_gt(mut self, composite_ticker_gt: impl Into<String>) -> Self {
        self.composite_ticker_gt = Some(composite_ticker_gt.into());
        self
    }

    /// Set the `composite_ticker_gte` argument.
    pub fn composite_ticker_gte(mut self, composite_ticker_gte: impl Into<String>) -> Self {
        self.composite_ticker_gte = Some(composite_ticker_gte.into());
        self
    }

    /// Set the `composite_ticker_lt` argument.
    pub fn composite_ticker_lt(mut self, composite_ticker_lt: impl Into<String>) -> Self {
        self.composite_ticker_lt = Some(composite_ticker_lt.into());
        self
    }

    /// Set the `composite_ticker_lte` argument.
    pub fn composite_ticker_lte(mut self, composite_ticker_lte: impl Into<String>) -> Self {
        self.composite_ticker_lte = Some(composite_ticker_lte.into());
        self
    }

    /// Set the `constituent_ticker` argument.
    pub fn constituent_ticker(mut self, constituent_ticker: impl Into<String>) -> Self {
        self.constituent_ticker = Some(constituent_ticker.into());
        self
    }

    /// Set the `constituent_ticker_any_of` argument.
    pub fn constituent_ticker_any_of(
        mut self,
        constituent_ticker_any_of: impl Into<String>,
    ) -> Self {
        self.constituent_ticker_any_of = Some(constituent_ticker_any_of.into());
        self
    }

    /// Set the `constituent_ticker_gt` argument.
    pub fn constituent_ticker_gt(mut self, constituent_ticker_gt: impl Into<String>) -> Self {
        self.constituent_ticker_gt = Some(constituent_ticker_gt.into());
        self
    }

    /// Set the `constituent_ticker_gte` argument.
    pub fn constituent_ticker_gte(mut self, constituent_ticker_gte: impl Into<String>) -> Self {
        self.constituent_ticker_gte = Some(constituent_ticker_gte.into());
        self
    }

    /// Set the `constituent_ticker_lt` argument.
    pub fn constituent_ticker_lt(mut self, constituent_ticker_lt: impl Into<String>) -> Self {
        self.constituent_ticker_lt = Some(constituent_ticker_lt.into());
        self
    }

    /// Set the `constituent_ticker_lte` argument.
    pub fn constituent_ticker_lte(mut self, constituent_ticker_lte: impl Into<String>) -> Self {
        self.constituent_ticker_lte = Some(constituent_ticker_lte.into());
        self
    }

    /// Set the `effective_date` argument.
    pub fn effective_date(mut self, effective_date: impl Into<String>) -> Self {
        self.effective_date = Some(effective_date.into());
        self
    }

    /// Set the `effective_date_gt` argument.
    pub fn effective_date_gt(mut self, effective_date_gt: impl Into<String>) -> Self {
        self.effective_date_gt = Some(effective_date_gt.into());
        self
    }

    /// Set the `effective_date_gte` argument.
    pub fn effective_date_gte(mut self, effective_date_gte: impl Into<String>) -> Self {
        self.effective_date_gte = Some(effective_date_gte.into());
        self
    }

    /// Set the `effective_date_lt` argument.
    pub fn effective_date_lt(mut self, effective_date_lt: impl Into<String>) -> Self {
        self.effective_date_lt = Some(effective_date_lt.into());
        self
    }

    /// Set the `effective_date_lte` argument.
    pub fn effective_date_lte(mut self, effective_date_lte: impl Into<String>) -> Self {
        self.effective_date_lte = Some(effective_date_lte.into());
        self
    }

    /// Set the `processed_date` argument.
    pub fn processed_date(mut self, processed_date: impl Into<String>) -> Self {
        self.processed_date = Some(processed_date.into());
        self
    }

    /// Set the `processed_date_gt` argument.
    pub fn processed_date_gt(mut self, processed_date_gt: impl Into<String>) -> Self {
        self.processed_date_gt = Some(processed_date_gt.into());
        self
    }

    /// Set the `processed_date_gte` argument.
    pub fn processed_date_gte(mut self, processed_date_gte: impl Into<String>) -> Self {
        self.processed_date_gte = Some(processed_date_gte.into());
        self
    }

    /// Set the `processed_date_lt` argument.
    pub fn processed_date_lt(mut self, processed_date_lt: impl Into<String>) -> Self {
        self.processed_date_lt = Some(processed_date_lt.into());
        self
    }

    /// Set the `processed_date_lte` argument.
    pub fn processed_date_lte(mut self, processed_date_lte: impl Into<String>) -> Self {
        self.processed_date_lte = Some(processed_date_lte.into());
        self
    }

    /// Set the `us_code` argument.
    pub fn us_code(mut self, us_code: impl Into<String>) -> Self {
        self.us_code = Some(us_code.into());
        self
    }

    /// Set the `us_code_any_of` argument.
    pub fn us_code_any_of(mut self, us_code_any_of: impl Into<String>) -> Self {
        self.us_code_any_of = Some(us_code_any_of.into());
        self
    }

    /// Set the `us_code_gt` argument.
    pub fn us_code_gt(mut self, us_code_gt: impl Into<String>) -> Self {
        self.us_code_gt = Some(us_code_gt.into());
        self
    }

    /// Set the `us_code_gte` argument.
    pub fn us_code_gte(mut self, us_code_gte: impl Into<String>) -> Self {
        self.us_code_gte = Some(us_code_gte.into());
        self
    }

    /// Set the `us_code_lt` argument.
    pub fn us_code_lt(mut self, us_code_lt: impl Into<String>) -> Self {
        self.us_code_lt = Some(us_code_lt.into());
        self
    }

    /// Set the `us_code_lte` argument.
    pub fn us_code_lte(mut self, us_code_lte: impl Into<String>) -> Self {
        self.us_code_lte = Some(us_code_lte.into());
        self
    }

    /// Set the `isin` argument.
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }

    /// Set the `isin_any_of` argument.
    pub fn isin_any_of(mut self, isin_any_of: impl Into<String>) -> Self {
        self.isin_any_of = Some(isin_any_of.into());
        self
    }

    /// Set the `isin_gt` argument.
    pub fn isin_gt(mut self, isin_gt: impl Into<String>) -> Self {
        self.isin_gt = Some(isin_gt.into());
        self
    }

    /// Set the `isin_gte` argument.
    pub fn isin_gte(mut self, isin_gte: impl Into<String>) -> Self {
        self.isin_gte = Some(isin_gte.into());
        self
    }

    /// Set the `isin_lt` argument.
    pub fn isin_lt(mut self, isin_lt: impl Into<String>) -> Self {
        self.isin_lt = Some(isin_lt.into());
        self
    }

    /// Set the `isin_lte` argument.
    pub fn isin_lte(mut self, isin_lte: impl Into<String>) -> Self {
        self.isin_lte = Some(isin_lte.into());
        self
    }

    /// Set the `figi` argument.
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        self
    }

    /// Set the `figi_any_of` argument.
    pub fn figi_any_of(mut self, figi_any_of: impl Into<String>) -> Self {
        self.figi_any_of = Some(figi_any_of.into());
        self
    }

    /// Set the `figi_gt` argument.
    pub fn figi_gt(mut self, figi_gt: impl Into<String>) -> Self {
        self.figi_gt = Some(figi_gt.into());
        self
    }

    /// Set the `figi_gte` argument.
    pub fn figi_gte(mut self, figi_gte: impl Into<String>) -> Self {
        self.figi_gte = Some(figi_gte.into());
        self
    }

    /// Set the `figi_lt` argument.
    pub fn figi_lt(mut self, figi_lt: impl Into<String>) -> Self {
        self.figi_lt = Some(figi_lt.into());
        self
    }

    /// Set the `figi_lte` argument.
    pub fn figi_lte(mut self, figi_lte: impl Into<String>) -> Self {
        self.figi_lte = Some(figi_lte.into());
        self
    }

    /// Set the `sedol` argument.
    pub fn sedol(mut self, sedol: impl Into<String>) -> Self {
        self.sedol = Some(sedol.into());
        self
    }

    /// Set the `sedol_any_of` argument.
    pub fn sedol_any_of(mut self, sedol_any_of: impl Into<String>) -> Self {
        self.sedol_any_of = Some(sedol_any_of.into());
        self
    }

    /// Set the `sedol_gt` argument.
    pub fn sedol_gt(mut self, sedol_gt: impl Into<String>) -> Self {
        self.sedol_gt = Some(sedol_gt.into());
        self
    }

    /// Set the `sedol_gte` argument.
    pub fn sedol_gte(mut self, sedol_gte: impl Into<String>) -> Self {
        self.sedol_gte = Some(sedol_gte.into());
        self
    }

    /// Set the `sedol_lt` argument.
    pub fn sedol_lt(mut self, sedol_lt: impl Into<String>) -> Self {
        self.sedol_lt = Some(sedol_lt.into());
        self
    }

    /// Set the `sedol_lte` argument.
    pub fn sedol_lte(mut self, sedol_lte: impl Into<String>) -> Self {
        self.sedol_lte = Some(sedol_lte.into());
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

/// Optional arguments for [`EtfGlobalApi::get_etf_global_fund_flows`].
#[derive(Debug, Default, Clone)]
pub struct GetEtfGlobalFundFlowsParams {
    /// The `processed_date` argument.
    pub processed_date: Option<String>,
    /// The `processed_date_gt` argument.
    pub processed_date_gt: Option<String>,
    /// The `processed_date_gte` argument.
    pub processed_date_gte: Option<String>,
    /// The `processed_date_lt` argument.
    pub processed_date_lt: Option<String>,
    /// The `processed_date_lte` argument.
    pub processed_date_lte: Option<String>,
    /// The `effective_date` argument.
    pub effective_date: Option<String>,
    /// The `effective_date_gt` argument.
    pub effective_date_gt: Option<String>,
    /// The `effective_date_gte` argument.
    pub effective_date_gte: Option<String>,
    /// The `effective_date_lt` argument.
    pub effective_date_lt: Option<String>,
    /// The `effective_date_lte` argument.
    pub effective_date_lte: Option<String>,
    /// The `composite_ticker` argument.
    pub composite_ticker: Option<String>,
    /// The `composite_ticker_any_of` argument.
    pub composite_ticker_any_of: Option<String>,
    /// The `composite_ticker_gt` argument.
    pub composite_ticker_gt: Option<String>,
    /// The `composite_ticker_gte` argument.
    pub composite_ticker_gte: Option<String>,
    /// The `composite_ticker_lt` argument.
    pub composite_ticker_lt: Option<String>,
    /// The `composite_ticker_lte` argument.
    pub composite_ticker_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetEtfGlobalFundFlowsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `processed_date` argument.
    pub fn processed_date(mut self, processed_date: impl Into<String>) -> Self {
        self.processed_date = Some(processed_date.into());
        self
    }

    /// Set the `processed_date_gt` argument.
    pub fn processed_date_gt(mut self, processed_date_gt: impl Into<String>) -> Self {
        self.processed_date_gt = Some(processed_date_gt.into());
        self
    }

    /// Set the `processed_date_gte` argument.
    pub fn processed_date_gte(mut self, processed_date_gte: impl Into<String>) -> Self {
        self.processed_date_gte = Some(processed_date_gte.into());
        self
    }

    /// Set the `processed_date_lt` argument.
    pub fn processed_date_lt(mut self, processed_date_lt: impl Into<String>) -> Self {
        self.processed_date_lt = Some(processed_date_lt.into());
        self
    }

    /// Set the `processed_date_lte` argument.
    pub fn processed_date_lte(mut self, processed_date_lte: impl Into<String>) -> Self {
        self.processed_date_lte = Some(processed_date_lte.into());
        self
    }

    /// Set the `effective_date` argument.
    pub fn effective_date(mut self, effective_date: impl Into<String>) -> Self {
        self.effective_date = Some(effective_date.into());
        self
    }

    /// Set the `effective_date_gt` argument.
    pub fn effective_date_gt(mut self, effective_date_gt: impl Into<String>) -> Self {
        self.effective_date_gt = Some(effective_date_gt.into());
        self
    }

    /// Set the `effective_date_gte` argument.
    pub fn effective_date_gte(mut self, effective_date_gte: impl Into<String>) -> Self {
        self.effective_date_gte = Some(effective_date_gte.into());
        self
    }

    /// Set the `effective_date_lt` argument.
    pub fn effective_date_lt(mut self, effective_date_lt: impl Into<String>) -> Self {
        self.effective_date_lt = Some(effective_date_lt.into());
        self
    }

    /// Set the `effective_date_lte` argument.
    pub fn effective_date_lte(mut self, effective_date_lte: impl Into<String>) -> Self {
        self.effective_date_lte = Some(effective_date_lte.into());
        self
    }

    /// Set the `composite_ticker` argument.
    pub fn composite_ticker(mut self, composite_ticker: impl Into<String>) -> Self {
        self.composite_ticker = Some(composite_ticker.into());
        self
    }

    /// Set the `composite_ticker_any_of` argument.
    pub fn composite_ticker_any_of(mut self, composite_ticker_any_of: impl Into<String>) -> Self {
        self.composite_ticker_any_of = Some(composite_ticker_any_of.into());
        self
    }

    /// Set the `composite_ticker_gt` argument.
    pub fn composite_ticker_gt(mut self, composite_ticker_gt: impl Into<String>) -> Self {
        self.composite_ticker_gt = Some(composite_ticker_gt.into());
        self
    }

    /// Set the `composite_ticker_gte` argument.
    pub fn composite_ticker_gte(mut self, composite_ticker_gte: impl Into<String>) -> Self {
        self.composite_ticker_gte = Some(composite_ticker_gte.into());
        self
    }

    /// Set the `composite_ticker_lt` argument.
    pub fn composite_ticker_lt(mut self, composite_ticker_lt: impl Into<String>) -> Self {
        self.composite_ticker_lt = Some(composite_ticker_lt.into());
        self
    }

    /// Set the `composite_ticker_lte` argument.
    pub fn composite_ticker_lte(mut self, composite_ticker_lte: impl Into<String>) -> Self {
        self.composite_ticker_lte = Some(composite_ticker_lte.into());
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

/// Optional arguments for [`EtfGlobalApi::get_etf_global_profiles`].
#[derive(Debug, Default, Clone)]
pub struct GetEtfGlobalProfilesParams {
    /// The `processed_date` argument.
    pub processed_date: Option<String>,
    /// The `processed_date_gt` argument.
    pub processed_date_gt: Option<String>,
    /// The `processed_date_gte` argument.
    pub processed_date_gte: Option<String>,
    /// The `processed_date_lt` argument.
    pub processed_date_lt: Option<String>,
    /// The `processed_date_lte` argument.
    pub processed_date_lte: Option<String>,
    /// The `effective_date` argument.
    pub effective_date: Option<String>,
    /// The `effective_date_gt` argument.
    pub effective_date_gt: Option<String>,
    /// The `effective_date_gte` argument.
    pub effective_date_gte: Option<String>,
    /// The `effective_date_lt` argument.
    pub effective_date_lt: Option<String>,
    /// The `effective_date_lte` argument.
    pub effective_date_lte: Option<String>,
    /// The `composite_ticker` argument.
    pub composite_ticker: Option<String>,
    /// The `composite_ticker_any_of` argument.
    pub composite_ticker_any_of: Option<String>,
    /// The `composite_ticker_gt` argument.
    pub composite_ticker_gt: Option<String>,
    /// The `composite_ticker_gte` argument.
    pub composite_ticker_gte: Option<String>,
    /// The `composite_ticker_lt` argument.
    pub composite_ticker_lt: Option<String>,
    /// The `composite_ticker_lte` argument.
    pub composite_ticker_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetEtfGlobalProfilesParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `processed_date` argument.
    pub fn processed_date(mut self, processed_date: impl Into<String>) -> Self {
        self.processed_date = Some(processed_date.into());
        self
    }

    /// Set the `processed_date_gt` argument.
    pub fn processed_date_gt(mut self, processed_date_gt: impl Into<String>) -> Self {
        self.processed_date_gt = Some(processed_date_gt.into());
        self
    }

    /// Set the `processed_date_gte` argument.
    pub fn processed_date_gte(mut self, processed_date_gte: impl Into<String>) -> Self {
        self.processed_date_gte = Some(processed_date_gte.into());
        self
    }

    /// Set the `processed_date_lt` argument.
    pub fn processed_date_lt(mut self, processed_date_lt: impl Into<String>) -> Self {
        self.processed_date_lt = Some(processed_date_lt.into());
        self
    }

    /// Set the `processed_date_lte` argument.
    pub fn processed_date_lte(mut self, processed_date_lte: impl Into<String>) -> Self {
        self.processed_date_lte = Some(processed_date_lte.into());
        self
    }

    /// Set the `effective_date` argument.
    pub fn effective_date(mut self, effective_date: impl Into<String>) -> Self {
        self.effective_date = Some(effective_date.into());
        self
    }

    /// Set the `effective_date_gt` argument.
    pub fn effective_date_gt(mut self, effective_date_gt: impl Into<String>) -> Self {
        self.effective_date_gt = Some(effective_date_gt.into());
        self
    }

    /// Set the `effective_date_gte` argument.
    pub fn effective_date_gte(mut self, effective_date_gte: impl Into<String>) -> Self {
        self.effective_date_gte = Some(effective_date_gte.into());
        self
    }

    /// Set the `effective_date_lt` argument.
    pub fn effective_date_lt(mut self, effective_date_lt: impl Into<String>) -> Self {
        self.effective_date_lt = Some(effective_date_lt.into());
        self
    }

    /// Set the `effective_date_lte` argument.
    pub fn effective_date_lte(mut self, effective_date_lte: impl Into<String>) -> Self {
        self.effective_date_lte = Some(effective_date_lte.into());
        self
    }

    /// Set the `composite_ticker` argument.
    pub fn composite_ticker(mut self, composite_ticker: impl Into<String>) -> Self {
        self.composite_ticker = Some(composite_ticker.into());
        self
    }

    /// Set the `composite_ticker_any_of` argument.
    pub fn composite_ticker_any_of(mut self, composite_ticker_any_of: impl Into<String>) -> Self {
        self.composite_ticker_any_of = Some(composite_ticker_any_of.into());
        self
    }

    /// Set the `composite_ticker_gt` argument.
    pub fn composite_ticker_gt(mut self, composite_ticker_gt: impl Into<String>) -> Self {
        self.composite_ticker_gt = Some(composite_ticker_gt.into());
        self
    }

    /// Set the `composite_ticker_gte` argument.
    pub fn composite_ticker_gte(mut self, composite_ticker_gte: impl Into<String>) -> Self {
        self.composite_ticker_gte = Some(composite_ticker_gte.into());
        self
    }

    /// Set the `composite_ticker_lt` argument.
    pub fn composite_ticker_lt(mut self, composite_ticker_lt: impl Into<String>) -> Self {
        self.composite_ticker_lt = Some(composite_ticker_lt.into());
        self
    }

    /// Set the `composite_ticker_lte` argument.
    pub fn composite_ticker_lte(mut self, composite_ticker_lte: impl Into<String>) -> Self {
        self.composite_ticker_lte = Some(composite_ticker_lte.into());
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

/// Optional arguments for [`EtfGlobalApi::get_etf_global_taxonomies`].
#[derive(Debug, Default, Clone)]
pub struct GetEtfGlobalTaxonomiesParams {
    /// The `processed_date` argument.
    pub processed_date: Option<String>,
    /// The `processed_date_gt` argument.
    pub processed_date_gt: Option<String>,
    /// The `processed_date_gte` argument.
    pub processed_date_gte: Option<String>,
    /// The `processed_date_lt` argument.
    pub processed_date_lt: Option<String>,
    /// The `processed_date_lte` argument.
    pub processed_date_lte: Option<String>,
    /// The `effective_date` argument.
    pub effective_date: Option<String>,
    /// The `effective_date_gt` argument.
    pub effective_date_gt: Option<String>,
    /// The `effective_date_gte` argument.
    pub effective_date_gte: Option<String>,
    /// The `effective_date_lt` argument.
    pub effective_date_lt: Option<String>,
    /// The `effective_date_lte` argument.
    pub effective_date_lte: Option<String>,
    /// The `composite_ticker` argument.
    pub composite_ticker: Option<String>,
    /// The `composite_ticker_any_of` argument.
    pub composite_ticker_any_of: Option<String>,
    /// The `composite_ticker_gt` argument.
    pub composite_ticker_gt: Option<String>,
    /// The `composite_ticker_gte` argument.
    pub composite_ticker_gte: Option<String>,
    /// The `composite_ticker_lt` argument.
    pub composite_ticker_lt: Option<String>,
    /// The `composite_ticker_lte` argument.
    pub composite_ticker_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetEtfGlobalTaxonomiesParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `processed_date` argument.
    pub fn processed_date(mut self, processed_date: impl Into<String>) -> Self {
        self.processed_date = Some(processed_date.into());
        self
    }

    /// Set the `processed_date_gt` argument.
    pub fn processed_date_gt(mut self, processed_date_gt: impl Into<String>) -> Self {
        self.processed_date_gt = Some(processed_date_gt.into());
        self
    }

    /// Set the `processed_date_gte` argument.
    pub fn processed_date_gte(mut self, processed_date_gte: impl Into<String>) -> Self {
        self.processed_date_gte = Some(processed_date_gte.into());
        self
    }

    /// Set the `processed_date_lt` argument.
    pub fn processed_date_lt(mut self, processed_date_lt: impl Into<String>) -> Self {
        self.processed_date_lt = Some(processed_date_lt.into());
        self
    }

    /// Set the `processed_date_lte` argument.
    pub fn processed_date_lte(mut self, processed_date_lte: impl Into<String>) -> Self {
        self.processed_date_lte = Some(processed_date_lte.into());
        self
    }

    /// Set the `effective_date` argument.
    pub fn effective_date(mut self, effective_date: impl Into<String>) -> Self {
        self.effective_date = Some(effective_date.into());
        self
    }

    /// Set the `effective_date_gt` argument.
    pub fn effective_date_gt(mut self, effective_date_gt: impl Into<String>) -> Self {
        self.effective_date_gt = Some(effective_date_gt.into());
        self
    }

    /// Set the `effective_date_gte` argument.
    pub fn effective_date_gte(mut self, effective_date_gte: impl Into<String>) -> Self {
        self.effective_date_gte = Some(effective_date_gte.into());
        self
    }

    /// Set the `effective_date_lt` argument.
    pub fn effective_date_lt(mut self, effective_date_lt: impl Into<String>) -> Self {
        self.effective_date_lt = Some(effective_date_lt.into());
        self
    }

    /// Set the `effective_date_lte` argument.
    pub fn effective_date_lte(mut self, effective_date_lte: impl Into<String>) -> Self {
        self.effective_date_lte = Some(effective_date_lte.into());
        self
    }

    /// Set the `composite_ticker` argument.
    pub fn composite_ticker(mut self, composite_ticker: impl Into<String>) -> Self {
        self.composite_ticker = Some(composite_ticker.into());
        self
    }

    /// Set the `composite_ticker_any_of` argument.
    pub fn composite_ticker_any_of(mut self, composite_ticker_any_of: impl Into<String>) -> Self {
        self.composite_ticker_any_of = Some(composite_ticker_any_of.into());
        self
    }

    /// Set the `composite_ticker_gt` argument.
    pub fn composite_ticker_gt(mut self, composite_ticker_gt: impl Into<String>) -> Self {
        self.composite_ticker_gt = Some(composite_ticker_gt.into());
        self
    }

    /// Set the `composite_ticker_gte` argument.
    pub fn composite_ticker_gte(mut self, composite_ticker_gte: impl Into<String>) -> Self {
        self.composite_ticker_gte = Some(composite_ticker_gte.into());
        self
    }

    /// Set the `composite_ticker_lt` argument.
    pub fn composite_ticker_lt(mut self, composite_ticker_lt: impl Into<String>) -> Self {
        self.composite_ticker_lt = Some(composite_ticker_lt.into());
        self
    }

    /// Set the `composite_ticker_lte` argument.
    pub fn composite_ticker_lte(mut self, composite_ticker_lte: impl Into<String>) -> Self {
        self.composite_ticker_lte = Some(composite_ticker_lte.into());
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
