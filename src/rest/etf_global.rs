use crate::client::{Client, RequestOptions};
use crate::error::Result;
use crate::models::{
    EtfGlobalAnalytics, EtfGlobalConstituent, EtfGlobalFundFlow, EtfGlobalProfile,
    EtfGlobalTaxonomy,
};
use futures::Stream;

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
    fn get_etf_global_analytics(
        &self,
        composite_ticker: Option<&str>,
        composite_ticker_any_of: Option<&str>,
        composite_ticker_gt: Option<&str>,
        composite_ticker_gte: Option<&str>,
        composite_ticker_lt: Option<&str>,
        composite_ticker_lte: Option<&str>,
        processed_date: Option<&str>,
        processed_date_gt: Option<&str>,
        processed_date_gte: Option<&str>,
        processed_date_lt: Option<&str>,
        processed_date_lte: Option<&str>,
        effective_date: Option<&str>,
        effective_date_gt: Option<&str>,
        effective_date_gte: Option<&str>,
        effective_date_lt: Option<&str>,
        effective_date_lte: Option<&str>,
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
        quant_grade: Option<&str>,
        quant_grade_any_of: Option<&str>,
        quant_grade_gt: Option<&str>,
        quant_grade_gte: Option<&str>,
        quant_grade_lt: Option<&str>,
        quant_grade_lte: Option<&str>,
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
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<EtfGlobalAnalytics>>;

    /// Get ETF Global constituents (paginated stream). Endpoint: GET /etf-global/v1/constituents.
    fn get_etf_global_constituents(
        &self,
        composite_ticker: Option<&str>,
        composite_ticker_any_of: Option<&str>,
        composite_ticker_gt: Option<&str>,
        composite_ticker_gte: Option<&str>,
        composite_ticker_lt: Option<&str>,
        composite_ticker_lte: Option<&str>,
        constituent_ticker: Option<&str>,
        constituent_ticker_any_of: Option<&str>,
        constituent_ticker_gt: Option<&str>,
        constituent_ticker_gte: Option<&str>,
        constituent_ticker_lt: Option<&str>,
        constituent_ticker_lte: Option<&str>,
        effective_date: Option<&str>,
        effective_date_gt: Option<&str>,
        effective_date_gte: Option<&str>,
        effective_date_lt: Option<&str>,
        effective_date_lte: Option<&str>,
        processed_date: Option<&str>,
        processed_date_gt: Option<&str>,
        processed_date_gte: Option<&str>,
        processed_date_lt: Option<&str>,
        processed_date_lte: Option<&str>,
        us_code: Option<&str>,
        us_code_any_of: Option<&str>,
        us_code_gt: Option<&str>,
        us_code_gte: Option<&str>,
        us_code_lt: Option<&str>,
        us_code_lte: Option<&str>,
        isin: Option<&str>,
        isin_any_of: Option<&str>,
        isin_gt: Option<&str>,
        isin_gte: Option<&str>,
        isin_lt: Option<&str>,
        isin_lte: Option<&str>,
        figi: Option<&str>,
        figi_any_of: Option<&str>,
        figi_gt: Option<&str>,
        figi_gte: Option<&str>,
        figi_lt: Option<&str>,
        figi_lte: Option<&str>,
        sedol: Option<&str>,
        sedol_any_of: Option<&str>,
        sedol_gt: Option<&str>,
        sedol_gte: Option<&str>,
        sedol_lt: Option<&str>,
        sedol_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<EtfGlobalConstituent>>;

    /// Get ETF Global fund flows (paginated stream). Endpoint: GET /etf-global/v1/fund-flows.
    fn get_etf_global_fund_flows(
        &self,
        processed_date: Option<&str>,
        processed_date_gt: Option<&str>,
        processed_date_gte: Option<&str>,
        processed_date_lt: Option<&str>,
        processed_date_lte: Option<&str>,
        effective_date: Option<&str>,
        effective_date_gt: Option<&str>,
        effective_date_gte: Option<&str>,
        effective_date_lt: Option<&str>,
        effective_date_lte: Option<&str>,
        composite_ticker: Option<&str>,
        composite_ticker_any_of: Option<&str>,
        composite_ticker_gt: Option<&str>,
        composite_ticker_gte: Option<&str>,
        composite_ticker_lt: Option<&str>,
        composite_ticker_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<EtfGlobalFundFlow>>;

    /// Get ETF Global profiles (paginated stream). Endpoint: GET /etf-global/v1/profiles.
    fn get_etf_global_profiles(
        &self,
        processed_date: Option<&str>,
        processed_date_gt: Option<&str>,
        processed_date_gte: Option<&str>,
        processed_date_lt: Option<&str>,
        processed_date_lte: Option<&str>,
        effective_date: Option<&str>,
        effective_date_gt: Option<&str>,
        effective_date_gte: Option<&str>,
        effective_date_lt: Option<&str>,
        effective_date_lte: Option<&str>,
        composite_ticker: Option<&str>,
        composite_ticker_any_of: Option<&str>,
        composite_ticker_gt: Option<&str>,
        composite_ticker_gte: Option<&str>,
        composite_ticker_lt: Option<&str>,
        composite_ticker_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<EtfGlobalProfile>>;

    /// Get ETF Global taxonomies (paginated stream). Endpoint: GET /etf-global/v1/taxonomies.
    fn get_etf_global_taxonomies(
        &self,
        processed_date: Option<&str>,
        processed_date_gt: Option<&str>,
        processed_date_gte: Option<&str>,
        processed_date_lt: Option<&str>,
        processed_date_lte: Option<&str>,
        effective_date: Option<&str>,
        effective_date_gt: Option<&str>,
        effective_date_gte: Option<&str>,
        effective_date_lt: Option<&str>,
        effective_date_lte: Option<&str>,
        composite_ticker: Option<&str>,
        composite_ticker_any_of: Option<&str>,
        composite_ticker_gt: Option<&str>,
        composite_ticker_gte: Option<&str>,
        composite_ticker_lt: Option<&str>,
        composite_ticker_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<EtfGlobalTaxonomy>>;
}

impl EtfGlobalApi for Client {
    fn get_etf_global_analytics(
        &self,
        composite_ticker: Option<&str>,
        composite_ticker_any_of: Option<&str>,
        composite_ticker_gt: Option<&str>,
        composite_ticker_gte: Option<&str>,
        composite_ticker_lt: Option<&str>,
        composite_ticker_lte: Option<&str>,
        processed_date: Option<&str>,
        processed_date_gt: Option<&str>,
        processed_date_gte: Option<&str>,
        processed_date_lt: Option<&str>,
        processed_date_lte: Option<&str>,
        effective_date: Option<&str>,
        effective_date_gt: Option<&str>,
        effective_date_gte: Option<&str>,
        effective_date_lt: Option<&str>,
        effective_date_lte: Option<&str>,
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
        quant_grade: Option<&str>,
        quant_grade_any_of: Option<&str>,
        quant_grade_gt: Option<&str>,
        quant_grade_gte: Option<&str>,
        quant_grade_lt: Option<&str>,
        quant_grade_lte: Option<&str>,
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
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<EtfGlobalAnalytics>> {
        let path = "/etf-global/v1/analytics".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        push_param(&mut params, "composite_ticker", composite_ticker);
        push_param(
            &mut params,
            "composite_ticker.any_of",
            composite_ticker_any_of,
        );
        push_param(&mut params, "composite_ticker.gt", composite_ticker_gt);
        push_param(&mut params, "composite_ticker.gte", composite_ticker_gte);
        push_param(&mut params, "composite_ticker.lt", composite_ticker_lt);
        push_param(&mut params, "composite_ticker.lte", composite_ticker_lte);
        push_param(&mut params, "processed_date", processed_date);
        push_param(&mut params, "processed_date.gt", processed_date_gt);
        push_param(&mut params, "processed_date.gte", processed_date_gte);
        push_param(&mut params, "processed_date.lt", processed_date_lt);
        push_param(&mut params, "processed_date.lte", processed_date_lte);
        push_param(&mut params, "effective_date", effective_date);
        push_param(&mut params, "effective_date.gt", effective_date_gt);
        push_param(&mut params, "effective_date.gte", effective_date_gte);
        push_param(&mut params, "effective_date.lt", effective_date_lt);
        push_param(&mut params, "effective_date.lte", effective_date_lte);
        push_param(&mut params, "risk_total_score", risk_total_score);
        push_param(&mut params, "risk_total_score.gt", risk_total_score_gt);
        push_param(&mut params, "risk_total_score.gte", risk_total_score_gte);
        push_param(&mut params, "risk_total_score.lt", risk_total_score_lt);
        push_param(&mut params, "risk_total_score.lte", risk_total_score_lte);
        push_param(&mut params, "reward_score", reward_score);
        push_param(&mut params, "reward_score.gt", reward_score_gt);
        push_param(&mut params, "reward_score.gte", reward_score_gte);
        push_param(&mut params, "reward_score.lt", reward_score_lt);
        push_param(&mut params, "reward_score.lte", reward_score_lte);
        push_param(&mut params, "quant_total_score", quant_total_score);
        push_param(&mut params, "quant_total_score.gt", quant_total_score_gt);
        push_param(&mut params, "quant_total_score.gte", quant_total_score_gte);
        push_param(&mut params, "quant_total_score.lt", quant_total_score_lt);
        push_param(&mut params, "quant_total_score.lte", quant_total_score_lte);
        push_param(&mut params, "quant_grade", quant_grade);
        push_param(&mut params, "quant_grade.any_of", quant_grade_any_of);
        push_param(&mut params, "quant_grade.gt", quant_grade_gt);
        push_param(&mut params, "quant_grade.gte", quant_grade_gte);
        push_param(&mut params, "quant_grade.lt", quant_grade_lt);
        push_param(&mut params, "quant_grade.lte", quant_grade_lte);
        push_param(
            &mut params,
            "quant_composite_technical",
            quant_composite_technical,
        );
        push_param(
            &mut params,
            "quant_composite_technical.gt",
            quant_composite_technical_gt,
        );
        push_param(
            &mut params,
            "quant_composite_technical.gte",
            quant_composite_technical_gte,
        );
        push_param(
            &mut params,
            "quant_composite_technical.lt",
            quant_composite_technical_lt,
        );
        push_param(
            &mut params,
            "quant_composite_technical.lte",
            quant_composite_technical_lte,
        );
        push_param(
            &mut params,
            "quant_composite_sentiment",
            quant_composite_sentiment,
        );
        push_param(
            &mut params,
            "quant_composite_sentiment.gt",
            quant_composite_sentiment_gt,
        );
        push_param(
            &mut params,
            "quant_composite_sentiment.gte",
            quant_composite_sentiment_gte,
        );
        push_param(
            &mut params,
            "quant_composite_sentiment.lt",
            quant_composite_sentiment_lt,
        );
        push_param(
            &mut params,
            "quant_composite_sentiment.lte",
            quant_composite_sentiment_lte,
        );
        push_param(
            &mut params,
            "quant_composite_behavioral",
            quant_composite_behavioral,
        );
        push_param(
            &mut params,
            "quant_composite_behavioral.gt",
            quant_composite_behavioral_gt,
        );
        push_param(
            &mut params,
            "quant_composite_behavioral.gte",
            quant_composite_behavioral_gte,
        );
        push_param(
            &mut params,
            "quant_composite_behavioral.lt",
            quant_composite_behavioral_lt,
        );
        push_param(
            &mut params,
            "quant_composite_behavioral.lte",
            quant_composite_behavioral_lte,
        );
        push_param(
            &mut params,
            "quant_composite_fundamental",
            quant_composite_fundamental,
        );
        push_param(
            &mut params,
            "quant_composite_fundamental.gt",
            quant_composite_fundamental_gt,
        );
        push_param(
            &mut params,
            "quant_composite_fundamental.gte",
            quant_composite_fundamental_gte,
        );
        push_param(
            &mut params,
            "quant_composite_fundamental.lt",
            quant_composite_fundamental_lt,
        );
        push_param(
            &mut params,
            "quant_composite_fundamental.lte",
            quant_composite_fundamental_lte,
        );
        push_param(
            &mut params,
            "quant_composite_global",
            quant_composite_global,
        );
        push_param(
            &mut params,
            "quant_composite_global.gt",
            quant_composite_global_gt,
        );
        push_param(
            &mut params,
            "quant_composite_global.gte",
            quant_composite_global_gte,
        );
        push_param(
            &mut params,
            "quant_composite_global.lt",
            quant_composite_global_lt,
        );
        push_param(
            &mut params,
            "quant_composite_global.lte",
            quant_composite_global_lte,
        );
        push_param(
            &mut params,
            "quant_composite_quality",
            quant_composite_quality,
        );
        push_param(
            &mut params,
            "quant_composite_quality.gt",
            quant_composite_quality_gt,
        );
        push_param(
            &mut params,
            "quant_composite_quality.gte",
            quant_composite_quality_gte,
        );
        push_param(
            &mut params,
            "quant_composite_quality.lt",
            quant_composite_quality_lt,
        );
        push_param(
            &mut params,
            "quant_composite_quality.lte",
            quant_composite_quality_lte,
        );
        push_param(&mut params, "limit", limit);
        push_param(&mut params, "sort", sort);
        if self.pagination {
            self.paginate::<EtfGlobalAnalytics>(&path, Some(&params), options)
        } else {
            self.single_page::<EtfGlobalAnalytics>(&path, Some(&params), options)
        }
    }

    fn get_etf_global_constituents(
        &self,
        composite_ticker: Option<&str>,
        composite_ticker_any_of: Option<&str>,
        composite_ticker_gt: Option<&str>,
        composite_ticker_gte: Option<&str>,
        composite_ticker_lt: Option<&str>,
        composite_ticker_lte: Option<&str>,
        constituent_ticker: Option<&str>,
        constituent_ticker_any_of: Option<&str>,
        constituent_ticker_gt: Option<&str>,
        constituent_ticker_gte: Option<&str>,
        constituent_ticker_lt: Option<&str>,
        constituent_ticker_lte: Option<&str>,
        effective_date: Option<&str>,
        effective_date_gt: Option<&str>,
        effective_date_gte: Option<&str>,
        effective_date_lt: Option<&str>,
        effective_date_lte: Option<&str>,
        processed_date: Option<&str>,
        processed_date_gt: Option<&str>,
        processed_date_gte: Option<&str>,
        processed_date_lt: Option<&str>,
        processed_date_lte: Option<&str>,
        us_code: Option<&str>,
        us_code_any_of: Option<&str>,
        us_code_gt: Option<&str>,
        us_code_gte: Option<&str>,
        us_code_lt: Option<&str>,
        us_code_lte: Option<&str>,
        isin: Option<&str>,
        isin_any_of: Option<&str>,
        isin_gt: Option<&str>,
        isin_gte: Option<&str>,
        isin_lt: Option<&str>,
        isin_lte: Option<&str>,
        figi: Option<&str>,
        figi_any_of: Option<&str>,
        figi_gt: Option<&str>,
        figi_gte: Option<&str>,
        figi_lt: Option<&str>,
        figi_lte: Option<&str>,
        sedol: Option<&str>,
        sedol_any_of: Option<&str>,
        sedol_gt: Option<&str>,
        sedol_gte: Option<&str>,
        sedol_lt: Option<&str>,
        sedol_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<EtfGlobalConstituent>> {
        let path = "/etf-global/v1/constituents".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        push_param(&mut params, "composite_ticker", composite_ticker);
        push_param(
            &mut params,
            "composite_ticker.any_of",
            composite_ticker_any_of,
        );
        push_param(&mut params, "composite_ticker.gt", composite_ticker_gt);
        push_param(&mut params, "composite_ticker.gte", composite_ticker_gte);
        push_param(&mut params, "composite_ticker.lt", composite_ticker_lt);
        push_param(&mut params, "composite_ticker.lte", composite_ticker_lte);
        push_param(&mut params, "constituent_ticker", constituent_ticker);
        push_param(
            &mut params,
            "constituent_ticker.any_of",
            constituent_ticker_any_of,
        );
        push_param(&mut params, "constituent_ticker.gt", constituent_ticker_gt);
        push_param(
            &mut params,
            "constituent_ticker.gte",
            constituent_ticker_gte,
        );
        push_param(&mut params, "constituent_ticker.lt", constituent_ticker_lt);
        push_param(
            &mut params,
            "constituent_ticker.lte",
            constituent_ticker_lte,
        );
        push_param(&mut params, "effective_date", effective_date);
        push_param(&mut params, "effective_date.gt", effective_date_gt);
        push_param(&mut params, "effective_date.gte", effective_date_gte);
        push_param(&mut params, "effective_date.lt", effective_date_lt);
        push_param(&mut params, "effective_date.lte", effective_date_lte);
        push_param(&mut params, "processed_date", processed_date);
        push_param(&mut params, "processed_date.gt", processed_date_gt);
        push_param(&mut params, "processed_date.gte", processed_date_gte);
        push_param(&mut params, "processed_date.lt", processed_date_lt);
        push_param(&mut params, "processed_date.lte", processed_date_lte);
        push_param(&mut params, "us_code", us_code);
        push_param(&mut params, "us_code.any_of", us_code_any_of);
        push_param(&mut params, "us_code.gt", us_code_gt);
        push_param(&mut params, "us_code.gte", us_code_gte);
        push_param(&mut params, "us_code.lt", us_code_lt);
        push_param(&mut params, "us_code.lte", us_code_lte);
        push_param(&mut params, "isin", isin);
        push_param(&mut params, "isin.any_of", isin_any_of);
        push_param(&mut params, "isin.gt", isin_gt);
        push_param(&mut params, "isin.gte", isin_gte);
        push_param(&mut params, "isin.lt", isin_lt);
        push_param(&mut params, "isin.lte", isin_lte);
        push_param(&mut params, "figi", figi);
        push_param(&mut params, "figi.any_of", figi_any_of);
        push_param(&mut params, "figi.gt", figi_gt);
        push_param(&mut params, "figi.gte", figi_gte);
        push_param(&mut params, "figi.lt", figi_lt);
        push_param(&mut params, "figi.lte", figi_lte);
        push_param(&mut params, "sedol", sedol);
        push_param(&mut params, "sedol.any_of", sedol_any_of);
        push_param(&mut params, "sedol.gt", sedol_gt);
        push_param(&mut params, "sedol.gte", sedol_gte);
        push_param(&mut params, "sedol.lt", sedol_lt);
        push_param(&mut params, "sedol.lte", sedol_lte);
        push_param(&mut params, "limit", limit);
        push_param(&mut params, "sort", sort);
        if self.pagination {
            self.paginate::<EtfGlobalConstituent>(&path, Some(&params), options)
        } else {
            self.single_page::<EtfGlobalConstituent>(&path, Some(&params), options)
        }
    }

    fn get_etf_global_fund_flows(
        &self,
        processed_date: Option<&str>,
        processed_date_gt: Option<&str>,
        processed_date_gte: Option<&str>,
        processed_date_lt: Option<&str>,
        processed_date_lte: Option<&str>,
        effective_date: Option<&str>,
        effective_date_gt: Option<&str>,
        effective_date_gte: Option<&str>,
        effective_date_lt: Option<&str>,
        effective_date_lte: Option<&str>,
        composite_ticker: Option<&str>,
        composite_ticker_any_of: Option<&str>,
        composite_ticker_gt: Option<&str>,
        composite_ticker_gte: Option<&str>,
        composite_ticker_lt: Option<&str>,
        composite_ticker_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<EtfGlobalFundFlow>> {
        let path = "/etf-global/v1/fund-flows".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        push_param(&mut params, "processed_date", processed_date);
        push_param(&mut params, "processed_date.gt", processed_date_gt);
        push_param(&mut params, "processed_date.gte", processed_date_gte);
        push_param(&mut params, "processed_date.lt", processed_date_lt);
        push_param(&mut params, "processed_date.lte", processed_date_lte);
        push_param(&mut params, "effective_date", effective_date);
        push_param(&mut params, "effective_date.gt", effective_date_gt);
        push_param(&mut params, "effective_date.gte", effective_date_gte);
        push_param(&mut params, "effective_date.lt", effective_date_lt);
        push_param(&mut params, "effective_date.lte", effective_date_lte);
        push_param(&mut params, "composite_ticker", composite_ticker);
        push_param(
            &mut params,
            "composite_ticker.any_of",
            composite_ticker_any_of,
        );
        push_param(&mut params, "composite_ticker.gt", composite_ticker_gt);
        push_param(&mut params, "composite_ticker.gte", composite_ticker_gte);
        push_param(&mut params, "composite_ticker.lt", composite_ticker_lt);
        push_param(&mut params, "composite_ticker.lte", composite_ticker_lte);
        push_param(&mut params, "limit", limit);
        push_param(&mut params, "sort", sort);
        if self.pagination {
            self.paginate::<EtfGlobalFundFlow>(&path, Some(&params), options)
        } else {
            self.single_page::<EtfGlobalFundFlow>(&path, Some(&params), options)
        }
    }

    fn get_etf_global_profiles(
        &self,
        processed_date: Option<&str>,
        processed_date_gt: Option<&str>,
        processed_date_gte: Option<&str>,
        processed_date_lt: Option<&str>,
        processed_date_lte: Option<&str>,
        effective_date: Option<&str>,
        effective_date_gt: Option<&str>,
        effective_date_gte: Option<&str>,
        effective_date_lt: Option<&str>,
        effective_date_lte: Option<&str>,
        composite_ticker: Option<&str>,
        composite_ticker_any_of: Option<&str>,
        composite_ticker_gt: Option<&str>,
        composite_ticker_gte: Option<&str>,
        composite_ticker_lt: Option<&str>,
        composite_ticker_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<EtfGlobalProfile>> {
        let path = "/etf-global/v1/profiles".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        push_param(&mut params, "processed_date", processed_date);
        push_param(&mut params, "processed_date.gt", processed_date_gt);
        push_param(&mut params, "processed_date.gte", processed_date_gte);
        push_param(&mut params, "processed_date.lt", processed_date_lt);
        push_param(&mut params, "processed_date.lte", processed_date_lte);
        push_param(&mut params, "effective_date", effective_date);
        push_param(&mut params, "effective_date.gt", effective_date_gt);
        push_param(&mut params, "effective_date.gte", effective_date_gte);
        push_param(&mut params, "effective_date.lt", effective_date_lt);
        push_param(&mut params, "effective_date.lte", effective_date_lte);
        push_param(&mut params, "composite_ticker", composite_ticker);
        push_param(
            &mut params,
            "composite_ticker.any_of",
            composite_ticker_any_of,
        );
        push_param(&mut params, "composite_ticker.gt", composite_ticker_gt);
        push_param(&mut params, "composite_ticker.gte", composite_ticker_gte);
        push_param(&mut params, "composite_ticker.lt", composite_ticker_lt);
        push_param(&mut params, "composite_ticker.lte", composite_ticker_lte);
        push_param(&mut params, "limit", limit);
        push_param(&mut params, "sort", sort);
        if self.pagination {
            self.paginate::<EtfGlobalProfile>(&path, Some(&params), options)
        } else {
            self.single_page::<EtfGlobalProfile>(&path, Some(&params), options)
        }
    }

    fn get_etf_global_taxonomies(
        &self,
        processed_date: Option<&str>,
        processed_date_gt: Option<&str>,
        processed_date_gte: Option<&str>,
        processed_date_lt: Option<&str>,
        processed_date_lte: Option<&str>,
        effective_date: Option<&str>,
        effective_date_gt: Option<&str>,
        effective_date_gte: Option<&str>,
        effective_date_lt: Option<&str>,
        effective_date_lte: Option<&str>,
        composite_ticker: Option<&str>,
        composite_ticker_any_of: Option<&str>,
        composite_ticker_gt: Option<&str>,
        composite_ticker_gte: Option<&str>,
        composite_ticker_lt: Option<&str>,
        composite_ticker_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<EtfGlobalTaxonomy>> {
        let path = "/etf-global/v1/taxonomies".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        push_param(&mut params, "processed_date", processed_date);
        push_param(&mut params, "processed_date.gt", processed_date_gt);
        push_param(&mut params, "processed_date.gte", processed_date_gte);
        push_param(&mut params, "processed_date.lt", processed_date_lt);
        push_param(&mut params, "processed_date.lte", processed_date_lte);
        push_param(&mut params, "effective_date", effective_date);
        push_param(&mut params, "effective_date.gt", effective_date_gt);
        push_param(&mut params, "effective_date.gte", effective_date_gte);
        push_param(&mut params, "effective_date.lt", effective_date_lt);
        push_param(&mut params, "effective_date.lte", effective_date_lte);
        push_param(&mut params, "composite_ticker", composite_ticker);
        push_param(
            &mut params,
            "composite_ticker.any_of",
            composite_ticker_any_of,
        );
        push_param(&mut params, "composite_ticker.gt", composite_ticker_gt);
        push_param(&mut params, "composite_ticker.gte", composite_ticker_gte);
        push_param(&mut params, "composite_ticker.lt", composite_ticker_lt);
        push_param(&mut params, "composite_ticker.lte", composite_ticker_lte);
        push_param(&mut params, "limit", limit);
        push_param(&mut params, "sort", sort);
        if self.pagination {
            self.paginate::<EtfGlobalTaxonomy>(&path, Some(&params), options)
        } else {
            self.single_page::<EtfGlobalTaxonomy>(&path, Some(&params), options)
        }
    }
}
