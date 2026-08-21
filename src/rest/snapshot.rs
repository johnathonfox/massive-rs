use super::{BoxFuture, BoxStream};
use crate::client::{Client, RequestOptions};
use crate::models::{
    IndicesSnapshot, OptionContractSnapshot, SnapshotTickerFullBook, TickerSnapshot,
    UniversalSnapshot,
};

/// Snapshot API.
pub trait SnapshotApi {
    /// Get snapshots for assets of all types (paginated stream).
    fn list_universal_snapshots<'a>(
        &'a self,
        r#type: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        order: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, UniversalSnapshot>;

    /// Same as [`Self::list_universal_snapshots`], but takes the optional arguments as a
    /// chainable [`ListUniversalSnapshotsParams`] struct.
    fn list_universal_snapshots_with_params<'a>(
        &'a self,
        params: ListUniversalSnapshotsParams,
    ) -> BoxStream<'a, UniversalSnapshot>;

    /// Get the most up-to-date market data for all traded symbols in a market.
    fn get_snapshot_all<'a>(
        &'a self,
        market_type: &'a str,
        tickers: Option<&'a str>,
        include_otc: Option<bool>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<TickerSnapshot>>;

    /// Same as [`Self::get_snapshot_all`], but takes the optional arguments as a
    /// chainable [`GetSnapshotAllParams`] struct.
    fn get_snapshot_all_with_params<'a>(
        &'a self,
        market_type: &'a str,
        params: GetSnapshotAllParams,
    ) -> BoxFuture<'a, Vec<TickerSnapshot>>;

    /// Get the current top 20 gainers or losers of the day in a market.
    fn get_snapshot_direction<'a>(
        &'a self,
        market_type: &'a str,
        direction: &'a str,
        include_otc: Option<bool>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<TickerSnapshot>>;

    /// Same as [`Self::get_snapshot_direction`], but takes the optional arguments as a
    /// chainable [`GetSnapshotDirectionParams`] struct.
    fn get_snapshot_direction_with_params<'a>(
        &'a self,
        market_type: &'a str,
        direction: &'a str,
        params: GetSnapshotDirectionParams,
    ) -> BoxFuture<'a, Vec<TickerSnapshot>>;

    /// Get the most up-to-date market data for a single ticker.
    fn get_snapshot_ticker<'a>(
        &'a self,
        market_type: &'a str,
        ticker: &'a str,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, TickerSnapshot>;

    /// Same as [`Self::get_snapshot_ticker`], but takes the optional arguments as a
    /// chainable [`GetSnapshotTickerParams`] struct.
    fn get_snapshot_ticker_with_params<'a>(
        &'a self,
        market_type: &'a str,
        ticker: &'a str,
        params: GetSnapshotTickerParams,
    ) -> BoxFuture<'a, TickerSnapshot>;

    /// Get the snapshot of an option contract for an underlying asset.
    fn get_snapshot_option<'a>(
        &'a self,
        underlying_asset: &'a str,
        option_contract: &'a str,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, OptionContractSnapshot>;

    /// Same as [`Self::get_snapshot_option`], but takes the optional arguments as a
    /// chainable [`GetSnapshotOptionParams`] struct.
    fn get_snapshot_option_with_params<'a>(
        &'a self,
        underlying_asset: &'a str,
        option_contract: &'a str,
        params: GetSnapshotOptionParams,
    ) -> BoxFuture<'a, OptionContractSnapshot>;

    /// Get the snapshot of all options contracts for an underlying ticker (paginated stream).
    fn list_snapshot_options_chain<'a>(
        &'a self,
        underlying_asset: &'a str,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, OptionContractSnapshot>;

    /// Same as [`Self::list_snapshot_options_chain`], but takes the optional arguments as a
    /// chainable [`ListSnapshotOptionsChainParams`] struct.
    fn list_snapshot_options_chain_with_params<'a>(
        &'a self,
        underlying_asset: &'a str,
        params: ListSnapshotOptionsChainParams,
    ) -> BoxStream<'a, OptionContractSnapshot>;

    /// Get the current level 2 book of a single crypto ticker (all exchanges combined).
    fn get_snapshot_crypto_book<'a>(
        &'a self,
        ticker: &'a str,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, SnapshotTickerFullBook>;

    /// Same as [`Self::get_snapshot_crypto_book`], but takes the optional arguments as a
    /// chainable [`GetSnapshotCryptoBookParams`] struct.
    fn get_snapshot_crypto_book_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetSnapshotCryptoBookParams,
    ) -> BoxFuture<'a, SnapshotTickerFullBook>;

    /// Get snapshots for indices.
    fn get_snapshot_indices<'a>(
        &'a self,
        ticker_any_of: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<IndicesSnapshot>>;

    /// Same as [`Self::get_snapshot_indices`], but takes the optional arguments as a
    /// chainable [`GetSnapshotIndicesParams`] struct.
    fn get_snapshot_indices_with_params<'a>(
        &'a self,
        params: GetSnapshotIndicesParams,
    ) -> BoxFuture<'a, Vec<IndicesSnapshot>>;
}

fn get_locale(market_type: &str) -> &'static str {
    if market_type == "stocks" {
        "us"
    } else {
        "global"
    }
}

impl SnapshotApi for Client {
    fn list_universal_snapshots<'a>(
        &'a self,
        r#type: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        order: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, UniversalSnapshot> {
        self.list_universal_snapshots_with_params(ListUniversalSnapshotsParams {
            r#type: r#type.map(String::from),
            ticker_any_of: ticker_any_of.map(String::from),
            order: order.map(String::from),
            limit,
            sort: sort.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_universal_snapshots_with_params<'a>(
        &'a self,
        params: ListUniversalSnapshotsParams,
    ) -> BoxStream<'a, UniversalSnapshot> {
        Box::pin({
            let ListUniversalSnapshotsParams {
                r#type,
                ticker_any_of,
                order,
                limit,
                sort,
                ticker_lt,
                ticker_lte,
                ticker_gt,
                ticker_gte,
                options,
            } = params;
            let r#type = r#type.as_deref();
            let ticker_any_of = ticker_any_of.as_deref();
            let order = order.as_deref();
            let sort = sort.as_deref();
            let ticker_lt = ticker_lt.as_deref();
            let ticker_lte = ticker_lte.as_deref();
            let ticker_gt = ticker_gt.as_deref();
            let ticker_gte = ticker_gte.as_deref();
            let options = options.as_ref();
            let path = "/v3/snapshot".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(t) = r#type {
                query.push(("type", t.to_string()));
            }
            if let Some(t) = ticker_any_of {
                query.push(("ticker_any_of", t.to_string()));
            }
            if let Some(o) = order {
                query.push(("order", o.to_string()));
            }
            if let Some(l) = limit {
                query.push(("limit", l.to_string()));
            }
            if let Some(s) = sort {
                query.push(("sort", s.to_string()));
            }
            if let Some(t) = ticker_lt {
                query.push(("ticker.lt", t.to_string()));
            }
            if let Some(t) = ticker_lte {
                query.push(("ticker.lte", t.to_string()));
            }
            if let Some(t) = ticker_gt {
                query.push(("ticker.gt", t.to_string()));
            }
            if let Some(t) = ticker_gte {
                query.push(("ticker.gte", t.to_string()));
            }
            if self.pagination {
                self.paginate::<UniversalSnapshot>(&path, Some(&query), options)
            } else {
                self.single_page::<UniversalSnapshot>(&path, Some(&query), options)
            }
        })
    }

    fn get_snapshot_all<'a>(
        &'a self,
        market_type: &'a str,
        tickers: Option<&'a str>,
        include_otc: Option<bool>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<TickerSnapshot>> {
        self.get_snapshot_all_with_params(
            market_type,
            GetSnapshotAllParams {
                tickers: tickers.map(String::from),
                include_otc,
                options: options.cloned(),
            },
        )
    }

    fn get_snapshot_all_with_params<'a>(
        &'a self,
        market_type: &'a str,
        params: GetSnapshotAllParams,
    ) -> BoxFuture<'a, Vec<TickerSnapshot>> {
        Box::pin(async move {
            let GetSnapshotAllParams {
                tickers,
                include_otc,
                options,
            } = params;
            let tickers = tickers.as_deref();
            let options = options.as_ref();
            let locale = get_locale(market_type);
            let path = format!(
                "/v2/snapshot/locale/{}/markets/{}/tickers",
                locale, market_type
            );
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(t) = tickers {
                query.push(("tickers", t.to_string()));
            }
            if let Some(i) = include_otc {
                query.push(("include_otc", i.to_string()));
            }
            #[derive(serde::Deserialize)]
            struct Resp {
                tickers: Option<Vec<TickerSnapshot>>,
            }
            let resp: Resp = self.get(&path, Some(&query), options).await?;
            Ok(resp.tickers.unwrap_or_default())
        })
    }

    fn get_snapshot_direction<'a>(
        &'a self,
        market_type: &'a str,
        direction: &'a str,
        include_otc: Option<bool>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<TickerSnapshot>> {
        self.get_snapshot_direction_with_params(
            market_type,
            direction,
            GetSnapshotDirectionParams {
                include_otc,
                options: options.cloned(),
            },
        )
    }

    fn get_snapshot_direction_with_params<'a>(
        &'a self,
        market_type: &'a str,
        direction: &'a str,
        params: GetSnapshotDirectionParams,
    ) -> BoxFuture<'a, Vec<TickerSnapshot>> {
        Box::pin(async move {
            let GetSnapshotDirectionParams {
                include_otc,
                options,
            } = params;
            let options = options.as_ref();
            let locale = get_locale(market_type);
            let path = format!(
                "/v2/snapshot/locale/{}/markets/{}/{}",
                locale, market_type, direction
            );
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(i) = include_otc {
                query.push(("include_otc", i.to_string()));
            }
            #[derive(serde::Deserialize)]
            struct Resp {
                tickers: Option<Vec<TickerSnapshot>>,
            }
            let resp: Resp = self.get(&path, Some(&query), options).await?;
            Ok(resp.tickers.unwrap_or_default())
        })
    }

    fn get_snapshot_ticker<'a>(
        &'a self,
        market_type: &'a str,
        ticker: &'a str,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, TickerSnapshot> {
        self.get_snapshot_ticker_with_params(
            market_type,
            ticker,
            GetSnapshotTickerParams {
                options: options.cloned(),
            },
        )
    }

    fn get_snapshot_ticker_with_params<'a>(
        &'a self,
        market_type: &'a str,
        ticker: &'a str,
        params: GetSnapshotTickerParams,
    ) -> BoxFuture<'a, TickerSnapshot> {
        Box::pin(async move {
            let GetSnapshotTickerParams { options } = params;
            let options = options.as_ref();
            let locale = get_locale(market_type);
            let path = format!(
                "/v2/snapshot/locale/{}/markets/{}/tickers/{}",
                locale, market_type, ticker
            );
            #[derive(serde::Deserialize)]
            struct Resp {
                ticker: TickerSnapshot,
            }
            let resp: Resp = self.get(&path, None, options).await?;
            Ok(resp.ticker)
        })
    }

    fn get_snapshot_option<'a>(
        &'a self,
        underlying_asset: &'a str,
        option_contract: &'a str,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, OptionContractSnapshot> {
        self.get_snapshot_option_with_params(
            underlying_asset,
            option_contract,
            GetSnapshotOptionParams {
                options: options.cloned(),
            },
        )
    }

    fn get_snapshot_option_with_params<'a>(
        &'a self,
        underlying_asset: &'a str,
        option_contract: &'a str,
        params: GetSnapshotOptionParams,
    ) -> BoxFuture<'a, OptionContractSnapshot> {
        Box::pin(async move {
            let GetSnapshotOptionParams { options } = params;
            let options = options.as_ref();
            let path = format!(
                "/v3/snapshot/options/{}/{}",
                underlying_asset, option_contract
            );
            #[derive(serde::Deserialize)]
            struct Resp {
                results: OptionContractSnapshot,
            }
            let resp: Resp = self.get(&path, None, options).await?;
            Ok(resp.results)
        })
    }

    fn list_snapshot_options_chain<'a>(
        &'a self,
        underlying_asset: &'a str,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, OptionContractSnapshot> {
        self.list_snapshot_options_chain_with_params(
            underlying_asset,
            ListSnapshotOptionsChainParams {
                options: options.cloned(),
            },
        )
    }

    fn list_snapshot_options_chain_with_params<'a>(
        &'a self,
        underlying_asset: &'a str,
        params: ListSnapshotOptionsChainParams,
    ) -> BoxStream<'a, OptionContractSnapshot> {
        Box::pin({
            let ListSnapshotOptionsChainParams { options } = params;
            let options = options.as_ref();
            let path = format!("/v3/snapshot/options/{}", underlying_asset);
            if self.pagination {
                self.paginate::<OptionContractSnapshot>(&path, None, options)
            } else {
                self.single_page::<OptionContractSnapshot>(&path, None, options)
            }
        })
    }

    fn get_snapshot_crypto_book<'a>(
        &'a self,
        ticker: &'a str,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, SnapshotTickerFullBook> {
        self.get_snapshot_crypto_book_with_params(
            ticker,
            GetSnapshotCryptoBookParams {
                options: options.cloned(),
            },
        )
    }

    fn get_snapshot_crypto_book_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetSnapshotCryptoBookParams,
    ) -> BoxFuture<'a, SnapshotTickerFullBook> {
        Box::pin(async move {
            let GetSnapshotCryptoBookParams { options } = params;
            let options = options.as_ref();
            let path = format!(
                "/v2/snapshot/locale/global/markets/crypto/tickers/{}/book",
                ticker
            );
            #[derive(serde::Deserialize)]
            struct Resp {
                data: SnapshotTickerFullBook,
            }
            let resp: Resp = self.get(&path, None, options).await?;
            Ok(resp.data)
        })
    }

    fn get_snapshot_indices<'a>(
        &'a self,
        ticker_any_of: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<IndicesSnapshot>> {
        self.get_snapshot_indices_with_params(GetSnapshotIndicesParams {
            ticker_any_of: ticker_any_of.map(String::from),
            options: options.cloned(),
        })
    }

    fn get_snapshot_indices_with_params<'a>(
        &'a self,
        params: GetSnapshotIndicesParams,
    ) -> BoxFuture<'a, Vec<IndicesSnapshot>> {
        Box::pin(async move {
            let GetSnapshotIndicesParams {
                ticker_any_of,
                options,
            } = params;
            let ticker_any_of = ticker_any_of.as_deref();
            let options = options.as_ref();
            let path = "/v3/snapshot/indices".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(t) = ticker_any_of {
                query.push(("ticker_any_of", t.to_string()));
            }
            #[derive(serde::Deserialize)]
            struct Resp {
                results: Option<Vec<IndicesSnapshot>>,
            }
            let resp: Resp = self.get(&path, Some(&query), options).await?;
            Ok(resp.results.unwrap_or_default())
        })
    }
}

// --- Params structs (additive builder API) ---

/// Optional arguments for [`SnapshotApi::list_universal_snapshots`].
#[derive(Debug, Default, Clone)]
pub struct ListUniversalSnapshotsParams {
    /// The `type` argument.
    pub r#type: Option<String>,
    /// The `ticker_any_of` argument.
    pub ticker_any_of: Option<String>,
    /// The `order` argument.
    pub order: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `ticker_lt` argument.
    pub ticker_lt: Option<String>,
    /// The `ticker_lte` argument.
    pub ticker_lte: Option<String>,
    /// The `ticker_gt` argument.
    pub ticker_gt: Option<String>,
    /// The `ticker_gte` argument.
    pub ticker_gte: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListUniversalSnapshotsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `type` argument.
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Set the `ticker_any_of` argument.
    pub fn ticker_any_of(mut self, ticker_any_of: impl Into<String>) -> Self {
        self.ticker_any_of = Some(ticker_any_of.into());
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

    /// Set the `sort` argument.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Set the `ticker_lt` argument.
    pub fn ticker_lt(mut self, ticker_lt: impl Into<String>) -> Self {
        self.ticker_lt = Some(ticker_lt.into());
        self
    }

    /// Set the `ticker_lte` argument.
    pub fn ticker_lte(mut self, ticker_lte: impl Into<String>) -> Self {
        self.ticker_lte = Some(ticker_lte.into());
        self
    }

    /// Set the `ticker_gt` argument.
    pub fn ticker_gt(mut self, ticker_gt: impl Into<String>) -> Self {
        self.ticker_gt = Some(ticker_gt.into());
        self
    }

    /// Set the `ticker_gte` argument.
    pub fn ticker_gte(mut self, ticker_gte: impl Into<String>) -> Self {
        self.ticker_gte = Some(ticker_gte.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`SnapshotApi::get_snapshot_all`].
#[derive(Debug, Default, Clone)]
pub struct GetSnapshotAllParams {
    /// The `tickers` argument.
    pub tickers: Option<String>,
    /// The `include_otc` argument.
    pub include_otc: Option<bool>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetSnapshotAllParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `tickers` argument.
    pub fn tickers(mut self, tickers: impl Into<String>) -> Self {
        self.tickers = Some(tickers.into());
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

/// Optional arguments for [`SnapshotApi::get_snapshot_direction`].
#[derive(Debug, Default, Clone)]
pub struct GetSnapshotDirectionParams {
    /// The `include_otc` argument.
    pub include_otc: Option<bool>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetSnapshotDirectionParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
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

/// Optional arguments for [`SnapshotApi::get_snapshot_ticker`].
#[derive(Debug, Default, Clone)]
pub struct GetSnapshotTickerParams {
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetSnapshotTickerParams {
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

/// Optional arguments for [`SnapshotApi::get_snapshot_option`].
#[derive(Debug, Default, Clone)]
pub struct GetSnapshotOptionParams {
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetSnapshotOptionParams {
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

/// Optional arguments for [`SnapshotApi::list_snapshot_options_chain`].
#[derive(Debug, Default, Clone)]
pub struct ListSnapshotOptionsChainParams {
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListSnapshotOptionsChainParams {
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

/// Optional arguments for [`SnapshotApi::get_snapshot_crypto_book`].
#[derive(Debug, Default, Clone)]
pub struct GetSnapshotCryptoBookParams {
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetSnapshotCryptoBookParams {
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

/// Optional arguments for [`SnapshotApi::get_snapshot_indices`].
#[derive(Debug, Default, Clone)]
pub struct GetSnapshotIndicesParams {
    /// The `ticker_any_of` argument.
    pub ticker_any_of: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetSnapshotIndicesParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `ticker_any_of` argument.
    pub fn ticker_any_of(mut self, ticker_any_of: impl Into<String>) -> Self {
        self.ticker_any_of = Some(ticker_any_of.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
