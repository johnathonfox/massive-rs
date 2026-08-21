use super::BoxFuture;
use crate::client::{Client, RequestOptions};
use crate::models::SummaryResult;

/// Summaries API.
pub trait SummariesApi {
    /// Get summaries for the given list of tickers. Endpoint: GET /v1/summaries.
    fn get_summaries<'a>(
        &'a self,
        ticker_any_of: Option<&'a [&'a str]>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<SummaryResult>>;

    /// Same as [`Self::get_summaries`], but takes the optional arguments as a
    /// chainable [`GetSummariesParams`] struct.
    fn get_summaries_with_params<'a>(
        &'a self,
        params: GetSummariesParams,
    ) -> BoxFuture<'a, Vec<SummaryResult>>;
}

impl SummariesApi for Client {
    fn get_summaries<'a>(
        &'a self,
        ticker_any_of: Option<&'a [&'a str]>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<SummaryResult>> {
        self.get_summaries_with_params(GetSummariesParams {
            ticker_any_of: ticker_any_of.map(|v| v.iter().map(|s| s.to_string()).collect()),
            options: options.cloned(),
        })
    }

    fn get_summaries_with_params<'a>(
        &'a self,
        params: GetSummariesParams,
    ) -> BoxFuture<'a, Vec<SummaryResult>> {
        Box::pin(async move {
            let GetSummariesParams {
                ticker_any_of,
                options,
            } = params;
            let ticker_any_of = ticker_any_of.as_deref();
            let options = options.as_ref();
            let path = "/v1/summaries".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(t) = ticker_any_of {
                query.push(("ticker.any_of", t.join(",")));
            }
            #[derive(serde::Deserialize)]
            struct Resp {
                results: Option<Vec<SummaryResult>>,
            }
            let resp: Resp = self.get(&path, Some(&query), options).await?;
            Ok(resp.results.unwrap_or_default())
        })
    }
}

// --- Params structs (additive builder API) ---

/// Optional arguments for [`SummariesApi::get_summaries`].
#[derive(Debug, Default, Clone)]
pub struct GetSummariesParams {
    /// The `ticker_any_of` argument.
    pub ticker_any_of: Option<Vec<String>>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetSummariesParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `ticker_any_of` argument.
    pub fn ticker_any_of(mut self, values: &[&str]) -> Self {
        self.ticker_any_of = Some(values.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
