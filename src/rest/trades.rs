use super::{encode_query, BoxFuture, BoxStream};
use crate::client::{Client, RequestOptions};
use crate::models::{CryptoTrade, LastTrade, Trade};

/// Trades API.
pub trait TradesApi {
    /// List trades for a ticker (paginated stream).
    fn list_trades<'a>(
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
    ) -> BoxStream<'a, Trade>;

    /// Same as [`Self::list_trades`], but takes the optional arguments as a
    /// chainable [`ListTradesParams`] struct.
    fn list_trades_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: ListTradesParams,
    ) -> BoxStream<'a, Trade>;

    /// Get the last trade for a ticker.
    fn get_last_trade<'a>(
        &'a self,
        ticker: &'a str,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, LastTrade>;

    /// Same as [`Self::get_last_trade`], but takes the optional arguments as a
    /// chainable [`GetLastTradeParams`] struct.
    fn get_last_trade_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetLastTradeParams,
    ) -> BoxFuture<'a, LastTrade>;

    /// Get the last trade tick for a cryptocurrency pair.
    fn get_last_crypto_trade<'a>(
        &'a self,
        from: &'a str,
        to: &'a str,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, CryptoTrade>;

    /// Same as [`Self::get_last_crypto_trade`], but takes the optional arguments as a
    /// chainable [`GetLastCryptoTradeParams`] struct.
    fn get_last_crypto_trade_with_params<'a>(
        &'a self,
        from: &'a str,
        to: &'a str,
        params: GetLastCryptoTradeParams,
    ) -> BoxFuture<'a, CryptoTrade>;
}

impl TradesApi for Client {
    fn list_trades<'a>(
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
    ) -> BoxStream<'a, Trade> {
        self.list_trades_with_params(
            ticker,
            ListTradesParams {
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

    fn list_trades_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: ListTradesParams,
    ) -> BoxStream<'a, Trade> {
        Box::pin({
            let path = format!("/v3/trades/{}", ticker);
            let query = encode_query(&params);
            self.list::<Trade>(&path, &query, params.options.as_ref())
        })
    }

    fn get_last_trade<'a>(
        &'a self,
        ticker: &'a str,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, LastTrade> {
        self.get_last_trade_with_params(
            ticker,
            GetLastTradeParams {
                options: options.cloned(),
            },
        )
    }

    fn get_last_trade_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetLastTradeParams,
    ) -> BoxFuture<'a, LastTrade> {
        Box::pin(async move {
            let path = format!("/v2/last/trade/{}", ticker);
            let query = encode_query(&params);
            #[derive(serde::Deserialize)]
            struct Resp {
                results: LastTrade,
            }
            let resp: Resp = self
                .get(&path, &query, params.options.as_ref())
                .await?;
            Ok(resp.results)
        })
    }

    fn get_last_crypto_trade<'a>(
        &'a self,
        from: &'a str,
        to: &'a str,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, CryptoTrade> {
        self.get_last_crypto_trade_with_params(
            from,
            to,
            GetLastCryptoTradeParams {
                options: options.cloned(),
            },
        )
    }

    fn get_last_crypto_trade_with_params<'a>(
        &'a self,
        from: &'a str,
        to: &'a str,
        params: GetLastCryptoTradeParams,
    ) -> BoxFuture<'a, CryptoTrade> {
        Box::pin(async move {
            let path = format!("/v1/last/crypto/{}/{}", from, to);
            let query = encode_query(&params);
            #[derive(serde::Deserialize)]
            struct Resp {
                last: CryptoTrade,
            }
            let resp: Resp = self
                .get(&path, &query, params.options.as_ref())
                .await?;
            Ok(resp.last)
        })
    }
}

// --- Params structs (additive builder API) ---
//
// Query serialization is derived: field order is wire order, `rename` carries
// dotted filter operators, unset fields are omitted, and `options` is skipped.

/// Optional arguments for [`TradesApi::list_trades`].
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ListTradesParams {
    /// The `timestamp` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// The `timestamp_lt` argument.
    #[serde(rename = "timestamp.lt", skip_serializing_if = "Option::is_none")]
    pub timestamp_lt: Option<String>,
    /// The `timestamp_lte` argument.
    #[serde(rename = "timestamp.lte", skip_serializing_if = "Option::is_none")]
    pub timestamp_lte: Option<String>,
    /// The `timestamp_gt` argument.
    #[serde(rename = "timestamp.gt", skip_serializing_if = "Option::is_none")]
    pub timestamp_gt: Option<String>,
    /// The `timestamp_gte` argument.
    #[serde(rename = "timestamp.gte", skip_serializing_if = "Option::is_none")]
    pub timestamp_gte: Option<String>,
    /// The `limit` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The `sort` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// The `order` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// The `options` argument.
    #[serde(skip)]
    pub options: Option<RequestOptions>,
}

impl ListTradesParams {
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

/// Optional arguments for [`TradesApi::get_last_trade`].
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct GetLastTradeParams {
    /// The `options` argument.
    #[serde(skip)]
    pub options: Option<RequestOptions>,
}

impl GetLastTradeParams {
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

/// Optional arguments for [`TradesApi::get_last_crypto_trade`].
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct GetLastCryptoTradeParams {
    /// The `options` argument.
    #[serde(skip)]
    pub options: Option<RequestOptions>,
}

impl GetLastCryptoTradeParams {
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
