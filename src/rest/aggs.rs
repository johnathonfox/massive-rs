use super::{BoxFuture, BoxStream};
use crate::client::{Client, RequestOptions};
use crate::models::{Agg, DailyOpenCloseAgg, GroupedDailyAgg, PreviousCloseAgg};

/// Aggregates (OHLCV bars) API.
pub trait AggsApi {
    /// List aggregate bars for a ticker over a given date range.
    /// Returns a stream that automatically paginates through all pages.
    fn list_aggs<'a>(
        &'a self,
        ticker: &'a str,
        multiplier: i64,
        timespan: &'a str,
        from: &'a str,
        to: &'a str,
        adjusted: Option<bool>,
        sort: Option<&'a str>,
        limit: Option<i64>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, Agg>;

    /// Same as [`Self::list_aggs`], but takes the optional arguments as a
    /// chainable [`ListAggsParams`] struct.
    fn list_aggs_with_params<'a>(
        &'a self,
        ticker: &'a str,
        multiplier: i64,
        timespan: &'a str,
        from: &'a str,
        to: &'a str,
        params: ListAggsParams,
    ) -> BoxStream<'a, Agg>;

    /// Get aggregate bars (single page, no pagination follow).
    fn get_aggs<'a>(
        &'a self,
        ticker: &'a str,
        multiplier: i64,
        timespan: &'a str,
        from: &'a str,
        to: &'a str,
        adjusted: Option<bool>,
        sort: Option<&'a str>,
        limit: Option<i64>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<Agg>>;

    /// Same as [`Self::get_aggs`], but takes the optional arguments as a
    /// chainable [`GetAggsParams`] struct.
    fn get_aggs_with_params<'a>(
        &'a self,
        ticker: &'a str,
        multiplier: i64,
        timespan: &'a str,
        from: &'a str,
        to: &'a str,
        params: GetAggsParams,
    ) -> BoxFuture<'a, Vec<Agg>>;

    /// Get the daily OHLC for the entire market.
    fn get_grouped_daily_aggs<'a>(
        &'a self,
        date: &'a str,
        adjusted: Option<bool>,
        locale: Option<&'a str>,
        market_type: Option<&'a str>,
        include_otc: Option<bool>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<GroupedDailyAgg>>;

    /// Same as [`Self::get_grouped_daily_aggs`], but takes the optional arguments as a
    /// chainable [`GetGroupedDailyAggsParams`] struct.
    fn get_grouped_daily_aggs_with_params<'a>(
        &'a self,
        date: &'a str,
        params: GetGroupedDailyAggsParams,
    ) -> BoxFuture<'a, Vec<GroupedDailyAgg>>;

    /// Get the open, close and afterhours prices for a ticker on a date.
    fn get_daily_open_close_agg<'a>(
        &'a self,
        ticker: &'a str,
        date: &'a str,
        adjusted: Option<bool>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, DailyOpenCloseAgg>;

    /// Same as [`Self::get_daily_open_close_agg`], but takes the optional arguments as a
    /// chainable [`GetDailyOpenCloseAggParams`] struct.
    fn get_daily_open_close_agg_with_params<'a>(
        &'a self,
        ticker: &'a str,
        date: &'a str,
        params: GetDailyOpenCloseAggParams,
    ) -> BoxFuture<'a, DailyOpenCloseAgg>;

    /// Get the previous day's OHLC for a ticker.
    fn get_previous_close_agg<'a>(
        &'a self,
        ticker: &'a str,
        adjusted: Option<bool>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<PreviousCloseAgg>>;

    /// Same as [`Self::get_previous_close_agg`], but takes the optional arguments as a
    /// chainable [`GetPreviousCloseAggParams`] struct.
    fn get_previous_close_agg_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetPreviousCloseAggParams,
    ) -> BoxFuture<'a, Vec<PreviousCloseAgg>>;
}

impl AggsApi for Client {
    fn list_aggs<'a>(
        &'a self,
        ticker: &'a str,
        multiplier: i64,
        timespan: &'a str,
        from: &'a str,
        to: &'a str,
        adjusted: Option<bool>,
        sort: Option<&'a str>,
        limit: Option<i64>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, Agg> {
        self.list_aggs_with_params(
            ticker,
            multiplier,
            timespan,
            from,
            to,
            ListAggsParams {
                adjusted,
                sort: sort.map(String::from),
                limit,
                options: options.cloned(),
            },
        )
    }

    fn list_aggs_with_params<'a>(
        &'a self,
        ticker: &'a str,
        multiplier: i64,
        timespan: &'a str,
        from: &'a str,
        to: &'a str,
        params: ListAggsParams,
    ) -> BoxStream<'a, Agg> {
        Box::pin({
            let ListAggsParams {
                adjusted,
                sort,
                limit,
                options,
            } = params;
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = format!(
                "/v2/aggs/ticker/{}/range/{}/{}/{}/{}",
                ticker, multiplier, timespan, from, to
            );
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(a) = adjusted {
                query.push(("adjusted", a.to_string()));
            }
            if let Some(s) = sort {
                query.push(("sort", s.to_string()));
            }
            if let Some(l) = limit {
                query.push(("limit", l.to_string()));
            }
            if self.pagination {
                self.paginate::<Agg>(&path, Some(&query), options)
            } else {
                self.single_page::<Agg>(&path, Some(&query), options)
            }
        })
    }

    fn get_aggs<'a>(
        &'a self,
        ticker: &'a str,
        multiplier: i64,
        timespan: &'a str,
        from: &'a str,
        to: &'a str,
        adjusted: Option<bool>,
        sort: Option<&'a str>,
        limit: Option<i64>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<Agg>> {
        self.get_aggs_with_params(
            ticker,
            multiplier,
            timespan,
            from,
            to,
            GetAggsParams {
                adjusted,
                sort: sort.map(String::from),
                limit,
                options: options.cloned(),
            },
        )
    }

    fn get_aggs_with_params<'a>(
        &'a self,
        ticker: &'a str,
        multiplier: i64,
        timespan: &'a str,
        from: &'a str,
        to: &'a str,
        params: GetAggsParams,
    ) -> BoxFuture<'a, Vec<Agg>> {
        Box::pin(async move {
            let GetAggsParams {
                adjusted,
                sort,
                limit,
                options,
            } = params;
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = format!(
                "/v2/aggs/ticker/{}/range/{}/{}/{}/{}",
                ticker, multiplier, timespan, from, to
            );
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(a) = adjusted {
                query.push(("adjusted", a.to_string()));
            }
            if let Some(s) = sort {
                query.push(("sort", s.to_string()));
            }
            if let Some(l) = limit {
                query.push(("limit", l.to_string()));
            }
            #[derive(serde::Deserialize)]
            struct Resp {
                results: Option<Vec<Agg>>,
            }
            let resp: Resp = self.get(&path, Some(&query), options).await?;
            Ok(resp.results.unwrap_or_default())
        })
    }

    fn get_grouped_daily_aggs<'a>(
        &'a self,
        date: &'a str,
        adjusted: Option<bool>,
        locale: Option<&'a str>,
        market_type: Option<&'a str>,
        include_otc: Option<bool>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<GroupedDailyAgg>> {
        self.get_grouped_daily_aggs_with_params(
            date,
            GetGroupedDailyAggsParams {
                adjusted,
                locale: locale.map(String::from),
                market_type: market_type.map(String::from),
                include_otc,
                options: options.cloned(),
            },
        )
    }

    fn get_grouped_daily_aggs_with_params<'a>(
        &'a self,
        date: &'a str,
        params: GetGroupedDailyAggsParams,
    ) -> BoxFuture<'a, Vec<GroupedDailyAgg>> {
        Box::pin(async move {
            let GetGroupedDailyAggsParams {
                adjusted,
                locale,
                market_type,
                include_otc,
                options,
            } = params;
            let locale = locale.as_deref();
            let market_type = market_type.as_deref();
            let options = options.as_ref();
            let locale = locale.unwrap_or("us");
            let market_type = market_type.unwrap_or("stocks");
            let path = format!(
                "/v2/aggs/grouped/locale/{}/market/{}/{}",
                locale, market_type, date
            );
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(a) = adjusted {
                query.push(("adjusted", a.to_string()));
            }
            if let Some(i) = include_otc {
                query.push(("include_otc", i.to_string()));
            }
            #[derive(serde::Deserialize)]
            struct Resp {
                results: Option<Vec<GroupedDailyAgg>>,
            }
            let resp: Resp = self.get(&path, Some(&query), options).await?;
            Ok(resp.results.unwrap_or_default())
        })
    }

    fn get_daily_open_close_agg<'a>(
        &'a self,
        ticker: &'a str,
        date: &'a str,
        adjusted: Option<bool>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, DailyOpenCloseAgg> {
        self.get_daily_open_close_agg_with_params(
            ticker,
            date,
            GetDailyOpenCloseAggParams {
                adjusted,
                options: options.cloned(),
            },
        )
    }

    fn get_daily_open_close_agg_with_params<'a>(
        &'a self,
        ticker: &'a str,
        date: &'a str,
        params: GetDailyOpenCloseAggParams,
    ) -> BoxFuture<'a, DailyOpenCloseAgg> {
        Box::pin(async move {
            let GetDailyOpenCloseAggParams { adjusted, options } = params;
            let options = options.as_ref();
            let path = format!("/v1/open-close/{}/{}", ticker, date);
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(a) = adjusted {
                query.push(("adjusted", a.to_string()));
            }
            self.get(&path, Some(&query), options).await
        })
    }

    fn get_previous_close_agg<'a>(
        &'a self,
        ticker: &'a str,
        adjusted: Option<bool>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<PreviousCloseAgg>> {
        self.get_previous_close_agg_with_params(
            ticker,
            GetPreviousCloseAggParams {
                adjusted,
                options: options.cloned(),
            },
        )
    }

    fn get_previous_close_agg_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetPreviousCloseAggParams,
    ) -> BoxFuture<'a, Vec<PreviousCloseAgg>> {
        Box::pin(async move {
            let GetPreviousCloseAggParams { adjusted, options } = params;
            let options = options.as_ref();
            let path = format!("/v2/aggs/ticker/{}/prev", ticker);
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(a) = adjusted {
                query.push(("adjusted", a.to_string()));
            }
            #[derive(serde::Deserialize)]
            struct Resp {
                results: Option<Vec<PreviousCloseAgg>>,
            }
            let resp: Resp = self.get(&path, Some(&query), options).await?;
            Ok(resp.results.unwrap_or_default())
        })
    }
}

// --- Params structs (additive builder API) ---

/// Optional arguments for [`AggsApi::list_aggs`].
#[derive(Debug, Default, Clone)]
pub struct ListAggsParams {
    /// The `adjusted` argument.
    pub adjusted: Option<bool>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListAggsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `adjusted` argument.
    pub fn adjusted(mut self, adjusted: bool) -> Self {
        self.adjusted = Some(adjusted);
        self
    }

    /// Set the `sort` argument.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
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

/// Optional arguments for [`AggsApi::get_aggs`].
#[derive(Debug, Default, Clone)]
pub struct GetAggsParams {
    /// The `adjusted` argument.
    pub adjusted: Option<bool>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetAggsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `adjusted` argument.
    pub fn adjusted(mut self, adjusted: bool) -> Self {
        self.adjusted = Some(adjusted);
        self
    }

    /// Set the `sort` argument.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
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

/// Optional arguments for [`AggsApi::get_grouped_daily_aggs`].
#[derive(Debug, Default, Clone)]
pub struct GetGroupedDailyAggsParams {
    /// The `adjusted` argument.
    pub adjusted: Option<bool>,
    /// The `locale` argument.
    pub locale: Option<String>,
    /// The `market_type` argument.
    pub market_type: Option<String>,
    /// The `include_otc` argument.
    pub include_otc: Option<bool>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetGroupedDailyAggsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `adjusted` argument.
    pub fn adjusted(mut self, adjusted: bool) -> Self {
        self.adjusted = Some(adjusted);
        self
    }

    /// Set the `locale` argument.
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    /// Set the `market_type` argument.
    pub fn market_type(mut self, market_type: impl Into<String>) -> Self {
        self.market_type = Some(market_type.into());
        self
    }

    /// Set the `include_otc` argument.
    pub fn include_otc(mut self, include_otc: bool) -> Self {
        self.include_otc = Some(include_otc);
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`AggsApi::get_daily_open_close_agg`].
#[derive(Debug, Default, Clone)]
pub struct GetDailyOpenCloseAggParams {
    /// The `adjusted` argument.
    pub adjusted: Option<bool>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetDailyOpenCloseAggParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `adjusted` argument.
    pub fn adjusted(mut self, adjusted: bool) -> Self {
        self.adjusted = Some(adjusted);
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`AggsApi::get_previous_close_agg`].
#[derive(Debug, Default, Clone)]
pub struct GetPreviousCloseAggParams {
    /// The `adjusted` argument.
    pub adjusted: Option<bool>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetPreviousCloseAggParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `adjusted` argument.
    pub fn adjusted(mut self, adjusted: bool) -> Self {
        self.adjusted = Some(adjusted);
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
