use crate::client::{Client, RequestOptions};
use crate::error::Result;
use crate::models::{MacdIndicatorResults, SingleIndicatorResults};

/// Technical indicators API.
pub trait IndicatorsApi {
    /// Get SMA values for a ticker over a given range with the specified parameters.
    async fn get_sma(
        &self,
        ticker: &str,
        timestamp: Option<&str>,
        timestamp_lt: Option<&str>,
        timestamp_lte: Option<&str>,
        timestamp_gt: Option<&str>,
        timestamp_gte: Option<&str>,
        timespan: Option<&str>,
        window: Option<i64>,
        adjusted: Option<bool>,
        expand_underlying: Option<bool>,
        order: Option<&str>,
        limit: Option<i64>,
        series_type: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<SingleIndicatorResults>;

    /// Get EMA values for a ticker over a given range with the specified parameters.
    async fn get_ema(
        &self,
        ticker: &str,
        timestamp: Option<&str>,
        timestamp_lt: Option<&str>,
        timestamp_lte: Option<&str>,
        timestamp_gt: Option<&str>,
        timestamp_gte: Option<&str>,
        timespan: Option<&str>,
        window: Option<i64>,
        adjusted: Option<bool>,
        expand_underlying: Option<bool>,
        order: Option<&str>,
        limit: Option<i64>,
        series_type: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<SingleIndicatorResults>;

    /// Get RSI values for a ticker over a given range with the specified parameters.
    async fn get_rsi(
        &self,
        ticker: &str,
        timestamp: Option<&str>,
        timestamp_lt: Option<&str>,
        timestamp_lte: Option<&str>,
        timestamp_gt: Option<&str>,
        timestamp_gte: Option<&str>,
        timespan: Option<&str>,
        window: Option<i64>,
        adjusted: Option<bool>,
        expand_underlying: Option<bool>,
        order: Option<&str>,
        limit: Option<i64>,
        series_type: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<SingleIndicatorResults>;

    /// Get MACD values for a ticker over a given range with the specified parameters.
    async fn get_macd(
        &self,
        ticker: &str,
        timestamp: Option<&str>,
        timestamp_lt: Option<&str>,
        timestamp_lte: Option<&str>,
        timestamp_gt: Option<&str>,
        timestamp_gte: Option<&str>,
        timespan: Option<&str>,
        short_window: Option<i64>,
        long_window: Option<i64>,
        signal_window: Option<i64>,
        adjusted: Option<bool>,
        expand_underlying: Option<bool>,
        order: Option<&str>,
        limit: Option<i64>,
        series_type: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<MacdIndicatorResults>;
}

impl IndicatorsApi for Client {
    async fn get_sma(
        &self,
        ticker: &str,
        timestamp: Option<&str>,
        timestamp_lt: Option<&str>,
        timestamp_lte: Option<&str>,
        timestamp_gt: Option<&str>,
        timestamp_gte: Option<&str>,
        timespan: Option<&str>,
        window: Option<i64>,
        adjusted: Option<bool>,
        expand_underlying: Option<bool>,
        order: Option<&str>,
        limit: Option<i64>,
        series_type: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<SingleIndicatorResults> {
        let path = format!("/v1/indicators/sma/{}", ticker);
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
        if let Some(t) = timespan {
            params.push(("timespan", t.to_string()));
        }
        if let Some(w) = window {
            params.push(("window", w.to_string()));
        }
        if let Some(a) = adjusted {
            params.push(("adjusted", a.to_string()));
        }
        if let Some(e) = expand_underlying {
            params.push(("expand_underlying", e.to_string()));
        }
        if let Some(o) = order {
            params.push(("order", o.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if let Some(s) = series_type {
            params.push(("series_type", s.to_string()));
        }
        #[derive(serde::Deserialize)]
        struct Resp {
            results: SingleIndicatorResults,
        }
        let resp: Resp = self.get(&path, Some(&params), options).await?;
        Ok(resp.results)
    }

    async fn get_ema(
        &self,
        ticker: &str,
        timestamp: Option<&str>,
        timestamp_lt: Option<&str>,
        timestamp_lte: Option<&str>,
        timestamp_gt: Option<&str>,
        timestamp_gte: Option<&str>,
        timespan: Option<&str>,
        window: Option<i64>,
        adjusted: Option<bool>,
        expand_underlying: Option<bool>,
        order: Option<&str>,
        limit: Option<i64>,
        series_type: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<SingleIndicatorResults> {
        let path = format!("/v1/indicators/ema/{}", ticker);
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
        if let Some(t) = timespan {
            params.push(("timespan", t.to_string()));
        }
        if let Some(w) = window {
            params.push(("window", w.to_string()));
        }
        if let Some(a) = adjusted {
            params.push(("adjusted", a.to_string()));
        }
        if let Some(e) = expand_underlying {
            params.push(("expand_underlying", e.to_string()));
        }
        if let Some(o) = order {
            params.push(("order", o.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if let Some(s) = series_type {
            params.push(("series_type", s.to_string()));
        }
        #[derive(serde::Deserialize)]
        struct Resp {
            results: SingleIndicatorResults,
        }
        let resp: Resp = self.get(&path, Some(&params), options).await?;
        Ok(resp.results)
    }

    async fn get_rsi(
        &self,
        ticker: &str,
        timestamp: Option<&str>,
        timestamp_lt: Option<&str>,
        timestamp_lte: Option<&str>,
        timestamp_gt: Option<&str>,
        timestamp_gte: Option<&str>,
        timespan: Option<&str>,
        window: Option<i64>,
        adjusted: Option<bool>,
        expand_underlying: Option<bool>,
        order: Option<&str>,
        limit: Option<i64>,
        series_type: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<SingleIndicatorResults> {
        let path = format!("/v1/indicators/rsi/{}", ticker);
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
        if let Some(t) = timespan {
            params.push(("timespan", t.to_string()));
        }
        if let Some(w) = window {
            params.push(("window", w.to_string()));
        }
        if let Some(a) = adjusted {
            params.push(("adjusted", a.to_string()));
        }
        if let Some(e) = expand_underlying {
            params.push(("expand_underlying", e.to_string()));
        }
        if let Some(o) = order {
            params.push(("order", o.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if let Some(s) = series_type {
            params.push(("series_type", s.to_string()));
        }
        #[derive(serde::Deserialize)]
        struct Resp {
            results: SingleIndicatorResults,
        }
        let resp: Resp = self.get(&path, Some(&params), options).await?;
        Ok(resp.results)
    }

    async fn get_macd(
        &self,
        ticker: &str,
        timestamp: Option<&str>,
        timestamp_lt: Option<&str>,
        timestamp_lte: Option<&str>,
        timestamp_gt: Option<&str>,
        timestamp_gte: Option<&str>,
        timespan: Option<&str>,
        short_window: Option<i64>,
        long_window: Option<i64>,
        signal_window: Option<i64>,
        adjusted: Option<bool>,
        expand_underlying: Option<bool>,
        order: Option<&str>,
        limit: Option<i64>,
        series_type: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<MacdIndicatorResults> {
        let path = format!("/v1/indicators/macd/{}", ticker);
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
        if let Some(t) = timespan {
            params.push(("timespan", t.to_string()));
        }
        if let Some(w) = short_window {
            params.push(("short_window", w.to_string()));
        }
        if let Some(w) = long_window {
            params.push(("long_window", w.to_string()));
        }
        if let Some(w) = signal_window {
            params.push(("signal_window", w.to_string()));
        }
        if let Some(a) = adjusted {
            params.push(("adjusted", a.to_string()));
        }
        if let Some(e) = expand_underlying {
            params.push(("expand_underlying", e.to_string()));
        }
        if let Some(o) = order {
            params.push(("order", o.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if let Some(s) = series_type {
            params.push(("series_type", s.to_string()));
        }
        #[derive(serde::Deserialize)]
        struct Resp {
            results: MacdIndicatorResults,
        }
        let resp: Resp = self.get(&path, Some(&params), options).await?;
        Ok(resp.results)
    }
}
