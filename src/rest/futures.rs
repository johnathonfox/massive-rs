use super::BoxStream;
use crate::client::{Client, RequestOptions};
use crate::models::{
    FuturesAgg, FuturesContract, FuturesExchange, FuturesMarketStatus, FuturesProduct,
    FuturesQuote, FuturesSchedule, FuturesSnapshot, FuturesTrade,
};

/// Futures API.
pub trait FuturesApi {
    /// Get aggregates for a futures contract in a given time range (paginated stream).
    fn list_futures_aggregates<'a>(
        &'a self,
        ticker: &'a str,
        resolution: Option<&'a str>,
        window_start: Option<&'a str>,
        window_start_lt: Option<&'a str>,
        window_start_lte: Option<&'a str>,
        window_start_gt: Option<&'a str>,
        window_start_gte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FuturesAgg>;

    /// Same as [`Self::list_futures_aggregates`], but takes the optional arguments as a
    /// chainable [`ListFuturesAggregatesParams`] struct.
    fn list_futures_aggregates_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: ListFuturesAggregatesParams,
    ) -> BoxStream<'a, FuturesAgg>;

    /// List futures contracts (paginated stream).
    fn list_futures_contracts<'a>(
        &'a self,
        date: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        product_code: Option<&'a str>,
        product_code_any_of: Option<&'a str>,
        product_code_gt: Option<&'a str>,
        product_code_gte: Option<&'a str>,
        product_code_lt: Option<&'a str>,
        product_code_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        active: Option<bool>,
        type_: Option<&'a str>,
        type_any_of: Option<&'a str>,
        first_trade_date: Option<&'a str>,
        first_trade_date_gt: Option<&'a str>,
        first_trade_date_gte: Option<&'a str>,
        first_trade_date_lt: Option<&'a str>,
        first_trade_date_lte: Option<&'a str>,
        last_trade_date: Option<&'a str>,
        last_trade_date_gt: Option<&'a str>,
        last_trade_date_gte: Option<&'a str>,
        last_trade_date_lt: Option<&'a str>,
        last_trade_date_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FuturesContract>;

    /// Same as [`Self::list_futures_contracts`], but takes the optional arguments as a
    /// chainable [`ListFuturesContractsParams`] struct.
    fn list_futures_contracts_with_params<'a>(
        &'a self,
        params: ListFuturesContractsParams,
    ) -> BoxStream<'a, FuturesContract>;

    /// List futures products, including combos (paginated stream).
    fn list_futures_products<'a>(
        &'a self,
        name: Option<&'a str>,
        name_any_of: Option<&'a str>,
        name_gt: Option<&'a str>,
        name_gte: Option<&'a str>,
        name_lt: Option<&'a str>,
        name_lte: Option<&'a str>,
        product_code: Option<&'a str>,
        product_code_any_of: Option<&'a str>,
        product_code_gt: Option<&'a str>,
        product_code_gte: Option<&'a str>,
        product_code_lt: Option<&'a str>,
        product_code_lte: Option<&'a str>,
        date: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        trading_venue: Option<&'a str>,
        trading_venue_any_of: Option<&'a str>,
        trading_venue_gt: Option<&'a str>,
        trading_venue_gte: Option<&'a str>,
        trading_venue_lt: Option<&'a str>,
        trading_venue_lte: Option<&'a str>,
        sector: Option<&'a str>,
        sector_any_of: Option<&'a str>,
        sub_sector: Option<&'a str>,
        sub_sector_any_of: Option<&'a str>,
        asset_class: Option<&'a str>,
        asset_class_any_of: Option<&'a str>,
        asset_sub_class: Option<&'a str>,
        asset_sub_class_any_of: Option<&'a str>,
        type_: Option<&'a str>,
        type_any_of: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FuturesProduct>;

    /// Same as [`Self::list_futures_products`], but takes the optional arguments as a
    /// chainable [`ListFuturesProductsParams`] struct.
    fn list_futures_products_with_params<'a>(
        &'a self,
        params: ListFuturesProductsParams,
    ) -> BoxStream<'a, FuturesProduct>;

    /// Get quotes for a futures contract in a given time range (paginated stream).
    fn list_futures_quotes<'a>(
        &'a self,
        ticker: &'a str,
        timestamp: Option<&'a str>,
        timestamp_lt: Option<&'a str>,
        timestamp_lte: Option<&'a str>,
        timestamp_gt: Option<&'a str>,
        timestamp_gte: Option<&'a str>,
        session_end_date: Option<&'a str>,
        session_end_date_lt: Option<&'a str>,
        session_end_date_lte: Option<&'a str>,
        session_end_date_gt: Option<&'a str>,
        session_end_date_gte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FuturesQuote>;

    /// Same as [`Self::list_futures_quotes`], but takes the optional arguments as a
    /// chainable [`ListFuturesQuotesParams`] struct.
    fn list_futures_quotes_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: ListFuturesQuotesParams,
    ) -> BoxStream<'a, FuturesQuote>;

    /// Get trades for a futures contract in a given time range (paginated stream).
    fn list_futures_trades<'a>(
        &'a self,
        ticker: &'a str,
        timestamp: Option<&'a str>,
        timestamp_lt: Option<&'a str>,
        timestamp_lte: Option<&'a str>,
        timestamp_gt: Option<&'a str>,
        timestamp_gte: Option<&'a str>,
        session_end_date: Option<&'a str>,
        session_end_date_lt: Option<&'a str>,
        session_end_date_lte: Option<&'a str>,
        session_end_date_gt: Option<&'a str>,
        session_end_date_gte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FuturesTrade>;

    /// Same as [`Self::list_futures_trades`], but takes the optional arguments as a
    /// chainable [`ListFuturesTradesParams`] struct.
    fn list_futures_trades_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: ListFuturesTradesParams,
    ) -> BoxStream<'a, FuturesTrade>;

    /// List trading schedules for futures products on a specific date (paginated stream).
    fn list_futures_schedules<'a>(
        &'a self,
        product_code: Option<&'a str>,
        product_code_any_of: Option<&'a str>,
        product_code_gt: Option<&'a str>,
        product_code_gte: Option<&'a str>,
        product_code_lt: Option<&'a str>,
        product_code_lte: Option<&'a str>,
        session_end_date: Option<&'a str>,
        session_end_date_gt: Option<&'a str>,
        session_end_date_gte: Option<&'a str>,
        session_end_date_lt: Option<&'a str>,
        session_end_date_lte: Option<&'a str>,
        trading_venue: Option<&'a str>,
        trading_venue_any_of: Option<&'a str>,
        trading_venue_gt: Option<&'a str>,
        trading_venue_gte: Option<&'a str>,
        trading_venue_lt: Option<&'a str>,
        trading_venue_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FuturesSchedule>;

    /// Same as [`Self::list_futures_schedules`], but takes the optional arguments as a
    /// chainable [`ListFuturesSchedulesParams`] struct.
    fn list_futures_schedules_with_params<'a>(
        &'a self,
        params: ListFuturesSchedulesParams,
    ) -> BoxStream<'a, FuturesSchedule>;

    /// List market statuses for futures products (paginated stream).
    fn list_futures_market_statuses<'a>(
        &'a self,
        product_code: Option<&'a str>,
        product_code_any_of: Option<&'a str>,
        product_code_gt: Option<&'a str>,
        product_code_gte: Option<&'a str>,
        product_code_lt: Option<&'a str>,
        product_code_lte: Option<&'a str>,
        limit: Option<i64>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FuturesMarketStatus>;

    /// Same as [`Self::list_futures_market_statuses`], but takes the optional arguments as a
    /// chainable [`ListFuturesMarketStatusesParams`] struct.
    fn list_futures_market_statuses_with_params<'a>(
        &'a self,
        params: ListFuturesMarketStatusesParams,
    ) -> BoxStream<'a, FuturesMarketStatus>;

    /// Get snapshots for futures contracts (paginated stream).
    fn get_futures_snapshot<'a>(
        &'a self,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        product_code: Option<&'a str>,
        product_code_any_of: Option<&'a str>,
        product_code_gt: Option<&'a str>,
        product_code_gte: Option<&'a str>,
        product_code_lt: Option<&'a str>,
        product_code_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FuturesSnapshot>;

    /// Same as [`Self::get_futures_snapshot`], but takes the optional arguments as a
    /// chainable [`GetFuturesSnapshotParams`] struct.
    fn get_futures_snapshot_with_params<'a>(
        &'a self,
        params: GetFuturesSnapshotParams,
    ) -> BoxStream<'a, FuturesSnapshot>;

    /// List US futures exchanges and trading venues (paginated stream).
    fn list_futures_exchanges<'a>(
        &'a self,
        limit: Option<i64>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FuturesExchange>;

    /// Same as [`Self::list_futures_exchanges`], but takes the optional arguments as a
    /// chainable [`ListFuturesExchangesParams`] struct.
    fn list_futures_exchanges_with_params<'a>(
        &'a self,
        params: ListFuturesExchangesParams,
    ) -> BoxStream<'a, FuturesExchange>;
}

impl FuturesApi for Client {
    fn list_futures_aggregates<'a>(
        &'a self,
        ticker: &'a str,
        resolution: Option<&'a str>,
        window_start: Option<&'a str>,
        window_start_lt: Option<&'a str>,
        window_start_lte: Option<&'a str>,
        window_start_gt: Option<&'a str>,
        window_start_gte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FuturesAgg> {
        self.list_futures_aggregates_with_params(
            ticker,
            ListFuturesAggregatesParams {
                resolution: resolution.map(String::from),
                window_start: window_start.map(String::from),
                window_start_lt: window_start_lt.map(String::from),
                window_start_lte: window_start_lte.map(String::from),
                window_start_gt: window_start_gt.map(String::from),
                window_start_gte: window_start_gte.map(String::from),
                limit,
                sort: sort.map(String::from),
                options: options.cloned(),
            },
        )
    }

    fn list_futures_aggregates_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: ListFuturesAggregatesParams,
    ) -> BoxStream<'a, FuturesAgg> {
        Box::pin({
            let ListFuturesAggregatesParams {
                resolution,
                window_start,
                window_start_lt,
                window_start_lte,
                window_start_gt,
                window_start_gte,
                limit,
                sort,
                options,
            } = params;
            let resolution = resolution.as_deref();
            let window_start = window_start.as_deref();
            let window_start_lt = window_start_lt.as_deref();
            let window_start_lte = window_start_lte.as_deref();
            let window_start_gt = window_start_gt.as_deref();
            let window_start_gte = window_start_gte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = format!("/futures/v1/aggs/{}", ticker);
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(r) = resolution {
                query.push(("resolution", r.to_string()));
            }
            if let Some(v) = window_start {
                query.push(("window_start", v.to_string()));
            }
            if let Some(v) = window_start_lt {
                query.push(("window_start.lt", v.to_string()));
            }
            if let Some(v) = window_start_lte {
                query.push(("window_start.lte", v.to_string()));
            }
            if let Some(v) = window_start_gt {
                query.push(("window_start.gt", v.to_string()));
            }
            if let Some(v) = window_start_gte {
                query.push(("window_start.gte", v.to_string()));
            }
            if let Some(l) = limit {
                query.push(("limit", l.to_string()));
            }
            if let Some(s) = sort {
                query.push(("sort", s.to_string()));
            }
            if self.pagination {
                self.paginate::<FuturesAgg>(&path, Some(&query), options)
            } else {
                self.single_page::<FuturesAgg>(&path, Some(&query), options)
            }
        })
    }

    fn list_futures_contracts<'a>(
        &'a self,
        date: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        product_code: Option<&'a str>,
        product_code_any_of: Option<&'a str>,
        product_code_gt: Option<&'a str>,
        product_code_gte: Option<&'a str>,
        product_code_lt: Option<&'a str>,
        product_code_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        active: Option<bool>,
        type_: Option<&'a str>,
        type_any_of: Option<&'a str>,
        first_trade_date: Option<&'a str>,
        first_trade_date_gt: Option<&'a str>,
        first_trade_date_gte: Option<&'a str>,
        first_trade_date_lt: Option<&'a str>,
        first_trade_date_lte: Option<&'a str>,
        last_trade_date: Option<&'a str>,
        last_trade_date_gt: Option<&'a str>,
        last_trade_date_gte: Option<&'a str>,
        last_trade_date_lt: Option<&'a str>,
        last_trade_date_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FuturesContract> {
        self.list_futures_contracts_with_params(ListFuturesContractsParams {
            date: date.map(String::from),
            date_gt: date_gt.map(String::from),
            date_gte: date_gte.map(String::from),
            date_lt: date_lt.map(String::from),
            date_lte: date_lte.map(String::from),
            product_code: product_code.map(String::from),
            product_code_any_of: product_code_any_of.map(String::from),
            product_code_gt: product_code_gt.map(String::from),
            product_code_gte: product_code_gte.map(String::from),
            product_code_lt: product_code_lt.map(String::from),
            product_code_lte: product_code_lte.map(String::from),
            ticker: ticker.map(String::from),
            ticker_any_of: ticker_any_of.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            active,
            type_: type_.map(String::from),
            type_any_of: type_any_of.map(String::from),
            first_trade_date: first_trade_date.map(String::from),
            first_trade_date_gt: first_trade_date_gt.map(String::from),
            first_trade_date_gte: first_trade_date_gte.map(String::from),
            first_trade_date_lt: first_trade_date_lt.map(String::from),
            first_trade_date_lte: first_trade_date_lte.map(String::from),
            last_trade_date: last_trade_date.map(String::from),
            last_trade_date_gt: last_trade_date_gt.map(String::from),
            last_trade_date_gte: last_trade_date_gte.map(String::from),
            last_trade_date_lt: last_trade_date_lt.map(String::from),
            last_trade_date_lte: last_trade_date_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_futures_contracts_with_params<'a>(
        &'a self,
        params: ListFuturesContractsParams,
    ) -> BoxStream<'a, FuturesContract> {
        Box::pin({
            let ListFuturesContractsParams {
                date,
                date_gt,
                date_gte,
                date_lt,
                date_lte,
                product_code,
                product_code_any_of,
                product_code_gt,
                product_code_gte,
                product_code_lt,
                product_code_lte,
                ticker,
                ticker_any_of,
                ticker_gt,
                ticker_gte,
                ticker_lt,
                ticker_lte,
                active,
                type_,
                type_any_of,
                first_trade_date,
                first_trade_date_gt,
                first_trade_date_gte,
                first_trade_date_lt,
                first_trade_date_lte,
                last_trade_date,
                last_trade_date_gt,
                last_trade_date_gte,
                last_trade_date_lt,
                last_trade_date_lte,
                limit,
                sort,
                options,
            } = params;
            let date = date.as_deref();
            let date_gt = date_gt.as_deref();
            let date_gte = date_gte.as_deref();
            let date_lt = date_lt.as_deref();
            let date_lte = date_lte.as_deref();
            let product_code = product_code.as_deref();
            let product_code_any_of = product_code_any_of.as_deref();
            let product_code_gt = product_code_gt.as_deref();
            let product_code_gte = product_code_gte.as_deref();
            let product_code_lt = product_code_lt.as_deref();
            let product_code_lte = product_code_lte.as_deref();
            let ticker = ticker.as_deref();
            let ticker_any_of = ticker_any_of.as_deref();
            let ticker_gt = ticker_gt.as_deref();
            let ticker_gte = ticker_gte.as_deref();
            let ticker_lt = ticker_lt.as_deref();
            let ticker_lte = ticker_lte.as_deref();
            let type_ = type_.as_deref();
            let type_any_of = type_any_of.as_deref();
            let first_trade_date = first_trade_date.as_deref();
            let first_trade_date_gt = first_trade_date_gt.as_deref();
            let first_trade_date_gte = first_trade_date_gte.as_deref();
            let first_trade_date_lt = first_trade_date_lt.as_deref();
            let first_trade_date_lte = first_trade_date_lte.as_deref();
            let last_trade_date = last_trade_date.as_deref();
            let last_trade_date_gt = last_trade_date_gt.as_deref();
            let last_trade_date_gte = last_trade_date_gte.as_deref();
            let last_trade_date_lt = last_trade_date_lt.as_deref();
            let last_trade_date_lte = last_trade_date_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/futures/v1/contracts";
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = date {
                query.push(("date", v.to_string()));
            }
            if let Some(v) = date_gt {
                query.push(("date.gt", v.to_string()));
            }
            if let Some(v) = date_gte {
                query.push(("date.gte", v.to_string()));
            }
            if let Some(v) = date_lt {
                query.push(("date.lt", v.to_string()));
            }
            if let Some(v) = date_lte {
                query.push(("date.lte", v.to_string()));
            }
            if let Some(v) = product_code {
                query.push(("product_code", v.to_string()));
            }
            if let Some(v) = product_code_any_of {
                query.push(("product_code.any_of", v.to_string()));
            }
            if let Some(v) = product_code_gt {
                query.push(("product_code.gt", v.to_string()));
            }
            if let Some(v) = product_code_gte {
                query.push(("product_code.gte", v.to_string()));
            }
            if let Some(v) = product_code_lt {
                query.push(("product_code.lt", v.to_string()));
            }
            if let Some(v) = product_code_lte {
                query.push(("product_code.lte", v.to_string()));
            }
            if let Some(v) = ticker {
                query.push(("ticker", v.to_string()));
            }
            if let Some(v) = ticker_any_of {
                query.push(("ticker.any_of", v.to_string()));
            }
            if let Some(v) = ticker_gt {
                query.push(("ticker.gt", v.to_string()));
            }
            if let Some(v) = ticker_gte {
                query.push(("ticker.gte", v.to_string()));
            }
            if let Some(v) = ticker_lt {
                query.push(("ticker.lt", v.to_string()));
            }
            if let Some(v) = ticker_lte {
                query.push(("ticker.lte", v.to_string()));
            }
            if let Some(a) = active {
                query.push(("active", a.to_string()));
            }
            if let Some(v) = type_ {
                query.push(("type", v.to_string()));
            }
            if let Some(v) = type_any_of {
                query.push(("type.any_of", v.to_string()));
            }
            if let Some(v) = first_trade_date {
                query.push(("first_trade_date", v.to_string()));
            }
            if let Some(v) = first_trade_date_gt {
                query.push(("first_trade_date.gt", v.to_string()));
            }
            if let Some(v) = first_trade_date_gte {
                query.push(("first_trade_date.gte", v.to_string()));
            }
            if let Some(v) = first_trade_date_lt {
                query.push(("first_trade_date.lt", v.to_string()));
            }
            if let Some(v) = first_trade_date_lte {
                query.push(("first_trade_date.lte", v.to_string()));
            }
            if let Some(v) = last_trade_date {
                query.push(("last_trade_date", v.to_string()));
            }
            if let Some(v) = last_trade_date_gt {
                query.push(("last_trade_date.gt", v.to_string()));
            }
            if let Some(v) = last_trade_date_gte {
                query.push(("last_trade_date.gte", v.to_string()));
            }
            if let Some(v) = last_trade_date_lt {
                query.push(("last_trade_date.lt", v.to_string()));
            }
            if let Some(v) = last_trade_date_lte {
                query.push(("last_trade_date.lte", v.to_string()));
            }
            if let Some(l) = limit {
                query.push(("limit", l.to_string()));
            }
            if let Some(s) = sort {
                query.push(("sort", s.to_string()));
            }
            if self.pagination {
                self.paginate::<FuturesContract>(path, Some(&query), options)
            } else {
                self.single_page::<FuturesContract>(path, Some(&query), options)
            }
        })
    }

    fn list_futures_products<'a>(
        &'a self,
        name: Option<&'a str>,
        name_any_of: Option<&'a str>,
        name_gt: Option<&'a str>,
        name_gte: Option<&'a str>,
        name_lt: Option<&'a str>,
        name_lte: Option<&'a str>,
        product_code: Option<&'a str>,
        product_code_any_of: Option<&'a str>,
        product_code_gt: Option<&'a str>,
        product_code_gte: Option<&'a str>,
        product_code_lt: Option<&'a str>,
        product_code_lte: Option<&'a str>,
        date: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        trading_venue: Option<&'a str>,
        trading_venue_any_of: Option<&'a str>,
        trading_venue_gt: Option<&'a str>,
        trading_venue_gte: Option<&'a str>,
        trading_venue_lt: Option<&'a str>,
        trading_venue_lte: Option<&'a str>,
        sector: Option<&'a str>,
        sector_any_of: Option<&'a str>,
        sub_sector: Option<&'a str>,
        sub_sector_any_of: Option<&'a str>,
        asset_class: Option<&'a str>,
        asset_class_any_of: Option<&'a str>,
        asset_sub_class: Option<&'a str>,
        asset_sub_class_any_of: Option<&'a str>,
        type_: Option<&'a str>,
        type_any_of: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FuturesProduct> {
        self.list_futures_products_with_params(ListFuturesProductsParams {
            name: name.map(String::from),
            name_any_of: name_any_of.map(String::from),
            name_gt: name_gt.map(String::from),
            name_gte: name_gte.map(String::from),
            name_lt: name_lt.map(String::from),
            name_lte: name_lte.map(String::from),
            product_code: product_code.map(String::from),
            product_code_any_of: product_code_any_of.map(String::from),
            product_code_gt: product_code_gt.map(String::from),
            product_code_gte: product_code_gte.map(String::from),
            product_code_lt: product_code_lt.map(String::from),
            product_code_lte: product_code_lte.map(String::from),
            date: date.map(String::from),
            date_gt: date_gt.map(String::from),
            date_gte: date_gte.map(String::from),
            date_lt: date_lt.map(String::from),
            date_lte: date_lte.map(String::from),
            trading_venue: trading_venue.map(String::from),
            trading_venue_any_of: trading_venue_any_of.map(String::from),
            trading_venue_gt: trading_venue_gt.map(String::from),
            trading_venue_gte: trading_venue_gte.map(String::from),
            trading_venue_lt: trading_venue_lt.map(String::from),
            trading_venue_lte: trading_venue_lte.map(String::from),
            sector: sector.map(String::from),
            sector_any_of: sector_any_of.map(String::from),
            sub_sector: sub_sector.map(String::from),
            sub_sector_any_of: sub_sector_any_of.map(String::from),
            asset_class: asset_class.map(String::from),
            asset_class_any_of: asset_class_any_of.map(String::from),
            asset_sub_class: asset_sub_class.map(String::from),
            asset_sub_class_any_of: asset_sub_class_any_of.map(String::from),
            type_: type_.map(String::from),
            type_any_of: type_any_of.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_futures_products_with_params<'a>(
        &'a self,
        params: ListFuturesProductsParams,
    ) -> BoxStream<'a, FuturesProduct> {
        Box::pin({
            let ListFuturesProductsParams {
                name,
                name_any_of,
                name_gt,
                name_gte,
                name_lt,
                name_lte,
                product_code,
                product_code_any_of,
                product_code_gt,
                product_code_gte,
                product_code_lt,
                product_code_lte,
                date,
                date_gt,
                date_gte,
                date_lt,
                date_lte,
                trading_venue,
                trading_venue_any_of,
                trading_venue_gt,
                trading_venue_gte,
                trading_venue_lt,
                trading_venue_lte,
                sector,
                sector_any_of,
                sub_sector,
                sub_sector_any_of,
                asset_class,
                asset_class_any_of,
                asset_sub_class,
                asset_sub_class_any_of,
                type_,
                type_any_of,
                limit,
                sort,
                options,
            } = params;
            let name = name.as_deref();
            let name_any_of = name_any_of.as_deref();
            let name_gt = name_gt.as_deref();
            let name_gte = name_gte.as_deref();
            let name_lt = name_lt.as_deref();
            let name_lte = name_lte.as_deref();
            let product_code = product_code.as_deref();
            let product_code_any_of = product_code_any_of.as_deref();
            let product_code_gt = product_code_gt.as_deref();
            let product_code_gte = product_code_gte.as_deref();
            let product_code_lt = product_code_lt.as_deref();
            let product_code_lte = product_code_lte.as_deref();
            let date = date.as_deref();
            let date_gt = date_gt.as_deref();
            let date_gte = date_gte.as_deref();
            let date_lt = date_lt.as_deref();
            let date_lte = date_lte.as_deref();
            let trading_venue = trading_venue.as_deref();
            let trading_venue_any_of = trading_venue_any_of.as_deref();
            let trading_venue_gt = trading_venue_gt.as_deref();
            let trading_venue_gte = trading_venue_gte.as_deref();
            let trading_venue_lt = trading_venue_lt.as_deref();
            let trading_venue_lte = trading_venue_lte.as_deref();
            let sector = sector.as_deref();
            let sector_any_of = sector_any_of.as_deref();
            let sub_sector = sub_sector.as_deref();
            let sub_sector_any_of = sub_sector_any_of.as_deref();
            let asset_class = asset_class.as_deref();
            let asset_class_any_of = asset_class_any_of.as_deref();
            let asset_sub_class = asset_sub_class.as_deref();
            let asset_sub_class_any_of = asset_sub_class_any_of.as_deref();
            let type_ = type_.as_deref();
            let type_any_of = type_any_of.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/futures/v1/products";
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = name {
                query.push(("name", v.to_string()));
            }
            if let Some(v) = name_any_of {
                query.push(("name.any_of", v.to_string()));
            }
            if let Some(v) = name_gt {
                query.push(("name.gt", v.to_string()));
            }
            if let Some(v) = name_gte {
                query.push(("name.gte", v.to_string()));
            }
            if let Some(v) = name_lt {
                query.push(("name.lt", v.to_string()));
            }
            if let Some(v) = name_lte {
                query.push(("name.lte", v.to_string()));
            }
            if let Some(v) = product_code {
                query.push(("product_code", v.to_string()));
            }
            if let Some(v) = product_code_any_of {
                query.push(("product_code.any_of", v.to_string()));
            }
            if let Some(v) = product_code_gt {
                query.push(("product_code.gt", v.to_string()));
            }
            if let Some(v) = product_code_gte {
                query.push(("product_code.gte", v.to_string()));
            }
            if let Some(v) = product_code_lt {
                query.push(("product_code.lt", v.to_string()));
            }
            if let Some(v) = product_code_lte {
                query.push(("product_code.lte", v.to_string()));
            }
            if let Some(v) = date {
                query.push(("date", v.to_string()));
            }
            if let Some(v) = date_gt {
                query.push(("date.gt", v.to_string()));
            }
            if let Some(v) = date_gte {
                query.push(("date.gte", v.to_string()));
            }
            if let Some(v) = date_lt {
                query.push(("date.lt", v.to_string()));
            }
            if let Some(v) = date_lte {
                query.push(("date.lte", v.to_string()));
            }
            if let Some(v) = trading_venue {
                query.push(("trading_venue", v.to_string()));
            }
            if let Some(v) = trading_venue_any_of {
                query.push(("trading_venue.any_of", v.to_string()));
            }
            if let Some(v) = trading_venue_gt {
                query.push(("trading_venue.gt", v.to_string()));
            }
            if let Some(v) = trading_venue_gte {
                query.push(("trading_venue.gte", v.to_string()));
            }
            if let Some(v) = trading_venue_lt {
                query.push(("trading_venue.lt", v.to_string()));
            }
            if let Some(v) = trading_venue_lte {
                query.push(("trading_venue.lte", v.to_string()));
            }
            if let Some(v) = sector {
                query.push(("sector", v.to_string()));
            }
            if let Some(v) = sector_any_of {
                query.push(("sector.any_of", v.to_string()));
            }
            if let Some(v) = sub_sector {
                query.push(("sub_sector", v.to_string()));
            }
            if let Some(v) = sub_sector_any_of {
                query.push(("sub_sector.any_of", v.to_string()));
            }
            if let Some(v) = asset_class {
                query.push(("asset_class", v.to_string()));
            }
            if let Some(v) = asset_class_any_of {
                query.push(("asset_class.any_of", v.to_string()));
            }
            if let Some(v) = asset_sub_class {
                query.push(("asset_sub_class", v.to_string()));
            }
            if let Some(v) = asset_sub_class_any_of {
                query.push(("asset_sub_class.any_of", v.to_string()));
            }
            if let Some(v) = type_ {
                query.push(("type", v.to_string()));
            }
            if let Some(v) = type_any_of {
                query.push(("type.any_of", v.to_string()));
            }
            if let Some(l) = limit {
                query.push(("limit", l.to_string()));
            }
            if let Some(s) = sort {
                query.push(("sort", s.to_string()));
            }
            if self.pagination {
                self.paginate::<FuturesProduct>(path, Some(&query), options)
            } else {
                self.single_page::<FuturesProduct>(path, Some(&query), options)
            }
        })
    }

    fn list_futures_quotes<'a>(
        &'a self,
        ticker: &'a str,
        timestamp: Option<&'a str>,
        timestamp_lt: Option<&'a str>,
        timestamp_lte: Option<&'a str>,
        timestamp_gt: Option<&'a str>,
        timestamp_gte: Option<&'a str>,
        session_end_date: Option<&'a str>,
        session_end_date_lt: Option<&'a str>,
        session_end_date_lte: Option<&'a str>,
        session_end_date_gt: Option<&'a str>,
        session_end_date_gte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FuturesQuote> {
        self.list_futures_quotes_with_params(
            ticker,
            ListFuturesQuotesParams {
                timestamp: timestamp.map(String::from),
                timestamp_lt: timestamp_lt.map(String::from),
                timestamp_lte: timestamp_lte.map(String::from),
                timestamp_gt: timestamp_gt.map(String::from),
                timestamp_gte: timestamp_gte.map(String::from),
                session_end_date: session_end_date.map(String::from),
                session_end_date_lt: session_end_date_lt.map(String::from),
                session_end_date_lte: session_end_date_lte.map(String::from),
                session_end_date_gt: session_end_date_gt.map(String::from),
                session_end_date_gte: session_end_date_gte.map(String::from),
                limit,
                sort: sort.map(String::from),
                options: options.cloned(),
            },
        )
    }

    fn list_futures_quotes_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: ListFuturesQuotesParams,
    ) -> BoxStream<'a, FuturesQuote> {
        Box::pin({
            let ListFuturesQuotesParams {
                timestamp,
                timestamp_lt,
                timestamp_lte,
                timestamp_gt,
                timestamp_gte,
                session_end_date,
                session_end_date_lt,
                session_end_date_lte,
                session_end_date_gt,
                session_end_date_gte,
                limit,
                sort,
                options,
            } = params;
            let timestamp = timestamp.as_deref();
            let timestamp_lt = timestamp_lt.as_deref();
            let timestamp_lte = timestamp_lte.as_deref();
            let timestamp_gt = timestamp_gt.as_deref();
            let timestamp_gte = timestamp_gte.as_deref();
            let session_end_date = session_end_date.as_deref();
            let session_end_date_lt = session_end_date_lt.as_deref();
            let session_end_date_lte = session_end_date_lte.as_deref();
            let session_end_date_gt = session_end_date_gt.as_deref();
            let session_end_date_gte = session_end_date_gte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = format!("/futures/v1/quotes/{}", ticker);
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = timestamp {
                query.push(("timestamp", v.to_string()));
            }
            if let Some(v) = timestamp_lt {
                query.push(("timestamp.lt", v.to_string()));
            }
            if let Some(v) = timestamp_lte {
                query.push(("timestamp.lte", v.to_string()));
            }
            if let Some(v) = timestamp_gt {
                query.push(("timestamp.gt", v.to_string()));
            }
            if let Some(v) = timestamp_gte {
                query.push(("timestamp.gte", v.to_string()));
            }
            if let Some(v) = session_end_date {
                query.push(("session_end_date", v.to_string()));
            }
            if let Some(v) = session_end_date_lt {
                query.push(("session_end_date.lt", v.to_string()));
            }
            if let Some(v) = session_end_date_lte {
                query.push(("session_end_date.lte", v.to_string()));
            }
            if let Some(v) = session_end_date_gt {
                query.push(("session_end_date.gt", v.to_string()));
            }
            if let Some(v) = session_end_date_gte {
                query.push(("session_end_date.gte", v.to_string()));
            }
            if let Some(l) = limit {
                query.push(("limit", l.to_string()));
            }
            if let Some(s) = sort {
                query.push(("sort", s.to_string()));
            }
            if self.pagination {
                self.paginate::<FuturesQuote>(&path, Some(&query), options)
            } else {
                self.single_page::<FuturesQuote>(&path, Some(&query), options)
            }
        })
    }

    fn list_futures_trades<'a>(
        &'a self,
        ticker: &'a str,
        timestamp: Option<&'a str>,
        timestamp_lt: Option<&'a str>,
        timestamp_lte: Option<&'a str>,
        timestamp_gt: Option<&'a str>,
        timestamp_gte: Option<&'a str>,
        session_end_date: Option<&'a str>,
        session_end_date_lt: Option<&'a str>,
        session_end_date_lte: Option<&'a str>,
        session_end_date_gt: Option<&'a str>,
        session_end_date_gte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FuturesTrade> {
        self.list_futures_trades_with_params(
            ticker,
            ListFuturesTradesParams {
                timestamp: timestamp.map(String::from),
                timestamp_lt: timestamp_lt.map(String::from),
                timestamp_lte: timestamp_lte.map(String::from),
                timestamp_gt: timestamp_gt.map(String::from),
                timestamp_gte: timestamp_gte.map(String::from),
                session_end_date: session_end_date.map(String::from),
                session_end_date_lt: session_end_date_lt.map(String::from),
                session_end_date_lte: session_end_date_lte.map(String::from),
                session_end_date_gt: session_end_date_gt.map(String::from),
                session_end_date_gte: session_end_date_gte.map(String::from),
                limit,
                sort: sort.map(String::from),
                options: options.cloned(),
            },
        )
    }

    fn list_futures_trades_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: ListFuturesTradesParams,
    ) -> BoxStream<'a, FuturesTrade> {
        Box::pin({
            let ListFuturesTradesParams {
                timestamp,
                timestamp_lt,
                timestamp_lte,
                timestamp_gt,
                timestamp_gte,
                session_end_date,
                session_end_date_lt,
                session_end_date_lte,
                session_end_date_gt,
                session_end_date_gte,
                limit,
                sort,
                options,
            } = params;
            let timestamp = timestamp.as_deref();
            let timestamp_lt = timestamp_lt.as_deref();
            let timestamp_lte = timestamp_lte.as_deref();
            let timestamp_gt = timestamp_gt.as_deref();
            let timestamp_gte = timestamp_gte.as_deref();
            let session_end_date = session_end_date.as_deref();
            let session_end_date_lt = session_end_date_lt.as_deref();
            let session_end_date_lte = session_end_date_lte.as_deref();
            let session_end_date_gt = session_end_date_gt.as_deref();
            let session_end_date_gte = session_end_date_gte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = format!("/futures/v1/trades/{}", ticker);
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = timestamp {
                query.push(("timestamp", v.to_string()));
            }
            if let Some(v) = timestamp_lt {
                query.push(("timestamp.lt", v.to_string()));
            }
            if let Some(v) = timestamp_lte {
                query.push(("timestamp.lte", v.to_string()));
            }
            if let Some(v) = timestamp_gt {
                query.push(("timestamp.gt", v.to_string()));
            }
            if let Some(v) = timestamp_gte {
                query.push(("timestamp.gte", v.to_string()));
            }
            if let Some(v) = session_end_date {
                query.push(("session_end_date", v.to_string()));
            }
            if let Some(v) = session_end_date_lt {
                query.push(("session_end_date.lt", v.to_string()));
            }
            if let Some(v) = session_end_date_lte {
                query.push(("session_end_date.lte", v.to_string()));
            }
            if let Some(v) = session_end_date_gt {
                query.push(("session_end_date.gt", v.to_string()));
            }
            if let Some(v) = session_end_date_gte {
                query.push(("session_end_date.gte", v.to_string()));
            }
            if let Some(l) = limit {
                query.push(("limit", l.to_string()));
            }
            if let Some(s) = sort {
                query.push(("sort", s.to_string()));
            }
            if self.pagination {
                self.paginate::<FuturesTrade>(&path, Some(&query), options)
            } else {
                self.single_page::<FuturesTrade>(&path, Some(&query), options)
            }
        })
    }

    fn list_futures_schedules<'a>(
        &'a self,
        product_code: Option<&'a str>,
        product_code_any_of: Option<&'a str>,
        product_code_gt: Option<&'a str>,
        product_code_gte: Option<&'a str>,
        product_code_lt: Option<&'a str>,
        product_code_lte: Option<&'a str>,
        session_end_date: Option<&'a str>,
        session_end_date_gt: Option<&'a str>,
        session_end_date_gte: Option<&'a str>,
        session_end_date_lt: Option<&'a str>,
        session_end_date_lte: Option<&'a str>,
        trading_venue: Option<&'a str>,
        trading_venue_any_of: Option<&'a str>,
        trading_venue_gt: Option<&'a str>,
        trading_venue_gte: Option<&'a str>,
        trading_venue_lt: Option<&'a str>,
        trading_venue_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FuturesSchedule> {
        self.list_futures_schedules_with_params(ListFuturesSchedulesParams {
            product_code: product_code.map(String::from),
            product_code_any_of: product_code_any_of.map(String::from),
            product_code_gt: product_code_gt.map(String::from),
            product_code_gte: product_code_gte.map(String::from),
            product_code_lt: product_code_lt.map(String::from),
            product_code_lte: product_code_lte.map(String::from),
            session_end_date: session_end_date.map(String::from),
            session_end_date_gt: session_end_date_gt.map(String::from),
            session_end_date_gte: session_end_date_gte.map(String::from),
            session_end_date_lt: session_end_date_lt.map(String::from),
            session_end_date_lte: session_end_date_lte.map(String::from),
            trading_venue: trading_venue.map(String::from),
            trading_venue_any_of: trading_venue_any_of.map(String::from),
            trading_venue_gt: trading_venue_gt.map(String::from),
            trading_venue_gte: trading_venue_gte.map(String::from),
            trading_venue_lt: trading_venue_lt.map(String::from),
            trading_venue_lte: trading_venue_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_futures_schedules_with_params<'a>(
        &'a self,
        params: ListFuturesSchedulesParams,
    ) -> BoxStream<'a, FuturesSchedule> {
        Box::pin({
            let ListFuturesSchedulesParams {
                product_code,
                product_code_any_of,
                product_code_gt,
                product_code_gte,
                product_code_lt,
                product_code_lte,
                session_end_date,
                session_end_date_gt,
                session_end_date_gte,
                session_end_date_lt,
                session_end_date_lte,
                trading_venue,
                trading_venue_any_of,
                trading_venue_gt,
                trading_venue_gte,
                trading_venue_lt,
                trading_venue_lte,
                limit,
                sort,
                options,
            } = params;
            let product_code = product_code.as_deref();
            let product_code_any_of = product_code_any_of.as_deref();
            let product_code_gt = product_code_gt.as_deref();
            let product_code_gte = product_code_gte.as_deref();
            let product_code_lt = product_code_lt.as_deref();
            let product_code_lte = product_code_lte.as_deref();
            let session_end_date = session_end_date.as_deref();
            let session_end_date_gt = session_end_date_gt.as_deref();
            let session_end_date_gte = session_end_date_gte.as_deref();
            let session_end_date_lt = session_end_date_lt.as_deref();
            let session_end_date_lte = session_end_date_lte.as_deref();
            let trading_venue = trading_venue.as_deref();
            let trading_venue_any_of = trading_venue_any_of.as_deref();
            let trading_venue_gt = trading_venue_gt.as_deref();
            let trading_venue_gte = trading_venue_gte.as_deref();
            let trading_venue_lt = trading_venue_lt.as_deref();
            let trading_venue_lte = trading_venue_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/futures/v1/schedules";
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = product_code {
                query.push(("product_code", v.to_string()));
            }
            if let Some(v) = product_code_any_of {
                query.push(("product_code.any_of", v.to_string()));
            }
            if let Some(v) = product_code_gt {
                query.push(("product_code.gt", v.to_string()));
            }
            if let Some(v) = product_code_gte {
                query.push(("product_code.gte", v.to_string()));
            }
            if let Some(v) = product_code_lt {
                query.push(("product_code.lt", v.to_string()));
            }
            if let Some(v) = product_code_lte {
                query.push(("product_code.lte", v.to_string()));
            }
            if let Some(v) = session_end_date {
                query.push(("session_end_date", v.to_string()));
            }
            if let Some(v) = session_end_date_gt {
                query.push(("session_end_date.gt", v.to_string()));
            }
            if let Some(v) = session_end_date_gte {
                query.push(("session_end_date.gte", v.to_string()));
            }
            if let Some(v) = session_end_date_lt {
                query.push(("session_end_date.lt", v.to_string()));
            }
            if let Some(v) = session_end_date_lte {
                query.push(("session_end_date.lte", v.to_string()));
            }
            if let Some(v) = trading_venue {
                query.push(("trading_venue", v.to_string()));
            }
            if let Some(v) = trading_venue_any_of {
                query.push(("trading_venue.any_of", v.to_string()));
            }
            if let Some(v) = trading_venue_gt {
                query.push(("trading_venue.gt", v.to_string()));
            }
            if let Some(v) = trading_venue_gte {
                query.push(("trading_venue.gte", v.to_string()));
            }
            if let Some(v) = trading_venue_lt {
                query.push(("trading_venue.lt", v.to_string()));
            }
            if let Some(v) = trading_venue_lte {
                query.push(("trading_venue.lte", v.to_string()));
            }
            if let Some(l) = limit {
                query.push(("limit", l.to_string()));
            }
            if let Some(s) = sort {
                query.push(("sort", s.to_string()));
            }
            if self.pagination {
                self.paginate::<FuturesSchedule>(path, Some(&query), options)
            } else {
                self.single_page::<FuturesSchedule>(path, Some(&query), options)
            }
        })
    }

    fn list_futures_market_statuses<'a>(
        &'a self,
        product_code: Option<&'a str>,
        product_code_any_of: Option<&'a str>,
        product_code_gt: Option<&'a str>,
        product_code_gte: Option<&'a str>,
        product_code_lt: Option<&'a str>,
        product_code_lte: Option<&'a str>,
        limit: Option<i64>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FuturesMarketStatus> {
        self.list_futures_market_statuses_with_params(ListFuturesMarketStatusesParams {
            product_code: product_code.map(String::from),
            product_code_any_of: product_code_any_of.map(String::from),
            product_code_gt: product_code_gt.map(String::from),
            product_code_gte: product_code_gte.map(String::from),
            product_code_lt: product_code_lt.map(String::from),
            product_code_lte: product_code_lte.map(String::from),
            limit,
            options: options.cloned(),
        })
    }

    fn list_futures_market_statuses_with_params<'a>(
        &'a self,
        params: ListFuturesMarketStatusesParams,
    ) -> BoxStream<'a, FuturesMarketStatus> {
        Box::pin({
            let ListFuturesMarketStatusesParams {
                product_code,
                product_code_any_of,
                product_code_gt,
                product_code_gte,
                product_code_lt,
                product_code_lte,
                limit,
                options,
            } = params;
            let product_code = product_code.as_deref();
            let product_code_any_of = product_code_any_of.as_deref();
            let product_code_gt = product_code_gt.as_deref();
            let product_code_gte = product_code_gte.as_deref();
            let product_code_lt = product_code_lt.as_deref();
            let product_code_lte = product_code_lte.as_deref();
            let options = options.as_ref();
            let path = "/futures/v1/market-status";
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = product_code {
                query.push(("product_code", v.to_string()));
            }
            if let Some(v) = product_code_any_of {
                query.push(("product_code.any_of", v.to_string()));
            }
            if let Some(v) = product_code_gt {
                query.push(("product_code.gt", v.to_string()));
            }
            if let Some(v) = product_code_gte {
                query.push(("product_code.gte", v.to_string()));
            }
            if let Some(v) = product_code_lt {
                query.push(("product_code.lt", v.to_string()));
            }
            if let Some(v) = product_code_lte {
                query.push(("product_code.lte", v.to_string()));
            }
            if let Some(l) = limit {
                query.push(("limit", l.to_string()));
            }
            if self.pagination {
                self.paginate::<FuturesMarketStatus>(path, Some(&query), options)
            } else {
                self.single_page::<FuturesMarketStatus>(path, Some(&query), options)
            }
        })
    }

    fn get_futures_snapshot<'a>(
        &'a self,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        product_code: Option<&'a str>,
        product_code_any_of: Option<&'a str>,
        product_code_gt: Option<&'a str>,
        product_code_gte: Option<&'a str>,
        product_code_lt: Option<&'a str>,
        product_code_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FuturesSnapshot> {
        self.get_futures_snapshot_with_params(GetFuturesSnapshotParams {
            ticker: ticker.map(String::from),
            ticker_any_of: ticker_any_of.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            product_code: product_code.map(String::from),
            product_code_any_of: product_code_any_of.map(String::from),
            product_code_gt: product_code_gt.map(String::from),
            product_code_gte: product_code_gte.map(String::from),
            product_code_lt: product_code_lt.map(String::from),
            product_code_lte: product_code_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn get_futures_snapshot_with_params<'a>(
        &'a self,
        params: GetFuturesSnapshotParams,
    ) -> BoxStream<'a, FuturesSnapshot> {
        Box::pin({
            let GetFuturesSnapshotParams {
                ticker,
                ticker_any_of,
                ticker_gt,
                ticker_gte,
                ticker_lt,
                ticker_lte,
                product_code,
                product_code_any_of,
                product_code_gt,
                product_code_gte,
                product_code_lt,
                product_code_lte,
                limit,
                sort,
                options,
            } = params;
            let ticker = ticker.as_deref();
            let ticker_any_of = ticker_any_of.as_deref();
            let ticker_gt = ticker_gt.as_deref();
            let ticker_gte = ticker_gte.as_deref();
            let ticker_lt = ticker_lt.as_deref();
            let ticker_lte = ticker_lte.as_deref();
            let product_code = product_code.as_deref();
            let product_code_any_of = product_code_any_of.as_deref();
            let product_code_gt = product_code_gt.as_deref();
            let product_code_gte = product_code_gte.as_deref();
            let product_code_lt = product_code_lt.as_deref();
            let product_code_lte = product_code_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/futures/v1/snapshot";
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = ticker {
                query.push(("ticker", v.to_string()));
            }
            if let Some(v) = ticker_any_of {
                query.push(("ticker.any_of", v.to_string()));
            }
            if let Some(v) = ticker_gt {
                query.push(("ticker.gt", v.to_string()));
            }
            if let Some(v) = ticker_gte {
                query.push(("ticker.gte", v.to_string()));
            }
            if let Some(v) = ticker_lt {
                query.push(("ticker.lt", v.to_string()));
            }
            if let Some(v) = ticker_lte {
                query.push(("ticker.lte", v.to_string()));
            }
            if let Some(v) = product_code {
                query.push(("product_code", v.to_string()));
            }
            if let Some(v) = product_code_any_of {
                query.push(("product_code.any_of", v.to_string()));
            }
            if let Some(v) = product_code_gt {
                query.push(("product_code.gt", v.to_string()));
            }
            if let Some(v) = product_code_gte {
                query.push(("product_code.gte", v.to_string()));
            }
            if let Some(v) = product_code_lt {
                query.push(("product_code.lt", v.to_string()));
            }
            if let Some(v) = product_code_lte {
                query.push(("product_code.lte", v.to_string()));
            }
            if let Some(l) = limit {
                query.push(("limit", l.to_string()));
            }
            if let Some(s) = sort {
                query.push(("sort", s.to_string()));
            }
            if self.pagination {
                self.paginate::<FuturesSnapshot>(path, Some(&query), options)
            } else {
                self.single_page::<FuturesSnapshot>(path, Some(&query), options)
            }
        })
    }

    fn list_futures_exchanges<'a>(
        &'a self,
        limit: Option<i64>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FuturesExchange> {
        self.list_futures_exchanges_with_params(ListFuturesExchangesParams {
            limit,
            options: options.cloned(),
        })
    }

    fn list_futures_exchanges_with_params<'a>(
        &'a self,
        params: ListFuturesExchangesParams,
    ) -> BoxStream<'a, FuturesExchange> {
        Box::pin({
            let ListFuturesExchangesParams { limit, options } = params;
            let options = options.as_ref();
            let path = "/futures/v1/exchanges";
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(l) = limit {
                query.push(("limit", l.to_string()));
            }
            if self.pagination {
                self.paginate::<FuturesExchange>(path, Some(&query), options)
            } else {
                self.single_page::<FuturesExchange>(path, Some(&query), options)
            }
        })
    }
}

// --- Params structs (additive builder API) ---

/// Optional arguments for [`FuturesApi::list_futures_aggregates`].
#[derive(Debug, Default, Clone)]
pub struct ListFuturesAggregatesParams {
    /// The `resolution` argument.
    pub resolution: Option<String>,
    /// The `window_start` argument.
    pub window_start: Option<String>,
    /// The `window_start_lt` argument.
    pub window_start_lt: Option<String>,
    /// The `window_start_lte` argument.
    pub window_start_lte: Option<String>,
    /// The `window_start_gt` argument.
    pub window_start_gt: Option<String>,
    /// The `window_start_gte` argument.
    pub window_start_gte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListFuturesAggregatesParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `resolution` argument.
    pub fn resolution(mut self, resolution: impl Into<String>) -> Self {
        self.resolution = Some(resolution.into());
        self
    }

    /// Set the `window_start` argument.
    pub fn window_start(mut self, window_start: impl Into<String>) -> Self {
        self.window_start = Some(window_start.into());
        self
    }

    /// Set the `window_start_lt` argument.
    pub fn window_start_lt(mut self, window_start_lt: impl Into<String>) -> Self {
        self.window_start_lt = Some(window_start_lt.into());
        self
    }

    /// Set the `window_start_lte` argument.
    pub fn window_start_lte(mut self, window_start_lte: impl Into<String>) -> Self {
        self.window_start_lte = Some(window_start_lte.into());
        self
    }

    /// Set the `window_start_gt` argument.
    pub fn window_start_gt(mut self, window_start_gt: impl Into<String>) -> Self {
        self.window_start_gt = Some(window_start_gt.into());
        self
    }

    /// Set the `window_start_gte` argument.
    pub fn window_start_gte(mut self, window_start_gte: impl Into<String>) -> Self {
        self.window_start_gte = Some(window_start_gte.into());
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

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`FuturesApi::list_futures_contracts`].
#[derive(Debug, Default, Clone)]
pub struct ListFuturesContractsParams {
    /// The `date` argument.
    pub date: Option<String>,
    /// The `date_gt` argument.
    pub date_gt: Option<String>,
    /// The `date_gte` argument.
    pub date_gte: Option<String>,
    /// The `date_lt` argument.
    pub date_lt: Option<String>,
    /// The `date_lte` argument.
    pub date_lte: Option<String>,
    /// The `product_code` argument.
    pub product_code: Option<String>,
    /// The `product_code_any_of` argument.
    pub product_code_any_of: Option<String>,
    /// The `product_code_gt` argument.
    pub product_code_gt: Option<String>,
    /// The `product_code_gte` argument.
    pub product_code_gte: Option<String>,
    /// The `product_code_lt` argument.
    pub product_code_lt: Option<String>,
    /// The `product_code_lte` argument.
    pub product_code_lte: Option<String>,
    /// The `ticker` argument.
    pub ticker: Option<String>,
    /// The `ticker_any_of` argument.
    pub ticker_any_of: Option<String>,
    /// The `ticker_gt` argument.
    pub ticker_gt: Option<String>,
    /// The `ticker_gte` argument.
    pub ticker_gte: Option<String>,
    /// The `ticker_lt` argument.
    pub ticker_lt: Option<String>,
    /// The `ticker_lte` argument.
    pub ticker_lte: Option<String>,
    /// The `active` argument.
    pub active: Option<bool>,
    /// The `type_` argument.
    pub type_: Option<String>,
    /// The `type_any_of` argument.
    pub type_any_of: Option<String>,
    /// The `first_trade_date` argument.
    pub first_trade_date: Option<String>,
    /// The `first_trade_date_gt` argument.
    pub first_trade_date_gt: Option<String>,
    /// The `first_trade_date_gte` argument.
    pub first_trade_date_gte: Option<String>,
    /// The `first_trade_date_lt` argument.
    pub first_trade_date_lt: Option<String>,
    /// The `first_trade_date_lte` argument.
    pub first_trade_date_lte: Option<String>,
    /// The `last_trade_date` argument.
    pub last_trade_date: Option<String>,
    /// The `last_trade_date_gt` argument.
    pub last_trade_date_gt: Option<String>,
    /// The `last_trade_date_gte` argument.
    pub last_trade_date_gte: Option<String>,
    /// The `last_trade_date_lt` argument.
    pub last_trade_date_lt: Option<String>,
    /// The `last_trade_date_lte` argument.
    pub last_trade_date_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListFuturesContractsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `date` argument.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }

    /// Set the `date_gt` argument.
    pub fn date_gt(mut self, date_gt: impl Into<String>) -> Self {
        self.date_gt = Some(date_gt.into());
        self
    }

    /// Set the `date_gte` argument.
    pub fn date_gte(mut self, date_gte: impl Into<String>) -> Self {
        self.date_gte = Some(date_gte.into());
        self
    }

    /// Set the `date_lt` argument.
    pub fn date_lt(mut self, date_lt: impl Into<String>) -> Self {
        self.date_lt = Some(date_lt.into());
        self
    }

    /// Set the `date_lte` argument.
    pub fn date_lte(mut self, date_lte: impl Into<String>) -> Self {
        self.date_lte = Some(date_lte.into());
        self
    }

    /// Set the `product_code` argument.
    pub fn product_code(mut self, product_code: impl Into<String>) -> Self {
        self.product_code = Some(product_code.into());
        self
    }

    /// Set the `product_code_any_of` argument.
    pub fn product_code_any_of(mut self, product_code_any_of: impl Into<String>) -> Self {
        self.product_code_any_of = Some(product_code_any_of.into());
        self
    }

    /// Set the `product_code_gt` argument.
    pub fn product_code_gt(mut self, product_code_gt: impl Into<String>) -> Self {
        self.product_code_gt = Some(product_code_gt.into());
        self
    }

    /// Set the `product_code_gte` argument.
    pub fn product_code_gte(mut self, product_code_gte: impl Into<String>) -> Self {
        self.product_code_gte = Some(product_code_gte.into());
        self
    }

    /// Set the `product_code_lt` argument.
    pub fn product_code_lt(mut self, product_code_lt: impl Into<String>) -> Self {
        self.product_code_lt = Some(product_code_lt.into());
        self
    }

    /// Set the `product_code_lte` argument.
    pub fn product_code_lte(mut self, product_code_lte: impl Into<String>) -> Self {
        self.product_code_lte = Some(product_code_lte.into());
        self
    }

    /// Set the `ticker` argument.
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    /// Set the `ticker_any_of` argument.
    pub fn ticker_any_of(mut self, ticker_any_of: impl Into<String>) -> Self {
        self.ticker_any_of = Some(ticker_any_of.into());
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

    /// Set the `active` argument.
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    /// Set the `type_` argument.
    pub fn type_(mut self, type_: impl Into<String>) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    /// Set the `type_any_of` argument.
    pub fn type_any_of(mut self, type_any_of: impl Into<String>) -> Self {
        self.type_any_of = Some(type_any_of.into());
        self
    }

    /// Set the `first_trade_date` argument.
    pub fn first_trade_date(mut self, first_trade_date: impl Into<String>) -> Self {
        self.first_trade_date = Some(first_trade_date.into());
        self
    }

    /// Set the `first_trade_date_gt` argument.
    pub fn first_trade_date_gt(mut self, first_trade_date_gt: impl Into<String>) -> Self {
        self.first_trade_date_gt = Some(first_trade_date_gt.into());
        self
    }

    /// Set the `first_trade_date_gte` argument.
    pub fn first_trade_date_gte(mut self, first_trade_date_gte: impl Into<String>) -> Self {
        self.first_trade_date_gte = Some(first_trade_date_gte.into());
        self
    }

    /// Set the `first_trade_date_lt` argument.
    pub fn first_trade_date_lt(mut self, first_trade_date_lt: impl Into<String>) -> Self {
        self.first_trade_date_lt = Some(first_trade_date_lt.into());
        self
    }

    /// Set the `first_trade_date_lte` argument.
    pub fn first_trade_date_lte(mut self, first_trade_date_lte: impl Into<String>) -> Self {
        self.first_trade_date_lte = Some(first_trade_date_lte.into());
        self
    }

    /// Set the `last_trade_date` argument.
    pub fn last_trade_date(mut self, last_trade_date: impl Into<String>) -> Self {
        self.last_trade_date = Some(last_trade_date.into());
        self
    }

    /// Set the `last_trade_date_gt` argument.
    pub fn last_trade_date_gt(mut self, last_trade_date_gt: impl Into<String>) -> Self {
        self.last_trade_date_gt = Some(last_trade_date_gt.into());
        self
    }

    /// Set the `last_trade_date_gte` argument.
    pub fn last_trade_date_gte(mut self, last_trade_date_gte: impl Into<String>) -> Self {
        self.last_trade_date_gte = Some(last_trade_date_gte.into());
        self
    }

    /// Set the `last_trade_date_lt` argument.
    pub fn last_trade_date_lt(mut self, last_trade_date_lt: impl Into<String>) -> Self {
        self.last_trade_date_lt = Some(last_trade_date_lt.into());
        self
    }

    /// Set the `last_trade_date_lte` argument.
    pub fn last_trade_date_lte(mut self, last_trade_date_lte: impl Into<String>) -> Self {
        self.last_trade_date_lte = Some(last_trade_date_lte.into());
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

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`FuturesApi::list_futures_products`].
#[derive(Debug, Default, Clone)]
pub struct ListFuturesProductsParams {
    /// The `name` argument.
    pub name: Option<String>,
    /// The `name_any_of` argument.
    pub name_any_of: Option<String>,
    /// The `name_gt` argument.
    pub name_gt: Option<String>,
    /// The `name_gte` argument.
    pub name_gte: Option<String>,
    /// The `name_lt` argument.
    pub name_lt: Option<String>,
    /// The `name_lte` argument.
    pub name_lte: Option<String>,
    /// The `product_code` argument.
    pub product_code: Option<String>,
    /// The `product_code_any_of` argument.
    pub product_code_any_of: Option<String>,
    /// The `product_code_gt` argument.
    pub product_code_gt: Option<String>,
    /// The `product_code_gte` argument.
    pub product_code_gte: Option<String>,
    /// The `product_code_lt` argument.
    pub product_code_lt: Option<String>,
    /// The `product_code_lte` argument.
    pub product_code_lte: Option<String>,
    /// The `date` argument.
    pub date: Option<String>,
    /// The `date_gt` argument.
    pub date_gt: Option<String>,
    /// The `date_gte` argument.
    pub date_gte: Option<String>,
    /// The `date_lt` argument.
    pub date_lt: Option<String>,
    /// The `date_lte` argument.
    pub date_lte: Option<String>,
    /// The `trading_venue` argument.
    pub trading_venue: Option<String>,
    /// The `trading_venue_any_of` argument.
    pub trading_venue_any_of: Option<String>,
    /// The `trading_venue_gt` argument.
    pub trading_venue_gt: Option<String>,
    /// The `trading_venue_gte` argument.
    pub trading_venue_gte: Option<String>,
    /// The `trading_venue_lt` argument.
    pub trading_venue_lt: Option<String>,
    /// The `trading_venue_lte` argument.
    pub trading_venue_lte: Option<String>,
    /// The `sector` argument.
    pub sector: Option<String>,
    /// The `sector_any_of` argument.
    pub sector_any_of: Option<String>,
    /// The `sub_sector` argument.
    pub sub_sector: Option<String>,
    /// The `sub_sector_any_of` argument.
    pub sub_sector_any_of: Option<String>,
    /// The `asset_class` argument.
    pub asset_class: Option<String>,
    /// The `asset_class_any_of` argument.
    pub asset_class_any_of: Option<String>,
    /// The `asset_sub_class` argument.
    pub asset_sub_class: Option<String>,
    /// The `asset_sub_class_any_of` argument.
    pub asset_sub_class_any_of: Option<String>,
    /// The `type_` argument.
    pub type_: Option<String>,
    /// The `type_any_of` argument.
    pub type_any_of: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListFuturesProductsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `name` argument.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the `name_any_of` argument.
    pub fn name_any_of(mut self, name_any_of: impl Into<String>) -> Self {
        self.name_any_of = Some(name_any_of.into());
        self
    }

    /// Set the `name_gt` argument.
    pub fn name_gt(mut self, name_gt: impl Into<String>) -> Self {
        self.name_gt = Some(name_gt.into());
        self
    }

    /// Set the `name_gte` argument.
    pub fn name_gte(mut self, name_gte: impl Into<String>) -> Self {
        self.name_gte = Some(name_gte.into());
        self
    }

    /// Set the `name_lt` argument.
    pub fn name_lt(mut self, name_lt: impl Into<String>) -> Self {
        self.name_lt = Some(name_lt.into());
        self
    }

    /// Set the `name_lte` argument.
    pub fn name_lte(mut self, name_lte: impl Into<String>) -> Self {
        self.name_lte = Some(name_lte.into());
        self
    }

    /// Set the `product_code` argument.
    pub fn product_code(mut self, product_code: impl Into<String>) -> Self {
        self.product_code = Some(product_code.into());
        self
    }

    /// Set the `product_code_any_of` argument.
    pub fn product_code_any_of(mut self, product_code_any_of: impl Into<String>) -> Self {
        self.product_code_any_of = Some(product_code_any_of.into());
        self
    }

    /// Set the `product_code_gt` argument.
    pub fn product_code_gt(mut self, product_code_gt: impl Into<String>) -> Self {
        self.product_code_gt = Some(product_code_gt.into());
        self
    }

    /// Set the `product_code_gte` argument.
    pub fn product_code_gte(mut self, product_code_gte: impl Into<String>) -> Self {
        self.product_code_gte = Some(product_code_gte.into());
        self
    }

    /// Set the `product_code_lt` argument.
    pub fn product_code_lt(mut self, product_code_lt: impl Into<String>) -> Self {
        self.product_code_lt = Some(product_code_lt.into());
        self
    }

    /// Set the `product_code_lte` argument.
    pub fn product_code_lte(mut self, product_code_lte: impl Into<String>) -> Self {
        self.product_code_lte = Some(product_code_lte.into());
        self
    }

    /// Set the `date` argument.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }

    /// Set the `date_gt` argument.
    pub fn date_gt(mut self, date_gt: impl Into<String>) -> Self {
        self.date_gt = Some(date_gt.into());
        self
    }

    /// Set the `date_gte` argument.
    pub fn date_gte(mut self, date_gte: impl Into<String>) -> Self {
        self.date_gte = Some(date_gte.into());
        self
    }

    /// Set the `date_lt` argument.
    pub fn date_lt(mut self, date_lt: impl Into<String>) -> Self {
        self.date_lt = Some(date_lt.into());
        self
    }

    /// Set the `date_lte` argument.
    pub fn date_lte(mut self, date_lte: impl Into<String>) -> Self {
        self.date_lte = Some(date_lte.into());
        self
    }

    /// Set the `trading_venue` argument.
    pub fn trading_venue(mut self, trading_venue: impl Into<String>) -> Self {
        self.trading_venue = Some(trading_venue.into());
        self
    }

    /// Set the `trading_venue_any_of` argument.
    pub fn trading_venue_any_of(mut self, trading_venue_any_of: impl Into<String>) -> Self {
        self.trading_venue_any_of = Some(trading_venue_any_of.into());
        self
    }

    /// Set the `trading_venue_gt` argument.
    pub fn trading_venue_gt(mut self, trading_venue_gt: impl Into<String>) -> Self {
        self.trading_venue_gt = Some(trading_venue_gt.into());
        self
    }

    /// Set the `trading_venue_gte` argument.
    pub fn trading_venue_gte(mut self, trading_venue_gte: impl Into<String>) -> Self {
        self.trading_venue_gte = Some(trading_venue_gte.into());
        self
    }

    /// Set the `trading_venue_lt` argument.
    pub fn trading_venue_lt(mut self, trading_venue_lt: impl Into<String>) -> Self {
        self.trading_venue_lt = Some(trading_venue_lt.into());
        self
    }

    /// Set the `trading_venue_lte` argument.
    pub fn trading_venue_lte(mut self, trading_venue_lte: impl Into<String>) -> Self {
        self.trading_venue_lte = Some(trading_venue_lte.into());
        self
    }

    /// Set the `sector` argument.
    pub fn sector(mut self, sector: impl Into<String>) -> Self {
        self.sector = Some(sector.into());
        self
    }

    /// Set the `sector_any_of` argument.
    pub fn sector_any_of(mut self, sector_any_of: impl Into<String>) -> Self {
        self.sector_any_of = Some(sector_any_of.into());
        self
    }

    /// Set the `sub_sector` argument.
    pub fn sub_sector(mut self, sub_sector: impl Into<String>) -> Self {
        self.sub_sector = Some(sub_sector.into());
        self
    }

    /// Set the `sub_sector_any_of` argument.
    pub fn sub_sector_any_of(mut self, sub_sector_any_of: impl Into<String>) -> Self {
        self.sub_sector_any_of = Some(sub_sector_any_of.into());
        self
    }

    /// Set the `asset_class` argument.
    pub fn asset_class(mut self, asset_class: impl Into<String>) -> Self {
        self.asset_class = Some(asset_class.into());
        self
    }

    /// Set the `asset_class_any_of` argument.
    pub fn asset_class_any_of(mut self, asset_class_any_of: impl Into<String>) -> Self {
        self.asset_class_any_of = Some(asset_class_any_of.into());
        self
    }

    /// Set the `asset_sub_class` argument.
    pub fn asset_sub_class(mut self, asset_sub_class: impl Into<String>) -> Self {
        self.asset_sub_class = Some(asset_sub_class.into());
        self
    }

    /// Set the `asset_sub_class_any_of` argument.
    pub fn asset_sub_class_any_of(mut self, asset_sub_class_any_of: impl Into<String>) -> Self {
        self.asset_sub_class_any_of = Some(asset_sub_class_any_of.into());
        self
    }

    /// Set the `type_` argument.
    pub fn type_(mut self, type_: impl Into<String>) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    /// Set the `type_any_of` argument.
    pub fn type_any_of(mut self, type_any_of: impl Into<String>) -> Self {
        self.type_any_of = Some(type_any_of.into());
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

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`FuturesApi::list_futures_quotes`].
#[derive(Debug, Default, Clone)]
pub struct ListFuturesQuotesParams {
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
    /// The `session_end_date` argument.
    pub session_end_date: Option<String>,
    /// The `session_end_date_lt` argument.
    pub session_end_date_lt: Option<String>,
    /// The `session_end_date_lte` argument.
    pub session_end_date_lte: Option<String>,
    /// The `session_end_date_gt` argument.
    pub session_end_date_gt: Option<String>,
    /// The `session_end_date_gte` argument.
    pub session_end_date_gte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListFuturesQuotesParams {
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

    /// Set the `session_end_date` argument.
    pub fn session_end_date(mut self, session_end_date: impl Into<String>) -> Self {
        self.session_end_date = Some(session_end_date.into());
        self
    }

    /// Set the `session_end_date_lt` argument.
    pub fn session_end_date_lt(mut self, session_end_date_lt: impl Into<String>) -> Self {
        self.session_end_date_lt = Some(session_end_date_lt.into());
        self
    }

    /// Set the `session_end_date_lte` argument.
    pub fn session_end_date_lte(mut self, session_end_date_lte: impl Into<String>) -> Self {
        self.session_end_date_lte = Some(session_end_date_lte.into());
        self
    }

    /// Set the `session_end_date_gt` argument.
    pub fn session_end_date_gt(mut self, session_end_date_gt: impl Into<String>) -> Self {
        self.session_end_date_gt = Some(session_end_date_gt.into());
        self
    }

    /// Set the `session_end_date_gte` argument.
    pub fn session_end_date_gte(mut self, session_end_date_gte: impl Into<String>) -> Self {
        self.session_end_date_gte = Some(session_end_date_gte.into());
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

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`FuturesApi::list_futures_trades`].
#[derive(Debug, Default, Clone)]
pub struct ListFuturesTradesParams {
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
    /// The `session_end_date` argument.
    pub session_end_date: Option<String>,
    /// The `session_end_date_lt` argument.
    pub session_end_date_lt: Option<String>,
    /// The `session_end_date_lte` argument.
    pub session_end_date_lte: Option<String>,
    /// The `session_end_date_gt` argument.
    pub session_end_date_gt: Option<String>,
    /// The `session_end_date_gte` argument.
    pub session_end_date_gte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListFuturesTradesParams {
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

    /// Set the `session_end_date` argument.
    pub fn session_end_date(mut self, session_end_date: impl Into<String>) -> Self {
        self.session_end_date = Some(session_end_date.into());
        self
    }

    /// Set the `session_end_date_lt` argument.
    pub fn session_end_date_lt(mut self, session_end_date_lt: impl Into<String>) -> Self {
        self.session_end_date_lt = Some(session_end_date_lt.into());
        self
    }

    /// Set the `session_end_date_lte` argument.
    pub fn session_end_date_lte(mut self, session_end_date_lte: impl Into<String>) -> Self {
        self.session_end_date_lte = Some(session_end_date_lte.into());
        self
    }

    /// Set the `session_end_date_gt` argument.
    pub fn session_end_date_gt(mut self, session_end_date_gt: impl Into<String>) -> Self {
        self.session_end_date_gt = Some(session_end_date_gt.into());
        self
    }

    /// Set the `session_end_date_gte` argument.
    pub fn session_end_date_gte(mut self, session_end_date_gte: impl Into<String>) -> Self {
        self.session_end_date_gte = Some(session_end_date_gte.into());
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

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`FuturesApi::list_futures_schedules`].
#[derive(Debug, Default, Clone)]
pub struct ListFuturesSchedulesParams {
    /// The `product_code` argument.
    pub product_code: Option<String>,
    /// The `product_code_any_of` argument.
    pub product_code_any_of: Option<String>,
    /// The `product_code_gt` argument.
    pub product_code_gt: Option<String>,
    /// The `product_code_gte` argument.
    pub product_code_gte: Option<String>,
    /// The `product_code_lt` argument.
    pub product_code_lt: Option<String>,
    /// The `product_code_lte` argument.
    pub product_code_lte: Option<String>,
    /// The `session_end_date` argument.
    pub session_end_date: Option<String>,
    /// The `session_end_date_gt` argument.
    pub session_end_date_gt: Option<String>,
    /// The `session_end_date_gte` argument.
    pub session_end_date_gte: Option<String>,
    /// The `session_end_date_lt` argument.
    pub session_end_date_lt: Option<String>,
    /// The `session_end_date_lte` argument.
    pub session_end_date_lte: Option<String>,
    /// The `trading_venue` argument.
    pub trading_venue: Option<String>,
    /// The `trading_venue_any_of` argument.
    pub trading_venue_any_of: Option<String>,
    /// The `trading_venue_gt` argument.
    pub trading_venue_gt: Option<String>,
    /// The `trading_venue_gte` argument.
    pub trading_venue_gte: Option<String>,
    /// The `trading_venue_lt` argument.
    pub trading_venue_lt: Option<String>,
    /// The `trading_venue_lte` argument.
    pub trading_venue_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListFuturesSchedulesParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `product_code` argument.
    pub fn product_code(mut self, product_code: impl Into<String>) -> Self {
        self.product_code = Some(product_code.into());
        self
    }

    /// Set the `product_code_any_of` argument.
    pub fn product_code_any_of(mut self, product_code_any_of: impl Into<String>) -> Self {
        self.product_code_any_of = Some(product_code_any_of.into());
        self
    }

    /// Set the `product_code_gt` argument.
    pub fn product_code_gt(mut self, product_code_gt: impl Into<String>) -> Self {
        self.product_code_gt = Some(product_code_gt.into());
        self
    }

    /// Set the `product_code_gte` argument.
    pub fn product_code_gte(mut self, product_code_gte: impl Into<String>) -> Self {
        self.product_code_gte = Some(product_code_gte.into());
        self
    }

    /// Set the `product_code_lt` argument.
    pub fn product_code_lt(mut self, product_code_lt: impl Into<String>) -> Self {
        self.product_code_lt = Some(product_code_lt.into());
        self
    }

    /// Set the `product_code_lte` argument.
    pub fn product_code_lte(mut self, product_code_lte: impl Into<String>) -> Self {
        self.product_code_lte = Some(product_code_lte.into());
        self
    }

    /// Set the `session_end_date` argument.
    pub fn session_end_date(mut self, session_end_date: impl Into<String>) -> Self {
        self.session_end_date = Some(session_end_date.into());
        self
    }

    /// Set the `session_end_date_gt` argument.
    pub fn session_end_date_gt(mut self, session_end_date_gt: impl Into<String>) -> Self {
        self.session_end_date_gt = Some(session_end_date_gt.into());
        self
    }

    /// Set the `session_end_date_gte` argument.
    pub fn session_end_date_gte(mut self, session_end_date_gte: impl Into<String>) -> Self {
        self.session_end_date_gte = Some(session_end_date_gte.into());
        self
    }

    /// Set the `session_end_date_lt` argument.
    pub fn session_end_date_lt(mut self, session_end_date_lt: impl Into<String>) -> Self {
        self.session_end_date_lt = Some(session_end_date_lt.into());
        self
    }

    /// Set the `session_end_date_lte` argument.
    pub fn session_end_date_lte(mut self, session_end_date_lte: impl Into<String>) -> Self {
        self.session_end_date_lte = Some(session_end_date_lte.into());
        self
    }

    /// Set the `trading_venue` argument.
    pub fn trading_venue(mut self, trading_venue: impl Into<String>) -> Self {
        self.trading_venue = Some(trading_venue.into());
        self
    }

    /// Set the `trading_venue_any_of` argument.
    pub fn trading_venue_any_of(mut self, trading_venue_any_of: impl Into<String>) -> Self {
        self.trading_venue_any_of = Some(trading_venue_any_of.into());
        self
    }

    /// Set the `trading_venue_gt` argument.
    pub fn trading_venue_gt(mut self, trading_venue_gt: impl Into<String>) -> Self {
        self.trading_venue_gt = Some(trading_venue_gt.into());
        self
    }

    /// Set the `trading_venue_gte` argument.
    pub fn trading_venue_gte(mut self, trading_venue_gte: impl Into<String>) -> Self {
        self.trading_venue_gte = Some(trading_venue_gte.into());
        self
    }

    /// Set the `trading_venue_lt` argument.
    pub fn trading_venue_lt(mut self, trading_venue_lt: impl Into<String>) -> Self {
        self.trading_venue_lt = Some(trading_venue_lt.into());
        self
    }

    /// Set the `trading_venue_lte` argument.
    pub fn trading_venue_lte(mut self, trading_venue_lte: impl Into<String>) -> Self {
        self.trading_venue_lte = Some(trading_venue_lte.into());
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

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`FuturesApi::list_futures_market_statuses`].
#[derive(Debug, Default, Clone)]
pub struct ListFuturesMarketStatusesParams {
    /// The `product_code` argument.
    pub product_code: Option<String>,
    /// The `product_code_any_of` argument.
    pub product_code_any_of: Option<String>,
    /// The `product_code_gt` argument.
    pub product_code_gt: Option<String>,
    /// The `product_code_gte` argument.
    pub product_code_gte: Option<String>,
    /// The `product_code_lt` argument.
    pub product_code_lt: Option<String>,
    /// The `product_code_lte` argument.
    pub product_code_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListFuturesMarketStatusesParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `product_code` argument.
    pub fn product_code(mut self, product_code: impl Into<String>) -> Self {
        self.product_code = Some(product_code.into());
        self
    }

    /// Set the `product_code_any_of` argument.
    pub fn product_code_any_of(mut self, product_code_any_of: impl Into<String>) -> Self {
        self.product_code_any_of = Some(product_code_any_of.into());
        self
    }

    /// Set the `product_code_gt` argument.
    pub fn product_code_gt(mut self, product_code_gt: impl Into<String>) -> Self {
        self.product_code_gt = Some(product_code_gt.into());
        self
    }

    /// Set the `product_code_gte` argument.
    pub fn product_code_gte(mut self, product_code_gte: impl Into<String>) -> Self {
        self.product_code_gte = Some(product_code_gte.into());
        self
    }

    /// Set the `product_code_lt` argument.
    pub fn product_code_lt(mut self, product_code_lt: impl Into<String>) -> Self {
        self.product_code_lt = Some(product_code_lt.into());
        self
    }

    /// Set the `product_code_lte` argument.
    pub fn product_code_lte(mut self, product_code_lte: impl Into<String>) -> Self {
        self.product_code_lte = Some(product_code_lte.into());
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

/// Optional arguments for [`FuturesApi::get_futures_snapshot`].
#[derive(Debug, Default, Clone)]
pub struct GetFuturesSnapshotParams {
    /// The `ticker` argument.
    pub ticker: Option<String>,
    /// The `ticker_any_of` argument.
    pub ticker_any_of: Option<String>,
    /// The `ticker_gt` argument.
    pub ticker_gt: Option<String>,
    /// The `ticker_gte` argument.
    pub ticker_gte: Option<String>,
    /// The `ticker_lt` argument.
    pub ticker_lt: Option<String>,
    /// The `ticker_lte` argument.
    pub ticker_lte: Option<String>,
    /// The `product_code` argument.
    pub product_code: Option<String>,
    /// The `product_code_any_of` argument.
    pub product_code_any_of: Option<String>,
    /// The `product_code_gt` argument.
    pub product_code_gt: Option<String>,
    /// The `product_code_gte` argument.
    pub product_code_gte: Option<String>,
    /// The `product_code_lt` argument.
    pub product_code_lt: Option<String>,
    /// The `product_code_lte` argument.
    pub product_code_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetFuturesSnapshotParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `ticker` argument.
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    /// Set the `ticker_any_of` argument.
    pub fn ticker_any_of(mut self, ticker_any_of: impl Into<String>) -> Self {
        self.ticker_any_of = Some(ticker_any_of.into());
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

    /// Set the `product_code` argument.
    pub fn product_code(mut self, product_code: impl Into<String>) -> Self {
        self.product_code = Some(product_code.into());
        self
    }

    /// Set the `product_code_any_of` argument.
    pub fn product_code_any_of(mut self, product_code_any_of: impl Into<String>) -> Self {
        self.product_code_any_of = Some(product_code_any_of.into());
        self
    }

    /// Set the `product_code_gt` argument.
    pub fn product_code_gt(mut self, product_code_gt: impl Into<String>) -> Self {
        self.product_code_gt = Some(product_code_gt.into());
        self
    }

    /// Set the `product_code_gte` argument.
    pub fn product_code_gte(mut self, product_code_gte: impl Into<String>) -> Self {
        self.product_code_gte = Some(product_code_gte.into());
        self
    }

    /// Set the `product_code_lt` argument.
    pub fn product_code_lt(mut self, product_code_lt: impl Into<String>) -> Self {
        self.product_code_lt = Some(product_code_lt.into());
        self
    }

    /// Set the `product_code_lte` argument.
    pub fn product_code_lte(mut self, product_code_lte: impl Into<String>) -> Self {
        self.product_code_lte = Some(product_code_lte.into());
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

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`FuturesApi::list_futures_exchanges`].
#[derive(Debug, Default, Clone)]
pub struct ListFuturesExchangesParams {
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListFuturesExchangesParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
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
