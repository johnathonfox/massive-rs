use crate::client::{Client, RequestOptions};
use crate::error::Result;
use crate::models::{Agg, DailyOpenCloseAgg, GroupedDailyAgg, PreviousCloseAgg};
use futures::Stream;

/// Aggregates (OHLCV bars) API.
pub trait AggsApi {
    /// List aggregate bars for a ticker over a given date range.
    /// Returns a stream that automatically paginates through all pages.
    fn list_aggs(
        &self,
        ticker: &str,
        multiplier: i64,
        timespan: &str,
        from: &str,
        to: &str,
        adjusted: Option<bool>,
        sort: Option<&str>,
        limit: Option<i64>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Agg>>;

    /// Get aggregate bars (single page, no pagination follow).
    async fn get_aggs(
        &self,
        ticker: &str,
        multiplier: i64,
        timespan: &str,
        from: &str,
        to: &str,
        adjusted: Option<bool>,
        sort: Option<&str>,
        limit: Option<i64>,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<Agg>>;

    /// Get the daily OHLC for the entire market.
    async fn get_grouped_daily_aggs(
        &self,
        date: &str,
        adjusted: Option<bool>,
        locale: Option<&str>,
        market_type: Option<&str>,
        include_otc: Option<bool>,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<GroupedDailyAgg>>;

    /// Get the open, close and afterhours prices for a ticker on a date.
    async fn get_daily_open_close_agg(
        &self,
        ticker: &str,
        date: &str,
        adjusted: Option<bool>,
        options: Option<&RequestOptions>,
    ) -> Result<DailyOpenCloseAgg>;

    /// Get the previous day's OHLC for a ticker.
    async fn get_previous_close_agg(
        &self,
        ticker: &str,
        adjusted: Option<bool>,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<PreviousCloseAgg>>;
}

impl AggsApi for Client {
    fn list_aggs(
        &self,
        ticker: &str,
        multiplier: i64,
        timespan: &str,
        from: &str,
        to: &str,
        adjusted: Option<bool>,
        sort: Option<&str>,
        limit: Option<i64>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Agg>> {
        let path = format!("/v2/aggs/ticker/{}/range/{}/{}/{}/{}", ticker, multiplier, timespan, from, to);
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(a) = adjusted {
            params.push(("adjusted", a.to_string()));
        }
        if let Some(s) = sort {
            params.push(("sort", s.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if self.pagination {
            self.paginate::<Agg>(&path, Some(&params), options)
        } else {
            self.single_page::<Agg>(&path, Some(&params), options)
        }
    }

    async fn get_aggs(
        &self,
        ticker: &str,
        multiplier: i64,
        timespan: &str,
        from: &str,
        to: &str,
        adjusted: Option<bool>,
        sort: Option<&str>,
        limit: Option<i64>,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<Agg>> {
        let path = format!("/v2/aggs/ticker/{}/range/{}/{}/{}/{}", ticker, multiplier, timespan, from, to);
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(a) = adjusted {
            params.push(("adjusted", a.to_string()));
        }
        if let Some(s) = sort {
            params.push(("sort", s.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        #[derive(serde::Deserialize)]
        struct Resp { results: Option<Vec<Agg>> }
        let resp: Resp = self.get(&path, Some(&params), options).await?;
        Ok(resp.results.unwrap_or_default())
    }

    async fn get_grouped_daily_aggs(
        &self,
        date: &str,
        adjusted: Option<bool>,
        locale: Option<&str>,
        market_type: Option<&str>,
        include_otc: Option<bool>,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<GroupedDailyAgg>> {
        let locale = locale.unwrap_or("us");
        let market_type = market_type.unwrap_or("stocks");
        let path = format!("/v2/aggs/grouped/locale/{}/market/{}/{}", locale, market_type, date);
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(a) = adjusted {
            params.push(("adjusted", a.to_string()));
        }
        if let Some(i) = include_otc {
            params.push(("include_otc", i.to_string()));
        }
        #[derive(serde::Deserialize)]
        struct Resp { results: Option<Vec<GroupedDailyAgg>> }
        let resp: Resp = self.get(&path, Some(&params), options).await?;
        Ok(resp.results.unwrap_or_default())
    }

    async fn get_daily_open_close_agg(
        &self,
        ticker: &str,
        date: &str,
        adjusted: Option<bool>,
        options: Option<&RequestOptions>,
    ) -> Result<DailyOpenCloseAgg> {
        let path = format!("/v1/open-close/{}/{}", ticker, date);
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(a) = adjusted {
            params.push(("adjusted", a.to_string()));
        }
        self.get(&path, Some(&params), options).await
    }

    async fn get_previous_close_agg(
        &self,
        ticker: &str,
        adjusted: Option<bool>,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<PreviousCloseAgg>> {
        let path = format!("/v2/aggs/ticker/{}/prev", ticker);
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(a) = adjusted {
            params.push(("adjusted", a.to_string()));
        }
        #[derive(serde::Deserialize)]
        struct Resp { results: Option<Vec<PreviousCloseAgg>> }
        let resp: Resp = self.get(&path, Some(&params), options).await?;
        Ok(resp.results.unwrap_or_default())
    }
}
