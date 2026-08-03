use crate::client::{Client, RequestOptions};
use crate::error::Result;
use crate::models::{
    EUMerchantAggregate, EUMerchantHierarchy, FedInflation, FedInflationExpectations,
    FedLaborMarket, TreasuryYield,
};
use futures::Stream;

/// Economy (Fed and EU consumer spending) API.
pub trait EconomyApi {
    /// Retrieve treasury yield data.
    fn list_treasury_yields(
        &self,
        date: Option<&str>,
        date_any_of: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<TreasuryYield>>;

    /// List inflation data from the Federal Reserve.
    fn list_inflation(
        &self,
        date: Option<&str>,
        date_any_of: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FedInflation>>;

    /// List inflation expectations from market-based and economic model perspectives.
    fn list_inflation_expectations(
        &self,
        date: Option<&str>,
        date_any_of: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FedInflationExpectations>>;

    /// List labor market indicators from the Federal Reserve.
    fn list_labor_market_indicators(
        &self,
        date: Option<&str>,
        date_any_of: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FedLaborMarket>>;

    /// List aggregated consumer transactions from European credit card panels.
    fn list_eu_merchant_aggregates(
        &self,
        transaction_date: Option<&str>,
        transaction_date_gt: Option<&str>,
        transaction_date_gte: Option<&str>,
        transaction_date_lt: Option<&str>,
        transaction_date_lte: Option<&str>,
        name: Option<&str>,
        name_any_of: Option<&str>,
        name_gt: Option<&str>,
        name_gte: Option<&str>,
        name_lt: Option<&str>,
        name_lte: Option<&str>,
        user_country: Option<&str>,
        user_country_any_of: Option<&str>,
        channel: Option<&str>,
        channel_any_of: Option<&str>,
        consumer_type: Option<&str>,
        consumer_type_any_of: Option<&str>,
        parent_name: Option<&str>,
        parent_name_any_of: Option<&str>,
        parent_name_gt: Option<&str>,
        parent_name_gte: Option<&str>,
        parent_name_lt: Option<&str>,
        parent_name_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<EUMerchantAggregate>>;

    /// List reference data mapping EU merchants to parent companies, tickers, sectors, and industries.
    fn list_eu_merchant_hierarchy(
        &self,
        lookup_name: Option<&str>,
        lookup_name_any_of: Option<&str>,
        lookup_name_gt: Option<&str>,
        lookup_name_gte: Option<&str>,
        lookup_name_lt: Option<&str>,
        lookup_name_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        listing_status: Option<&str>,
        listing_status_any_of: Option<&str>,
        active_from: Option<&str>,
        active_from_gt: Option<&str>,
        active_from_gte: Option<&str>,
        active_from_lt: Option<&str>,
        active_from_lte: Option<&str>,
        active_to: Option<&str>,
        active_to_gt: Option<&str>,
        active_to_gte: Option<&str>,
        active_to_lt: Option<&str>,
        active_to_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<EUMerchantHierarchy>>;
}

impl EconomyApi for Client {
    fn list_treasury_yields(
        &self,
        date: Option<&str>,
        date_any_of: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<TreasuryYield>> {
        let path = "/fed/v1/treasury-yields".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = date {
            params.push(("date", v.to_string()));
        }
        if let Some(v) = date_any_of {
            params.push(("date.any_of", v.to_string()));
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
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if let Some(v) = order {
            params.push(("order", v.to_string()));
        }
        if self.pagination {
            self.paginate::<TreasuryYield>(&path, Some(&params), options)
        } else {
            self.single_page::<TreasuryYield>(&path, Some(&params), options)
        }
    }

    fn list_inflation(
        &self,
        date: Option<&str>,
        date_any_of: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FedInflation>> {
        let path = "/fed/v1/inflation".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = date {
            params.push(("date", v.to_string()));
        }
        if let Some(v) = date_any_of {
            params.push(("date.any_of", v.to_string()));
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
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<FedInflation>(&path, Some(&params), options)
        } else {
            self.single_page::<FedInflation>(&path, Some(&params), options)
        }
    }

    fn list_inflation_expectations(
        &self,
        date: Option<&str>,
        date_any_of: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FedInflationExpectations>> {
        let path = "/fed/v1/inflation-expectations".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = date {
            params.push(("date", v.to_string()));
        }
        if let Some(v) = date_any_of {
            params.push(("date.any_of", v.to_string()));
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
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<FedInflationExpectations>(&path, Some(&params), options)
        } else {
            self.single_page::<FedInflationExpectations>(&path, Some(&params), options)
        }
    }

    fn list_labor_market_indicators(
        &self,
        date: Option<&str>,
        date_any_of: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FedLaborMarket>> {
        let path = "/fed/v1/labor-market".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = date {
            params.push(("date", v.to_string()));
        }
        if let Some(v) = date_any_of {
            params.push(("date.any_of", v.to_string()));
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
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<FedLaborMarket>(&path, Some(&params), options)
        } else {
            self.single_page::<FedLaborMarket>(&path, Some(&params), options)
        }
    }

    fn list_eu_merchant_aggregates(
        &self,
        transaction_date: Option<&str>,
        transaction_date_gt: Option<&str>,
        transaction_date_gte: Option<&str>,
        transaction_date_lt: Option<&str>,
        transaction_date_lte: Option<&str>,
        name: Option<&str>,
        name_any_of: Option<&str>,
        name_gt: Option<&str>,
        name_gte: Option<&str>,
        name_lt: Option<&str>,
        name_lte: Option<&str>,
        user_country: Option<&str>,
        user_country_any_of: Option<&str>,
        channel: Option<&str>,
        channel_any_of: Option<&str>,
        consumer_type: Option<&str>,
        consumer_type_any_of: Option<&str>,
        parent_name: Option<&str>,
        parent_name_any_of: Option<&str>,
        parent_name_gt: Option<&str>,
        parent_name_gte: Option<&str>,
        parent_name_lt: Option<&str>,
        parent_name_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<EUMerchantAggregate>> {
        let path = "/consumer-spending/eu/v1/merchant-aggregates".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = transaction_date {
            params.push(("transaction_date", v.to_string()));
        }
        if let Some(v) = transaction_date_gt {
            params.push(("transaction_date.gt", v.to_string()));
        }
        if let Some(v) = transaction_date_gte {
            params.push(("transaction_date.gte", v.to_string()));
        }
        if let Some(v) = transaction_date_lt {
            params.push(("transaction_date.lt", v.to_string()));
        }
        if let Some(v) = transaction_date_lte {
            params.push(("transaction_date.lte", v.to_string()));
        }
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
        if let Some(v) = user_country {
            params.push(("user_country", v.to_string()));
        }
        if let Some(v) = user_country_any_of {
            params.push(("user_country.any_of", v.to_string()));
        }
        if let Some(v) = channel {
            params.push(("channel", v.to_string()));
        }
        if let Some(v) = channel_any_of {
            params.push(("channel.any_of", v.to_string()));
        }
        if let Some(v) = consumer_type {
            params.push(("consumer_type", v.to_string()));
        }
        if let Some(v) = consumer_type_any_of {
            params.push(("consumer_type.any_of", v.to_string()));
        }
        if let Some(v) = parent_name {
            params.push(("parent_name", v.to_string()));
        }
        if let Some(v) = parent_name_any_of {
            params.push(("parent_name.any_of", v.to_string()));
        }
        if let Some(v) = parent_name_gt {
            params.push(("parent_name.gt", v.to_string()));
        }
        if let Some(v) = parent_name_gte {
            params.push(("parent_name.gte", v.to_string()));
        }
        if let Some(v) = parent_name_lt {
            params.push(("parent_name.lt", v.to_string()));
        }
        if let Some(v) = parent_name_lte {
            params.push(("parent_name.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<EUMerchantAggregate>(&path, Some(&params), options)
        } else {
            self.single_page::<EUMerchantAggregate>(&path, Some(&params), options)
        }
    }

    fn list_eu_merchant_hierarchy(
        &self,
        lookup_name: Option<&str>,
        lookup_name_any_of: Option<&str>,
        lookup_name_gt: Option<&str>,
        lookup_name_gte: Option<&str>,
        lookup_name_lt: Option<&str>,
        lookup_name_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        listing_status: Option<&str>,
        listing_status_any_of: Option<&str>,
        active_from: Option<&str>,
        active_from_gt: Option<&str>,
        active_from_gte: Option<&str>,
        active_from_lt: Option<&str>,
        active_from_lte: Option<&str>,
        active_to: Option<&str>,
        active_to_gt: Option<&str>,
        active_to_gte: Option<&str>,
        active_to_lt: Option<&str>,
        active_to_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<EUMerchantHierarchy>> {
        let path = "/consumer-spending/eu/v1/merchant-hierarchy".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = lookup_name {
            params.push(("lookup_name", v.to_string()));
        }
        if let Some(v) = lookup_name_any_of {
            params.push(("lookup_name.any_of", v.to_string()));
        }
        if let Some(v) = lookup_name_gt {
            params.push(("lookup_name.gt", v.to_string()));
        }
        if let Some(v) = lookup_name_gte {
            params.push(("lookup_name.gte", v.to_string()));
        }
        if let Some(v) = lookup_name_lt {
            params.push(("lookup_name.lt", v.to_string()));
        }
        if let Some(v) = lookup_name_lte {
            params.push(("lookup_name.lte", v.to_string()));
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
        if let Some(v) = listing_status {
            params.push(("listing_status", v.to_string()));
        }
        if let Some(v) = listing_status_any_of {
            params.push(("listing_status.any_of", v.to_string()));
        }
        if let Some(v) = active_from {
            params.push(("active_from", v.to_string()));
        }
        if let Some(v) = active_from_gt {
            params.push(("active_from.gt", v.to_string()));
        }
        if let Some(v) = active_from_gte {
            params.push(("active_from.gte", v.to_string()));
        }
        if let Some(v) = active_from_lt {
            params.push(("active_from.lt", v.to_string()));
        }
        if let Some(v) = active_from_lte {
            params.push(("active_from.lte", v.to_string()));
        }
        if let Some(v) = active_to {
            params.push(("active_to", v.to_string()));
        }
        if let Some(v) = active_to_gt {
            params.push(("active_to.gt", v.to_string()));
        }
        if let Some(v) = active_to_gte {
            params.push(("active_to.gte", v.to_string()));
        }
        if let Some(v) = active_to_lt {
            params.push(("active_to.lt", v.to_string()));
        }
        if let Some(v) = active_to_lte {
            params.push(("active_to.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<EUMerchantHierarchy>(&path, Some(&params), options)
        } else {
            self.single_page::<EUMerchantHierarchy>(&path, Some(&params), options)
        }
    }
}
