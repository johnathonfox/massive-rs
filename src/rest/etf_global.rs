use super::{encode_query, BoxStream};
use crate::client::{Client, RequestOptions};
use crate::models::{
    EtfGlobalAnalytics, EtfGlobalConstituent, EtfGlobalFundFlow, EtfGlobalProfile,
    EtfGlobalTaxonomy,
};

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
            let path = "/etf-global/v1/analytics".to_string();
            let query = encode_query(&params);
            self.list::<EtfGlobalAnalytics>(&path, &query, params.options.as_ref())
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
            let path = "/etf-global/v1/constituents".to_string();
            let query = encode_query(&params);
            self.list::<EtfGlobalConstituent>(&path, &query, params.options.as_ref())
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
            let path = "/etf-global/v1/fund-flows".to_string();
            let query = encode_query(&params);
            self.list::<EtfGlobalFundFlow>(&path, &query, params.options.as_ref())
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
            let path = "/etf-global/v1/profiles".to_string();
            let query = encode_query(&params);
            self.list::<EtfGlobalProfile>(&path, &query, params.options.as_ref())
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
            let path = "/etf-global/v1/taxonomies".to_string();
            let query = encode_query(&params);
            self.list::<EtfGlobalTaxonomy>(&path, &query, params.options.as_ref())
        })
    }
}

// --- Params structs (additive builder API) ---

/// Optional arguments for [`EtfGlobalApi::get_etf_global_analytics`].
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct GetEtfGlobalAnalyticsParams {
    /// The `composite_ticker` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_ticker: Option<String>,
    /// The `composite_ticker_any_of` argument.
    #[serde(
        rename = "composite_ticker.any_of",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_any_of: Option<String>,
    /// The `composite_ticker_gt` argument.
    #[serde(
        rename = "composite_ticker.gt",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_gt: Option<String>,
    /// The `composite_ticker_gte` argument.
    #[serde(
        rename = "composite_ticker.gte",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_gte: Option<String>,
    /// The `composite_ticker_lt` argument.
    #[serde(
        rename = "composite_ticker.lt",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_lt: Option<String>,
    /// The `composite_ticker_lte` argument.
    #[serde(
        rename = "composite_ticker.lte",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_lte: Option<String>,
    /// The `processed_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_date: Option<String>,
    /// The `processed_date_gt` argument.
    #[serde(rename = "processed_date.gt", skip_serializing_if = "Option::is_none")]
    pub processed_date_gt: Option<String>,
    /// The `processed_date_gte` argument.
    #[serde(rename = "processed_date.gte", skip_serializing_if = "Option::is_none")]
    pub processed_date_gte: Option<String>,
    /// The `processed_date_lt` argument.
    #[serde(rename = "processed_date.lt", skip_serializing_if = "Option::is_none")]
    pub processed_date_lt: Option<String>,
    /// The `processed_date_lte` argument.
    #[serde(rename = "processed_date.lte", skip_serializing_if = "Option::is_none")]
    pub processed_date_lte: Option<String>,
    /// The `effective_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// The `effective_date_gt` argument.
    #[serde(rename = "effective_date.gt", skip_serializing_if = "Option::is_none")]
    pub effective_date_gt: Option<String>,
    /// The `effective_date_gte` argument.
    #[serde(rename = "effective_date.gte", skip_serializing_if = "Option::is_none")]
    pub effective_date_gte: Option<String>,
    /// The `effective_date_lt` argument.
    #[serde(rename = "effective_date.lt", skip_serializing_if = "Option::is_none")]
    pub effective_date_lt: Option<String>,
    /// The `effective_date_lte` argument.
    #[serde(rename = "effective_date.lte", skip_serializing_if = "Option::is_none")]
    pub effective_date_lte: Option<String>,
    /// The `risk_total_score` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub risk_total_score: Option<f64>,
    /// The `risk_total_score_gt` argument.
    #[serde(
        rename = "risk_total_score.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub risk_total_score_gt: Option<f64>,
    /// The `risk_total_score_gte` argument.
    #[serde(
        rename = "risk_total_score.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub risk_total_score_gte: Option<f64>,
    /// The `risk_total_score_lt` argument.
    #[serde(
        rename = "risk_total_score.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub risk_total_score_lt: Option<f64>,
    /// The `risk_total_score_lte` argument.
    #[serde(
        rename = "risk_total_score.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub risk_total_score_lte: Option<f64>,
    /// The `reward_score` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub reward_score: Option<f64>,
    /// The `reward_score_gt` argument.
    #[serde(
        rename = "reward_score.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub reward_score_gt: Option<f64>,
    /// The `reward_score_gte` argument.
    #[serde(
        rename = "reward_score.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub reward_score_gte: Option<f64>,
    /// The `reward_score_lt` argument.
    #[serde(
        rename = "reward_score.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub reward_score_lt: Option<f64>,
    /// The `reward_score_lte` argument.
    #[serde(
        rename = "reward_score.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub reward_score_lte: Option<f64>,
    /// The `quant_total_score` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_total_score: Option<f64>,
    /// The `quant_total_score_gt` argument.
    #[serde(
        rename = "quant_total_score.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_total_score_gt: Option<f64>,
    /// The `quant_total_score_gte` argument.
    #[serde(
        rename = "quant_total_score.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_total_score_gte: Option<f64>,
    /// The `quant_total_score_lt` argument.
    #[serde(
        rename = "quant_total_score.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_total_score_lt: Option<f64>,
    /// The `quant_total_score_lte` argument.
    #[serde(
        rename = "quant_total_score.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_total_score_lte: Option<f64>,
    /// The `quant_grade` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quant_grade: Option<String>,
    /// The `quant_grade_any_of` argument.
    #[serde(rename = "quant_grade.any_of", skip_serializing_if = "Option::is_none")]
    pub quant_grade_any_of: Option<String>,
    /// The `quant_grade_gt` argument.
    #[serde(rename = "quant_grade.gt", skip_serializing_if = "Option::is_none")]
    pub quant_grade_gt: Option<String>,
    /// The `quant_grade_gte` argument.
    #[serde(rename = "quant_grade.gte", skip_serializing_if = "Option::is_none")]
    pub quant_grade_gte: Option<String>,
    /// The `quant_grade_lt` argument.
    #[serde(rename = "quant_grade.lt", skip_serializing_if = "Option::is_none")]
    pub quant_grade_lt: Option<String>,
    /// The `quant_grade_lte` argument.
    #[serde(rename = "quant_grade.lte", skip_serializing_if = "Option::is_none")]
    pub quant_grade_lte: Option<String>,
    /// The `quant_composite_technical` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_technical: Option<f64>,
    /// The `quant_composite_technical_gt` argument.
    #[serde(
        rename = "quant_composite_technical.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_technical_gt: Option<f64>,
    /// The `quant_composite_technical_gte` argument.
    #[serde(
        rename = "quant_composite_technical.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_technical_gte: Option<f64>,
    /// The `quant_composite_technical_lt` argument.
    #[serde(
        rename = "quant_composite_technical.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_technical_lt: Option<f64>,
    /// The `quant_composite_technical_lte` argument.
    #[serde(
        rename = "quant_composite_technical.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_technical_lte: Option<f64>,
    /// The `quant_composite_sentiment` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_sentiment: Option<f64>,
    /// The `quant_composite_sentiment_gt` argument.
    #[serde(
        rename = "quant_composite_sentiment.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_sentiment_gt: Option<f64>,
    /// The `quant_composite_sentiment_gte` argument.
    #[serde(
        rename = "quant_composite_sentiment.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_sentiment_gte: Option<f64>,
    /// The `quant_composite_sentiment_lt` argument.
    #[serde(
        rename = "quant_composite_sentiment.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_sentiment_lt: Option<f64>,
    /// The `quant_composite_sentiment_lte` argument.
    #[serde(
        rename = "quant_composite_sentiment.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_sentiment_lte: Option<f64>,
    /// The `quant_composite_behavioral` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_behavioral: Option<f64>,
    /// The `quant_composite_behavioral_gt` argument.
    #[serde(
        rename = "quant_composite_behavioral.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_behavioral_gt: Option<f64>,
    /// The `quant_composite_behavioral_gte` argument.
    #[serde(
        rename = "quant_composite_behavioral.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_behavioral_gte: Option<f64>,
    /// The `quant_composite_behavioral_lt` argument.
    #[serde(
        rename = "quant_composite_behavioral.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_behavioral_lt: Option<f64>,
    /// The `quant_composite_behavioral_lte` argument.
    #[serde(
        rename = "quant_composite_behavioral.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_behavioral_lte: Option<f64>,
    /// The `quant_composite_fundamental` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_fundamental: Option<f64>,
    /// The `quant_composite_fundamental_gt` argument.
    #[serde(
        rename = "quant_composite_fundamental.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_fundamental_gt: Option<f64>,
    /// The `quant_composite_fundamental_gte` argument.
    #[serde(
        rename = "quant_composite_fundamental.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_fundamental_gte: Option<f64>,
    /// The `quant_composite_fundamental_lt` argument.
    #[serde(
        rename = "quant_composite_fundamental.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_fundamental_lt: Option<f64>,
    /// The `quant_composite_fundamental_lte` argument.
    #[serde(
        rename = "quant_composite_fundamental.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_fundamental_lte: Option<f64>,
    /// The `quant_composite_global` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_global: Option<f64>,
    /// The `quant_composite_global_gt` argument.
    #[serde(
        rename = "quant_composite_global.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_global_gt: Option<f64>,
    /// The `quant_composite_global_gte` argument.
    #[serde(
        rename = "quant_composite_global.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_global_gte: Option<f64>,
    /// The `quant_composite_global_lt` argument.
    #[serde(
        rename = "quant_composite_global.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_global_lt: Option<f64>,
    /// The `quant_composite_global_lte` argument.
    #[serde(
        rename = "quant_composite_global.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_global_lte: Option<f64>,
    /// The `quant_composite_quality` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_quality: Option<f64>,
    /// The `quant_composite_quality_gt` argument.
    #[serde(
        rename = "quant_composite_quality.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_quality_gt: Option<f64>,
    /// The `quant_composite_quality_gte` argument.
    #[serde(
        rename = "quant_composite_quality.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_quality_gte: Option<f64>,
    /// The `quant_composite_quality_lt` argument.
    #[serde(
        rename = "quant_composite_quality.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_quality_lt: Option<f64>,
    /// The `quant_composite_quality_lte` argument.
    #[serde(
        rename = "quant_composite_quality.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quant_composite_quality_lte: Option<f64>,
    /// The `limit` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The `sort` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// The `options` argument.
    #[serde(skip)]
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
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct GetEtfGlobalConstituentsParams {
    /// The `composite_ticker` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_ticker: Option<String>,
    /// The `composite_ticker_any_of` argument.
    #[serde(
        rename = "composite_ticker.any_of",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_any_of: Option<String>,
    /// The `composite_ticker_gt` argument.
    #[serde(
        rename = "composite_ticker.gt",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_gt: Option<String>,
    /// The `composite_ticker_gte` argument.
    #[serde(
        rename = "composite_ticker.gte",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_gte: Option<String>,
    /// The `composite_ticker_lt` argument.
    #[serde(
        rename = "composite_ticker.lt",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_lt: Option<String>,
    /// The `composite_ticker_lte` argument.
    #[serde(
        rename = "composite_ticker.lte",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_lte: Option<String>,
    /// The `constituent_ticker` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constituent_ticker: Option<String>,
    /// The `constituent_ticker_any_of` argument.
    #[serde(
        rename = "constituent_ticker.any_of",
        skip_serializing_if = "Option::is_none"
    )]
    pub constituent_ticker_any_of: Option<String>,
    /// The `constituent_ticker_gt` argument.
    #[serde(
        rename = "constituent_ticker.gt",
        skip_serializing_if = "Option::is_none"
    )]
    pub constituent_ticker_gt: Option<String>,
    /// The `constituent_ticker_gte` argument.
    #[serde(
        rename = "constituent_ticker.gte",
        skip_serializing_if = "Option::is_none"
    )]
    pub constituent_ticker_gte: Option<String>,
    /// The `constituent_ticker_lt` argument.
    #[serde(
        rename = "constituent_ticker.lt",
        skip_serializing_if = "Option::is_none"
    )]
    pub constituent_ticker_lt: Option<String>,
    /// The `constituent_ticker_lte` argument.
    #[serde(
        rename = "constituent_ticker.lte",
        skip_serializing_if = "Option::is_none"
    )]
    pub constituent_ticker_lte: Option<String>,
    /// The `effective_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// The `effective_date_gt` argument.
    #[serde(rename = "effective_date.gt", skip_serializing_if = "Option::is_none")]
    pub effective_date_gt: Option<String>,
    /// The `effective_date_gte` argument.
    #[serde(rename = "effective_date.gte", skip_serializing_if = "Option::is_none")]
    pub effective_date_gte: Option<String>,
    /// The `effective_date_lt` argument.
    #[serde(rename = "effective_date.lt", skip_serializing_if = "Option::is_none")]
    pub effective_date_lt: Option<String>,
    /// The `effective_date_lte` argument.
    #[serde(rename = "effective_date.lte", skip_serializing_if = "Option::is_none")]
    pub effective_date_lte: Option<String>,
    /// The `processed_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_date: Option<String>,
    /// The `processed_date_gt` argument.
    #[serde(rename = "processed_date.gt", skip_serializing_if = "Option::is_none")]
    pub processed_date_gt: Option<String>,
    /// The `processed_date_gte` argument.
    #[serde(rename = "processed_date.gte", skip_serializing_if = "Option::is_none")]
    pub processed_date_gte: Option<String>,
    /// The `processed_date_lt` argument.
    #[serde(rename = "processed_date.lt", skip_serializing_if = "Option::is_none")]
    pub processed_date_lt: Option<String>,
    /// The `processed_date_lte` argument.
    #[serde(rename = "processed_date.lte", skip_serializing_if = "Option::is_none")]
    pub processed_date_lte: Option<String>,
    /// The `us_code` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_code: Option<String>,
    /// The `us_code_any_of` argument.
    #[serde(rename = "us_code.any_of", skip_serializing_if = "Option::is_none")]
    pub us_code_any_of: Option<String>,
    /// The `us_code_gt` argument.
    #[serde(rename = "us_code.gt", skip_serializing_if = "Option::is_none")]
    pub us_code_gt: Option<String>,
    /// The `us_code_gte` argument.
    #[serde(rename = "us_code.gte", skip_serializing_if = "Option::is_none")]
    pub us_code_gte: Option<String>,
    /// The `us_code_lt` argument.
    #[serde(rename = "us_code.lt", skip_serializing_if = "Option::is_none")]
    pub us_code_lt: Option<String>,
    /// The `us_code_lte` argument.
    #[serde(rename = "us_code.lte", skip_serializing_if = "Option::is_none")]
    pub us_code_lte: Option<String>,
    /// The `isin` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isin: Option<String>,
    /// The `isin_any_of` argument.
    #[serde(rename = "isin.any_of", skip_serializing_if = "Option::is_none")]
    pub isin_any_of: Option<String>,
    /// The `isin_gt` argument.
    #[serde(rename = "isin.gt", skip_serializing_if = "Option::is_none")]
    pub isin_gt: Option<String>,
    /// The `isin_gte` argument.
    #[serde(rename = "isin.gte", skip_serializing_if = "Option::is_none")]
    pub isin_gte: Option<String>,
    /// The `isin_lt` argument.
    #[serde(rename = "isin.lt", skip_serializing_if = "Option::is_none")]
    pub isin_lt: Option<String>,
    /// The `isin_lte` argument.
    #[serde(rename = "isin.lte", skip_serializing_if = "Option::is_none")]
    pub isin_lte: Option<String>,
    /// The `figi` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub figi: Option<String>,
    /// The `figi_any_of` argument.
    #[serde(rename = "figi.any_of", skip_serializing_if = "Option::is_none")]
    pub figi_any_of: Option<String>,
    /// The `figi_gt` argument.
    #[serde(rename = "figi.gt", skip_serializing_if = "Option::is_none")]
    pub figi_gt: Option<String>,
    /// The `figi_gte` argument.
    #[serde(rename = "figi.gte", skip_serializing_if = "Option::is_none")]
    pub figi_gte: Option<String>,
    /// The `figi_lt` argument.
    #[serde(rename = "figi.lt", skip_serializing_if = "Option::is_none")]
    pub figi_lt: Option<String>,
    /// The `figi_lte` argument.
    #[serde(rename = "figi.lte", skip_serializing_if = "Option::is_none")]
    pub figi_lte: Option<String>,
    /// The `sedol` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sedol: Option<String>,
    /// The `sedol_any_of` argument.
    #[serde(rename = "sedol.any_of", skip_serializing_if = "Option::is_none")]
    pub sedol_any_of: Option<String>,
    /// The `sedol_gt` argument.
    #[serde(rename = "sedol.gt", skip_serializing_if = "Option::is_none")]
    pub sedol_gt: Option<String>,
    /// The `sedol_gte` argument.
    #[serde(rename = "sedol.gte", skip_serializing_if = "Option::is_none")]
    pub sedol_gte: Option<String>,
    /// The `sedol_lt` argument.
    #[serde(rename = "sedol.lt", skip_serializing_if = "Option::is_none")]
    pub sedol_lt: Option<String>,
    /// The `sedol_lte` argument.
    #[serde(rename = "sedol.lte", skip_serializing_if = "Option::is_none")]
    pub sedol_lte: Option<String>,
    /// The `limit` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The `sort` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// The `options` argument.
    #[serde(skip)]
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
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct GetEtfGlobalFundFlowsParams {
    /// The `processed_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_date: Option<String>,
    /// The `processed_date_gt` argument.
    #[serde(rename = "processed_date.gt", skip_serializing_if = "Option::is_none")]
    pub processed_date_gt: Option<String>,
    /// The `processed_date_gte` argument.
    #[serde(rename = "processed_date.gte", skip_serializing_if = "Option::is_none")]
    pub processed_date_gte: Option<String>,
    /// The `processed_date_lt` argument.
    #[serde(rename = "processed_date.lt", skip_serializing_if = "Option::is_none")]
    pub processed_date_lt: Option<String>,
    /// The `processed_date_lte` argument.
    #[serde(rename = "processed_date.lte", skip_serializing_if = "Option::is_none")]
    pub processed_date_lte: Option<String>,
    /// The `effective_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// The `effective_date_gt` argument.
    #[serde(rename = "effective_date.gt", skip_serializing_if = "Option::is_none")]
    pub effective_date_gt: Option<String>,
    /// The `effective_date_gte` argument.
    #[serde(rename = "effective_date.gte", skip_serializing_if = "Option::is_none")]
    pub effective_date_gte: Option<String>,
    /// The `effective_date_lt` argument.
    #[serde(rename = "effective_date.lt", skip_serializing_if = "Option::is_none")]
    pub effective_date_lt: Option<String>,
    /// The `effective_date_lte` argument.
    #[serde(rename = "effective_date.lte", skip_serializing_if = "Option::is_none")]
    pub effective_date_lte: Option<String>,
    /// The `composite_ticker` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_ticker: Option<String>,
    /// The `composite_ticker_any_of` argument.
    #[serde(
        rename = "composite_ticker.any_of",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_any_of: Option<String>,
    /// The `composite_ticker_gt` argument.
    #[serde(
        rename = "composite_ticker.gt",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_gt: Option<String>,
    /// The `composite_ticker_gte` argument.
    #[serde(
        rename = "composite_ticker.gte",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_gte: Option<String>,
    /// The `composite_ticker_lt` argument.
    #[serde(
        rename = "composite_ticker.lt",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_lt: Option<String>,
    /// The `composite_ticker_lte` argument.
    #[serde(
        rename = "composite_ticker.lte",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_lte: Option<String>,
    /// The `limit` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The `sort` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// The `options` argument.
    #[serde(skip)]
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
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct GetEtfGlobalProfilesParams {
    /// The `processed_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_date: Option<String>,
    /// The `processed_date_gt` argument.
    #[serde(rename = "processed_date.gt", skip_serializing_if = "Option::is_none")]
    pub processed_date_gt: Option<String>,
    /// The `processed_date_gte` argument.
    #[serde(rename = "processed_date.gte", skip_serializing_if = "Option::is_none")]
    pub processed_date_gte: Option<String>,
    /// The `processed_date_lt` argument.
    #[serde(rename = "processed_date.lt", skip_serializing_if = "Option::is_none")]
    pub processed_date_lt: Option<String>,
    /// The `processed_date_lte` argument.
    #[serde(rename = "processed_date.lte", skip_serializing_if = "Option::is_none")]
    pub processed_date_lte: Option<String>,
    /// The `effective_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// The `effective_date_gt` argument.
    #[serde(rename = "effective_date.gt", skip_serializing_if = "Option::is_none")]
    pub effective_date_gt: Option<String>,
    /// The `effective_date_gte` argument.
    #[serde(rename = "effective_date.gte", skip_serializing_if = "Option::is_none")]
    pub effective_date_gte: Option<String>,
    /// The `effective_date_lt` argument.
    #[serde(rename = "effective_date.lt", skip_serializing_if = "Option::is_none")]
    pub effective_date_lt: Option<String>,
    /// The `effective_date_lte` argument.
    #[serde(rename = "effective_date.lte", skip_serializing_if = "Option::is_none")]
    pub effective_date_lte: Option<String>,
    /// The `composite_ticker` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_ticker: Option<String>,
    /// The `composite_ticker_any_of` argument.
    #[serde(
        rename = "composite_ticker.any_of",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_any_of: Option<String>,
    /// The `composite_ticker_gt` argument.
    #[serde(
        rename = "composite_ticker.gt",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_gt: Option<String>,
    /// The `composite_ticker_gte` argument.
    #[serde(
        rename = "composite_ticker.gte",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_gte: Option<String>,
    /// The `composite_ticker_lt` argument.
    #[serde(
        rename = "composite_ticker.lt",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_lt: Option<String>,
    /// The `composite_ticker_lte` argument.
    #[serde(
        rename = "composite_ticker.lte",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_lte: Option<String>,
    /// The `limit` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The `sort` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// The `options` argument.
    #[serde(skip)]
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
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct GetEtfGlobalTaxonomiesParams {
    /// The `processed_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_date: Option<String>,
    /// The `processed_date_gt` argument.
    #[serde(rename = "processed_date.gt", skip_serializing_if = "Option::is_none")]
    pub processed_date_gt: Option<String>,
    /// The `processed_date_gte` argument.
    #[serde(rename = "processed_date.gte", skip_serializing_if = "Option::is_none")]
    pub processed_date_gte: Option<String>,
    /// The `processed_date_lt` argument.
    #[serde(rename = "processed_date.lt", skip_serializing_if = "Option::is_none")]
    pub processed_date_lt: Option<String>,
    /// The `processed_date_lte` argument.
    #[serde(rename = "processed_date.lte", skip_serializing_if = "Option::is_none")]
    pub processed_date_lte: Option<String>,
    /// The `effective_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// The `effective_date_gt` argument.
    #[serde(rename = "effective_date.gt", skip_serializing_if = "Option::is_none")]
    pub effective_date_gt: Option<String>,
    /// The `effective_date_gte` argument.
    #[serde(rename = "effective_date.gte", skip_serializing_if = "Option::is_none")]
    pub effective_date_gte: Option<String>,
    /// The `effective_date_lt` argument.
    #[serde(rename = "effective_date.lt", skip_serializing_if = "Option::is_none")]
    pub effective_date_lt: Option<String>,
    /// The `effective_date_lte` argument.
    #[serde(rename = "effective_date.lte", skip_serializing_if = "Option::is_none")]
    pub effective_date_lte: Option<String>,
    /// The `composite_ticker` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_ticker: Option<String>,
    /// The `composite_ticker_any_of` argument.
    #[serde(
        rename = "composite_ticker.any_of",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_any_of: Option<String>,
    /// The `composite_ticker_gt` argument.
    #[serde(
        rename = "composite_ticker.gt",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_gt: Option<String>,
    /// The `composite_ticker_gte` argument.
    #[serde(
        rename = "composite_ticker.gte",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_gte: Option<String>,
    /// The `composite_ticker_lt` argument.
    #[serde(
        rename = "composite_ticker.lt",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_lt: Option<String>,
    /// The `composite_ticker_lte` argument.
    #[serde(
        rename = "composite_ticker.lte",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_ticker_lte: Option<String>,
    /// The `limit` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The `sort` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// The `options` argument.
    #[serde(skip)]
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
