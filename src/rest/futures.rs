use crate::client::{Client, RequestOptions};
use crate::error::Result;
use crate::models::{
    FuturesAgg, FuturesContract, FuturesExchange, FuturesMarketStatus, FuturesProduct,
    FuturesQuote, FuturesSchedule, FuturesSnapshot, FuturesTrade,
};
use futures::Stream;

/// Futures API.
pub trait FuturesApi {
    /// Get aggregates for a futures contract in a given time range (paginated stream).
    fn list_futures_aggregates(
        &self,
        ticker: &str,
        resolution: Option<&str>,
        window_start: Option<&str>,
        window_start_lt: Option<&str>,
        window_start_lte: Option<&str>,
        window_start_gt: Option<&str>,
        window_start_gte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FuturesAgg>>;

    /// List futures contracts (paginated stream).
    fn list_futures_contracts(
        &self,
        date: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        product_code: Option<&str>,
        product_code_any_of: Option<&str>,
        product_code_gt: Option<&str>,
        product_code_gte: Option<&str>,
        product_code_lt: Option<&str>,
        product_code_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        active: Option<bool>,
        type_: Option<&str>,
        type_any_of: Option<&str>,
        first_trade_date: Option<&str>,
        first_trade_date_gt: Option<&str>,
        first_trade_date_gte: Option<&str>,
        first_trade_date_lt: Option<&str>,
        first_trade_date_lte: Option<&str>,
        last_trade_date: Option<&str>,
        last_trade_date_gt: Option<&str>,
        last_trade_date_gte: Option<&str>,
        last_trade_date_lt: Option<&str>,
        last_trade_date_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FuturesContract>>;

    /// List futures products, including combos (paginated stream).
    fn list_futures_products(
        &self,
        name: Option<&str>,
        name_any_of: Option<&str>,
        name_gt: Option<&str>,
        name_gte: Option<&str>,
        name_lt: Option<&str>,
        name_lte: Option<&str>,
        product_code: Option<&str>,
        product_code_any_of: Option<&str>,
        product_code_gt: Option<&str>,
        product_code_gte: Option<&str>,
        product_code_lt: Option<&str>,
        product_code_lte: Option<&str>,
        date: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        trading_venue: Option<&str>,
        trading_venue_any_of: Option<&str>,
        trading_venue_gt: Option<&str>,
        trading_venue_gte: Option<&str>,
        trading_venue_lt: Option<&str>,
        trading_venue_lte: Option<&str>,
        sector: Option<&str>,
        sector_any_of: Option<&str>,
        sub_sector: Option<&str>,
        sub_sector_any_of: Option<&str>,
        asset_class: Option<&str>,
        asset_class_any_of: Option<&str>,
        asset_sub_class: Option<&str>,
        asset_sub_class_any_of: Option<&str>,
        type_: Option<&str>,
        type_any_of: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FuturesProduct>>;

    /// Get quotes for a futures contract in a given time range (paginated stream).
    fn list_futures_quotes(
        &self,
        ticker: &str,
        timestamp: Option<&str>,
        timestamp_lt: Option<&str>,
        timestamp_lte: Option<&str>,
        timestamp_gt: Option<&str>,
        timestamp_gte: Option<&str>,
        session_end_date: Option<&str>,
        session_end_date_lt: Option<&str>,
        session_end_date_lte: Option<&str>,
        session_end_date_gt: Option<&str>,
        session_end_date_gte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FuturesQuote>>;

    /// Get trades for a futures contract in a given time range (paginated stream).
    fn list_futures_trades(
        &self,
        ticker: &str,
        timestamp: Option<&str>,
        timestamp_lt: Option<&str>,
        timestamp_lte: Option<&str>,
        timestamp_gt: Option<&str>,
        timestamp_gte: Option<&str>,
        session_end_date: Option<&str>,
        session_end_date_lt: Option<&str>,
        session_end_date_lte: Option<&str>,
        session_end_date_gt: Option<&str>,
        session_end_date_gte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FuturesTrade>>;

    /// List trading schedules for futures products on a specific date (paginated stream).
    fn list_futures_schedules(
        &self,
        product_code: Option<&str>,
        product_code_any_of: Option<&str>,
        product_code_gt: Option<&str>,
        product_code_gte: Option<&str>,
        product_code_lt: Option<&str>,
        product_code_lte: Option<&str>,
        session_end_date: Option<&str>,
        session_end_date_gt: Option<&str>,
        session_end_date_gte: Option<&str>,
        session_end_date_lt: Option<&str>,
        session_end_date_lte: Option<&str>,
        trading_venue: Option<&str>,
        trading_venue_any_of: Option<&str>,
        trading_venue_gt: Option<&str>,
        trading_venue_gte: Option<&str>,
        trading_venue_lt: Option<&str>,
        trading_venue_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FuturesSchedule>>;

    /// List market statuses for futures products (paginated stream).
    fn list_futures_market_statuses(
        &self,
        product_code: Option<&str>,
        product_code_any_of: Option<&str>,
        product_code_gt: Option<&str>,
        product_code_gte: Option<&str>,
        product_code_lt: Option<&str>,
        product_code_lte: Option<&str>,
        limit: Option<i64>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FuturesMarketStatus>>;

    /// Get snapshots for futures contracts (paginated stream).
    fn get_futures_snapshot(
        &self,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        product_code: Option<&str>,
        product_code_any_of: Option<&str>,
        product_code_gt: Option<&str>,
        product_code_gte: Option<&str>,
        product_code_lt: Option<&str>,
        product_code_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FuturesSnapshot>>;

    /// List US futures exchanges and trading venues (paginated stream).
    fn list_futures_exchanges(
        &self,
        limit: Option<i64>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FuturesExchange>>;
}

impl FuturesApi for Client {
    fn list_futures_aggregates(
        &self,
        ticker: &str,
        resolution: Option<&str>,
        window_start: Option<&str>,
        window_start_lt: Option<&str>,
        window_start_lte: Option<&str>,
        window_start_gt: Option<&str>,
        window_start_gte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FuturesAgg>> {
        let path = format!("/futures/v1/aggs/{}", ticker);
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(r) = resolution {
            params.push(("resolution", r.to_string()));
        }
        if let Some(v) = window_start {
            params.push(("window_start", v.to_string()));
        }
        if let Some(v) = window_start_lt {
            params.push(("window_start.lt", v.to_string()));
        }
        if let Some(v) = window_start_lte {
            params.push(("window_start.lte", v.to_string()));
        }
        if let Some(v) = window_start_gt {
            params.push(("window_start.gt", v.to_string()));
        }
        if let Some(v) = window_start_gte {
            params.push(("window_start.gte", v.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if let Some(s) = sort {
            params.push(("sort", s.to_string()));
        }
        if self.pagination {
            self.paginate::<FuturesAgg>(&path, Some(&params), options)
        } else {
            self.single_page::<FuturesAgg>(&path, Some(&params), options)
        }
    }

    fn list_futures_contracts(
        &self,
        date: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        product_code: Option<&str>,
        product_code_any_of: Option<&str>,
        product_code_gt: Option<&str>,
        product_code_gte: Option<&str>,
        product_code_lt: Option<&str>,
        product_code_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        active: Option<bool>,
        type_: Option<&str>,
        type_any_of: Option<&str>,
        first_trade_date: Option<&str>,
        first_trade_date_gt: Option<&str>,
        first_trade_date_gte: Option<&str>,
        first_trade_date_lt: Option<&str>,
        first_trade_date_lte: Option<&str>,
        last_trade_date: Option<&str>,
        last_trade_date_gt: Option<&str>,
        last_trade_date_gte: Option<&str>,
        last_trade_date_lt: Option<&str>,
        last_trade_date_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FuturesContract>> {
        let path = "/futures/v1/contracts";
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = date {
            params.push(("date", v.to_string()));
        }
        if let Some(v) = date_gt {
            params.push(("date.gt", v.to_string()));
        }
        if let Some(v) = date_gte {
            params.push(("date.gte", v.to_string()));
        }
        if let Some(v) = date_lt {
            params.push(("date.lt", v.to_string()));
        }
        if let Some(v) = date_lte {
            params.push(("date.lte", v.to_string()));
        }
        if let Some(v) = product_code {
            params.push(("product_code", v.to_string()));
        }
        if let Some(v) = product_code_any_of {
            params.push(("product_code.any_of", v.to_string()));
        }
        if let Some(v) = product_code_gt {
            params.push(("product_code.gt", v.to_string()));
        }
        if let Some(v) = product_code_gte {
            params.push(("product_code.gte", v.to_string()));
        }
        if let Some(v) = product_code_lt {
            params.push(("product_code.lt", v.to_string()));
        }
        if let Some(v) = product_code_lte {
            params.push(("product_code.lte", v.to_string()));
        }
        if let Some(v) = ticker {
            params.push(("ticker", v.to_string()));
        }
        if let Some(v) = ticker_any_of {
            params.push(("ticker.any_of", v.to_string()));
        }
        if let Some(v) = ticker_gt {
            params.push(("ticker.gt", v.to_string()));
        }
        if let Some(v) = ticker_gte {
            params.push(("ticker.gte", v.to_string()));
        }
        if let Some(v) = ticker_lt {
            params.push(("ticker.lt", v.to_string()));
        }
        if let Some(v) = ticker_lte {
            params.push(("ticker.lte", v.to_string()));
        }
        if let Some(a) = active {
            params.push(("active", a.to_string()));
        }
        if let Some(v) = type_ {
            params.push(("type", v.to_string()));
        }
        if let Some(v) = type_any_of {
            params.push(("type.any_of", v.to_string()));
        }
        if let Some(v) = first_trade_date {
            params.push(("first_trade_date", v.to_string()));
        }
        if let Some(v) = first_trade_date_gt {
            params.push(("first_trade_date.gt", v.to_string()));
        }
        if let Some(v) = first_trade_date_gte {
            params.push(("first_trade_date.gte", v.to_string()));
        }
        if let Some(v) = first_trade_date_lt {
            params.push(("first_trade_date.lt", v.to_string()));
        }
        if let Some(v) = first_trade_date_lte {
            params.push(("first_trade_date.lte", v.to_string()));
        }
        if let Some(v) = last_trade_date {
            params.push(("last_trade_date", v.to_string()));
        }
        if let Some(v) = last_trade_date_gt {
            params.push(("last_trade_date.gt", v.to_string()));
        }
        if let Some(v) = last_trade_date_gte {
            params.push(("last_trade_date.gte", v.to_string()));
        }
        if let Some(v) = last_trade_date_lt {
            params.push(("last_trade_date.lt", v.to_string()));
        }
        if let Some(v) = last_trade_date_lte {
            params.push(("last_trade_date.lte", v.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if let Some(s) = sort {
            params.push(("sort", s.to_string()));
        }
        if self.pagination {
            self.paginate::<FuturesContract>(path, Some(&params), options)
        } else {
            self.single_page::<FuturesContract>(path, Some(&params), options)
        }
    }

    fn list_futures_products(
        &self,
        name: Option<&str>,
        name_any_of: Option<&str>,
        name_gt: Option<&str>,
        name_gte: Option<&str>,
        name_lt: Option<&str>,
        name_lte: Option<&str>,
        product_code: Option<&str>,
        product_code_any_of: Option<&str>,
        product_code_gt: Option<&str>,
        product_code_gte: Option<&str>,
        product_code_lt: Option<&str>,
        product_code_lte: Option<&str>,
        date: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        trading_venue: Option<&str>,
        trading_venue_any_of: Option<&str>,
        trading_venue_gt: Option<&str>,
        trading_venue_gte: Option<&str>,
        trading_venue_lt: Option<&str>,
        trading_venue_lte: Option<&str>,
        sector: Option<&str>,
        sector_any_of: Option<&str>,
        sub_sector: Option<&str>,
        sub_sector_any_of: Option<&str>,
        asset_class: Option<&str>,
        asset_class_any_of: Option<&str>,
        asset_sub_class: Option<&str>,
        asset_sub_class_any_of: Option<&str>,
        type_: Option<&str>,
        type_any_of: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FuturesProduct>> {
        let path = "/futures/v1/products";
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = name {
            params.push(("name", v.to_string()));
        }
        if let Some(v) = name_any_of {
            params.push(("name.any_of", v.to_string()));
        }
        if let Some(v) = name_gt {
            params.push(("name.gt", v.to_string()));
        }
        if let Some(v) = name_gte {
            params.push(("name.gte", v.to_string()));
        }
        if let Some(v) = name_lt {
            params.push(("name.lt", v.to_string()));
        }
        if let Some(v) = name_lte {
            params.push(("name.lte", v.to_string()));
        }
        if let Some(v) = product_code {
            params.push(("product_code", v.to_string()));
        }
        if let Some(v) = product_code_any_of {
            params.push(("product_code.any_of", v.to_string()));
        }
        if let Some(v) = product_code_gt {
            params.push(("product_code.gt", v.to_string()));
        }
        if let Some(v) = product_code_gte {
            params.push(("product_code.gte", v.to_string()));
        }
        if let Some(v) = product_code_lt {
            params.push(("product_code.lt", v.to_string()));
        }
        if let Some(v) = product_code_lte {
            params.push(("product_code.lte", v.to_string()));
        }
        if let Some(v) = date {
            params.push(("date", v.to_string()));
        }
        if let Some(v) = date_gt {
            params.push(("date.gt", v.to_string()));
        }
        if let Some(v) = date_gte {
            params.push(("date.gte", v.to_string()));
        }
        if let Some(v) = date_lt {
            params.push(("date.lt", v.to_string()));
        }
        if let Some(v) = date_lte {
            params.push(("date.lte", v.to_string()));
        }
        if let Some(v) = trading_venue {
            params.push(("trading_venue", v.to_string()));
        }
        if let Some(v) = trading_venue_any_of {
            params.push(("trading_venue.any_of", v.to_string()));
        }
        if let Some(v) = trading_venue_gt {
            params.push(("trading_venue.gt", v.to_string()));
        }
        if let Some(v) = trading_venue_gte {
            params.push(("trading_venue.gte", v.to_string()));
        }
        if let Some(v) = trading_venue_lt {
            params.push(("trading_venue.lt", v.to_string()));
        }
        if let Some(v) = trading_venue_lte {
            params.push(("trading_venue.lte", v.to_string()));
        }
        if let Some(v) = sector {
            params.push(("sector", v.to_string()));
        }
        if let Some(v) = sector_any_of {
            params.push(("sector.any_of", v.to_string()));
        }
        if let Some(v) = sub_sector {
            params.push(("sub_sector", v.to_string()));
        }
        if let Some(v) = sub_sector_any_of {
            params.push(("sub_sector.any_of", v.to_string()));
        }
        if let Some(v) = asset_class {
            params.push(("asset_class", v.to_string()));
        }
        if let Some(v) = asset_class_any_of {
            params.push(("asset_class.any_of", v.to_string()));
        }
        if let Some(v) = asset_sub_class {
            params.push(("asset_sub_class", v.to_string()));
        }
        if let Some(v) = asset_sub_class_any_of {
            params.push(("asset_sub_class.any_of", v.to_string()));
        }
        if let Some(v) = type_ {
            params.push(("type", v.to_string()));
        }
        if let Some(v) = type_any_of {
            params.push(("type.any_of", v.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if let Some(s) = sort {
            params.push(("sort", s.to_string()));
        }
        if self.pagination {
            self.paginate::<FuturesProduct>(path, Some(&params), options)
        } else {
            self.single_page::<FuturesProduct>(path, Some(&params), options)
        }
    }

    fn list_futures_quotes(
        &self,
        ticker: &str,
        timestamp: Option<&str>,
        timestamp_lt: Option<&str>,
        timestamp_lte: Option<&str>,
        timestamp_gt: Option<&str>,
        timestamp_gte: Option<&str>,
        session_end_date: Option<&str>,
        session_end_date_lt: Option<&str>,
        session_end_date_lte: Option<&str>,
        session_end_date_gt: Option<&str>,
        session_end_date_gte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FuturesQuote>> {
        let path = format!("/futures/v1/quotes/{}", ticker);
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = timestamp {
            params.push(("timestamp", v.to_string()));
        }
        if let Some(v) = timestamp_lt {
            params.push(("timestamp.lt", v.to_string()));
        }
        if let Some(v) = timestamp_lte {
            params.push(("timestamp.lte", v.to_string()));
        }
        if let Some(v) = timestamp_gt {
            params.push(("timestamp.gt", v.to_string()));
        }
        if let Some(v) = timestamp_gte {
            params.push(("timestamp.gte", v.to_string()));
        }
        if let Some(v) = session_end_date {
            params.push(("session_end_date", v.to_string()));
        }
        if let Some(v) = session_end_date_lt {
            params.push(("session_end_date.lt", v.to_string()));
        }
        if let Some(v) = session_end_date_lte {
            params.push(("session_end_date.lte", v.to_string()));
        }
        if let Some(v) = session_end_date_gt {
            params.push(("session_end_date.gt", v.to_string()));
        }
        if let Some(v) = session_end_date_gte {
            params.push(("session_end_date.gte", v.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if let Some(s) = sort {
            params.push(("sort", s.to_string()));
        }
        if self.pagination {
            self.paginate::<FuturesQuote>(&path, Some(&params), options)
        } else {
            self.single_page::<FuturesQuote>(&path, Some(&params), options)
        }
    }

    fn list_futures_trades(
        &self,
        ticker: &str,
        timestamp: Option<&str>,
        timestamp_lt: Option<&str>,
        timestamp_lte: Option<&str>,
        timestamp_gt: Option<&str>,
        timestamp_gte: Option<&str>,
        session_end_date: Option<&str>,
        session_end_date_lt: Option<&str>,
        session_end_date_lte: Option<&str>,
        session_end_date_gt: Option<&str>,
        session_end_date_gte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FuturesTrade>> {
        let path = format!("/futures/v1/trades/{}", ticker);
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = timestamp {
            params.push(("timestamp", v.to_string()));
        }
        if let Some(v) = timestamp_lt {
            params.push(("timestamp.lt", v.to_string()));
        }
        if let Some(v) = timestamp_lte {
            params.push(("timestamp.lte", v.to_string()));
        }
        if let Some(v) = timestamp_gt {
            params.push(("timestamp.gt", v.to_string()));
        }
        if let Some(v) = timestamp_gte {
            params.push(("timestamp.gte", v.to_string()));
        }
        if let Some(v) = session_end_date {
            params.push(("session_end_date", v.to_string()));
        }
        if let Some(v) = session_end_date_lt {
            params.push(("session_end_date.lt", v.to_string()));
        }
        if let Some(v) = session_end_date_lte {
            params.push(("session_end_date.lte", v.to_string()));
        }
        if let Some(v) = session_end_date_gt {
            params.push(("session_end_date.gt", v.to_string()));
        }
        if let Some(v) = session_end_date_gte {
            params.push(("session_end_date.gte", v.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if let Some(s) = sort {
            params.push(("sort", s.to_string()));
        }
        if self.pagination {
            self.paginate::<FuturesTrade>(&path, Some(&params), options)
        } else {
            self.single_page::<FuturesTrade>(&path, Some(&params), options)
        }
    }

    fn list_futures_schedules(
        &self,
        product_code: Option<&str>,
        product_code_any_of: Option<&str>,
        product_code_gt: Option<&str>,
        product_code_gte: Option<&str>,
        product_code_lt: Option<&str>,
        product_code_lte: Option<&str>,
        session_end_date: Option<&str>,
        session_end_date_gt: Option<&str>,
        session_end_date_gte: Option<&str>,
        session_end_date_lt: Option<&str>,
        session_end_date_lte: Option<&str>,
        trading_venue: Option<&str>,
        trading_venue_any_of: Option<&str>,
        trading_venue_gt: Option<&str>,
        trading_venue_gte: Option<&str>,
        trading_venue_lt: Option<&str>,
        trading_venue_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FuturesSchedule>> {
        let path = "/futures/v1/schedules";
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = product_code {
            params.push(("product_code", v.to_string()));
        }
        if let Some(v) = product_code_any_of {
            params.push(("product_code.any_of", v.to_string()));
        }
        if let Some(v) = product_code_gt {
            params.push(("product_code.gt", v.to_string()));
        }
        if let Some(v) = product_code_gte {
            params.push(("product_code.gte", v.to_string()));
        }
        if let Some(v) = product_code_lt {
            params.push(("product_code.lt", v.to_string()));
        }
        if let Some(v) = product_code_lte {
            params.push(("product_code.lte", v.to_string()));
        }
        if let Some(v) = session_end_date {
            params.push(("session_end_date", v.to_string()));
        }
        if let Some(v) = session_end_date_gt {
            params.push(("session_end_date.gt", v.to_string()));
        }
        if let Some(v) = session_end_date_gte {
            params.push(("session_end_date.gte", v.to_string()));
        }
        if let Some(v) = session_end_date_lt {
            params.push(("session_end_date.lt", v.to_string()));
        }
        if let Some(v) = session_end_date_lte {
            params.push(("session_end_date.lte", v.to_string()));
        }
        if let Some(v) = trading_venue {
            params.push(("trading_venue", v.to_string()));
        }
        if let Some(v) = trading_venue_any_of {
            params.push(("trading_venue.any_of", v.to_string()));
        }
        if let Some(v) = trading_venue_gt {
            params.push(("trading_venue.gt", v.to_string()));
        }
        if let Some(v) = trading_venue_gte {
            params.push(("trading_venue.gte", v.to_string()));
        }
        if let Some(v) = trading_venue_lt {
            params.push(("trading_venue.lt", v.to_string()));
        }
        if let Some(v) = trading_venue_lte {
            params.push(("trading_venue.lte", v.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if let Some(s) = sort {
            params.push(("sort", s.to_string()));
        }
        if self.pagination {
            self.paginate::<FuturesSchedule>(path, Some(&params), options)
        } else {
            self.single_page::<FuturesSchedule>(path, Some(&params), options)
        }
    }

    fn list_futures_market_statuses(
        &self,
        product_code: Option<&str>,
        product_code_any_of: Option<&str>,
        product_code_gt: Option<&str>,
        product_code_gte: Option<&str>,
        product_code_lt: Option<&str>,
        product_code_lte: Option<&str>,
        limit: Option<i64>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FuturesMarketStatus>> {
        let path = "/futures/v1/market-status";
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = product_code {
            params.push(("product_code", v.to_string()));
        }
        if let Some(v) = product_code_any_of {
            params.push(("product_code.any_of", v.to_string()));
        }
        if let Some(v) = product_code_gt {
            params.push(("product_code.gt", v.to_string()));
        }
        if let Some(v) = product_code_gte {
            params.push(("product_code.gte", v.to_string()));
        }
        if let Some(v) = product_code_lt {
            params.push(("product_code.lt", v.to_string()));
        }
        if let Some(v) = product_code_lte {
            params.push(("product_code.lte", v.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if self.pagination {
            self.paginate::<FuturesMarketStatus>(path, Some(&params), options)
        } else {
            self.single_page::<FuturesMarketStatus>(path, Some(&params), options)
        }
    }

    fn get_futures_snapshot(
        &self,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        product_code: Option<&str>,
        product_code_any_of: Option<&str>,
        product_code_gt: Option<&str>,
        product_code_gte: Option<&str>,
        product_code_lt: Option<&str>,
        product_code_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FuturesSnapshot>> {
        let path = "/futures/v1/snapshot";
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = ticker {
            params.push(("ticker", v.to_string()));
        }
        if let Some(v) = ticker_any_of {
            params.push(("ticker.any_of", v.to_string()));
        }
        if let Some(v) = ticker_gt {
            params.push(("ticker.gt", v.to_string()));
        }
        if let Some(v) = ticker_gte {
            params.push(("ticker.gte", v.to_string()));
        }
        if let Some(v) = ticker_lt {
            params.push(("ticker.lt", v.to_string()));
        }
        if let Some(v) = ticker_lte {
            params.push(("ticker.lte", v.to_string()));
        }
        if let Some(v) = product_code {
            params.push(("product_code", v.to_string()));
        }
        if let Some(v) = product_code_any_of {
            params.push(("product_code.any_of", v.to_string()));
        }
        if let Some(v) = product_code_gt {
            params.push(("product_code.gt", v.to_string()));
        }
        if let Some(v) = product_code_gte {
            params.push(("product_code.gte", v.to_string()));
        }
        if let Some(v) = product_code_lt {
            params.push(("product_code.lt", v.to_string()));
        }
        if let Some(v) = product_code_lte {
            params.push(("product_code.lte", v.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if let Some(s) = sort {
            params.push(("sort", s.to_string()));
        }
        if self.pagination {
            self.paginate::<FuturesSnapshot>(path, Some(&params), options)
        } else {
            self.single_page::<FuturesSnapshot>(path, Some(&params), options)
        }
    }

    fn list_futures_exchanges(
        &self,
        limit: Option<i64>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FuturesExchange>> {
        let path = "/futures/v1/exchanges";
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if self.pagination {
            self.paginate::<FuturesExchange>(path, Some(&params), options)
        } else {
            self.single_page::<FuturesExchange>(path, Some(&params), options)
        }
    }
}
