use crate::client::{Client, RequestOptions};
use crate::error::Result;
use crate::models::{LastForexQuote, LastQuote, Quote, RealTimeCurrencyConversion};
use futures::Stream;

/// Quotes (NBBO) API.
pub trait QuotesApi {
    /// List quotes for a ticker (paginated stream).
    fn list_quotes(
        &self,
        ticker: &str,
        timestamp: Option<&str>,
        timestamp_lt: Option<&str>,
        timestamp_lte: Option<&str>,
        timestamp_gt: Option<&str>,
        timestamp_gte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Quote>>;

    /// Get the last quote for a ticker.
    async fn get_last_quote(
        &self,
        ticker: &str,
        options: Option<&RequestOptions>,
    ) -> Result<LastQuote>;

    /// Get the last quote tick for a forex currency pair.
    async fn get_last_forex_quote(
        &self,
        from: &str,
        to: &str,
        options: Option<&RequestOptions>,
    ) -> Result<LastForexQuote>;

    /// Get currency conversions using the latest market conversion rates.
    async fn get_real_time_currency_conversion(
        &self,
        from: &str,
        to: &str,
        amount: Option<f64>,
        precision: Option<i64>,
        options: Option<&RequestOptions>,
    ) -> Result<RealTimeCurrencyConversion>;
}

impl QuotesApi for Client {
    fn list_quotes(
        &self,
        ticker: &str,
        timestamp: Option<&str>,
        timestamp_lt: Option<&str>,
        timestamp_lte: Option<&str>,
        timestamp_gt: Option<&str>,
        timestamp_gte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Quote>> {
        let path = format!("/v3/quotes/{}", ticker);
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(t) = timestamp {
            params.push(("timestamp", t.to_string()));
        }
        if let Some(t) = timestamp_lt {
            params.push(("timestamp.lt", t.to_string()));
        }
        if let Some(t) = timestamp_lte {
            params.push(("timestamp.lte", t.to_string()));
        }
        if let Some(t) = timestamp_gt {
            params.push(("timestamp.gt", t.to_string()));
        }
        if let Some(t) = timestamp_gte {
            params.push(("timestamp.gte", t.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if let Some(s) = sort {
            params.push(("sort", s.to_string()));
        }
        if let Some(o) = order {
            params.push(("order", o.to_string()));
        }
        if self.pagination {
            self.paginate::<Quote>(&path, Some(&params), options)
        } else {
            self.single_page::<Quote>(&path, Some(&params), options)
        }
    }

    async fn get_last_quote(
        &self,
        ticker: &str,
        options: Option<&RequestOptions>,
    ) -> Result<LastQuote> {
        let path = format!("/v2/last/nbbo/{}", ticker);
        #[derive(serde::Deserialize)]
        struct Resp {
            results: LastQuote,
        }
        let resp: Resp = self.get(&path, None, options).await?;
        Ok(resp.results)
    }

    async fn get_last_forex_quote(
        &self,
        from: &str,
        to: &str,
        options: Option<&RequestOptions>,
    ) -> Result<LastForexQuote> {
        let path = format!("/v1/last_quote/currencies/{}/{}", from, to);
        self.get(&path, None, options).await
    }

    async fn get_real_time_currency_conversion(
        &self,
        from: &str,
        to: &str,
        amount: Option<f64>,
        precision: Option<i64>,
        options: Option<&RequestOptions>,
    ) -> Result<RealTimeCurrencyConversion> {
        let path = format!("/v1/conversion/{}/{}", from, to);
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(a) = amount {
            params.push(("amount", a.to_string()));
        }
        if let Some(p) = precision {
            params.push(("precision", p.to_string()));
        }
        self.get(&path, Some(&params), options).await
    }
}
