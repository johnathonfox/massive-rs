use crate::client::{Client, RequestOptions};
use crate::error::Result;
use crate::models::SummaryResult;

/// Summaries API.
pub trait SummariesApi {
    /// Get summaries for the given list of tickers. Endpoint: GET /v1/summaries.
    async fn get_summaries(
        &self,
        ticker_any_of: Option<&[&str]>,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<SummaryResult>>;
}

impl SummariesApi for Client {
    async fn get_summaries(
        &self,
        ticker_any_of: Option<&[&str]>,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<SummaryResult>> {
        let path = "/v1/summaries".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(t) = ticker_any_of {
            params.push(("ticker.any_of", t.join(",")));
        }
        #[derive(serde::Deserialize)]
        struct Resp {
            results: Option<Vec<SummaryResult>>,
        }
        let resp: Resp = self.get(&path, Some(&params), options).await?;
        Ok(resp.results.unwrap_or_default())
    }
}
