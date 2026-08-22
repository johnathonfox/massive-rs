use super::BoxStream;
use crate::client::{Client, RequestOptions};
use crate::models::{
    EUMerchantAggregate, EUMerchantHierarchy, FedInflation, FedInflationExpectations,
    FedLaborMarket, TreasuryYield,
};

/// Economy (Fed and EU consumer spending) API.
pub trait EconomyApi {
    /// Retrieve treasury yield data.
    fn list_treasury_yields<'a>(
        &'a self,
        date: Option<&'a str>,
        date_any_of: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, TreasuryYield>;

    /// Same as [`Self::list_treasury_yields`], but takes the optional arguments as a
    /// chainable [`ListTreasuryYieldsParams`] struct.
    fn list_treasury_yields_with_params<'a>(
        &'a self,
        params: ListTreasuryYieldsParams,
    ) -> BoxStream<'a, TreasuryYield>;

    /// List inflation data from the Federal Reserve.
    fn list_inflation<'a>(
        &'a self,
        date: Option<&'a str>,
        date_any_of: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FedInflation>;

    /// Same as [`Self::list_inflation`], but takes the optional arguments as a
    /// chainable [`ListInflationParams`] struct.
    fn list_inflation_with_params<'a>(
        &'a self,
        params: ListInflationParams,
    ) -> BoxStream<'a, FedInflation>;

    /// List inflation expectations from market-based and economic model perspectives.
    fn list_inflation_expectations<'a>(
        &'a self,
        date: Option<&'a str>,
        date_any_of: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FedInflationExpectations>;

    /// Same as [`Self::list_inflation_expectations`], but takes the optional arguments as a
    /// chainable [`ListInflationExpectationsParams`] struct.
    fn list_inflation_expectations_with_params<'a>(
        &'a self,
        params: ListInflationExpectationsParams,
    ) -> BoxStream<'a, FedInflationExpectations>;

    /// List labor market indicators from the Federal Reserve.
    fn list_labor_market_indicators<'a>(
        &'a self,
        date: Option<&'a str>,
        date_any_of: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FedLaborMarket>;

    /// Same as [`Self::list_labor_market_indicators`], but takes the optional arguments as a
    /// chainable [`ListLaborMarketIndicatorsParams`] struct.
    fn list_labor_market_indicators_with_params<'a>(
        &'a self,
        params: ListLaborMarketIndicatorsParams,
    ) -> BoxStream<'a, FedLaborMarket>;

    /// List aggregated consumer transactions from European credit card panels.
    fn list_eu_merchant_aggregates<'a>(
        &'a self,
        transaction_date: Option<&'a str>,
        transaction_date_gt: Option<&'a str>,
        transaction_date_gte: Option<&'a str>,
        transaction_date_lt: Option<&'a str>,
        transaction_date_lte: Option<&'a str>,
        name: Option<&'a str>,
        name_any_of: Option<&'a str>,
        name_gt: Option<&'a str>,
        name_gte: Option<&'a str>,
        name_lt: Option<&'a str>,
        name_lte: Option<&'a str>,
        user_country: Option<&'a str>,
        user_country_any_of: Option<&'a str>,
        channel: Option<&'a str>,
        channel_any_of: Option<&'a str>,
        consumer_type: Option<&'a str>,
        consumer_type_any_of: Option<&'a str>,
        parent_name: Option<&'a str>,
        parent_name_any_of: Option<&'a str>,
        parent_name_gt: Option<&'a str>,
        parent_name_gte: Option<&'a str>,
        parent_name_lt: Option<&'a str>,
        parent_name_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, EUMerchantAggregate>;

    /// Same as [`Self::list_eu_merchant_aggregates`], but takes the optional arguments as a
    /// chainable [`ListEuMerchantAggregatesParams`] struct.
    fn list_eu_merchant_aggregates_with_params<'a>(
        &'a self,
        params: ListEuMerchantAggregatesParams,
    ) -> BoxStream<'a, EUMerchantAggregate>;

    /// List reference data mapping EU merchants to parent companies, tickers, sectors, and industries.
    fn list_eu_merchant_hierarchy<'a>(
        &'a self,
        lookup_name: Option<&'a str>,
        lookup_name_any_of: Option<&'a str>,
        lookup_name_gt: Option<&'a str>,
        lookup_name_gte: Option<&'a str>,
        lookup_name_lt: Option<&'a str>,
        lookup_name_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        listing_status: Option<&'a str>,
        listing_status_any_of: Option<&'a str>,
        active_from: Option<&'a str>,
        active_from_gt: Option<&'a str>,
        active_from_gte: Option<&'a str>,
        active_from_lt: Option<&'a str>,
        active_from_lte: Option<&'a str>,
        active_to: Option<&'a str>,
        active_to_gt: Option<&'a str>,
        active_to_gte: Option<&'a str>,
        active_to_lt: Option<&'a str>,
        active_to_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, EUMerchantHierarchy>;

    /// Same as [`Self::list_eu_merchant_hierarchy`], but takes the optional arguments as a
    /// chainable [`ListEuMerchantHierarchyParams`] struct.
    fn list_eu_merchant_hierarchy_with_params<'a>(
        &'a self,
        params: ListEuMerchantHierarchyParams,
    ) -> BoxStream<'a, EUMerchantHierarchy>;
}

impl EconomyApi for Client {
    fn list_treasury_yields<'a>(
        &'a self,
        date: Option<&'a str>,
        date_any_of: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, TreasuryYield> {
        self.list_treasury_yields_with_params(ListTreasuryYieldsParams {
            date: date.map(String::from),
            date_any_of: date_any_of.map(String::from),
            date_gt: date_gt.map(String::from),
            date_gte: date_gte.map(String::from),
            date_lt: date_lt.map(String::from),
            date_lte: date_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            order: order.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_treasury_yields_with_params<'a>(
        &'a self,
        params: ListTreasuryYieldsParams,
    ) -> BoxStream<'a, TreasuryYield> {
        Box::pin({
            let ListTreasuryYieldsParams {
                date,
                date_any_of,
                date_gt,
                date_gte,
                date_lt,
                date_lte,
                limit,
                sort,
                order,
                options,
            } = params;
            let date = date.as_deref();
            let date_any_of = date_any_of.as_deref();
            let date_gt = date_gt.as_deref();
            let date_gte = date_gte.as_deref();
            let date_lt = date_lt.as_deref();
            let date_lte = date_lte.as_deref();
            let sort = sort.as_deref();
            let order = order.as_deref();
            let options = options.as_ref();
            let path = "/fed/v1/treasury-yields".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = date {
                query.push(("date", v.to_string()));
            }
            if let Some(v) = date_any_of {
                query.push(("date.any_of", v.to_string()));
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
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            if let Some(v) = order {
                query.push(("order", v.to_string()));
            }
            self.list::<TreasuryYield>(&path, Some(&query), options)
        })
    }

    fn list_inflation<'a>(
        &'a self,
        date: Option<&'a str>,
        date_any_of: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FedInflation> {
        self.list_inflation_with_params(ListInflationParams {
            date: date.map(String::from),
            date_any_of: date_any_of.map(String::from),
            date_gt: date_gt.map(String::from),
            date_gte: date_gte.map(String::from),
            date_lt: date_lt.map(String::from),
            date_lte: date_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_inflation_with_params<'a>(
        &'a self,
        params: ListInflationParams,
    ) -> BoxStream<'a, FedInflation> {
        Box::pin({
            let ListInflationParams {
                date,
                date_any_of,
                date_gt,
                date_gte,
                date_lt,
                date_lte,
                limit,
                sort,
                options,
            } = params;
            let date = date.as_deref();
            let date_any_of = date_any_of.as_deref();
            let date_gt = date_gt.as_deref();
            let date_gte = date_gte.as_deref();
            let date_lt = date_lt.as_deref();
            let date_lte = date_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/fed/v1/inflation".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = date {
                query.push(("date", v.to_string()));
            }
            if let Some(v) = date_any_of {
                query.push(("date.any_of", v.to_string()));
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
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<FedInflation>(&path, Some(&query), options)
        })
    }

    fn list_inflation_expectations<'a>(
        &'a self,
        date: Option<&'a str>,
        date_any_of: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FedInflationExpectations> {
        self.list_inflation_expectations_with_params(ListInflationExpectationsParams {
            date: date.map(String::from),
            date_any_of: date_any_of.map(String::from),
            date_gt: date_gt.map(String::from),
            date_gte: date_gte.map(String::from),
            date_lt: date_lt.map(String::from),
            date_lte: date_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_inflation_expectations_with_params<'a>(
        &'a self,
        params: ListInflationExpectationsParams,
    ) -> BoxStream<'a, FedInflationExpectations> {
        Box::pin({
            let ListInflationExpectationsParams {
                date,
                date_any_of,
                date_gt,
                date_gte,
                date_lt,
                date_lte,
                limit,
                sort,
                options,
            } = params;
            let date = date.as_deref();
            let date_any_of = date_any_of.as_deref();
            let date_gt = date_gt.as_deref();
            let date_gte = date_gte.as_deref();
            let date_lt = date_lt.as_deref();
            let date_lte = date_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/fed/v1/inflation-expectations".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = date {
                query.push(("date", v.to_string()));
            }
            if let Some(v) = date_any_of {
                query.push(("date.any_of", v.to_string()));
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
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<FedInflationExpectations>(&path, Some(&query), options)
        })
    }

    fn list_labor_market_indicators<'a>(
        &'a self,
        date: Option<&'a str>,
        date_any_of: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FedLaborMarket> {
        self.list_labor_market_indicators_with_params(ListLaborMarketIndicatorsParams {
            date: date.map(String::from),
            date_any_of: date_any_of.map(String::from),
            date_gt: date_gt.map(String::from),
            date_gte: date_gte.map(String::from),
            date_lt: date_lt.map(String::from),
            date_lte: date_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_labor_market_indicators_with_params<'a>(
        &'a self,
        params: ListLaborMarketIndicatorsParams,
    ) -> BoxStream<'a, FedLaborMarket> {
        Box::pin({
            let ListLaborMarketIndicatorsParams {
                date,
                date_any_of,
                date_gt,
                date_gte,
                date_lt,
                date_lte,
                limit,
                sort,
                options,
            } = params;
            let date = date.as_deref();
            let date_any_of = date_any_of.as_deref();
            let date_gt = date_gt.as_deref();
            let date_gte = date_gte.as_deref();
            let date_lt = date_lt.as_deref();
            let date_lte = date_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/fed/v1/labor-market".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = date {
                query.push(("date", v.to_string()));
            }
            if let Some(v) = date_any_of {
                query.push(("date.any_of", v.to_string()));
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
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<FedLaborMarket>(&path, Some(&query), options)
        })
    }

    fn list_eu_merchant_aggregates<'a>(
        &'a self,
        transaction_date: Option<&'a str>,
        transaction_date_gt: Option<&'a str>,
        transaction_date_gte: Option<&'a str>,
        transaction_date_lt: Option<&'a str>,
        transaction_date_lte: Option<&'a str>,
        name: Option<&'a str>,
        name_any_of: Option<&'a str>,
        name_gt: Option<&'a str>,
        name_gte: Option<&'a str>,
        name_lt: Option<&'a str>,
        name_lte: Option<&'a str>,
        user_country: Option<&'a str>,
        user_country_any_of: Option<&'a str>,
        channel: Option<&'a str>,
        channel_any_of: Option<&'a str>,
        consumer_type: Option<&'a str>,
        consumer_type_any_of: Option<&'a str>,
        parent_name: Option<&'a str>,
        parent_name_any_of: Option<&'a str>,
        parent_name_gt: Option<&'a str>,
        parent_name_gte: Option<&'a str>,
        parent_name_lt: Option<&'a str>,
        parent_name_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, EUMerchantAggregate> {
        self.list_eu_merchant_aggregates_with_params(ListEuMerchantAggregatesParams {
            transaction_date: transaction_date.map(String::from),
            transaction_date_gt: transaction_date_gt.map(String::from),
            transaction_date_gte: transaction_date_gte.map(String::from),
            transaction_date_lt: transaction_date_lt.map(String::from),
            transaction_date_lte: transaction_date_lte.map(String::from),
            name: name.map(String::from),
            name_any_of: name_any_of.map(String::from),
            name_gt: name_gt.map(String::from),
            name_gte: name_gte.map(String::from),
            name_lt: name_lt.map(String::from),
            name_lte: name_lte.map(String::from),
            user_country: user_country.map(String::from),
            user_country_any_of: user_country_any_of.map(String::from),
            channel: channel.map(String::from),
            channel_any_of: channel_any_of.map(String::from),
            consumer_type: consumer_type.map(String::from),
            consumer_type_any_of: consumer_type_any_of.map(String::from),
            parent_name: parent_name.map(String::from),
            parent_name_any_of: parent_name_any_of.map(String::from),
            parent_name_gt: parent_name_gt.map(String::from),
            parent_name_gte: parent_name_gte.map(String::from),
            parent_name_lt: parent_name_lt.map(String::from),
            parent_name_lte: parent_name_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_eu_merchant_aggregates_with_params<'a>(
        &'a self,
        params: ListEuMerchantAggregatesParams,
    ) -> BoxStream<'a, EUMerchantAggregate> {
        Box::pin({
            let ListEuMerchantAggregatesParams {
                transaction_date,
                transaction_date_gt,
                transaction_date_gte,
                transaction_date_lt,
                transaction_date_lte,
                name,
                name_any_of,
                name_gt,
                name_gte,
                name_lt,
                name_lte,
                user_country,
                user_country_any_of,
                channel,
                channel_any_of,
                consumer_type,
                consumer_type_any_of,
                parent_name,
                parent_name_any_of,
                parent_name_gt,
                parent_name_gte,
                parent_name_lt,
                parent_name_lte,
                limit,
                sort,
                options,
            } = params;
            let transaction_date = transaction_date.as_deref();
            let transaction_date_gt = transaction_date_gt.as_deref();
            let transaction_date_gte = transaction_date_gte.as_deref();
            let transaction_date_lt = transaction_date_lt.as_deref();
            let transaction_date_lte = transaction_date_lte.as_deref();
            let name = name.as_deref();
            let name_any_of = name_any_of.as_deref();
            let name_gt = name_gt.as_deref();
            let name_gte = name_gte.as_deref();
            let name_lt = name_lt.as_deref();
            let name_lte = name_lte.as_deref();
            let user_country = user_country.as_deref();
            let user_country_any_of = user_country_any_of.as_deref();
            let channel = channel.as_deref();
            let channel_any_of = channel_any_of.as_deref();
            let consumer_type = consumer_type.as_deref();
            let consumer_type_any_of = consumer_type_any_of.as_deref();
            let parent_name = parent_name.as_deref();
            let parent_name_any_of = parent_name_any_of.as_deref();
            let parent_name_gt = parent_name_gt.as_deref();
            let parent_name_gte = parent_name_gte.as_deref();
            let parent_name_lt = parent_name_lt.as_deref();
            let parent_name_lte = parent_name_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/consumer-spending/eu/v1/merchant-aggregates".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = transaction_date {
                query.push(("transaction_date", v.to_string()));
            }
            if let Some(v) = transaction_date_gt {
                query.push(("transaction_date.gt", v.to_string()));
            }
            if let Some(v) = transaction_date_gte {
                query.push(("transaction_date.gte", v.to_string()));
            }
            if let Some(v) = transaction_date_lt {
                query.push(("transaction_date.lt", v.to_string()));
            }
            if let Some(v) = transaction_date_lte {
                query.push(("transaction_date.lte", v.to_string()));
            }
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
            if let Some(v) = user_country {
                query.push(("user_country", v.to_string()));
            }
            if let Some(v) = user_country_any_of {
                query.push(("user_country.any_of", v.to_string()));
            }
            if let Some(v) = channel {
                query.push(("channel", v.to_string()));
            }
            if let Some(v) = channel_any_of {
                query.push(("channel.any_of", v.to_string()));
            }
            if let Some(v) = consumer_type {
                query.push(("consumer_type", v.to_string()));
            }
            if let Some(v) = consumer_type_any_of {
                query.push(("consumer_type.any_of", v.to_string()));
            }
            if let Some(v) = parent_name {
                query.push(("parent_name", v.to_string()));
            }
            if let Some(v) = parent_name_any_of {
                query.push(("parent_name.any_of", v.to_string()));
            }
            if let Some(v) = parent_name_gt {
                query.push(("parent_name.gt", v.to_string()));
            }
            if let Some(v) = parent_name_gte {
                query.push(("parent_name.gte", v.to_string()));
            }
            if let Some(v) = parent_name_lt {
                query.push(("parent_name.lt", v.to_string()));
            }
            if let Some(v) = parent_name_lte {
                query.push(("parent_name.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<EUMerchantAggregate>(&path, Some(&query), options)
        })
    }

    fn list_eu_merchant_hierarchy<'a>(
        &'a self,
        lookup_name: Option<&'a str>,
        lookup_name_any_of: Option<&'a str>,
        lookup_name_gt: Option<&'a str>,
        lookup_name_gte: Option<&'a str>,
        lookup_name_lt: Option<&'a str>,
        lookup_name_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        listing_status: Option<&'a str>,
        listing_status_any_of: Option<&'a str>,
        active_from: Option<&'a str>,
        active_from_gt: Option<&'a str>,
        active_from_gte: Option<&'a str>,
        active_from_lt: Option<&'a str>,
        active_from_lte: Option<&'a str>,
        active_to: Option<&'a str>,
        active_to_gt: Option<&'a str>,
        active_to_gte: Option<&'a str>,
        active_to_lt: Option<&'a str>,
        active_to_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, EUMerchantHierarchy> {
        self.list_eu_merchant_hierarchy_with_params(ListEuMerchantHierarchyParams {
            lookup_name: lookup_name.map(String::from),
            lookup_name_any_of: lookup_name_any_of.map(String::from),
            lookup_name_gt: lookup_name_gt.map(String::from),
            lookup_name_gte: lookup_name_gte.map(String::from),
            lookup_name_lt: lookup_name_lt.map(String::from),
            lookup_name_lte: lookup_name_lte.map(String::from),
            ticker: ticker.map(String::from),
            ticker_any_of: ticker_any_of.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            listing_status: listing_status.map(String::from),
            listing_status_any_of: listing_status_any_of.map(String::from),
            active_from: active_from.map(String::from),
            active_from_gt: active_from_gt.map(String::from),
            active_from_gte: active_from_gte.map(String::from),
            active_from_lt: active_from_lt.map(String::from),
            active_from_lte: active_from_lte.map(String::from),
            active_to: active_to.map(String::from),
            active_to_gt: active_to_gt.map(String::from),
            active_to_gte: active_to_gte.map(String::from),
            active_to_lt: active_to_lt.map(String::from),
            active_to_lte: active_to_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_eu_merchant_hierarchy_with_params<'a>(
        &'a self,
        params: ListEuMerchantHierarchyParams,
    ) -> BoxStream<'a, EUMerchantHierarchy> {
        Box::pin({
            let ListEuMerchantHierarchyParams {
                lookup_name,
                lookup_name_any_of,
                lookup_name_gt,
                lookup_name_gte,
                lookup_name_lt,
                lookup_name_lte,
                ticker,
                ticker_any_of,
                ticker_gt,
                ticker_gte,
                ticker_lt,
                ticker_lte,
                listing_status,
                listing_status_any_of,
                active_from,
                active_from_gt,
                active_from_gte,
                active_from_lt,
                active_from_lte,
                active_to,
                active_to_gt,
                active_to_gte,
                active_to_lt,
                active_to_lte,
                limit,
                sort,
                options,
            } = params;
            let lookup_name = lookup_name.as_deref();
            let lookup_name_any_of = lookup_name_any_of.as_deref();
            let lookup_name_gt = lookup_name_gt.as_deref();
            let lookup_name_gte = lookup_name_gte.as_deref();
            let lookup_name_lt = lookup_name_lt.as_deref();
            let lookup_name_lte = lookup_name_lte.as_deref();
            let ticker = ticker.as_deref();
            let ticker_any_of = ticker_any_of.as_deref();
            let ticker_gt = ticker_gt.as_deref();
            let ticker_gte = ticker_gte.as_deref();
            let ticker_lt = ticker_lt.as_deref();
            let ticker_lte = ticker_lte.as_deref();
            let listing_status = listing_status.as_deref();
            let listing_status_any_of = listing_status_any_of.as_deref();
            let active_from = active_from.as_deref();
            let active_from_gt = active_from_gt.as_deref();
            let active_from_gte = active_from_gte.as_deref();
            let active_from_lt = active_from_lt.as_deref();
            let active_from_lte = active_from_lte.as_deref();
            let active_to = active_to.as_deref();
            let active_to_gt = active_to_gt.as_deref();
            let active_to_gte = active_to_gte.as_deref();
            let active_to_lt = active_to_lt.as_deref();
            let active_to_lte = active_to_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/consumer-spending/eu/v1/merchant-hierarchy".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = lookup_name {
                query.push(("lookup_name", v.to_string()));
            }
            if let Some(v) = lookup_name_any_of {
                query.push(("lookup_name.any_of", v.to_string()));
            }
            if let Some(v) = lookup_name_gt {
                query.push(("lookup_name.gt", v.to_string()));
            }
            if let Some(v) = lookup_name_gte {
                query.push(("lookup_name.gte", v.to_string()));
            }
            if let Some(v) = lookup_name_lt {
                query.push(("lookup_name.lt", v.to_string()));
            }
            if let Some(v) = lookup_name_lte {
                query.push(("lookup_name.lte", v.to_string()));
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
            if let Some(v) = listing_status {
                query.push(("listing_status", v.to_string()));
            }
            if let Some(v) = listing_status_any_of {
                query.push(("listing_status.any_of", v.to_string()));
            }
            if let Some(v) = active_from {
                query.push(("active_from", v.to_string()));
            }
            if let Some(v) = active_from_gt {
                query.push(("active_from.gt", v.to_string()));
            }
            if let Some(v) = active_from_gte {
                query.push(("active_from.gte", v.to_string()));
            }
            if let Some(v) = active_from_lt {
                query.push(("active_from.lt", v.to_string()));
            }
            if let Some(v) = active_from_lte {
                query.push(("active_from.lte", v.to_string()));
            }
            if let Some(v) = active_to {
                query.push(("active_to", v.to_string()));
            }
            if let Some(v) = active_to_gt {
                query.push(("active_to.gt", v.to_string()));
            }
            if let Some(v) = active_to_gte {
                query.push(("active_to.gte", v.to_string()));
            }
            if let Some(v) = active_to_lt {
                query.push(("active_to.lt", v.to_string()));
            }
            if let Some(v) = active_to_lte {
                query.push(("active_to.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<EUMerchantHierarchy>(&path, Some(&query), options)
        })
    }
}

// --- Params structs (additive builder API) ---

/// Optional arguments for [`EconomyApi::list_treasury_yields`].
#[derive(Debug, Default, Clone)]
pub struct ListTreasuryYieldsParams {
    /// The `date` argument.
    pub date: Option<String>,
    /// The `date_any_of` argument.
    pub date_any_of: Option<String>,
    /// The `date_gt` argument.
    pub date_gt: Option<String>,
    /// The `date_gte` argument.
    pub date_gte: Option<String>,
    /// The `date_lt` argument.
    pub date_lt: Option<String>,
    /// The `date_lte` argument.
    pub date_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `order` argument.
    pub order: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListTreasuryYieldsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `date` argument.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }

    /// Set the `date_any_of` argument.
    pub fn date_any_of(mut self, date_any_of: impl Into<String>) -> Self {
        self.date_any_of = Some(date_any_of.into());
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

/// Optional arguments for [`EconomyApi::list_inflation`].
#[derive(Debug, Default, Clone)]
pub struct ListInflationParams {
    /// The `date` argument.
    pub date: Option<String>,
    /// The `date_any_of` argument.
    pub date_any_of: Option<String>,
    /// The `date_gt` argument.
    pub date_gt: Option<String>,
    /// The `date_gte` argument.
    pub date_gte: Option<String>,
    /// The `date_lt` argument.
    pub date_lt: Option<String>,
    /// The `date_lte` argument.
    pub date_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListInflationParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `date` argument.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }

    /// Set the `date_any_of` argument.
    pub fn date_any_of(mut self, date_any_of: impl Into<String>) -> Self {
        self.date_any_of = Some(date_any_of.into());
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

/// Optional arguments for [`EconomyApi::list_inflation_expectations`].
#[derive(Debug, Default, Clone)]
pub struct ListInflationExpectationsParams {
    /// The `date` argument.
    pub date: Option<String>,
    /// The `date_any_of` argument.
    pub date_any_of: Option<String>,
    /// The `date_gt` argument.
    pub date_gt: Option<String>,
    /// The `date_gte` argument.
    pub date_gte: Option<String>,
    /// The `date_lt` argument.
    pub date_lt: Option<String>,
    /// The `date_lte` argument.
    pub date_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListInflationExpectationsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `date` argument.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }

    /// Set the `date_any_of` argument.
    pub fn date_any_of(mut self, date_any_of: impl Into<String>) -> Self {
        self.date_any_of = Some(date_any_of.into());
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

/// Optional arguments for [`EconomyApi::list_labor_market_indicators`].
#[derive(Debug, Default, Clone)]
pub struct ListLaborMarketIndicatorsParams {
    /// The `date` argument.
    pub date: Option<String>,
    /// The `date_any_of` argument.
    pub date_any_of: Option<String>,
    /// The `date_gt` argument.
    pub date_gt: Option<String>,
    /// The `date_gte` argument.
    pub date_gte: Option<String>,
    /// The `date_lt` argument.
    pub date_lt: Option<String>,
    /// The `date_lte` argument.
    pub date_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListLaborMarketIndicatorsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `date` argument.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }

    /// Set the `date_any_of` argument.
    pub fn date_any_of(mut self, date_any_of: impl Into<String>) -> Self {
        self.date_any_of = Some(date_any_of.into());
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

/// Optional arguments for [`EconomyApi::list_eu_merchant_aggregates`].
#[derive(Debug, Default, Clone)]
pub struct ListEuMerchantAggregatesParams {
    /// The `transaction_date` argument.
    pub transaction_date: Option<String>,
    /// The `transaction_date_gt` argument.
    pub transaction_date_gt: Option<String>,
    /// The `transaction_date_gte` argument.
    pub transaction_date_gte: Option<String>,
    /// The `transaction_date_lt` argument.
    pub transaction_date_lt: Option<String>,
    /// The `transaction_date_lte` argument.
    pub transaction_date_lte: Option<String>,
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
    /// The `user_country` argument.
    pub user_country: Option<String>,
    /// The `user_country_any_of` argument.
    pub user_country_any_of: Option<String>,
    /// The `channel` argument.
    pub channel: Option<String>,
    /// The `channel_any_of` argument.
    pub channel_any_of: Option<String>,
    /// The `consumer_type` argument.
    pub consumer_type: Option<String>,
    /// The `consumer_type_any_of` argument.
    pub consumer_type_any_of: Option<String>,
    /// The `parent_name` argument.
    pub parent_name: Option<String>,
    /// The `parent_name_any_of` argument.
    pub parent_name_any_of: Option<String>,
    /// The `parent_name_gt` argument.
    pub parent_name_gt: Option<String>,
    /// The `parent_name_gte` argument.
    pub parent_name_gte: Option<String>,
    /// The `parent_name_lt` argument.
    pub parent_name_lt: Option<String>,
    /// The `parent_name_lte` argument.
    pub parent_name_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListEuMerchantAggregatesParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `transaction_date` argument.
    pub fn transaction_date(mut self, transaction_date: impl Into<String>) -> Self {
        self.transaction_date = Some(transaction_date.into());
        self
    }

    /// Set the `transaction_date_gt` argument.
    pub fn transaction_date_gt(mut self, transaction_date_gt: impl Into<String>) -> Self {
        self.transaction_date_gt = Some(transaction_date_gt.into());
        self
    }

    /// Set the `transaction_date_gte` argument.
    pub fn transaction_date_gte(mut self, transaction_date_gte: impl Into<String>) -> Self {
        self.transaction_date_gte = Some(transaction_date_gte.into());
        self
    }

    /// Set the `transaction_date_lt` argument.
    pub fn transaction_date_lt(mut self, transaction_date_lt: impl Into<String>) -> Self {
        self.transaction_date_lt = Some(transaction_date_lt.into());
        self
    }

    /// Set the `transaction_date_lte` argument.
    pub fn transaction_date_lte(mut self, transaction_date_lte: impl Into<String>) -> Self {
        self.transaction_date_lte = Some(transaction_date_lte.into());
        self
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

    /// Set the `user_country` argument.
    pub fn user_country(mut self, user_country: impl Into<String>) -> Self {
        self.user_country = Some(user_country.into());
        self
    }

    /// Set the `user_country_any_of` argument.
    pub fn user_country_any_of(mut self, user_country_any_of: impl Into<String>) -> Self {
        self.user_country_any_of = Some(user_country_any_of.into());
        self
    }

    /// Set the `channel` argument.
    pub fn channel(mut self, channel: impl Into<String>) -> Self {
        self.channel = Some(channel.into());
        self
    }

    /// Set the `channel_any_of` argument.
    pub fn channel_any_of(mut self, channel_any_of: impl Into<String>) -> Self {
        self.channel_any_of = Some(channel_any_of.into());
        self
    }

    /// Set the `consumer_type` argument.
    pub fn consumer_type(mut self, consumer_type: impl Into<String>) -> Self {
        self.consumer_type = Some(consumer_type.into());
        self
    }

    /// Set the `consumer_type_any_of` argument.
    pub fn consumer_type_any_of(mut self, consumer_type_any_of: impl Into<String>) -> Self {
        self.consumer_type_any_of = Some(consumer_type_any_of.into());
        self
    }

    /// Set the `parent_name` argument.
    pub fn parent_name(mut self, parent_name: impl Into<String>) -> Self {
        self.parent_name = Some(parent_name.into());
        self
    }

    /// Set the `parent_name_any_of` argument.
    pub fn parent_name_any_of(mut self, parent_name_any_of: impl Into<String>) -> Self {
        self.parent_name_any_of = Some(parent_name_any_of.into());
        self
    }

    /// Set the `parent_name_gt` argument.
    pub fn parent_name_gt(mut self, parent_name_gt: impl Into<String>) -> Self {
        self.parent_name_gt = Some(parent_name_gt.into());
        self
    }

    /// Set the `parent_name_gte` argument.
    pub fn parent_name_gte(mut self, parent_name_gte: impl Into<String>) -> Self {
        self.parent_name_gte = Some(parent_name_gte.into());
        self
    }

    /// Set the `parent_name_lt` argument.
    pub fn parent_name_lt(mut self, parent_name_lt: impl Into<String>) -> Self {
        self.parent_name_lt = Some(parent_name_lt.into());
        self
    }

    /// Set the `parent_name_lte` argument.
    pub fn parent_name_lte(mut self, parent_name_lte: impl Into<String>) -> Self {
        self.parent_name_lte = Some(parent_name_lte.into());
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

/// Optional arguments for [`EconomyApi::list_eu_merchant_hierarchy`].
#[derive(Debug, Default, Clone)]
pub struct ListEuMerchantHierarchyParams {
    /// The `lookup_name` argument.
    pub lookup_name: Option<String>,
    /// The `lookup_name_any_of` argument.
    pub lookup_name_any_of: Option<String>,
    /// The `lookup_name_gt` argument.
    pub lookup_name_gt: Option<String>,
    /// The `lookup_name_gte` argument.
    pub lookup_name_gte: Option<String>,
    /// The `lookup_name_lt` argument.
    pub lookup_name_lt: Option<String>,
    /// The `lookup_name_lte` argument.
    pub lookup_name_lte: Option<String>,
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
    /// The `listing_status` argument.
    pub listing_status: Option<String>,
    /// The `listing_status_any_of` argument.
    pub listing_status_any_of: Option<String>,
    /// The `active_from` argument.
    pub active_from: Option<String>,
    /// The `active_from_gt` argument.
    pub active_from_gt: Option<String>,
    /// The `active_from_gte` argument.
    pub active_from_gte: Option<String>,
    /// The `active_from_lt` argument.
    pub active_from_lt: Option<String>,
    /// The `active_from_lte` argument.
    pub active_from_lte: Option<String>,
    /// The `active_to` argument.
    pub active_to: Option<String>,
    /// The `active_to_gt` argument.
    pub active_to_gt: Option<String>,
    /// The `active_to_gte` argument.
    pub active_to_gte: Option<String>,
    /// The `active_to_lt` argument.
    pub active_to_lt: Option<String>,
    /// The `active_to_lte` argument.
    pub active_to_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListEuMerchantHierarchyParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `lookup_name` argument.
    pub fn lookup_name(mut self, lookup_name: impl Into<String>) -> Self {
        self.lookup_name = Some(lookup_name.into());
        self
    }

    /// Set the `lookup_name_any_of` argument.
    pub fn lookup_name_any_of(mut self, lookup_name_any_of: impl Into<String>) -> Self {
        self.lookup_name_any_of = Some(lookup_name_any_of.into());
        self
    }

    /// Set the `lookup_name_gt` argument.
    pub fn lookup_name_gt(mut self, lookup_name_gt: impl Into<String>) -> Self {
        self.lookup_name_gt = Some(lookup_name_gt.into());
        self
    }

    /// Set the `lookup_name_gte` argument.
    pub fn lookup_name_gte(mut self, lookup_name_gte: impl Into<String>) -> Self {
        self.lookup_name_gte = Some(lookup_name_gte.into());
        self
    }

    /// Set the `lookup_name_lt` argument.
    pub fn lookup_name_lt(mut self, lookup_name_lt: impl Into<String>) -> Self {
        self.lookup_name_lt = Some(lookup_name_lt.into());
        self
    }

    /// Set the `lookup_name_lte` argument.
    pub fn lookup_name_lte(mut self, lookup_name_lte: impl Into<String>) -> Self {
        self.lookup_name_lte = Some(lookup_name_lte.into());
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

    /// Set the `listing_status` argument.
    pub fn listing_status(mut self, listing_status: impl Into<String>) -> Self {
        self.listing_status = Some(listing_status.into());
        self
    }

    /// Set the `listing_status_any_of` argument.
    pub fn listing_status_any_of(mut self, listing_status_any_of: impl Into<String>) -> Self {
        self.listing_status_any_of = Some(listing_status_any_of.into());
        self
    }

    /// Set the `active_from` argument.
    pub fn active_from(mut self, active_from: impl Into<String>) -> Self {
        self.active_from = Some(active_from.into());
        self
    }

    /// Set the `active_from_gt` argument.
    pub fn active_from_gt(mut self, active_from_gt: impl Into<String>) -> Self {
        self.active_from_gt = Some(active_from_gt.into());
        self
    }

    /// Set the `active_from_gte` argument.
    pub fn active_from_gte(mut self, active_from_gte: impl Into<String>) -> Self {
        self.active_from_gte = Some(active_from_gte.into());
        self
    }

    /// Set the `active_from_lt` argument.
    pub fn active_from_lt(mut self, active_from_lt: impl Into<String>) -> Self {
        self.active_from_lt = Some(active_from_lt.into());
        self
    }

    /// Set the `active_from_lte` argument.
    pub fn active_from_lte(mut self, active_from_lte: impl Into<String>) -> Self {
        self.active_from_lte = Some(active_from_lte.into());
        self
    }

    /// Set the `active_to` argument.
    pub fn active_to(mut self, active_to: impl Into<String>) -> Self {
        self.active_to = Some(active_to.into());
        self
    }

    /// Set the `active_to_gt` argument.
    pub fn active_to_gt(mut self, active_to_gt: impl Into<String>) -> Self {
        self.active_to_gt = Some(active_to_gt.into());
        self
    }

    /// Set the `active_to_gte` argument.
    pub fn active_to_gte(mut self, active_to_gte: impl Into<String>) -> Self {
        self.active_to_gte = Some(active_to_gte.into());
        self
    }

    /// Set the `active_to_lt` argument.
    pub fn active_to_lt(mut self, active_to_lt: impl Into<String>) -> Self {
        self.active_to_lt = Some(active_to_lt.into());
        self
    }

    /// Set the `active_to_lte` argument.
    pub fn active_to_lte(mut self, active_to_lte: impl Into<String>) -> Self {
        self.active_to_lte = Some(active_to_lte.into());
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
