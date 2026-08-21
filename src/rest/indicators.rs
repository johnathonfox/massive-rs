use super::BoxFuture;
use crate::client::{Client, RequestOptions};
use crate::models::{MacdIndicatorResults, SingleIndicatorResults};

/// Technical indicators API.
pub trait IndicatorsApi {
    /// Get SMA values for a ticker over a given range with the specified parameters.
    fn get_sma<'a>(
        &'a self,
        ticker: &'a str,
        timestamp: Option<&'a str>,
        timestamp_lt: Option<&'a str>,
        timestamp_lte: Option<&'a str>,
        timestamp_gt: Option<&'a str>,
        timestamp_gte: Option<&'a str>,
        timespan: Option<&'a str>,
        window: Option<i64>,
        adjusted: Option<bool>,
        expand_underlying: Option<bool>,
        order: Option<&'a str>,
        limit: Option<i64>,
        series_type: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, SingleIndicatorResults>;

    /// Same as [`Self::get_sma`], but takes the optional arguments as a
    /// chainable [`GetSmaParams`] struct.
    fn get_sma_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetSmaParams,
    ) -> BoxFuture<'a, SingleIndicatorResults>;

    /// Get EMA values for a ticker over a given range with the specified parameters.
    fn get_ema<'a>(
        &'a self,
        ticker: &'a str,
        timestamp: Option<&'a str>,
        timestamp_lt: Option<&'a str>,
        timestamp_lte: Option<&'a str>,
        timestamp_gt: Option<&'a str>,
        timestamp_gte: Option<&'a str>,
        timespan: Option<&'a str>,
        window: Option<i64>,
        adjusted: Option<bool>,
        expand_underlying: Option<bool>,
        order: Option<&'a str>,
        limit: Option<i64>,
        series_type: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, SingleIndicatorResults>;

    /// Same as [`Self::get_ema`], but takes the optional arguments as a
    /// chainable [`GetEmaParams`] struct.
    fn get_ema_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetEmaParams,
    ) -> BoxFuture<'a, SingleIndicatorResults>;

    /// Get RSI values for a ticker over a given range with the specified parameters.
    fn get_rsi<'a>(
        &'a self,
        ticker: &'a str,
        timestamp: Option<&'a str>,
        timestamp_lt: Option<&'a str>,
        timestamp_lte: Option<&'a str>,
        timestamp_gt: Option<&'a str>,
        timestamp_gte: Option<&'a str>,
        timespan: Option<&'a str>,
        window: Option<i64>,
        adjusted: Option<bool>,
        expand_underlying: Option<bool>,
        order: Option<&'a str>,
        limit: Option<i64>,
        series_type: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, SingleIndicatorResults>;

    /// Same as [`Self::get_rsi`], but takes the optional arguments as a
    /// chainable [`GetRsiParams`] struct.
    fn get_rsi_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetRsiParams,
    ) -> BoxFuture<'a, SingleIndicatorResults>;

    /// Get MACD values for a ticker over a given range with the specified parameters.
    fn get_macd<'a>(
        &'a self,
        ticker: &'a str,
        timestamp: Option<&'a str>,
        timestamp_lt: Option<&'a str>,
        timestamp_lte: Option<&'a str>,
        timestamp_gt: Option<&'a str>,
        timestamp_gte: Option<&'a str>,
        timespan: Option<&'a str>,
        short_window: Option<i64>,
        long_window: Option<i64>,
        signal_window: Option<i64>,
        adjusted: Option<bool>,
        expand_underlying: Option<bool>,
        order: Option<&'a str>,
        limit: Option<i64>,
        series_type: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, MacdIndicatorResults>;

    /// Same as [`Self::get_macd`], but takes the optional arguments as a
    /// chainable [`GetMacdParams`] struct.
    fn get_macd_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetMacdParams,
    ) -> BoxFuture<'a, MacdIndicatorResults>;
}

impl IndicatorsApi for Client {
    fn get_sma<'a>(
        &'a self,
        ticker: &'a str,
        timestamp: Option<&'a str>,
        timestamp_lt: Option<&'a str>,
        timestamp_lte: Option<&'a str>,
        timestamp_gt: Option<&'a str>,
        timestamp_gte: Option<&'a str>,
        timespan: Option<&'a str>,
        window: Option<i64>,
        adjusted: Option<bool>,
        expand_underlying: Option<bool>,
        order: Option<&'a str>,
        limit: Option<i64>,
        series_type: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, SingleIndicatorResults> {
        self.get_sma_with_params(
            ticker,
            GetSmaParams {
                timestamp: timestamp.map(String::from),
                timestamp_lt: timestamp_lt.map(String::from),
                timestamp_lte: timestamp_lte.map(String::from),
                timestamp_gt: timestamp_gt.map(String::from),
                timestamp_gte: timestamp_gte.map(String::from),
                timespan: timespan.map(String::from),
                window,
                adjusted,
                expand_underlying,
                order: order.map(String::from),
                limit,
                series_type: series_type.map(String::from),
                options: options.cloned(),
            },
        )
    }

    fn get_sma_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetSmaParams,
    ) -> BoxFuture<'a, SingleIndicatorResults> {
        Box::pin(async move {
            let GetSmaParams {
                timestamp,
                timestamp_lt,
                timestamp_lte,
                timestamp_gt,
                timestamp_gte,
                timespan,
                window,
                adjusted,
                expand_underlying,
                order,
                limit,
                series_type,
                options,
            } = params;
            let timestamp = timestamp.as_deref();
            let timestamp_lt = timestamp_lt.as_deref();
            let timestamp_lte = timestamp_lte.as_deref();
            let timestamp_gt = timestamp_gt.as_deref();
            let timestamp_gte = timestamp_gte.as_deref();
            let timespan = timespan.as_deref();
            let order = order.as_deref();
            let series_type = series_type.as_deref();
            let options = options.as_ref();
            let path = format!("/v1/indicators/sma/{}", ticker);
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(t) = timestamp {
                query.push(("timestamp", t.to_string()));
            }
            if let Some(t) = timestamp_lt {
                query.push(("timestamp.lt", t.to_string()));
            }
            if let Some(t) = timestamp_lte {
                query.push(("timestamp.lte", t.to_string()));
            }
            if let Some(t) = timestamp_gt {
                query.push(("timestamp.gt", t.to_string()));
            }
            if let Some(t) = timestamp_gte {
                query.push(("timestamp.gte", t.to_string()));
            }
            if let Some(t) = timespan {
                query.push(("timespan", t.to_string()));
            }
            if let Some(w) = window {
                query.push(("window", w.to_string()));
            }
            if let Some(a) = adjusted {
                query.push(("adjusted", a.to_string()));
            }
            if let Some(e) = expand_underlying {
                query.push(("expand_underlying", e.to_string()));
            }
            if let Some(o) = order {
                query.push(("order", o.to_string()));
            }
            if let Some(l) = limit {
                query.push(("limit", l.to_string()));
            }
            if let Some(s) = series_type {
                query.push(("series_type", s.to_string()));
            }
            #[derive(serde::Deserialize)]
            struct Resp {
                results: SingleIndicatorResults,
            }
            let resp: Resp = self.get(&path, Some(&query), options).await?;
            Ok(resp.results)
        })
    }

    fn get_ema<'a>(
        &'a self,
        ticker: &'a str,
        timestamp: Option<&'a str>,
        timestamp_lt: Option<&'a str>,
        timestamp_lte: Option<&'a str>,
        timestamp_gt: Option<&'a str>,
        timestamp_gte: Option<&'a str>,
        timespan: Option<&'a str>,
        window: Option<i64>,
        adjusted: Option<bool>,
        expand_underlying: Option<bool>,
        order: Option<&'a str>,
        limit: Option<i64>,
        series_type: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, SingleIndicatorResults> {
        self.get_ema_with_params(
            ticker,
            GetEmaParams {
                timestamp: timestamp.map(String::from),
                timestamp_lt: timestamp_lt.map(String::from),
                timestamp_lte: timestamp_lte.map(String::from),
                timestamp_gt: timestamp_gt.map(String::from),
                timestamp_gte: timestamp_gte.map(String::from),
                timespan: timespan.map(String::from),
                window,
                adjusted,
                expand_underlying,
                order: order.map(String::from),
                limit,
                series_type: series_type.map(String::from),
                options: options.cloned(),
            },
        )
    }

    fn get_ema_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetEmaParams,
    ) -> BoxFuture<'a, SingleIndicatorResults> {
        Box::pin(async move {
            let GetEmaParams {
                timestamp,
                timestamp_lt,
                timestamp_lte,
                timestamp_gt,
                timestamp_gte,
                timespan,
                window,
                adjusted,
                expand_underlying,
                order,
                limit,
                series_type,
                options,
            } = params;
            let timestamp = timestamp.as_deref();
            let timestamp_lt = timestamp_lt.as_deref();
            let timestamp_lte = timestamp_lte.as_deref();
            let timestamp_gt = timestamp_gt.as_deref();
            let timestamp_gte = timestamp_gte.as_deref();
            let timespan = timespan.as_deref();
            let order = order.as_deref();
            let series_type = series_type.as_deref();
            let options = options.as_ref();
            let path = format!("/v1/indicators/ema/{}", ticker);
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(t) = timestamp {
                query.push(("timestamp", t.to_string()));
            }
            if let Some(t) = timestamp_lt {
                query.push(("timestamp.lt", t.to_string()));
            }
            if let Some(t) = timestamp_lte {
                query.push(("timestamp.lte", t.to_string()));
            }
            if let Some(t) = timestamp_gt {
                query.push(("timestamp.gt", t.to_string()));
            }
            if let Some(t) = timestamp_gte {
                query.push(("timestamp.gte", t.to_string()));
            }
            if let Some(t) = timespan {
                query.push(("timespan", t.to_string()));
            }
            if let Some(w) = window {
                query.push(("window", w.to_string()));
            }
            if let Some(a) = adjusted {
                query.push(("adjusted", a.to_string()));
            }
            if let Some(e) = expand_underlying {
                query.push(("expand_underlying", e.to_string()));
            }
            if let Some(o) = order {
                query.push(("order", o.to_string()));
            }
            if let Some(l) = limit {
                query.push(("limit", l.to_string()));
            }
            if let Some(s) = series_type {
                query.push(("series_type", s.to_string()));
            }
            #[derive(serde::Deserialize)]
            struct Resp {
                results: SingleIndicatorResults,
            }
            let resp: Resp = self.get(&path, Some(&query), options).await?;
            Ok(resp.results)
        })
    }

    fn get_rsi<'a>(
        &'a self,
        ticker: &'a str,
        timestamp: Option<&'a str>,
        timestamp_lt: Option<&'a str>,
        timestamp_lte: Option<&'a str>,
        timestamp_gt: Option<&'a str>,
        timestamp_gte: Option<&'a str>,
        timespan: Option<&'a str>,
        window: Option<i64>,
        adjusted: Option<bool>,
        expand_underlying: Option<bool>,
        order: Option<&'a str>,
        limit: Option<i64>,
        series_type: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, SingleIndicatorResults> {
        self.get_rsi_with_params(
            ticker,
            GetRsiParams {
                timestamp: timestamp.map(String::from),
                timestamp_lt: timestamp_lt.map(String::from),
                timestamp_lte: timestamp_lte.map(String::from),
                timestamp_gt: timestamp_gt.map(String::from),
                timestamp_gte: timestamp_gte.map(String::from),
                timespan: timespan.map(String::from),
                window,
                adjusted,
                expand_underlying,
                order: order.map(String::from),
                limit,
                series_type: series_type.map(String::from),
                options: options.cloned(),
            },
        )
    }

    fn get_rsi_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetRsiParams,
    ) -> BoxFuture<'a, SingleIndicatorResults> {
        Box::pin(async move {
            let GetRsiParams {
                timestamp,
                timestamp_lt,
                timestamp_lte,
                timestamp_gt,
                timestamp_gte,
                timespan,
                window,
                adjusted,
                expand_underlying,
                order,
                limit,
                series_type,
                options,
            } = params;
            let timestamp = timestamp.as_deref();
            let timestamp_lt = timestamp_lt.as_deref();
            let timestamp_lte = timestamp_lte.as_deref();
            let timestamp_gt = timestamp_gt.as_deref();
            let timestamp_gte = timestamp_gte.as_deref();
            let timespan = timespan.as_deref();
            let order = order.as_deref();
            let series_type = series_type.as_deref();
            let options = options.as_ref();
            let path = format!("/v1/indicators/rsi/{}", ticker);
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(t) = timestamp {
                query.push(("timestamp", t.to_string()));
            }
            if let Some(t) = timestamp_lt {
                query.push(("timestamp.lt", t.to_string()));
            }
            if let Some(t) = timestamp_lte {
                query.push(("timestamp.lte", t.to_string()));
            }
            if let Some(t) = timestamp_gt {
                query.push(("timestamp.gt", t.to_string()));
            }
            if let Some(t) = timestamp_gte {
                query.push(("timestamp.gte", t.to_string()));
            }
            if let Some(t) = timespan {
                query.push(("timespan", t.to_string()));
            }
            if let Some(w) = window {
                query.push(("window", w.to_string()));
            }
            if let Some(a) = adjusted {
                query.push(("adjusted", a.to_string()));
            }
            if let Some(e) = expand_underlying {
                query.push(("expand_underlying", e.to_string()));
            }
            if let Some(o) = order {
                query.push(("order", o.to_string()));
            }
            if let Some(l) = limit {
                query.push(("limit", l.to_string()));
            }
            if let Some(s) = series_type {
                query.push(("series_type", s.to_string()));
            }
            #[derive(serde::Deserialize)]
            struct Resp {
                results: SingleIndicatorResults,
            }
            let resp: Resp = self.get(&path, Some(&query), options).await?;
            Ok(resp.results)
        })
    }

    fn get_macd<'a>(
        &'a self,
        ticker: &'a str,
        timestamp: Option<&'a str>,
        timestamp_lt: Option<&'a str>,
        timestamp_lte: Option<&'a str>,
        timestamp_gt: Option<&'a str>,
        timestamp_gte: Option<&'a str>,
        timespan: Option<&'a str>,
        short_window: Option<i64>,
        long_window: Option<i64>,
        signal_window: Option<i64>,
        adjusted: Option<bool>,
        expand_underlying: Option<bool>,
        order: Option<&'a str>,
        limit: Option<i64>,
        series_type: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, MacdIndicatorResults> {
        self.get_macd_with_params(
            ticker,
            GetMacdParams {
                timestamp: timestamp.map(String::from),
                timestamp_lt: timestamp_lt.map(String::from),
                timestamp_lte: timestamp_lte.map(String::from),
                timestamp_gt: timestamp_gt.map(String::from),
                timestamp_gte: timestamp_gte.map(String::from),
                timespan: timespan.map(String::from),
                short_window,
                long_window,
                signal_window,
                adjusted,
                expand_underlying,
                order: order.map(String::from),
                limit,
                series_type: series_type.map(String::from),
                options: options.cloned(),
            },
        )
    }

    fn get_macd_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetMacdParams,
    ) -> BoxFuture<'a, MacdIndicatorResults> {
        Box::pin(async move {
            let GetMacdParams {
                timestamp,
                timestamp_lt,
                timestamp_lte,
                timestamp_gt,
                timestamp_gte,
                timespan,
                short_window,
                long_window,
                signal_window,
                adjusted,
                expand_underlying,
                order,
                limit,
                series_type,
                options,
            } = params;
            let timestamp = timestamp.as_deref();
            let timestamp_lt = timestamp_lt.as_deref();
            let timestamp_lte = timestamp_lte.as_deref();
            let timestamp_gt = timestamp_gt.as_deref();
            let timestamp_gte = timestamp_gte.as_deref();
            let timespan = timespan.as_deref();
            let order = order.as_deref();
            let series_type = series_type.as_deref();
            let options = options.as_ref();
            let path = format!("/v1/indicators/macd/{}", ticker);
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(t) = timestamp {
                query.push(("timestamp", t.to_string()));
            }
            if let Some(t) = timestamp_lt {
                query.push(("timestamp.lt", t.to_string()));
            }
            if let Some(t) = timestamp_lte {
                query.push(("timestamp.lte", t.to_string()));
            }
            if let Some(t) = timestamp_gt {
                query.push(("timestamp.gt", t.to_string()));
            }
            if let Some(t) = timestamp_gte {
                query.push(("timestamp.gte", t.to_string()));
            }
            if let Some(t) = timespan {
                query.push(("timespan", t.to_string()));
            }
            if let Some(w) = short_window {
                query.push(("short_window", w.to_string()));
            }
            if let Some(w) = long_window {
                query.push(("long_window", w.to_string()));
            }
            if let Some(w) = signal_window {
                query.push(("signal_window", w.to_string()));
            }
            if let Some(a) = adjusted {
                query.push(("adjusted", a.to_string()));
            }
            if let Some(e) = expand_underlying {
                query.push(("expand_underlying", e.to_string()));
            }
            if let Some(o) = order {
                query.push(("order", o.to_string()));
            }
            if let Some(l) = limit {
                query.push(("limit", l.to_string()));
            }
            if let Some(s) = series_type {
                query.push(("series_type", s.to_string()));
            }
            #[derive(serde::Deserialize)]
            struct Resp {
                results: MacdIndicatorResults,
            }
            let resp: Resp = self.get(&path, Some(&query), options).await?;
            Ok(resp.results)
        })
    }
}

// --- Params structs (additive builder API) ---

/// Optional arguments for [`IndicatorsApi::get_sma`].
#[derive(Debug, Default, Clone)]
pub struct GetSmaParams {
    /// The `timestamp` argument.
    pub timestamp: Option<String>,
    /// The `timestamp_lt` argument.
    pub timestamp_lt: Option<String>,
    /// The `timestamp_lte` argument.
    pub timestamp_lte: Option<String>,
    /// The `timestamp_gt` argument.
    pub timestamp_gt: Option<String>,
    /// The `timestamp_gte` argument.
    pub timestamp_gte: Option<String>,
    /// The `timespan` argument.
    pub timespan: Option<String>,
    /// The `window` argument.
    pub window: Option<i64>,
    /// The `adjusted` argument.
    pub adjusted: Option<bool>,
    /// The `expand_underlying` argument.
    pub expand_underlying: Option<bool>,
    /// The `order` argument.
    pub order: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `series_type` argument.
    pub series_type: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetSmaParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `timestamp` argument.
    pub fn timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }

    /// Set the `timestamp_lt` argument.
    pub fn timestamp_lt(mut self, timestamp_lt: impl Into<String>) -> Self {
        self.timestamp_lt = Some(timestamp_lt.into());
        self
    }

    /// Set the `timestamp_lte` argument.
    pub fn timestamp_lte(mut self, timestamp_lte: impl Into<String>) -> Self {
        self.timestamp_lte = Some(timestamp_lte.into());
        self
    }

    /// Set the `timestamp_gt` argument.
    pub fn timestamp_gt(mut self, timestamp_gt: impl Into<String>) -> Self {
        self.timestamp_gt = Some(timestamp_gt.into());
        self
    }

    /// Set the `timestamp_gte` argument.
    pub fn timestamp_gte(mut self, timestamp_gte: impl Into<String>) -> Self {
        self.timestamp_gte = Some(timestamp_gte.into());
        self
    }

    /// Set the `timespan` argument.
    pub fn timespan(mut self, timespan: impl Into<String>) -> Self {
        self.timespan = Some(timespan.into());
        self
    }

    /// Set the `window` argument.
    pub fn window(mut self, window: i64) -> Self {
        self.window = Some(window);
        self
    }

    /// Set the `adjusted` argument.
    pub fn adjusted(mut self, adjusted: bool) -> Self {
        self.adjusted = Some(adjusted);
        self
    }

    /// Set the `expand_underlying` argument.
    pub fn expand_underlying(mut self, expand_underlying: bool) -> Self {
        self.expand_underlying = Some(expand_underlying);
        self
    }

    /// Set the `order` argument.
    pub fn order(mut self, order: impl Into<String>) -> Self {
        self.order = Some(order.into());
        self
    }

    /// Set the `limit` argument.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the `series_type` argument.
    pub fn series_type(mut self, series_type: impl Into<String>) -> Self {
        self.series_type = Some(series_type.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`IndicatorsApi::get_ema`].
#[derive(Debug, Default, Clone)]
pub struct GetEmaParams {
    /// The `timestamp` argument.
    pub timestamp: Option<String>,
    /// The `timestamp_lt` argument.
    pub timestamp_lt: Option<String>,
    /// The `timestamp_lte` argument.
    pub timestamp_lte: Option<String>,
    /// The `timestamp_gt` argument.
    pub timestamp_gt: Option<String>,
    /// The `timestamp_gte` argument.
    pub timestamp_gte: Option<String>,
    /// The `timespan` argument.
    pub timespan: Option<String>,
    /// The `window` argument.
    pub window: Option<i64>,
    /// The `adjusted` argument.
    pub adjusted: Option<bool>,
    /// The `expand_underlying` argument.
    pub expand_underlying: Option<bool>,
    /// The `order` argument.
    pub order: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `series_type` argument.
    pub series_type: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetEmaParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `timestamp` argument.
    pub fn timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }

    /// Set the `timestamp_lt` argument.
    pub fn timestamp_lt(mut self, timestamp_lt: impl Into<String>) -> Self {
        self.timestamp_lt = Some(timestamp_lt.into());
        self
    }

    /// Set the `timestamp_lte` argument.
    pub fn timestamp_lte(mut self, timestamp_lte: impl Into<String>) -> Self {
        self.timestamp_lte = Some(timestamp_lte.into());
        self
    }

    /// Set the `timestamp_gt` argument.
    pub fn timestamp_gt(mut self, timestamp_gt: impl Into<String>) -> Self {
        self.timestamp_gt = Some(timestamp_gt.into());
        self
    }

    /// Set the `timestamp_gte` argument.
    pub fn timestamp_gte(mut self, timestamp_gte: impl Into<String>) -> Self {
        self.timestamp_gte = Some(timestamp_gte.into());
        self
    }

    /// Set the `timespan` argument.
    pub fn timespan(mut self, timespan: impl Into<String>) -> Self {
        self.timespan = Some(timespan.into());
        self
    }

    /// Set the `window` argument.
    pub fn window(mut self, window: i64) -> Self {
        self.window = Some(window);
        self
    }

    /// Set the `adjusted` argument.
    pub fn adjusted(mut self, adjusted: bool) -> Self {
        self.adjusted = Some(adjusted);
        self
    }

    /// Set the `expand_underlying` argument.
    pub fn expand_underlying(mut self, expand_underlying: bool) -> Self {
        self.expand_underlying = Some(expand_underlying);
        self
    }

    /// Set the `order` argument.
    pub fn order(mut self, order: impl Into<String>) -> Self {
        self.order = Some(order.into());
        self
    }

    /// Set the `limit` argument.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the `series_type` argument.
    pub fn series_type(mut self, series_type: impl Into<String>) -> Self {
        self.series_type = Some(series_type.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`IndicatorsApi::get_rsi`].
#[derive(Debug, Default, Clone)]
pub struct GetRsiParams {
    /// The `timestamp` argument.
    pub timestamp: Option<String>,
    /// The `timestamp_lt` argument.
    pub timestamp_lt: Option<String>,
    /// The `timestamp_lte` argument.
    pub timestamp_lte: Option<String>,
    /// The `timestamp_gt` argument.
    pub timestamp_gt: Option<String>,
    /// The `timestamp_gte` argument.
    pub timestamp_gte: Option<String>,
    /// The `timespan` argument.
    pub timespan: Option<String>,
    /// The `window` argument.
    pub window: Option<i64>,
    /// The `adjusted` argument.
    pub adjusted: Option<bool>,
    /// The `expand_underlying` argument.
    pub expand_underlying: Option<bool>,
    /// The `order` argument.
    pub order: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `series_type` argument.
    pub series_type: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetRsiParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `timestamp` argument.
    pub fn timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }

    /// Set the `timestamp_lt` argument.
    pub fn timestamp_lt(mut self, timestamp_lt: impl Into<String>) -> Self {
        self.timestamp_lt = Some(timestamp_lt.into());
        self
    }

    /// Set the `timestamp_lte` argument.
    pub fn timestamp_lte(mut self, timestamp_lte: impl Into<String>) -> Self {
        self.timestamp_lte = Some(timestamp_lte.into());
        self
    }

    /// Set the `timestamp_gt` argument.
    pub fn timestamp_gt(mut self, timestamp_gt: impl Into<String>) -> Self {
        self.timestamp_gt = Some(timestamp_gt.into());
        self
    }

    /// Set the `timestamp_gte` argument.
    pub fn timestamp_gte(mut self, timestamp_gte: impl Into<String>) -> Self {
        self.timestamp_gte = Some(timestamp_gte.into());
        self
    }

    /// Set the `timespan` argument.
    pub fn timespan(mut self, timespan: impl Into<String>) -> Self {
        self.timespan = Some(timespan.into());
        self
    }

    /// Set the `window` argument.
    pub fn window(mut self, window: i64) -> Self {
        self.window = Some(window);
        self
    }

    /// Set the `adjusted` argument.
    pub fn adjusted(mut self, adjusted: bool) -> Self {
        self.adjusted = Some(adjusted);
        self
    }

    /// Set the `expand_underlying` argument.
    pub fn expand_underlying(mut self, expand_underlying: bool) -> Self {
        self.expand_underlying = Some(expand_underlying);
        self
    }

    /// Set the `order` argument.
    pub fn order(mut self, order: impl Into<String>) -> Self {
        self.order = Some(order.into());
        self
    }

    /// Set the `limit` argument.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the `series_type` argument.
    pub fn series_type(mut self, series_type: impl Into<String>) -> Self {
        self.series_type = Some(series_type.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`IndicatorsApi::get_macd`].
#[derive(Debug, Default, Clone)]
pub struct GetMacdParams {
    /// The `timestamp` argument.
    pub timestamp: Option<String>,
    /// The `timestamp_lt` argument.
    pub timestamp_lt: Option<String>,
    /// The `timestamp_lte` argument.
    pub timestamp_lte: Option<String>,
    /// The `timestamp_gt` argument.
    pub timestamp_gt: Option<String>,
    /// The `timestamp_gte` argument.
    pub timestamp_gte: Option<String>,
    /// The `timespan` argument.
    pub timespan: Option<String>,
    /// The `short_window` argument.
    pub short_window: Option<i64>,
    /// The `long_window` argument.
    pub long_window: Option<i64>,
    /// The `signal_window` argument.
    pub signal_window: Option<i64>,
    /// The `adjusted` argument.
    pub adjusted: Option<bool>,
    /// The `expand_underlying` argument.
    pub expand_underlying: Option<bool>,
    /// The `order` argument.
    pub order: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `series_type` argument.
    pub series_type: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetMacdParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `timestamp` argument.
    pub fn timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }

    /// Set the `timestamp_lt` argument.
    pub fn timestamp_lt(mut self, timestamp_lt: impl Into<String>) -> Self {
        self.timestamp_lt = Some(timestamp_lt.into());
        self
    }

    /// Set the `timestamp_lte` argument.
    pub fn timestamp_lte(mut self, timestamp_lte: impl Into<String>) -> Self {
        self.timestamp_lte = Some(timestamp_lte.into());
        self
    }

    /// Set the `timestamp_gt` argument.
    pub fn timestamp_gt(mut self, timestamp_gt: impl Into<String>) -> Self {
        self.timestamp_gt = Some(timestamp_gt.into());
        self
    }

    /// Set the `timestamp_gte` argument.
    pub fn timestamp_gte(mut self, timestamp_gte: impl Into<String>) -> Self {
        self.timestamp_gte = Some(timestamp_gte.into());
        self
    }

    /// Set the `timespan` argument.
    pub fn timespan(mut self, timespan: impl Into<String>) -> Self {
        self.timespan = Some(timespan.into());
        self
    }

    /// Set the `short_window` argument.
    pub fn short_window(mut self, short_window: i64) -> Self {
        self.short_window = Some(short_window);
        self
    }

    /// Set the `long_window` argument.
    pub fn long_window(mut self, long_window: i64) -> Self {
        self.long_window = Some(long_window);
        self
    }

    /// Set the `signal_window` argument.
    pub fn signal_window(mut self, signal_window: i64) -> Self {
        self.signal_window = Some(signal_window);
        self
    }

    /// Set the `adjusted` argument.
    pub fn adjusted(mut self, adjusted: bool) -> Self {
        self.adjusted = Some(adjusted);
        self
    }

    /// Set the `expand_underlying` argument.
    pub fn expand_underlying(mut self, expand_underlying: bool) -> Self {
        self.expand_underlying = Some(expand_underlying);
        self
    }

    /// Set the `order` argument.
    pub fn order(mut self, order: impl Into<String>) -> Self {
        self.order = Some(order.into());
        self
    }

    /// Set the `limit` argument.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the `series_type` argument.
    pub fn series_type(mut self, series_type: impl Into<String>) -> Self {
        self.series_type = Some(series_type.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
