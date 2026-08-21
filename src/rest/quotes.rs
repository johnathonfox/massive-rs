use super::{BoxFuture, BoxStream};
use crate::client::{Client, RequestOptions};
use crate::models::{LastForexQuote, LastQuote, Quote, RealTimeCurrencyConversion};

/// Quotes (NBBO) API.
pub trait QuotesApi {
    /// List quotes for a ticker (paginated stream).
    fn list_quotes<'a>(
        &'a self,
        ticker: &'a str,
        timestamp: Option<&'a str>,
        timestamp_lt: Option<&'a str>,
        timestamp_lte: Option<&'a str>,
        timestamp_gt: Option<&'a str>,
        timestamp_gte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, Quote>;

    /// Same as [`Self::list_quotes`], but takes the optional arguments as a
    /// chainable [`ListQuotesParams`] struct.
    fn list_quotes_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: ListQuotesParams,
    ) -> BoxStream<'a, Quote>;

    /// Get the last quote for a ticker.
    fn get_last_quote<'a>(
        &'a self,
        ticker: &'a str,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, LastQuote>;

    /// Same as [`Self::get_last_quote`], but takes the optional arguments as a
    /// chainable [`GetLastQuoteParams`] struct.
    fn get_last_quote_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetLastQuoteParams,
    ) -> BoxFuture<'a, LastQuote>;

    /// Get the last quote tick for a forex currency pair.
    fn get_last_forex_quote<'a>(
        &'a self,
        from: &'a str,
        to: &'a str,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, LastForexQuote>;

    /// Same as [`Self::get_last_forex_quote`], but takes the optional arguments as a
    /// chainable [`GetLastForexQuoteParams`] struct.
    fn get_last_forex_quote_with_params<'a>(
        &'a self,
        from: &'a str,
        to: &'a str,
        params: GetLastForexQuoteParams,
    ) -> BoxFuture<'a, LastForexQuote>;

    /// Get currency conversions using the latest market conversion rates.
    fn get_real_time_currency_conversion<'a>(
        &'a self,
        from: &'a str,
        to: &'a str,
        amount: Option<f64>,
        precision: Option<i64>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, RealTimeCurrencyConversion>;

    /// Same as [`Self::get_real_time_currency_conversion`], but takes the optional arguments as a
    /// chainable [`GetRealTimeCurrencyConversionParams`] struct.
    fn get_real_time_currency_conversion_with_params<'a>(
        &'a self,
        from: &'a str,
        to: &'a str,
        params: GetRealTimeCurrencyConversionParams,
    ) -> BoxFuture<'a, RealTimeCurrencyConversion>;
}

impl QuotesApi for Client {
    fn list_quotes<'a>(
        &'a self,
        ticker: &'a str,
        timestamp: Option<&'a str>,
        timestamp_lt: Option<&'a str>,
        timestamp_lte: Option<&'a str>,
        timestamp_gt: Option<&'a str>,
        timestamp_gte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, Quote> {
        self.list_quotes_with_params(
            ticker,
            ListQuotesParams {
                timestamp: timestamp.map(String::from),
                timestamp_lt: timestamp_lt.map(String::from),
                timestamp_lte: timestamp_lte.map(String::from),
                timestamp_gt: timestamp_gt.map(String::from),
                timestamp_gte: timestamp_gte.map(String::from),
                limit,
                sort: sort.map(String::from),
                order: order.map(String::from),
                options: options.cloned(),
            },
        )
    }

    fn list_quotes_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: ListQuotesParams,
    ) -> BoxStream<'a, Quote> {
        Box::pin({
            let ListQuotesParams {
                timestamp,
                timestamp_lt,
                timestamp_lte,
                timestamp_gt,
                timestamp_gte,
                limit,
                sort,
                order,
                options,
            } = params;
            let timestamp = timestamp.as_deref();
            let timestamp_lt = timestamp_lt.as_deref();
            let timestamp_lte = timestamp_lte.as_deref();
            let timestamp_gt = timestamp_gt.as_deref();
            let timestamp_gte = timestamp_gte.as_deref();
            let sort = sort.as_deref();
            let order = order.as_deref();
            let options = options.as_ref();
            let path = format!("/v3/quotes/{}", ticker);
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
            if let Some(l) = limit {
                query.push(("limit", l.to_string()));
            }
            if let Some(s) = sort {
                query.push(("sort", s.to_string()));
            }
            if let Some(o) = order {
                query.push(("order", o.to_string()));
            }
            if self.pagination {
                self.paginate::<Quote>(&path, Some(&query), options)
            } else {
                self.single_page::<Quote>(&path, Some(&query), options)
            }
        })
    }

    fn get_last_quote<'a>(
        &'a self,
        ticker: &'a str,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, LastQuote> {
        self.get_last_quote_with_params(
            ticker,
            GetLastQuoteParams {
                options: options.cloned(),
            },
        )
    }

    fn get_last_quote_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetLastQuoteParams,
    ) -> BoxFuture<'a, LastQuote> {
        Box::pin(async move {
            let GetLastQuoteParams { options } = params;
            let options = options.as_ref();
            let path = format!("/v2/last/nbbo/{}", ticker);
            #[derive(serde::Deserialize)]
            struct Resp {
                results: LastQuote,
            }
            let resp: Resp = self.get(&path, None, options).await?;
            Ok(resp.results)
        })
    }

    fn get_last_forex_quote<'a>(
        &'a self,
        from: &'a str,
        to: &'a str,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, LastForexQuote> {
        self.get_last_forex_quote_with_params(
            from,
            to,
            GetLastForexQuoteParams {
                options: options.cloned(),
            },
        )
    }

    fn get_last_forex_quote_with_params<'a>(
        &'a self,
        from: &'a str,
        to: &'a str,
        params: GetLastForexQuoteParams,
    ) -> BoxFuture<'a, LastForexQuote> {
        Box::pin(async move {
            let GetLastForexQuoteParams { options } = params;
            let options = options.as_ref();
            let path = format!("/v1/last_quote/currencies/{}/{}", from, to);
            self.get(&path, None, options).await
        })
    }

    fn get_real_time_currency_conversion<'a>(
        &'a self,
        from: &'a str,
        to: &'a str,
        amount: Option<f64>,
        precision: Option<i64>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, RealTimeCurrencyConversion> {
        self.get_real_time_currency_conversion_with_params(
            from,
            to,
            GetRealTimeCurrencyConversionParams {
                amount,
                precision,
                options: options.cloned(),
            },
        )
    }

    fn get_real_time_currency_conversion_with_params<'a>(
        &'a self,
        from: &'a str,
        to: &'a str,
        params: GetRealTimeCurrencyConversionParams,
    ) -> BoxFuture<'a, RealTimeCurrencyConversion> {
        Box::pin(async move {
            let GetRealTimeCurrencyConversionParams {
                amount,
                precision,
                options,
            } = params;
            let options = options.as_ref();
            let path = format!("/v1/conversion/{}/{}", from, to);
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(a) = amount {
                query.push(("amount", a.to_string()));
            }
            if let Some(p) = precision {
                query.push(("precision", p.to_string()));
            }
            self.get(&path, Some(&query), options).await
        })
    }
}

// --- Params structs (additive builder API) ---

/// Optional arguments for [`QuotesApi::list_quotes`].
#[derive(Debug, Default, Clone)]
pub struct ListQuotesParams {
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
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `order` argument.
    pub order: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListQuotesParams {
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

    /// Set the `order` argument.
    pub fn order(mut self, order: impl Into<String>) -> Self {
        self.order = Some(order.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`QuotesApi::get_last_quote`].
#[derive(Debug, Default, Clone)]
pub struct GetLastQuoteParams {
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetLastQuoteParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`QuotesApi::get_last_forex_quote`].
#[derive(Debug, Default, Clone)]
pub struct GetLastForexQuoteParams {
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetLastForexQuoteParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`QuotesApi::get_real_time_currency_conversion`].
#[derive(Debug, Default, Clone)]
pub struct GetRealTimeCurrencyConversionParams {
    /// The `amount` argument.
    pub amount: Option<f64>,
    /// The `precision` argument.
    pub precision: Option<i64>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetRealTimeCurrencyConversionParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `amount` argument.
    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Set the `precision` argument.
    pub fn precision(mut self, precision: i64) -> Self {
        self.precision = Some(precision);
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
