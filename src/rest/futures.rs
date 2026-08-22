use super::{encode_query, BoxStream};
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
            let path = format!("/futures/v1/aggs/{}", ticker);
            let query = encode_query(&params);
            self.list::<FuturesAgg>(&path, &query, params.options.as_ref())
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
            let path = "/futures/v1/contracts";
            let query = encode_query(&params);
            self.list::<FuturesContract>(path, &query, params.options.as_ref())
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
            let path = "/futures/v1/products";
            let query = encode_query(&params);
            self.list::<FuturesProduct>(path, &query, params.options.as_ref())
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
            let path = format!("/futures/v1/quotes/{}", ticker);
            let query = encode_query(&params);
            self.list::<FuturesQuote>(&path, &query, params.options.as_ref())
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
            let path = format!("/futures/v1/trades/{}", ticker);
            let query = encode_query(&params);
            self.list::<FuturesTrade>(&path, &query, params.options.as_ref())
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
            let path = "/futures/v1/schedules";
            let query = encode_query(&params);
            self.list::<FuturesSchedule>(path, &query, params.options.as_ref())
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
            let path = "/futures/v1/market-status";
            let query = encode_query(&params);
            self.list::<FuturesMarketStatus>(path, &query, params.options.as_ref())
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
            let path = "/futures/v1/snapshot";
            let query = encode_query(&params);
            self.list::<FuturesSnapshot>(path, &query, params.options.as_ref())
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
            let path = "/futures/v1/exchanges";
            let query = encode_query(&params);
            self.list::<FuturesExchange>(path, &query, params.options.as_ref())
        })
    }
}

// --- Params structs (additive builder API) ---
//
// Query serialization is derived: field order is wire order, `rename` carries
// dotted filter operators, unset fields are omitted, and `options` is skipped.

/// Optional arguments for [`FuturesApi::list_futures_aggregates`].
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ListFuturesAggregatesParams {
    /// The `resolution` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// The `window_start` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_start: Option<String>,
    /// The `window_start_lt` argument.
    #[serde(rename = "window_start.lt", skip_serializing_if = "Option::is_none")]
    pub window_start_lt: Option<String>,
    /// The `window_start_lte` argument.
    #[serde(rename = "window_start.lte", skip_serializing_if = "Option::is_none")]
    pub window_start_lte: Option<String>,
    /// The `window_start_gt` argument.
    #[serde(rename = "window_start.gt", skip_serializing_if = "Option::is_none")]
    pub window_start_gt: Option<String>,
    /// The `window_start_gte` argument.
    #[serde(rename = "window_start.gte", skip_serializing_if = "Option::is_none")]
    pub window_start_gte: Option<String>,
    /// The `limit` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The `sort` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// The `options` argument.
    #[serde(skip)]
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
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ListFuturesContractsParams {
    /// The `date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// The `date_gt` argument.
    #[serde(rename = "date.gt", skip_serializing_if = "Option::is_none")]
    pub date_gt: Option<String>,
    /// The `date_gte` argument.
    #[serde(rename = "date.gte", skip_serializing_if = "Option::is_none")]
    pub date_gte: Option<String>,
    /// The `date_lt` argument.
    #[serde(rename = "date.lt", skip_serializing_if = "Option::is_none")]
    pub date_lt: Option<String>,
    /// The `date_lte` argument.
    #[serde(rename = "date.lte", skip_serializing_if = "Option::is_none")]
    pub date_lte: Option<String>,
    /// The `product_code` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    /// The `product_code_any_of` argument.
    #[serde(rename = "product_code.any_of", skip_serializing_if = "Option::is_none")]
    pub product_code_any_of: Option<String>,
    /// The `product_code_gt` argument.
    #[serde(rename = "product_code.gt", skip_serializing_if = "Option::is_none")]
    pub product_code_gt: Option<String>,
    /// The `product_code_gte` argument.
    #[serde(rename = "product_code.gte", skip_serializing_if = "Option::is_none")]
    pub product_code_gte: Option<String>,
    /// The `product_code_lt` argument.
    #[serde(rename = "product_code.lt", skip_serializing_if = "Option::is_none")]
    pub product_code_lt: Option<String>,
    /// The `product_code_lte` argument.
    #[serde(rename = "product_code.lte", skip_serializing_if = "Option::is_none")]
    pub product_code_lte: Option<String>,
    /// The `ticker` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// The `ticker_any_of` argument.
    #[serde(rename = "ticker.any_of", skip_serializing_if = "Option::is_none")]
    pub ticker_any_of: Option<String>,
    /// The `ticker_gt` argument.
    #[serde(rename = "ticker.gt", skip_serializing_if = "Option::is_none")]
    pub ticker_gt: Option<String>,
    /// The `ticker_gte` argument.
    #[serde(rename = "ticker.gte", skip_serializing_if = "Option::is_none")]
    pub ticker_gte: Option<String>,
    /// The `ticker_lt` argument.
    #[serde(rename = "ticker.lt", skip_serializing_if = "Option::is_none")]
    pub ticker_lt: Option<String>,
    /// The `ticker_lte` argument.
    #[serde(rename = "ticker.lte", skip_serializing_if = "Option::is_none")]
    pub ticker_lte: Option<String>,
    /// The `active` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// The `type_` argument.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// The `type_any_of` argument.
    #[serde(rename = "type.any_of", skip_serializing_if = "Option::is_none")]
    pub type_any_of: Option<String>,
    /// The `first_trade_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_trade_date: Option<String>,
    /// The `first_trade_date_gt` argument.
    #[serde(rename = "first_trade_date.gt", skip_serializing_if = "Option::is_none")]
    pub first_trade_date_gt: Option<String>,
    /// The `first_trade_date_gte` argument.
    #[serde(rename = "first_trade_date.gte", skip_serializing_if = "Option::is_none")]
    pub first_trade_date_gte: Option<String>,
    /// The `first_trade_date_lt` argument.
    #[serde(rename = "first_trade_date.lt", skip_serializing_if = "Option::is_none")]
    pub first_trade_date_lt: Option<String>,
    /// The `first_trade_date_lte` argument.
    #[serde(rename = "first_trade_date.lte", skip_serializing_if = "Option::is_none")]
    pub first_trade_date_lte: Option<String>,
    /// The `last_trade_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_trade_date: Option<String>,
    /// The `last_trade_date_gt` argument.
    #[serde(rename = "last_trade_date.gt", skip_serializing_if = "Option::is_none")]
    pub last_trade_date_gt: Option<String>,
    /// The `last_trade_date_gte` argument.
    #[serde(rename = "last_trade_date.gte", skip_serializing_if = "Option::is_none")]
    pub last_trade_date_gte: Option<String>,
    /// The `last_trade_date_lt` argument.
    #[serde(rename = "last_trade_date.lt", skip_serializing_if = "Option::is_none")]
    pub last_trade_date_lt: Option<String>,
    /// The `last_trade_date_lte` argument.
    #[serde(rename = "last_trade_date.lte", skip_serializing_if = "Option::is_none")]
    pub last_trade_date_lte: Option<String>,
    /// The `limit` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The `sort` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// The `options` argument.
    #[serde(skip)]
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
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ListFuturesProductsParams {
    /// The `name` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The `name_any_of` argument.
    #[serde(rename = "name.any_of", skip_serializing_if = "Option::is_none")]
    pub name_any_of: Option<String>,
    /// The `name_gt` argument.
    #[serde(rename = "name.gt", skip_serializing_if = "Option::is_none")]
    pub name_gt: Option<String>,
    /// The `name_gte` argument.
    #[serde(rename = "name.gte", skip_serializing_if = "Option::is_none")]
    pub name_gte: Option<String>,
    /// The `name_lt` argument.
    #[serde(rename = "name.lt", skip_serializing_if = "Option::is_none")]
    pub name_lt: Option<String>,
    /// The `name_lte` argument.
    #[serde(rename = "name.lte", skip_serializing_if = "Option::is_none")]
    pub name_lte: Option<String>,
    /// The `product_code` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    /// The `product_code_any_of` argument.
    #[serde(rename = "product_code.any_of", skip_serializing_if = "Option::is_none")]
    pub product_code_any_of: Option<String>,
    /// The `product_code_gt` argument.
    #[serde(rename = "product_code.gt", skip_serializing_if = "Option::is_none")]
    pub product_code_gt: Option<String>,
    /// The `product_code_gte` argument.
    #[serde(rename = "product_code.gte", skip_serializing_if = "Option::is_none")]
    pub product_code_gte: Option<String>,
    /// The `product_code_lt` argument.
    #[serde(rename = "product_code.lt", skip_serializing_if = "Option::is_none")]
    pub product_code_lt: Option<String>,
    /// The `product_code_lte` argument.
    #[serde(rename = "product_code.lte", skip_serializing_if = "Option::is_none")]
    pub product_code_lte: Option<String>,
    /// The `date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// The `date_gt` argument.
    #[serde(rename = "date.gt", skip_serializing_if = "Option::is_none")]
    pub date_gt: Option<String>,
    /// The `date_gte` argument.
    #[serde(rename = "date.gte", skip_serializing_if = "Option::is_none")]
    pub date_gte: Option<String>,
    /// The `date_lt` argument.
    #[serde(rename = "date.lt", skip_serializing_if = "Option::is_none")]
    pub date_lt: Option<String>,
    /// The `date_lte` argument.
    #[serde(rename = "date.lte", skip_serializing_if = "Option::is_none")]
    pub date_lte: Option<String>,
    /// The `trading_venue` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_venue: Option<String>,
    /// The `trading_venue_any_of` argument.
    #[serde(rename = "trading_venue.any_of", skip_serializing_if = "Option::is_none")]
    pub trading_venue_any_of: Option<String>,
    /// The `trading_venue_gt` argument.
    #[serde(rename = "trading_venue.gt", skip_serializing_if = "Option::is_none")]
    pub trading_venue_gt: Option<String>,
    /// The `trading_venue_gte` argument.
    #[serde(rename = "trading_venue.gte", skip_serializing_if = "Option::is_none")]
    pub trading_venue_gte: Option<String>,
    /// The `trading_venue_lt` argument.
    #[serde(rename = "trading_venue.lt", skip_serializing_if = "Option::is_none")]
    pub trading_venue_lt: Option<String>,
    /// The `trading_venue_lte` argument.
    #[serde(rename = "trading_venue.lte", skip_serializing_if = "Option::is_none")]
    pub trading_venue_lte: Option<String>,
    /// The `sector` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sector: Option<String>,
    /// The `sector_any_of` argument.
    #[serde(rename = "sector.any_of", skip_serializing_if = "Option::is_none")]
    pub sector_any_of: Option<String>,
    /// The `sub_sector` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_sector: Option<String>,
    /// The `sub_sector_any_of` argument.
    #[serde(rename = "sub_sector.any_of", skip_serializing_if = "Option::is_none")]
    pub sub_sector_any_of: Option<String>,
    /// The `asset_class` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<String>,
    /// The `asset_class_any_of` argument.
    #[serde(rename = "asset_class.any_of", skip_serializing_if = "Option::is_none")]
    pub asset_class_any_of: Option<String>,
    /// The `asset_sub_class` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_sub_class: Option<String>,
    /// The `asset_sub_class_any_of` argument.
    #[serde(rename = "asset_sub_class.any_of", skip_serializing_if = "Option::is_none")]
    pub asset_sub_class_any_of: Option<String>,
    /// The `type_` argument.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// The `type_any_of` argument.
    #[serde(rename = "type.any_of", skip_serializing_if = "Option::is_none")]
    pub type_any_of: Option<String>,
    /// The `limit` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The `sort` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// The `options` argument.
    #[serde(skip)]
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
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ListFuturesQuotesParams {
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
    /// The `session_end_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_end_date: Option<String>,
    /// The `session_end_date_lt` argument.
    #[serde(rename = "session_end_date.lt", skip_serializing_if = "Option::is_none")]
    pub session_end_date_lt: Option<String>,
    /// The `session_end_date_lte` argument.
    #[serde(rename = "session_end_date.lte", skip_serializing_if = "Option::is_none")]
    pub session_end_date_lte: Option<String>,
    /// The `session_end_date_gt` argument.
    #[serde(rename = "session_end_date.gt", skip_serializing_if = "Option::is_none")]
    pub session_end_date_gt: Option<String>,
    /// The `session_end_date_gte` argument.
    #[serde(rename = "session_end_date.gte", skip_serializing_if = "Option::is_none")]
    pub session_end_date_gte: Option<String>,
    /// The `limit` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The `sort` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// The `options` argument.
    #[serde(skip)]
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
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ListFuturesTradesParams {
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
    /// The `session_end_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_end_date: Option<String>,
    /// The `session_end_date_lt` argument.
    #[serde(rename = "session_end_date.lt", skip_serializing_if = "Option::is_none")]
    pub session_end_date_lt: Option<String>,
    /// The `session_end_date_lte` argument.
    #[serde(rename = "session_end_date.lte", skip_serializing_if = "Option::is_none")]
    pub session_end_date_lte: Option<String>,
    /// The `session_end_date_gt` argument.
    #[serde(rename = "session_end_date.gt", skip_serializing_if = "Option::is_none")]
    pub session_end_date_gt: Option<String>,
    /// The `session_end_date_gte` argument.
    #[serde(rename = "session_end_date.gte", skip_serializing_if = "Option::is_none")]
    pub session_end_date_gte: Option<String>,
    /// The `limit` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The `sort` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// The `options` argument.
    #[serde(skip)]
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
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ListFuturesSchedulesParams {
    /// The `product_code` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    /// The `product_code_any_of` argument.
    #[serde(rename = "product_code.any_of", skip_serializing_if = "Option::is_none")]
    pub product_code_any_of: Option<String>,
    /// The `product_code_gt` argument.
    #[serde(rename = "product_code.gt", skip_serializing_if = "Option::is_none")]
    pub product_code_gt: Option<String>,
    /// The `product_code_gte` argument.
    #[serde(rename = "product_code.gte", skip_serializing_if = "Option::is_none")]
    pub product_code_gte: Option<String>,
    /// The `product_code_lt` argument.
    #[serde(rename = "product_code.lt", skip_serializing_if = "Option::is_none")]
    pub product_code_lt: Option<String>,
    /// The `product_code_lte` argument.
    #[serde(rename = "product_code.lte", skip_serializing_if = "Option::is_none")]
    pub product_code_lte: Option<String>,
    /// The `session_end_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_end_date: Option<String>,
    /// The `session_end_date_gt` argument.
    #[serde(rename = "session_end_date.gt", skip_serializing_if = "Option::is_none")]
    pub session_end_date_gt: Option<String>,
    /// The `session_end_date_gte` argument.
    #[serde(rename = "session_end_date.gte", skip_serializing_if = "Option::is_none")]
    pub session_end_date_gte: Option<String>,
    /// The `session_end_date_lt` argument.
    #[serde(rename = "session_end_date.lt", skip_serializing_if = "Option::is_none")]
    pub session_end_date_lt: Option<String>,
    /// The `session_end_date_lte` argument.
    #[serde(rename = "session_end_date.lte", skip_serializing_if = "Option::is_none")]
    pub session_end_date_lte: Option<String>,
    /// The `trading_venue` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_venue: Option<String>,
    /// The `trading_venue_any_of` argument.
    #[serde(rename = "trading_venue.any_of", skip_serializing_if = "Option::is_none")]
    pub trading_venue_any_of: Option<String>,
    /// The `trading_venue_gt` argument.
    #[serde(rename = "trading_venue.gt", skip_serializing_if = "Option::is_none")]
    pub trading_venue_gt: Option<String>,
    /// The `trading_venue_gte` argument.
    #[serde(rename = "trading_venue.gte", skip_serializing_if = "Option::is_none")]
    pub trading_venue_gte: Option<String>,
    /// The `trading_venue_lt` argument.
    #[serde(rename = "trading_venue.lt", skip_serializing_if = "Option::is_none")]
    pub trading_venue_lt: Option<String>,
    /// The `trading_venue_lte` argument.
    #[serde(rename = "trading_venue.lte", skip_serializing_if = "Option::is_none")]
    pub trading_venue_lte: Option<String>,
    /// The `limit` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The `sort` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// The `options` argument.
    #[serde(skip)]
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
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ListFuturesMarketStatusesParams {
    /// The `product_code` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    /// The `product_code_any_of` argument.
    #[serde(rename = "product_code.any_of", skip_serializing_if = "Option::is_none")]
    pub product_code_any_of: Option<String>,
    /// The `product_code_gt` argument.
    #[serde(rename = "product_code.gt", skip_serializing_if = "Option::is_none")]
    pub product_code_gt: Option<String>,
    /// The `product_code_gte` argument.
    #[serde(rename = "product_code.gte", skip_serializing_if = "Option::is_none")]
    pub product_code_gte: Option<String>,
    /// The `product_code_lt` argument.
    #[serde(rename = "product_code.lt", skip_serializing_if = "Option::is_none")]
    pub product_code_lt: Option<String>,
    /// The `product_code_lte` argument.
    #[serde(rename = "product_code.lte", skip_serializing_if = "Option::is_none")]
    pub product_code_lte: Option<String>,
    /// The `limit` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The `options` argument.
    #[serde(skip)]
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
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct GetFuturesSnapshotParams {
    /// The `ticker` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// The `ticker_any_of` argument.
    #[serde(rename = "ticker.any_of", skip_serializing_if = "Option::is_none")]
    pub ticker_any_of: Option<String>,
    /// The `ticker_gt` argument.
    #[serde(rename = "ticker.gt", skip_serializing_if = "Option::is_none")]
    pub ticker_gt: Option<String>,
    /// The `ticker_gte` argument.
    #[serde(rename = "ticker.gte", skip_serializing_if = "Option::is_none")]
    pub ticker_gte: Option<String>,
    /// The `ticker_lt` argument.
    #[serde(rename = "ticker.lt", skip_serializing_if = "Option::is_none")]
    pub ticker_lt: Option<String>,
    /// The `ticker_lte` argument.
    #[serde(rename = "ticker.lte", skip_serializing_if = "Option::is_none")]
    pub ticker_lte: Option<String>,
    /// The `product_code` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    /// The `product_code_any_of` argument.
    #[serde(rename = "product_code.any_of", skip_serializing_if = "Option::is_none")]
    pub product_code_any_of: Option<String>,
    /// The `product_code_gt` argument.
    #[serde(rename = "product_code.gt", skip_serializing_if = "Option::is_none")]
    pub product_code_gt: Option<String>,
    /// The `product_code_gte` argument.
    #[serde(rename = "product_code.gte", skip_serializing_if = "Option::is_none")]
    pub product_code_gte: Option<String>,
    /// The `product_code_lt` argument.
    #[serde(rename = "product_code.lt", skip_serializing_if = "Option::is_none")]
    pub product_code_lt: Option<String>,
    /// The `product_code_lte` argument.
    #[serde(rename = "product_code.lte", skip_serializing_if = "Option::is_none")]
    pub product_code_lte: Option<String>,
    /// The `limit` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The `sort` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// The `options` argument.
    #[serde(skip)]
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
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ListFuturesExchangesParams {
    /// The `limit` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The `options` argument.
    #[serde(skip)]
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
