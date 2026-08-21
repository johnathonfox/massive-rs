use super::{BoxFuture, BoxStream};
use crate::client::{Client, RequestOptions};
use crate::models::{
    Condition, Disclosure, DisclosureTaxonomy, Dividend, Exchange, Filing13F, Filing8K,
    FilingForm3, FilingForm4, FilingIndex, FilingSection, MarketHoliday, MarketStatus,
    OptionsContract, RelatedCompany, RiskFactor, RiskFactorTaxonomy, ShortInterest, ShortVolume,
    Split, StockDividend, StockSplit, Ticker, TickerChangeResults, TickerDetails, TickerNews,
    TickerTypes,
};

/// Reference data API.
pub trait ReferenceApi {
    /// Get upcoming market holidays and their open/close times.
    fn get_market_holidays<'a>(
        &'a self,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<MarketHoliday>>;

    /// Same as [`Self::get_market_holidays`], but takes the optional arguments as a
    /// chainable [`GetMarketHolidaysParams`] struct.
    fn get_market_holidays_with_params<'a>(
        &'a self,
        params: GetMarketHolidaysParams,
    ) -> BoxFuture<'a, Vec<MarketHoliday>>;

    /// Get the current trading status of the exchanges and overall financial markets.
    fn get_market_status<'a>(
        &'a self,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, MarketStatus>;

    /// Same as [`Self::get_market_status`], but takes the optional arguments as a
    /// chainable [`GetMarketStatusParams`] struct.
    fn get_market_status_with_params<'a>(
        &'a self,
        params: GetMarketStatusParams,
    ) -> BoxFuture<'a, MarketStatus>;

    /// Query all ticker symbols supported by Massive.com (stocks, indices, forex, crypto).
    fn list_tickers<'a>(
        &'a self,
        ticker: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        r#type: Option<&'a str>,
        market: Option<&'a str>,
        exchange: Option<&'a str>,
        cusip: Option<i64>,
        cik: Option<i64>,
        date: Option<&'a str>,
        active: Option<bool>,
        search: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, Ticker>;

    /// Same as [`Self::list_tickers`], but takes the optional arguments as a
    /// chainable [`ListTickersParams`] struct.
    fn list_tickers_with_params<'a>(&'a self, params: ListTickersParams) -> BoxStream<'a, Ticker>;

    /// Get detailed information about a single ticker and the company behind it.
    fn get_ticker_details<'a>(
        &'a self,
        ticker: &'a str,
        date: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, TickerDetails>;

    /// Same as [`Self::get_ticker_details`], but takes the optional arguments as a
    /// chainable [`GetTickerDetailsParams`] struct.
    fn get_ticker_details_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetTickerDetailsParams,
    ) -> BoxFuture<'a, TickerDetails>;

    /// Get event history of a ticker given a particular point in time.
    fn get_ticker_events<'a>(
        &'a self,
        ticker: &'a str,
        types: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, TickerChangeResults>;

    /// Same as [`Self::get_ticker_events`], but takes the optional arguments as a
    /// chainable [`GetTickerEventsParams`] struct.
    fn get_ticker_events_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetTickerEventsParams,
    ) -> BoxFuture<'a, TickerChangeResults>;

    /// Get the most recent news articles relating to a stock ticker symbol.
    fn list_ticker_news<'a>(
        &'a self,
        ticker: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        published_utc: Option<&'a str>,
        published_utc_lt: Option<&'a str>,
        published_utc_lte: Option<&'a str>,
        published_utc_gt: Option<&'a str>,
        published_utc_gte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, TickerNews>;

    /// Same as [`Self::list_ticker_news`], but takes the optional arguments as a
    /// chainable [`ListTickerNewsParams`] struct.
    fn list_ticker_news_with_params<'a>(
        &'a self,
        params: ListTickerNewsParams,
    ) -> BoxStream<'a, TickerNews>;

    /// List all ticker types that Massive.com has.
    fn get_ticker_types<'a>(
        &'a self,
        asset_class: Option<&'a str>,
        locale: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<TickerTypes>>;

    /// Same as [`Self::get_ticker_types`], but takes the optional arguments as a
    /// chainable [`GetTickerTypesParams`] struct.
    fn get_ticker_types_with_params<'a>(
        &'a self,
        params: GetTickerTypesParams,
    ) -> BoxFuture<'a, Vec<TickerTypes>>;

    /// Get a list of tickers related to the queried ticker based on News and Returns data.
    fn get_related_companies<'a>(
        &'a self,
        ticker: &'a str,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, RelatedCompany>;

    /// Same as [`Self::get_related_companies`], but takes the optional arguments as a
    /// chainable [`GetRelatedCompaniesParams`] struct.
    fn get_related_companies_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetRelatedCompaniesParams,
    ) -> BoxFuture<'a, RelatedCompany>;

    /// Get a list of historical stock splits.
    fn list_splits<'a>(
        &'a self,
        ticker: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        execution_date: Option<&'a str>,
        execution_date_lt: Option<&'a str>,
        execution_date_lte: Option<&'a str>,
        execution_date_gt: Option<&'a str>,
        execution_date_gte: Option<&'a str>,
        reverse_split: Option<bool>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, Split>;

    /// Same as [`Self::list_splits`], but takes the optional arguments as a
    /// chainable [`ListSplitsParams`] struct.
    fn list_splits_with_params<'a>(&'a self, params: ListSplitsParams) -> BoxStream<'a, Split>;

    /// Get a list of historical cash dividends.
    fn list_dividends<'a>(
        &'a self,
        ticker: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ex_dividend_date: Option<&'a str>,
        ex_dividend_date_lt: Option<&'a str>,
        ex_dividend_date_lte: Option<&'a str>,
        ex_dividend_date_gt: Option<&'a str>,
        ex_dividend_date_gte: Option<&'a str>,
        record_date: Option<&'a str>,
        record_date_lt: Option<&'a str>,
        record_date_lte: Option<&'a str>,
        record_date_gt: Option<&'a str>,
        record_date_gte: Option<&'a str>,
        declaration_date: Option<&'a str>,
        declaration_date_lt: Option<&'a str>,
        declaration_date_lte: Option<&'a str>,
        declaration_date_gt: Option<&'a str>,
        declaration_date_gte: Option<&'a str>,
        pay_date: Option<&'a str>,
        pay_date_lt: Option<&'a str>,
        pay_date_lte: Option<&'a str>,
        pay_date_gt: Option<&'a str>,
        pay_date_gte: Option<&'a str>,
        frequency: Option<i64>,
        cash_amount: Option<f64>,
        cash_amount_lt: Option<f64>,
        cash_amount_lte: Option<f64>,
        cash_amount_gt: Option<f64>,
        cash_amount_gte: Option<f64>,
        dividend_type: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, Dividend>;

    /// Same as [`Self::list_dividends`], but takes the optional arguments as a
    /// chainable [`ListDividendsParams`] struct.
    fn list_dividends_with_params<'a>(
        &'a self,
        params: ListDividendsParams,
    ) -> BoxStream<'a, Dividend>;

    /// List all conditions that Massive.com uses.
    fn list_conditions<'a>(
        &'a self,
        asset_class: Option<&'a str>,
        data_type: Option<&'a str>,
        id: Option<i64>,
        sip: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, Condition>;

    /// Same as [`Self::list_conditions`], but takes the optional arguments as a
    /// chainable [`ListConditionsParams`] struct.
    fn list_conditions_with_params<'a>(
        &'a self,
        params: ListConditionsParams,
    ) -> BoxStream<'a, Condition>;

    /// List all exchanges that Massive.com knows about.
    fn get_exchanges<'a>(
        &'a self,
        asset_class: Option<&'a str>,
        locale: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<Exchange>>;

    /// Same as [`Self::get_exchanges`], but takes the optional arguments as a
    /// chainable [`GetExchangesParams`] struct.
    fn get_exchanges_with_params<'a>(
        &'a self,
        params: GetExchangesParams,
    ) -> BoxFuture<'a, Vec<Exchange>>;

    /// Get a single options contract by ticker.
    fn get_options_contract<'a>(
        &'a self,
        ticker: &'a str,
        as_of: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, OptionsContract>;

    /// Same as [`Self::get_options_contract`], but takes the optional arguments as a
    /// chainable [`GetOptionsContractParams`] struct.
    fn get_options_contract_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetOptionsContractParams,
    ) -> BoxFuture<'a, OptionsContract>;

    /// List historical options contracts.
    fn list_options_contracts<'a>(
        &'a self,
        underlying_ticker: Option<&'a str>,
        underlying_ticker_lt: Option<&'a str>,
        underlying_ticker_lte: Option<&'a str>,
        underlying_ticker_gt: Option<&'a str>,
        underlying_ticker_gte: Option<&'a str>,
        contract_type: Option<&'a str>,
        expiration_date: Option<&'a str>,
        expiration_date_lt: Option<&'a str>,
        expiration_date_lte: Option<&'a str>,
        expiration_date_gt: Option<&'a str>,
        expiration_date_gte: Option<&'a str>,
        as_of: Option<&'a str>,
        strike_price: Option<f64>,
        strike_price_lt: Option<f64>,
        strike_price_lte: Option<f64>,
        strike_price_gt: Option<f64>,
        strike_price_gte: Option<f64>,
        expired: Option<bool>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, OptionsContract>;

    /// Same as [`Self::list_options_contracts`], but takes the optional arguments as a
    /// chainable [`ListOptionsContractsParams`] struct.
    fn list_options_contracts_with_params<'a>(
        &'a self,
        params: ListOptionsContractsParams,
    ) -> BoxStream<'a, OptionsContract>;

    /// Retrieve short interest data for stocks.
    fn list_short_interest<'a>(
        &'a self,
        ticker: Option<&'a str>,
        days_to_cover: Option<&'a str>,
        days_to_cover_lt: Option<&'a str>,
        days_to_cover_lte: Option<&'a str>,
        days_to_cover_gt: Option<&'a str>,
        days_to_cover_gte: Option<&'a str>,
        settlement_date: Option<&'a str>,
        settlement_date_lt: Option<&'a str>,
        settlement_date_lte: Option<&'a str>,
        settlement_date_gt: Option<&'a str>,
        settlement_date_gte: Option<&'a str>,
        avg_daily_volume: Option<&'a str>,
        avg_daily_volume_lt: Option<&'a str>,
        avg_daily_volume_lte: Option<&'a str>,
        avg_daily_volume_gt: Option<&'a str>,
        avg_daily_volume_gte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, ShortInterest>;

    /// Same as [`Self::list_short_interest`], but takes the optional arguments as a
    /// chainable [`ListShortInterestParams`] struct.
    fn list_short_interest_with_params<'a>(
        &'a self,
        params: ListShortInterestParams,
    ) -> BoxStream<'a, ShortInterest>;

    /// Retrieve short volume data for stocks.
    fn list_short_volume<'a>(
        &'a self,
        ticker: Option<&'a str>,
        date: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        short_volume_ratio: Option<&'a str>,
        short_volume_ratio_lt: Option<&'a str>,
        short_volume_ratio_lte: Option<&'a str>,
        short_volume_ratio_gt: Option<&'a str>,
        short_volume_ratio_gte: Option<&'a str>,
        total_volume: Option<&'a str>,
        total_volume_lt: Option<&'a str>,
        total_volume_lte: Option<&'a str>,
        total_volume_gt: Option<&'a str>,
        total_volume_gte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, ShortVolume>;

    /// Same as [`Self::list_short_volume`], but takes the optional arguments as a
    /// chainable [`ListShortVolumeParams`] struct.
    fn list_short_volume_with_params<'a>(
        &'a self,
        params: ListShortVolumeParams,
    ) -> BoxStream<'a, ShortVolume>;

    /// List stock splits (GET /stocks/v1/splits).
    fn list_stocks_splits<'a>(
        &'a self,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        execution_date: Option<&'a str>,
        execution_date_gt: Option<&'a str>,
        execution_date_gte: Option<&'a str>,
        execution_date_lt: Option<&'a str>,
        execution_date_lte: Option<&'a str>,
        adjustment_type: Option<&'a str>,
        adjustment_type_any_of: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, StockSplit>;

    /// Same as [`Self::list_stocks_splits`], but takes the optional arguments as a
    /// chainable [`ListStocksSplitsParams`] struct.
    fn list_stocks_splits_with_params<'a>(
        &'a self,
        params: ListStocksSplitsParams,
    ) -> BoxStream<'a, StockSplit>;

    /// List stock dividends (GET /stocks/v1/dividends).
    fn list_stocks_dividends<'a>(
        &'a self,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        ex_dividend_date: Option<&'a str>,
        ex_dividend_date_gt: Option<&'a str>,
        ex_dividend_date_gte: Option<&'a str>,
        ex_dividend_date_lt: Option<&'a str>,
        ex_dividend_date_lte: Option<&'a str>,
        frequency: Option<i64>,
        frequency_gt: Option<i64>,
        frequency_gte: Option<i64>,
        frequency_lt: Option<i64>,
        frequency_lte: Option<i64>,
        distribution_type: Option<&'a str>,
        distribution_type_any_of: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, StockDividend>;

    /// Same as [`Self::list_stocks_dividends`], but takes the optional arguments as a
    /// chainable [`ListStocksDividendsParams`] struct.
    fn list_stocks_dividends_with_params<'a>(
        &'a self,
        params: ListStocksDividendsParams,
    ) -> BoxStream<'a, StockDividend>;

    /// Get categorized risk factors extracted from 10-K filings (with supporting_text).
    fn list_stocks_filings_risk_factors<'a>(
        &'a self,
        filing_date: Option<&'a str>,
        filing_date_any_of: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        cik: Option<&'a str>,
        cik_any_of: Option<&'a str>,
        cik_gt: Option<&'a str>,
        cik_gte: Option<&'a str>,
        cik_lt: Option<&'a str>,
        cik_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, RiskFactor>;

    /// Same as [`Self::list_stocks_filings_risk_factors`], but takes the optional arguments as a
    /// chainable [`ListStocksFilingsRiskFactorsParams`] struct.
    fn list_stocks_filings_risk_factors_with_params<'a>(
        &'a self,
        params: ListStocksFilingsRiskFactorsParams,
    ) -> BoxStream<'a, RiskFactor>;

    /// Get the taxonomy/categories used to classify risk factors.
    fn list_stocks_taxonomies_risk_factors<'a>(
        &'a self,
        taxonomy: Option<f64>,
        taxonomy_gt: Option<f64>,
        taxonomy_gte: Option<f64>,
        taxonomy_lt: Option<f64>,
        taxonomy_lte: Option<f64>,
        primary_category: Option<&'a str>,
        primary_category_any_of: Option<&'a str>,
        primary_category_gt: Option<&'a str>,
        primary_category_gte: Option<&'a str>,
        primary_category_lt: Option<&'a str>,
        primary_category_lte: Option<&'a str>,
        secondary_category: Option<&'a str>,
        secondary_category_any_of: Option<&'a str>,
        secondary_category_gt: Option<&'a str>,
        secondary_category_gte: Option<&'a str>,
        secondary_category_lt: Option<&'a str>,
        secondary_category_lte: Option<&'a str>,
        tertiary_category: Option<&'a str>,
        tertiary_category_any_of: Option<&'a str>,
        tertiary_category_gt: Option<&'a str>,
        tertiary_category_gte: Option<&'a str>,
        tertiary_category_lt: Option<&'a str>,
        tertiary_category_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, RiskFactorTaxonomy>;

    /// Same as [`Self::list_stocks_taxonomies_risk_factors`], but takes the optional arguments as a
    /// chainable [`ListStocksTaxonomiesRiskFactorsParams`] struct.
    fn list_stocks_taxonomies_risk_factors_with_params<'a>(
        &'a self,
        params: ListStocksTaxonomiesRiskFactorsParams,
    ) -> BoxStream<'a, RiskFactorTaxonomy>;

    /// SEC 8-K filing disclosure categorization.
    fn list_stocks_filings_8k_disclosures<'a>(
        &'a self,
        cik: Option<&'a str>,
        cik_any_of: Option<&'a str>,
        tickers: Option<&'a str>,
        tickers_all_of: Option<&'a str>,
        tickers_any_of: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_any_of: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        tertiary_category: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, Disclosure>;

    /// Same as [`Self::list_stocks_filings_8k_disclosures`], but takes the optional arguments as a
    /// chainable [`ListStocksFilings8kDisclosuresParams`] struct.
    fn list_stocks_filings_8k_disclosures_with_params<'a>(
        &'a self,
        params: ListStocksFilings8kDisclosuresParams,
    ) -> BoxStream<'a, Disclosure>;

    /// The complete list of 8-K disclosure classifications.
    fn list_stocks_taxonomies_disclosures<'a>(
        &'a self,
        taxonomy: Option<&'a str>,
        taxonomy_any_of: Option<&'a str>,
        taxonomy_gt: Option<&'a str>,
        taxonomy_gte: Option<&'a str>,
        taxonomy_lt: Option<&'a str>,
        taxonomy_lte: Option<&'a str>,
        primary_category: Option<&'a str>,
        primary_category_any_of: Option<&'a str>,
        primary_category_gt: Option<&'a str>,
        primary_category_gte: Option<&'a str>,
        primary_category_lt: Option<&'a str>,
        primary_category_lte: Option<&'a str>,
        secondary_category: Option<&'a str>,
        secondary_category_any_of: Option<&'a str>,
        secondary_category_gt: Option<&'a str>,
        secondary_category_gte: Option<&'a str>,
        secondary_category_lt: Option<&'a str>,
        secondary_category_lte: Option<&'a str>,
        tertiary_category: Option<&'a str>,
        tertiary_category_any_of: Option<&'a str>,
        tertiary_category_gt: Option<&'a str>,
        tertiary_category_gte: Option<&'a str>,
        tertiary_category_lt: Option<&'a str>,
        tertiary_category_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, DisclosureTaxonomy>;

    /// Same as [`Self::list_stocks_taxonomies_disclosures`], but takes the optional arguments as a
    /// chainable [`ListStocksTaxonomiesDisclosuresParams`] struct.
    fn list_stocks_taxonomies_disclosures_with_params<'a>(
        &'a self,
        params: ListStocksTaxonomiesDisclosuresParams,
    ) -> BoxStream<'a, DisclosureTaxonomy>;

    /// Get raw text sections from 10-K/10-Q filings (business, risk_factors, etc.).
    fn list_stocks_filings_10k_sections<'a>(
        &'a self,
        cik: Option<&'a str>,
        cik_any_of: Option<&'a str>,
        cik_gt: Option<&'a str>,
        cik_gte: Option<&'a str>,
        cik_lt: Option<&'a str>,
        cik_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        section: Option<&'a str>,
        section_any_of: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        period_end: Option<&'a str>,
        period_end_gt: Option<&'a str>,
        period_end_gte: Option<&'a str>,
        period_end_lt: Option<&'a str>,
        period_end_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FilingSection>;

    /// Same as [`Self::list_stocks_filings_10k_sections`], but takes the optional arguments as a
    /// chainable [`ListStocksFilings10kSectionsParams`] struct.
    fn list_stocks_filings_10k_sections_with_params<'a>(
        &'a self,
        params: ListStocksFilings10kSectionsParams,
    ) -> BoxStream<'a, FilingSection>;

    /// Get parsed 8-K filings (earnings, acquisitions, executive changes, etc.).
    fn list_stocks_filings_8k_text<'a>(
        &'a self,
        cik: Option<&'a str>,
        cik_any_of: Option<&'a str>,
        cik_gt: Option<&'a str>,
        cik_gte: Option<&'a str>,
        cik_lt: Option<&'a str>,
        cik_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        form_type: Option<&'a str>,
        form_type_any_of: Option<&'a str>,
        form_type_gt: Option<&'a str>,
        form_type_gte: Option<&'a str>,
        form_type_lt: Option<&'a str>,
        form_type_lte: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, Filing8K>;

    /// Same as [`Self::list_stocks_filings_8k_text`], but takes the optional arguments as a
    /// chainable [`ListStocksFilings8kTextParams`] struct.
    fn list_stocks_filings_8k_text_with_params<'a>(
        &'a self,
        params: ListStocksFilings8kTextParams,
    ) -> BoxStream<'a, Filing8K>;

    /// Get the master index of all SEC filings (10-K, 8-K, 10-Q, S-1, 4, etc.).
    fn list_stocks_filings_index<'a>(
        &'a self,
        cik: Option<&'a str>,
        cik_any_of: Option<&'a str>,
        cik_gt: Option<&'a str>,
        cik_gte: Option<&'a str>,
        cik_lt: Option<&'a str>,
        cik_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        form_type: Option<&'a str>,
        form_type_any_of: Option<&'a str>,
        form_type_gt: Option<&'a str>,
        form_type_gte: Option<&'a str>,
        form_type_lt: Option<&'a str>,
        form_type_lte: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FilingIndex>;

    /// Same as [`Self::list_stocks_filings_index`], but takes the optional arguments as a
    /// chainable [`ListStocksFilingsIndexParams`] struct.
    fn list_stocks_filings_index_with_params<'a>(
        &'a self,
        params: ListStocksFilingsIndexParams,
    ) -> BoxStream<'a, FilingIndex>;

    /// SEC Form 13F filings data showing institutional investment manager holdings.
    fn list_stocks_filings_13f<'a>(
        &'a self,
        filer_cik: Option<&'a str>,
        filer_cik_any_of: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, Filing13F>;

    /// Same as [`Self::list_stocks_filings_13f`], but takes the optional arguments as a
    /// chainable [`ListStocksFilings13fParams`] struct.
    fn list_stocks_filings_13f_with_params<'a>(
        &'a self,
        params: ListStocksFilings13fParams,
    ) -> BoxStream<'a, Filing13F>;

    /// SEC Form 3 filings reporting initial statements of beneficial ownership of securities.
    fn list_stocks_filings_form_3<'a>(
        &'a self,
        issuer_cik: Option<&'a str>,
        issuer_cik_any_of: Option<&'a str>,
        owner_cik: Option<&'a str>,
        owner_cik_any_of: Option<&'a str>,
        tickers: Option<&'a str>,
        tickers_all_of: Option<&'a str>,
        tickers_any_of: Option<&'a str>,
        form_type: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        max_ticker: Option<&'a str>,
        max_ticker_any_of: Option<&'a str>,
        max_ticker_gt: Option<&'a str>,
        max_ticker_gte: Option<&'a str>,
        max_ticker_lt: Option<&'a str>,
        max_ticker_lte: Option<&'a str>,
        min_ticker: Option<&'a str>,
        min_ticker_any_of: Option<&'a str>,
        min_ticker_gt: Option<&'a str>,
        min_ticker_gte: Option<&'a str>,
        min_ticker_lt: Option<&'a str>,
        min_ticker_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FilingForm3>;

    /// Same as [`Self::list_stocks_filings_form_3`], but takes the optional arguments as a
    /// chainable [`ListStocksFilingsForm3Params`] struct.
    fn list_stocks_filings_form_3_with_params<'a>(
        &'a self,
        params: ListStocksFilingsForm3Params,
    ) -> BoxStream<'a, FilingForm3>;

    /// SEC Form 4 filings reporting changes in beneficial ownership of securities.
    fn list_stocks_filings_form_4<'a>(
        &'a self,
        issuer_cik: Option<&'a str>,
        issuer_cik_any_of: Option<&'a str>,
        owner_cik: Option<&'a str>,
        owner_cik_any_of: Option<&'a str>,
        tickers: Option<&'a str>,
        tickers_all_of: Option<&'a str>,
        tickers_any_of: Option<&'a str>,
        form_type: Option<&'a str>,
        transaction_code: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        max_ticker: Option<&'a str>,
        max_ticker_any_of: Option<&'a str>,
        max_ticker_gt: Option<&'a str>,
        max_ticker_gte: Option<&'a str>,
        max_ticker_lt: Option<&'a str>,
        max_ticker_lte: Option<&'a str>,
        min_ticker: Option<&'a str>,
        min_ticker_any_of: Option<&'a str>,
        min_ticker_gt: Option<&'a str>,
        min_ticker_gte: Option<&'a str>,
        min_ticker_lt: Option<&'a str>,
        min_ticker_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FilingForm4>;

    /// Same as [`Self::list_stocks_filings_form_4`], but takes the optional arguments as a
    /// chainable [`ListStocksFilingsForm4Params`] struct.
    fn list_stocks_filings_form_4_with_params<'a>(
        &'a self,
        params: ListStocksFilingsForm4Params,
    ) -> BoxStream<'a, FilingForm4>;
}

impl ReferenceApi for Client {
    fn get_market_holidays<'a>(
        &'a self,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<MarketHoliday>> {
        self.get_market_holidays_with_params(GetMarketHolidaysParams {
            options: options.cloned(),
        })
    }

    fn get_market_holidays_with_params<'a>(
        &'a self,
        params: GetMarketHolidaysParams,
    ) -> BoxFuture<'a, Vec<MarketHoliday>> {
        Box::pin(async move {
            let GetMarketHolidaysParams { options } = params;
            let options = options.as_ref();
            let path = "/v1/marketstatus/upcoming".to_string();
            self.get(&path, None, options).await
        })
    }

    fn get_market_status<'a>(
        &'a self,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, MarketStatus> {
        self.get_market_status_with_params(GetMarketStatusParams {
            options: options.cloned(),
        })
    }

    fn get_market_status_with_params<'a>(
        &'a self,
        params: GetMarketStatusParams,
    ) -> BoxFuture<'a, MarketStatus> {
        Box::pin(async move {
            let GetMarketStatusParams { options } = params;
            let options = options.as_ref();
            let path = "/v1/marketstatus/now".to_string();
            self.get(&path, None, options).await
        })
    }

    fn list_tickers<'a>(
        &'a self,
        ticker: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        r#type: Option<&'a str>,
        market: Option<&'a str>,
        exchange: Option<&'a str>,
        cusip: Option<i64>,
        cik: Option<i64>,
        date: Option<&'a str>,
        active: Option<bool>,
        search: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, Ticker> {
        self.list_tickers_with_params(ListTickersParams {
            ticker: ticker.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            r#type: r#type.map(String::from),
            market: market.map(String::from),
            exchange: exchange.map(String::from),
            cusip,
            cik,
            date: date.map(String::from),
            active,
            search: search.map(String::from),
            limit,
            sort: sort.map(String::from),
            order: order.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_tickers_with_params<'a>(&'a self, params: ListTickersParams) -> BoxStream<'a, Ticker> {
        Box::pin({
            let ListTickersParams {
                ticker,
                ticker_lt,
                ticker_lte,
                ticker_gt,
                ticker_gte,
                r#type,
                market,
                exchange,
                cusip,
                cik,
                date,
                active,
                search,
                limit,
                sort,
                order,
                options,
            } = params;
            let ticker = ticker.as_deref();
            let ticker_lt = ticker_lt.as_deref();
            let ticker_lte = ticker_lte.as_deref();
            let ticker_gt = ticker_gt.as_deref();
            let ticker_gte = ticker_gte.as_deref();
            let r#type = r#type.as_deref();
            let market = market.as_deref();
            let exchange = exchange.as_deref();
            let date = date.as_deref();
            let search = search.as_deref();
            let sort = sort.as_deref();
            let order = order.as_deref();
            let options = options.as_ref();
            let path = "/v3/reference/tickers".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = ticker {
                query.push(("ticker", v.to_string()));
            }
            if let Some(v) = ticker_lt {
                query.push(("ticker.lt", v.to_string()));
            }
            if let Some(v) = ticker_lte {
                query.push(("ticker.lte", v.to_string()));
            }
            if let Some(v) = ticker_gt {
                query.push(("ticker.gt", v.to_string()));
            }
            if let Some(v) = ticker_gte {
                query.push(("ticker.gte", v.to_string()));
            }
            if let Some(v) = r#type {
                query.push(("type", v.to_string()));
            }
            if let Some(v) = market {
                query.push(("market", v.to_string()));
            }
            if let Some(v) = exchange {
                query.push(("exchange", v.to_string()));
            }
            if let Some(v) = cusip {
                query.push(("cusip", v.to_string()));
            }
            if let Some(v) = cik {
                query.push(("cik", v.to_string()));
            }
            if let Some(v) = date {
                query.push(("date", v.to_string()));
            }
            if let Some(v) = active {
                query.push(("active", v.to_string()));
            }
            if let Some(v) = search {
                query.push(("search", v.to_string()));
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
            if self.pagination {
                self.paginate::<Ticker>(&path, Some(&query), options)
            } else {
                self.single_page::<Ticker>(&path, Some(&query), options)
            }
        })
    }

    fn get_ticker_details<'a>(
        &'a self,
        ticker: &'a str,
        date: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, TickerDetails> {
        self.get_ticker_details_with_params(
            ticker,
            GetTickerDetailsParams {
                date: date.map(String::from),
                options: options.cloned(),
            },
        )
    }

    fn get_ticker_details_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetTickerDetailsParams,
    ) -> BoxFuture<'a, TickerDetails> {
        Box::pin(async move {
            let GetTickerDetailsParams { date, options } = params;
            let date = date.as_deref();
            let options = options.as_ref();
            let path = format!("/v3/reference/tickers/{}", ticker);
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = date {
                query.push(("date", v.to_string()));
            }
            #[derive(serde::Deserialize)]
            struct Resp {
                results: TickerDetails,
            }
            let resp: Resp = self.get(&path, Some(&query), options).await?;
            Ok(resp.results)
        })
    }

    fn get_ticker_events<'a>(
        &'a self,
        ticker: &'a str,
        types: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, TickerChangeResults> {
        self.get_ticker_events_with_params(
            ticker,
            GetTickerEventsParams {
                types: types.map(String::from),
                options: options.cloned(),
            },
        )
    }

    fn get_ticker_events_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetTickerEventsParams,
    ) -> BoxFuture<'a, TickerChangeResults> {
        Box::pin(async move {
            let GetTickerEventsParams { types, options } = params;
            let types = types.as_deref();
            let options = options.as_ref();
            let path = format!("/vX/reference/tickers/{}/events", ticker);
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = types {
                query.push(("types", v.to_string()));
            }
            #[derive(serde::Deserialize)]
            struct Resp {
                results: TickerChangeResults,
            }
            let resp: Resp = self.get(&path, Some(&query), options).await?;
            Ok(resp.results)
        })
    }

    fn list_ticker_news<'a>(
        &'a self,
        ticker: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        published_utc: Option<&'a str>,
        published_utc_lt: Option<&'a str>,
        published_utc_lte: Option<&'a str>,
        published_utc_gt: Option<&'a str>,
        published_utc_gte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, TickerNews> {
        self.list_ticker_news_with_params(ListTickerNewsParams {
            ticker: ticker.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            published_utc: published_utc.map(String::from),
            published_utc_lt: published_utc_lt.map(String::from),
            published_utc_lte: published_utc_lte.map(String::from),
            published_utc_gt: published_utc_gt.map(String::from),
            published_utc_gte: published_utc_gte.map(String::from),
            limit,
            sort: sort.map(String::from),
            order: order.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_ticker_news_with_params<'a>(
        &'a self,
        params: ListTickerNewsParams,
    ) -> BoxStream<'a, TickerNews> {
        Box::pin({
            let ListTickerNewsParams {
                ticker,
                ticker_lt,
                ticker_lte,
                ticker_gt,
                ticker_gte,
                published_utc,
                published_utc_lt,
                published_utc_lte,
                published_utc_gt,
                published_utc_gte,
                limit,
                sort,
                order,
                options,
            } = params;
            let ticker = ticker.as_deref();
            let ticker_lt = ticker_lt.as_deref();
            let ticker_lte = ticker_lte.as_deref();
            let ticker_gt = ticker_gt.as_deref();
            let ticker_gte = ticker_gte.as_deref();
            let published_utc = published_utc.as_deref();
            let published_utc_lt = published_utc_lt.as_deref();
            let published_utc_lte = published_utc_lte.as_deref();
            let published_utc_gt = published_utc_gt.as_deref();
            let published_utc_gte = published_utc_gte.as_deref();
            let sort = sort.as_deref();
            let order = order.as_deref();
            let options = options.as_ref();
            let path = "/v2/reference/news".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = ticker {
                query.push(("ticker", v.to_string()));
            }
            if let Some(v) = ticker_lt {
                query.push(("ticker.lt", v.to_string()));
            }
            if let Some(v) = ticker_lte {
                query.push(("ticker.lte", v.to_string()));
            }
            if let Some(v) = ticker_gt {
                query.push(("ticker.gt", v.to_string()));
            }
            if let Some(v) = ticker_gte {
                query.push(("ticker.gte", v.to_string()));
            }
            if let Some(v) = published_utc {
                query.push(("published_utc", v.to_string()));
            }
            if let Some(v) = published_utc_lt {
                query.push(("published_utc.lt", v.to_string()));
            }
            if let Some(v) = published_utc_lte {
                query.push(("published_utc.lte", v.to_string()));
            }
            if let Some(v) = published_utc_gt {
                query.push(("published_utc.gt", v.to_string()));
            }
            if let Some(v) = published_utc_gte {
                query.push(("published_utc.gte", v.to_string()));
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
            if self.pagination {
                self.paginate::<TickerNews>(&path, Some(&query), options)
            } else {
                self.single_page::<TickerNews>(&path, Some(&query), options)
            }
        })
    }

    fn get_ticker_types<'a>(
        &'a self,
        asset_class: Option<&'a str>,
        locale: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<TickerTypes>> {
        self.get_ticker_types_with_params(GetTickerTypesParams {
            asset_class: asset_class.map(String::from),
            locale: locale.map(String::from),
            options: options.cloned(),
        })
    }

    fn get_ticker_types_with_params<'a>(
        &'a self,
        params: GetTickerTypesParams,
    ) -> BoxFuture<'a, Vec<TickerTypes>> {
        Box::pin(async move {
            let GetTickerTypesParams {
                asset_class,
                locale,
                options,
            } = params;
            let asset_class = asset_class.as_deref();
            let locale = locale.as_deref();
            let options = options.as_ref();
            let path = "/v3/reference/tickers/types".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = asset_class {
                query.push(("asset_class", v.to_string()));
            }
            if let Some(v) = locale {
                query.push(("locale", v.to_string()));
            }
            #[derive(serde::Deserialize)]
            struct Resp {
                results: Option<Vec<TickerTypes>>,
            }
            let resp: Resp = self.get(&path, Some(&query), options).await?;
            Ok(resp.results.unwrap_or_default())
        })
    }

    fn get_related_companies<'a>(
        &'a self,
        ticker: &'a str,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, RelatedCompany> {
        self.get_related_companies_with_params(
            ticker,
            GetRelatedCompaniesParams {
                options: options.cloned(),
            },
        )
    }

    fn get_related_companies_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetRelatedCompaniesParams,
    ) -> BoxFuture<'a, RelatedCompany> {
        Box::pin(async move {
            let GetRelatedCompaniesParams { options } = params;
            let options = options.as_ref();
            let path = format!("/v1/related-companies/{}", ticker);
            #[derive(serde::Deserialize)]
            struct Resp {
                results: RelatedCompany,
            }
            let resp: Resp = self.get(&path, None, options).await?;
            Ok(resp.results)
        })
    }

    fn list_splits<'a>(
        &'a self,
        ticker: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        execution_date: Option<&'a str>,
        execution_date_lt: Option<&'a str>,
        execution_date_lte: Option<&'a str>,
        execution_date_gt: Option<&'a str>,
        execution_date_gte: Option<&'a str>,
        reverse_split: Option<bool>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, Split> {
        self.list_splits_with_params(ListSplitsParams {
            ticker: ticker.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            execution_date: execution_date.map(String::from),
            execution_date_lt: execution_date_lt.map(String::from),
            execution_date_lte: execution_date_lte.map(String::from),
            execution_date_gt: execution_date_gt.map(String::from),
            execution_date_gte: execution_date_gte.map(String::from),
            reverse_split,
            limit,
            sort: sort.map(String::from),
            order: order.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_splits_with_params<'a>(&'a self, params: ListSplitsParams) -> BoxStream<'a, Split> {
        Box::pin({
            let ListSplitsParams {
                ticker,
                ticker_lt,
                ticker_lte,
                ticker_gt,
                ticker_gte,
                execution_date,
                execution_date_lt,
                execution_date_lte,
                execution_date_gt,
                execution_date_gte,
                reverse_split,
                limit,
                sort,
                order,
                options,
            } = params;
            let ticker = ticker.as_deref();
            let ticker_lt = ticker_lt.as_deref();
            let ticker_lte = ticker_lte.as_deref();
            let ticker_gt = ticker_gt.as_deref();
            let ticker_gte = ticker_gte.as_deref();
            let execution_date = execution_date.as_deref();
            let execution_date_lt = execution_date_lt.as_deref();
            let execution_date_lte = execution_date_lte.as_deref();
            let execution_date_gt = execution_date_gt.as_deref();
            let execution_date_gte = execution_date_gte.as_deref();
            let sort = sort.as_deref();
            let order = order.as_deref();
            let options = options.as_ref();
            let path = "/v3/reference/splits".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = ticker {
                query.push(("ticker", v.to_string()));
            }
            if let Some(v) = ticker_lt {
                query.push(("ticker.lt", v.to_string()));
            }
            if let Some(v) = ticker_lte {
                query.push(("ticker.lte", v.to_string()));
            }
            if let Some(v) = ticker_gt {
                query.push(("ticker.gt", v.to_string()));
            }
            if let Some(v) = ticker_gte {
                query.push(("ticker.gte", v.to_string()));
            }
            if let Some(v) = execution_date {
                query.push(("execution_date", v.to_string()));
            }
            if let Some(v) = execution_date_lt {
                query.push(("execution_date.lt", v.to_string()));
            }
            if let Some(v) = execution_date_lte {
                query.push(("execution_date.lte", v.to_string()));
            }
            if let Some(v) = execution_date_gt {
                query.push(("execution_date.gt", v.to_string()));
            }
            if let Some(v) = execution_date_gte {
                query.push(("execution_date.gte", v.to_string()));
            }
            if let Some(v) = reverse_split {
                query.push(("reverse_split", v.to_string()));
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
            if self.pagination {
                self.paginate::<Split>(&path, Some(&query), options)
            } else {
                self.single_page::<Split>(&path, Some(&query), options)
            }
        })
    }

    fn list_dividends<'a>(
        &'a self,
        ticker: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ex_dividend_date: Option<&'a str>,
        ex_dividend_date_lt: Option<&'a str>,
        ex_dividend_date_lte: Option<&'a str>,
        ex_dividend_date_gt: Option<&'a str>,
        ex_dividend_date_gte: Option<&'a str>,
        record_date: Option<&'a str>,
        record_date_lt: Option<&'a str>,
        record_date_lte: Option<&'a str>,
        record_date_gt: Option<&'a str>,
        record_date_gte: Option<&'a str>,
        declaration_date: Option<&'a str>,
        declaration_date_lt: Option<&'a str>,
        declaration_date_lte: Option<&'a str>,
        declaration_date_gt: Option<&'a str>,
        declaration_date_gte: Option<&'a str>,
        pay_date: Option<&'a str>,
        pay_date_lt: Option<&'a str>,
        pay_date_lte: Option<&'a str>,
        pay_date_gt: Option<&'a str>,
        pay_date_gte: Option<&'a str>,
        frequency: Option<i64>,
        cash_amount: Option<f64>,
        cash_amount_lt: Option<f64>,
        cash_amount_lte: Option<f64>,
        cash_amount_gt: Option<f64>,
        cash_amount_gte: Option<f64>,
        dividend_type: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, Dividend> {
        self.list_dividends_with_params(ListDividendsParams {
            ticker: ticker.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            ex_dividend_date: ex_dividend_date.map(String::from),
            ex_dividend_date_lt: ex_dividend_date_lt.map(String::from),
            ex_dividend_date_lte: ex_dividend_date_lte.map(String::from),
            ex_dividend_date_gt: ex_dividend_date_gt.map(String::from),
            ex_dividend_date_gte: ex_dividend_date_gte.map(String::from),
            record_date: record_date.map(String::from),
            record_date_lt: record_date_lt.map(String::from),
            record_date_lte: record_date_lte.map(String::from),
            record_date_gt: record_date_gt.map(String::from),
            record_date_gte: record_date_gte.map(String::from),
            declaration_date: declaration_date.map(String::from),
            declaration_date_lt: declaration_date_lt.map(String::from),
            declaration_date_lte: declaration_date_lte.map(String::from),
            declaration_date_gt: declaration_date_gt.map(String::from),
            declaration_date_gte: declaration_date_gte.map(String::from),
            pay_date: pay_date.map(String::from),
            pay_date_lt: pay_date_lt.map(String::from),
            pay_date_lte: pay_date_lte.map(String::from),
            pay_date_gt: pay_date_gt.map(String::from),
            pay_date_gte: pay_date_gte.map(String::from),
            frequency,
            cash_amount,
            cash_amount_lt,
            cash_amount_lte,
            cash_amount_gt,
            cash_amount_gte,
            dividend_type: dividend_type.map(String::from),
            limit,
            sort: sort.map(String::from),
            order: order.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_dividends_with_params<'a>(
        &'a self,
        params: ListDividendsParams,
    ) -> BoxStream<'a, Dividend> {
        Box::pin({
            let ListDividendsParams {
                ticker,
                ticker_lt,
                ticker_lte,
                ticker_gt,
                ticker_gte,
                ex_dividend_date,
                ex_dividend_date_lt,
                ex_dividend_date_lte,
                ex_dividend_date_gt,
                ex_dividend_date_gte,
                record_date,
                record_date_lt,
                record_date_lte,
                record_date_gt,
                record_date_gte,
                declaration_date,
                declaration_date_lt,
                declaration_date_lte,
                declaration_date_gt,
                declaration_date_gte,
                pay_date,
                pay_date_lt,
                pay_date_lte,
                pay_date_gt,
                pay_date_gte,
                frequency,
                cash_amount,
                cash_amount_lt,
                cash_amount_lte,
                cash_amount_gt,
                cash_amount_gte,
                dividend_type,
                limit,
                sort,
                order,
                options,
            } = params;
            let ticker = ticker.as_deref();
            let ticker_lt = ticker_lt.as_deref();
            let ticker_lte = ticker_lte.as_deref();
            let ticker_gt = ticker_gt.as_deref();
            let ticker_gte = ticker_gte.as_deref();
            let ex_dividend_date = ex_dividend_date.as_deref();
            let ex_dividend_date_lt = ex_dividend_date_lt.as_deref();
            let ex_dividend_date_lte = ex_dividend_date_lte.as_deref();
            let ex_dividend_date_gt = ex_dividend_date_gt.as_deref();
            let ex_dividend_date_gte = ex_dividend_date_gte.as_deref();
            let record_date = record_date.as_deref();
            let record_date_lt = record_date_lt.as_deref();
            let record_date_lte = record_date_lte.as_deref();
            let record_date_gt = record_date_gt.as_deref();
            let record_date_gte = record_date_gte.as_deref();
            let declaration_date = declaration_date.as_deref();
            let declaration_date_lt = declaration_date_lt.as_deref();
            let declaration_date_lte = declaration_date_lte.as_deref();
            let declaration_date_gt = declaration_date_gt.as_deref();
            let declaration_date_gte = declaration_date_gte.as_deref();
            let pay_date = pay_date.as_deref();
            let pay_date_lt = pay_date_lt.as_deref();
            let pay_date_lte = pay_date_lte.as_deref();
            let pay_date_gt = pay_date_gt.as_deref();
            let pay_date_gte = pay_date_gte.as_deref();
            let dividend_type = dividend_type.as_deref();
            let sort = sort.as_deref();
            let order = order.as_deref();
            let options = options.as_ref();
            let path = "/v3/reference/dividends".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = ticker {
                query.push(("ticker", v.to_string()));
            }
            if let Some(v) = ticker_lt {
                query.push(("ticker.lt", v.to_string()));
            }
            if let Some(v) = ticker_lte {
                query.push(("ticker.lte", v.to_string()));
            }
            if let Some(v) = ticker_gt {
                query.push(("ticker.gt", v.to_string()));
            }
            if let Some(v) = ticker_gte {
                query.push(("ticker.gte", v.to_string()));
            }
            if let Some(v) = ex_dividend_date {
                query.push(("ex_dividend_date", v.to_string()));
            }
            if let Some(v) = ex_dividend_date_lt {
                query.push(("ex_dividend_date.lt", v.to_string()));
            }
            if let Some(v) = ex_dividend_date_lte {
                query.push(("ex_dividend_date.lte", v.to_string()));
            }
            if let Some(v) = ex_dividend_date_gt {
                query.push(("ex_dividend_date.gt", v.to_string()));
            }
            if let Some(v) = ex_dividend_date_gte {
                query.push(("ex_dividend_date.gte", v.to_string()));
            }
            if let Some(v) = record_date {
                query.push(("record_date", v.to_string()));
            }
            if let Some(v) = record_date_lt {
                query.push(("record_date.lt", v.to_string()));
            }
            if let Some(v) = record_date_lte {
                query.push(("record_date.lte", v.to_string()));
            }
            if let Some(v) = record_date_gt {
                query.push(("record_date.gt", v.to_string()));
            }
            if let Some(v) = record_date_gte {
                query.push(("record_date.gte", v.to_string()));
            }
            if let Some(v) = declaration_date {
                query.push(("declaration_date", v.to_string()));
            }
            if let Some(v) = declaration_date_lt {
                query.push(("declaration_date.lt", v.to_string()));
            }
            if let Some(v) = declaration_date_lte {
                query.push(("declaration_date.lte", v.to_string()));
            }
            if let Some(v) = declaration_date_gt {
                query.push(("declaration_date.gt", v.to_string()));
            }
            if let Some(v) = declaration_date_gte {
                query.push(("declaration_date.gte", v.to_string()));
            }
            if let Some(v) = pay_date {
                query.push(("pay_date", v.to_string()));
            }
            if let Some(v) = pay_date_lt {
                query.push(("pay_date.lt", v.to_string()));
            }
            if let Some(v) = pay_date_lte {
                query.push(("pay_date.lte", v.to_string()));
            }
            if let Some(v) = pay_date_gt {
                query.push(("pay_date.gt", v.to_string()));
            }
            if let Some(v) = pay_date_gte {
                query.push(("pay_date.gte", v.to_string()));
            }
            if let Some(v) = frequency {
                query.push(("frequency", v.to_string()));
            }
            if let Some(v) = cash_amount {
                query.push(("cash_amount", v.to_string()));
            }
            if let Some(v) = cash_amount_lt {
                query.push(("cash_amount.lt", v.to_string()));
            }
            if let Some(v) = cash_amount_lte {
                query.push(("cash_amount.lte", v.to_string()));
            }
            if let Some(v) = cash_amount_gt {
                query.push(("cash_amount.gt", v.to_string()));
            }
            if let Some(v) = cash_amount_gte {
                query.push(("cash_amount.gte", v.to_string()));
            }
            if let Some(v) = dividend_type {
                query.push(("dividend_type", v.to_string()));
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
            if self.pagination {
                self.paginate::<Dividend>(&path, Some(&query), options)
            } else {
                self.single_page::<Dividend>(&path, Some(&query), options)
            }
        })
    }

    fn list_conditions<'a>(
        &'a self,
        asset_class: Option<&'a str>,
        data_type: Option<&'a str>,
        id: Option<i64>,
        sip: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, Condition> {
        self.list_conditions_with_params(ListConditionsParams {
            asset_class: asset_class.map(String::from),
            data_type: data_type.map(String::from),
            id,
            sip: sip.map(String::from),
            limit,
            sort: sort.map(String::from),
            order: order.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_conditions_with_params<'a>(
        &'a self,
        params: ListConditionsParams,
    ) -> BoxStream<'a, Condition> {
        Box::pin({
            let ListConditionsParams {
                asset_class,
                data_type,
                id,
                sip,
                limit,
                sort,
                order,
                options,
            } = params;
            let asset_class = asset_class.as_deref();
            let data_type = data_type.as_deref();
            let sip = sip.as_deref();
            let sort = sort.as_deref();
            let order = order.as_deref();
            let options = options.as_ref();
            let path = "/v3/reference/conditions".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = asset_class {
                query.push(("asset_class", v.to_string()));
            }
            if let Some(v) = data_type {
                query.push(("data_type", v.to_string()));
            }
            if let Some(v) = id {
                query.push(("id", v.to_string()));
            }
            if let Some(v) = sip {
                query.push(("sip", v.to_string()));
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
            if self.pagination {
                self.paginate::<Condition>(&path, Some(&query), options)
            } else {
                self.single_page::<Condition>(&path, Some(&query), options)
            }
        })
    }

    fn get_exchanges<'a>(
        &'a self,
        asset_class: Option<&'a str>,
        locale: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, Vec<Exchange>> {
        self.get_exchanges_with_params(GetExchangesParams {
            asset_class: asset_class.map(String::from),
            locale: locale.map(String::from),
            options: options.cloned(),
        })
    }

    fn get_exchanges_with_params<'a>(
        &'a self,
        params: GetExchangesParams,
    ) -> BoxFuture<'a, Vec<Exchange>> {
        Box::pin(async move {
            let GetExchangesParams {
                asset_class,
                locale,
                options,
            } = params;
            let asset_class = asset_class.as_deref();
            let locale = locale.as_deref();
            let options = options.as_ref();
            let path = "/v3/reference/exchanges".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = asset_class {
                query.push(("asset_class", v.to_string()));
            }
            if let Some(v) = locale {
                query.push(("locale", v.to_string()));
            }
            #[derive(serde::Deserialize)]
            struct Resp {
                results: Option<Vec<Exchange>>,
            }
            let resp: Resp = self.get(&path, Some(&query), options).await?;
            Ok(resp.results.unwrap_or_default())
        })
    }

    fn get_options_contract<'a>(
        &'a self,
        ticker: &'a str,
        as_of: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxFuture<'a, OptionsContract> {
        self.get_options_contract_with_params(
            ticker,
            GetOptionsContractParams {
                as_of: as_of.map(String::from),
                options: options.cloned(),
            },
        )
    }

    fn get_options_contract_with_params<'a>(
        &'a self,
        ticker: &'a str,
        params: GetOptionsContractParams,
    ) -> BoxFuture<'a, OptionsContract> {
        Box::pin(async move {
            let GetOptionsContractParams { as_of, options } = params;
            let as_of = as_of.as_deref();
            let options = options.as_ref();
            let path = format!("/v3/reference/options/contracts/{}", ticker);
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = as_of {
                query.push(("as_of", v.to_string()));
            }
            #[derive(serde::Deserialize)]
            struct Resp {
                results: OptionsContract,
            }
            let resp: Resp = self.get(&path, Some(&query), options).await?;
            Ok(resp.results)
        })
    }

    fn list_options_contracts<'a>(
        &'a self,
        underlying_ticker: Option<&'a str>,
        underlying_ticker_lt: Option<&'a str>,
        underlying_ticker_lte: Option<&'a str>,
        underlying_ticker_gt: Option<&'a str>,
        underlying_ticker_gte: Option<&'a str>,
        contract_type: Option<&'a str>,
        expiration_date: Option<&'a str>,
        expiration_date_lt: Option<&'a str>,
        expiration_date_lte: Option<&'a str>,
        expiration_date_gt: Option<&'a str>,
        expiration_date_gte: Option<&'a str>,
        as_of: Option<&'a str>,
        strike_price: Option<f64>,
        strike_price_lt: Option<f64>,
        strike_price_lte: Option<f64>,
        strike_price_gt: Option<f64>,
        strike_price_gte: Option<f64>,
        expired: Option<bool>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, OptionsContract> {
        self.list_options_contracts_with_params(ListOptionsContractsParams {
            underlying_ticker: underlying_ticker.map(String::from),
            underlying_ticker_lt: underlying_ticker_lt.map(String::from),
            underlying_ticker_lte: underlying_ticker_lte.map(String::from),
            underlying_ticker_gt: underlying_ticker_gt.map(String::from),
            underlying_ticker_gte: underlying_ticker_gte.map(String::from),
            contract_type: contract_type.map(String::from),
            expiration_date: expiration_date.map(String::from),
            expiration_date_lt: expiration_date_lt.map(String::from),
            expiration_date_lte: expiration_date_lte.map(String::from),
            expiration_date_gt: expiration_date_gt.map(String::from),
            expiration_date_gte: expiration_date_gte.map(String::from),
            as_of: as_of.map(String::from),
            strike_price,
            strike_price_lt,
            strike_price_lte,
            strike_price_gt,
            strike_price_gte,
            expired,
            limit,
            sort: sort.map(String::from),
            order: order.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_options_contracts_with_params<'a>(
        &'a self,
        params: ListOptionsContractsParams,
    ) -> BoxStream<'a, OptionsContract> {
        Box::pin({
            let ListOptionsContractsParams {
                underlying_ticker,
                underlying_ticker_lt,
                underlying_ticker_lte,
                underlying_ticker_gt,
                underlying_ticker_gte,
                contract_type,
                expiration_date,
                expiration_date_lt,
                expiration_date_lte,
                expiration_date_gt,
                expiration_date_gte,
                as_of,
                strike_price,
                strike_price_lt,
                strike_price_lte,
                strike_price_gt,
                strike_price_gte,
                expired,
                limit,
                sort,
                order,
                options,
            } = params;
            let underlying_ticker = underlying_ticker.as_deref();
            let underlying_ticker_lt = underlying_ticker_lt.as_deref();
            let underlying_ticker_lte = underlying_ticker_lte.as_deref();
            let underlying_ticker_gt = underlying_ticker_gt.as_deref();
            let underlying_ticker_gte = underlying_ticker_gte.as_deref();
            let contract_type = contract_type.as_deref();
            let expiration_date = expiration_date.as_deref();
            let expiration_date_lt = expiration_date_lt.as_deref();
            let expiration_date_lte = expiration_date_lte.as_deref();
            let expiration_date_gt = expiration_date_gt.as_deref();
            let expiration_date_gte = expiration_date_gte.as_deref();
            let as_of = as_of.as_deref();
            let sort = sort.as_deref();
            let order = order.as_deref();
            let options = options.as_ref();
            let path = "/v3/reference/options/contracts".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = underlying_ticker {
                query.push(("underlying_ticker", v.to_string()));
            }
            if let Some(v) = underlying_ticker_lt {
                query.push(("underlying_ticker.lt", v.to_string()));
            }
            if let Some(v) = underlying_ticker_lte {
                query.push(("underlying_ticker.lte", v.to_string()));
            }
            if let Some(v) = underlying_ticker_gt {
                query.push(("underlying_ticker.gt", v.to_string()));
            }
            if let Some(v) = underlying_ticker_gte {
                query.push(("underlying_ticker.gte", v.to_string()));
            }
            if let Some(v) = contract_type {
                query.push(("contract_type", v.to_string()));
            }
            if let Some(v) = expiration_date {
                query.push(("expiration_date", v.to_string()));
            }
            if let Some(v) = expiration_date_lt {
                query.push(("expiration_date.lt", v.to_string()));
            }
            if let Some(v) = expiration_date_lte {
                query.push(("expiration_date.lte", v.to_string()));
            }
            if let Some(v) = expiration_date_gt {
                query.push(("expiration_date.gt", v.to_string()));
            }
            if let Some(v) = expiration_date_gte {
                query.push(("expiration_date.gte", v.to_string()));
            }
            if let Some(v) = as_of {
                query.push(("as_of", v.to_string()));
            }
            if let Some(v) = strike_price {
                query.push(("strike_price", v.to_string()));
            }
            if let Some(v) = strike_price_lt {
                query.push(("strike_price.lt", v.to_string()));
            }
            if let Some(v) = strike_price_lte {
                query.push(("strike_price.lte", v.to_string()));
            }
            if let Some(v) = strike_price_gt {
                query.push(("strike_price.gt", v.to_string()));
            }
            if let Some(v) = strike_price_gte {
                query.push(("strike_price.gte", v.to_string()));
            }
            if let Some(v) = expired {
                query.push(("expired", v.to_string()));
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
            if self.pagination {
                self.paginate::<OptionsContract>(&path, Some(&query), options)
            } else {
                self.single_page::<OptionsContract>(&path, Some(&query), options)
            }
        })
    }

    fn list_short_interest<'a>(
        &'a self,
        ticker: Option<&'a str>,
        days_to_cover: Option<&'a str>,
        days_to_cover_lt: Option<&'a str>,
        days_to_cover_lte: Option<&'a str>,
        days_to_cover_gt: Option<&'a str>,
        days_to_cover_gte: Option<&'a str>,
        settlement_date: Option<&'a str>,
        settlement_date_lt: Option<&'a str>,
        settlement_date_lte: Option<&'a str>,
        settlement_date_gt: Option<&'a str>,
        settlement_date_gte: Option<&'a str>,
        avg_daily_volume: Option<&'a str>,
        avg_daily_volume_lt: Option<&'a str>,
        avg_daily_volume_lte: Option<&'a str>,
        avg_daily_volume_gt: Option<&'a str>,
        avg_daily_volume_gte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, ShortInterest> {
        self.list_short_interest_with_params(ListShortInterestParams {
            ticker: ticker.map(String::from),
            days_to_cover: days_to_cover.map(String::from),
            days_to_cover_lt: days_to_cover_lt.map(String::from),
            days_to_cover_lte: days_to_cover_lte.map(String::from),
            days_to_cover_gt: days_to_cover_gt.map(String::from),
            days_to_cover_gte: days_to_cover_gte.map(String::from),
            settlement_date: settlement_date.map(String::from),
            settlement_date_lt: settlement_date_lt.map(String::from),
            settlement_date_lte: settlement_date_lte.map(String::from),
            settlement_date_gt: settlement_date_gt.map(String::from),
            settlement_date_gte: settlement_date_gte.map(String::from),
            avg_daily_volume: avg_daily_volume.map(String::from),
            avg_daily_volume_lt: avg_daily_volume_lt.map(String::from),
            avg_daily_volume_lte: avg_daily_volume_lte.map(String::from),
            avg_daily_volume_gt: avg_daily_volume_gt.map(String::from),
            avg_daily_volume_gte: avg_daily_volume_gte.map(String::from),
            limit,
            sort: sort.map(String::from),
            order: order.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_short_interest_with_params<'a>(
        &'a self,
        params: ListShortInterestParams,
    ) -> BoxStream<'a, ShortInterest> {
        Box::pin({
            let ListShortInterestParams {
                ticker,
                days_to_cover,
                days_to_cover_lt,
                days_to_cover_lte,
                days_to_cover_gt,
                days_to_cover_gte,
                settlement_date,
                settlement_date_lt,
                settlement_date_lte,
                settlement_date_gt,
                settlement_date_gte,
                avg_daily_volume,
                avg_daily_volume_lt,
                avg_daily_volume_lte,
                avg_daily_volume_gt,
                avg_daily_volume_gte,
                limit,
                sort,
                order,
                options,
            } = params;
            let ticker = ticker.as_deref();
            let days_to_cover = days_to_cover.as_deref();
            let days_to_cover_lt = days_to_cover_lt.as_deref();
            let days_to_cover_lte = days_to_cover_lte.as_deref();
            let days_to_cover_gt = days_to_cover_gt.as_deref();
            let days_to_cover_gte = days_to_cover_gte.as_deref();
            let settlement_date = settlement_date.as_deref();
            let settlement_date_lt = settlement_date_lt.as_deref();
            let settlement_date_lte = settlement_date_lte.as_deref();
            let settlement_date_gt = settlement_date_gt.as_deref();
            let settlement_date_gte = settlement_date_gte.as_deref();
            let avg_daily_volume = avg_daily_volume.as_deref();
            let avg_daily_volume_lt = avg_daily_volume_lt.as_deref();
            let avg_daily_volume_lte = avg_daily_volume_lte.as_deref();
            let avg_daily_volume_gt = avg_daily_volume_gt.as_deref();
            let avg_daily_volume_gte = avg_daily_volume_gte.as_deref();
            let sort = sort.as_deref();
            let order = order.as_deref();
            let options = options.as_ref();
            let path = "/stocks/v1/short-interest".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = ticker {
                query.push(("ticker", v.to_string()));
            }
            if let Some(v) = days_to_cover {
                query.push(("days_to_cover", v.to_string()));
            }
            if let Some(v) = days_to_cover_lt {
                query.push(("days_to_cover.lt", v.to_string()));
            }
            if let Some(v) = days_to_cover_lte {
                query.push(("days_to_cover.lte", v.to_string()));
            }
            if let Some(v) = days_to_cover_gt {
                query.push(("days_to_cover.gt", v.to_string()));
            }
            if let Some(v) = days_to_cover_gte {
                query.push(("days_to_cover.gte", v.to_string()));
            }
            if let Some(v) = settlement_date {
                query.push(("settlement_date", v.to_string()));
            }
            if let Some(v) = settlement_date_lt {
                query.push(("settlement_date.lt", v.to_string()));
            }
            if let Some(v) = settlement_date_lte {
                query.push(("settlement_date.lte", v.to_string()));
            }
            if let Some(v) = settlement_date_gt {
                query.push(("settlement_date.gt", v.to_string()));
            }
            if let Some(v) = settlement_date_gte {
                query.push(("settlement_date.gte", v.to_string()));
            }
            if let Some(v) = avg_daily_volume {
                query.push(("avg_daily_volume", v.to_string()));
            }
            if let Some(v) = avg_daily_volume_lt {
                query.push(("avg_daily_volume.lt", v.to_string()));
            }
            if let Some(v) = avg_daily_volume_lte {
                query.push(("avg_daily_volume.lte", v.to_string()));
            }
            if let Some(v) = avg_daily_volume_gt {
                query.push(("avg_daily_volume.gt", v.to_string()));
            }
            if let Some(v) = avg_daily_volume_gte {
                query.push(("avg_daily_volume.gte", v.to_string()));
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
            if self.pagination {
                self.paginate::<ShortInterest>(&path, Some(&query), options)
            } else {
                self.single_page::<ShortInterest>(&path, Some(&query), options)
            }
        })
    }

    fn list_short_volume<'a>(
        &'a self,
        ticker: Option<&'a str>,
        date: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        short_volume_ratio: Option<&'a str>,
        short_volume_ratio_lt: Option<&'a str>,
        short_volume_ratio_lte: Option<&'a str>,
        short_volume_ratio_gt: Option<&'a str>,
        short_volume_ratio_gte: Option<&'a str>,
        total_volume: Option<&'a str>,
        total_volume_lt: Option<&'a str>,
        total_volume_lte: Option<&'a str>,
        total_volume_gt: Option<&'a str>,
        total_volume_gte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, ShortVolume> {
        self.list_short_volume_with_params(ListShortVolumeParams {
            ticker: ticker.map(String::from),
            date: date.map(String::from),
            date_lt: date_lt.map(String::from),
            date_lte: date_lte.map(String::from),
            date_gt: date_gt.map(String::from),
            date_gte: date_gte.map(String::from),
            short_volume_ratio: short_volume_ratio.map(String::from),
            short_volume_ratio_lt: short_volume_ratio_lt.map(String::from),
            short_volume_ratio_lte: short_volume_ratio_lte.map(String::from),
            short_volume_ratio_gt: short_volume_ratio_gt.map(String::from),
            short_volume_ratio_gte: short_volume_ratio_gte.map(String::from),
            total_volume: total_volume.map(String::from),
            total_volume_lt: total_volume_lt.map(String::from),
            total_volume_lte: total_volume_lte.map(String::from),
            total_volume_gt: total_volume_gt.map(String::from),
            total_volume_gte: total_volume_gte.map(String::from),
            limit,
            sort: sort.map(String::from),
            order: order.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_short_volume_with_params<'a>(
        &'a self,
        params: ListShortVolumeParams,
    ) -> BoxStream<'a, ShortVolume> {
        Box::pin({
            let ListShortVolumeParams {
                ticker,
                date,
                date_lt,
                date_lte,
                date_gt,
                date_gte,
                short_volume_ratio,
                short_volume_ratio_lt,
                short_volume_ratio_lte,
                short_volume_ratio_gt,
                short_volume_ratio_gte,
                total_volume,
                total_volume_lt,
                total_volume_lte,
                total_volume_gt,
                total_volume_gte,
                limit,
                sort,
                order,
                options,
            } = params;
            let ticker = ticker.as_deref();
            let date = date.as_deref();
            let date_lt = date_lt.as_deref();
            let date_lte = date_lte.as_deref();
            let date_gt = date_gt.as_deref();
            let date_gte = date_gte.as_deref();
            let short_volume_ratio = short_volume_ratio.as_deref();
            let short_volume_ratio_lt = short_volume_ratio_lt.as_deref();
            let short_volume_ratio_lte = short_volume_ratio_lte.as_deref();
            let short_volume_ratio_gt = short_volume_ratio_gt.as_deref();
            let short_volume_ratio_gte = short_volume_ratio_gte.as_deref();
            let total_volume = total_volume.as_deref();
            let total_volume_lt = total_volume_lt.as_deref();
            let total_volume_lte = total_volume_lte.as_deref();
            let total_volume_gt = total_volume_gt.as_deref();
            let total_volume_gte = total_volume_gte.as_deref();
            let sort = sort.as_deref();
            let order = order.as_deref();
            let options = options.as_ref();
            let path = "/stocks/v1/short-volume".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = ticker {
                query.push(("ticker", v.to_string()));
            }
            if let Some(v) = date {
                query.push(("date", v.to_string()));
            }
            if let Some(v) = date_lt {
                query.push(("date.lt", v.to_string()));
            }
            if let Some(v) = date_lte {
                query.push(("date.lte", v.to_string()));
            }
            if let Some(v) = date_gt {
                query.push(("date.gt", v.to_string()));
            }
            if let Some(v) = date_gte {
                query.push(("date.gte", v.to_string()));
            }
            if let Some(v) = short_volume_ratio {
                query.push(("short_volume_ratio", v.to_string()));
            }
            if let Some(v) = short_volume_ratio_lt {
                query.push(("short_volume_ratio.lt", v.to_string()));
            }
            if let Some(v) = short_volume_ratio_lte {
                query.push(("short_volume_ratio.lte", v.to_string()));
            }
            if let Some(v) = short_volume_ratio_gt {
                query.push(("short_volume_ratio.gt", v.to_string()));
            }
            if let Some(v) = short_volume_ratio_gte {
                query.push(("short_volume_ratio.gte", v.to_string()));
            }
            if let Some(v) = total_volume {
                query.push(("total_volume", v.to_string()));
            }
            if let Some(v) = total_volume_lt {
                query.push(("total_volume.lt", v.to_string()));
            }
            if let Some(v) = total_volume_lte {
                query.push(("total_volume.lte", v.to_string()));
            }
            if let Some(v) = total_volume_gt {
                query.push(("total_volume.gt", v.to_string()));
            }
            if let Some(v) = total_volume_gte {
                query.push(("total_volume.gte", v.to_string()));
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
            if self.pagination {
                self.paginate::<ShortVolume>(&path, Some(&query), options)
            } else {
                self.single_page::<ShortVolume>(&path, Some(&query), options)
            }
        })
    }

    fn list_stocks_splits<'a>(
        &'a self,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        execution_date: Option<&'a str>,
        execution_date_gt: Option<&'a str>,
        execution_date_gte: Option<&'a str>,
        execution_date_lt: Option<&'a str>,
        execution_date_lte: Option<&'a str>,
        adjustment_type: Option<&'a str>,
        adjustment_type_any_of: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, StockSplit> {
        self.list_stocks_splits_with_params(ListStocksSplitsParams {
            ticker: ticker.map(String::from),
            ticker_any_of: ticker_any_of.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            execution_date: execution_date.map(String::from),
            execution_date_gt: execution_date_gt.map(String::from),
            execution_date_gte: execution_date_gte.map(String::from),
            execution_date_lt: execution_date_lt.map(String::from),
            execution_date_lte: execution_date_lte.map(String::from),
            adjustment_type: adjustment_type.map(String::from),
            adjustment_type_any_of: adjustment_type_any_of.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_stocks_splits_with_params<'a>(
        &'a self,
        params: ListStocksSplitsParams,
    ) -> BoxStream<'a, StockSplit> {
        Box::pin({
            let ListStocksSplitsParams {
                ticker,
                ticker_any_of,
                ticker_gt,
                ticker_gte,
                ticker_lt,
                ticker_lte,
                execution_date,
                execution_date_gt,
                execution_date_gte,
                execution_date_lt,
                execution_date_lte,
                adjustment_type,
                adjustment_type_any_of,
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
            let execution_date = execution_date.as_deref();
            let execution_date_gt = execution_date_gt.as_deref();
            let execution_date_gte = execution_date_gte.as_deref();
            let execution_date_lt = execution_date_lt.as_deref();
            let execution_date_lte = execution_date_lte.as_deref();
            let adjustment_type = adjustment_type.as_deref();
            let adjustment_type_any_of = adjustment_type_any_of.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/stocks/v1/splits".to_string();
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
            if let Some(v) = execution_date {
                query.push(("execution_date", v.to_string()));
            }
            if let Some(v) = execution_date_gt {
                query.push(("execution_date.gt", v.to_string()));
            }
            if let Some(v) = execution_date_gte {
                query.push(("execution_date.gte", v.to_string()));
            }
            if let Some(v) = execution_date_lt {
                query.push(("execution_date.lt", v.to_string()));
            }
            if let Some(v) = execution_date_lte {
                query.push(("execution_date.lte", v.to_string()));
            }
            if let Some(v) = adjustment_type {
                query.push(("adjustment_type", v.to_string()));
            }
            if let Some(v) = adjustment_type_any_of {
                query.push(("adjustment_type.any_of", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            if self.pagination {
                self.paginate::<StockSplit>(&path, Some(&query), options)
            } else {
                self.single_page::<StockSplit>(&path, Some(&query), options)
            }
        })
    }

    fn list_stocks_dividends<'a>(
        &'a self,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        ex_dividend_date: Option<&'a str>,
        ex_dividend_date_gt: Option<&'a str>,
        ex_dividend_date_gte: Option<&'a str>,
        ex_dividend_date_lt: Option<&'a str>,
        ex_dividend_date_lte: Option<&'a str>,
        frequency: Option<i64>,
        frequency_gt: Option<i64>,
        frequency_gte: Option<i64>,
        frequency_lt: Option<i64>,
        frequency_lte: Option<i64>,
        distribution_type: Option<&'a str>,
        distribution_type_any_of: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, StockDividend> {
        self.list_stocks_dividends_with_params(ListStocksDividendsParams {
            ticker: ticker.map(String::from),
            ticker_any_of: ticker_any_of.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            ex_dividend_date: ex_dividend_date.map(String::from),
            ex_dividend_date_gt: ex_dividend_date_gt.map(String::from),
            ex_dividend_date_gte: ex_dividend_date_gte.map(String::from),
            ex_dividend_date_lt: ex_dividend_date_lt.map(String::from),
            ex_dividend_date_lte: ex_dividend_date_lte.map(String::from),
            frequency,
            frequency_gt,
            frequency_gte,
            frequency_lt,
            frequency_lte,
            distribution_type: distribution_type.map(String::from),
            distribution_type_any_of: distribution_type_any_of.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_stocks_dividends_with_params<'a>(
        &'a self,
        params: ListStocksDividendsParams,
    ) -> BoxStream<'a, StockDividend> {
        Box::pin({
            let ListStocksDividendsParams {
                ticker,
                ticker_any_of,
                ticker_gt,
                ticker_gte,
                ticker_lt,
                ticker_lte,
                ex_dividend_date,
                ex_dividend_date_gt,
                ex_dividend_date_gte,
                ex_dividend_date_lt,
                ex_dividend_date_lte,
                frequency,
                frequency_gt,
                frequency_gte,
                frequency_lt,
                frequency_lte,
                distribution_type,
                distribution_type_any_of,
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
            let ex_dividend_date = ex_dividend_date.as_deref();
            let ex_dividend_date_gt = ex_dividend_date_gt.as_deref();
            let ex_dividend_date_gte = ex_dividend_date_gte.as_deref();
            let ex_dividend_date_lt = ex_dividend_date_lt.as_deref();
            let ex_dividend_date_lte = ex_dividend_date_lte.as_deref();
            let distribution_type = distribution_type.as_deref();
            let distribution_type_any_of = distribution_type_any_of.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/stocks/v1/dividends".to_string();
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
            if let Some(v) = ex_dividend_date {
                query.push(("ex_dividend_date", v.to_string()));
            }
            if let Some(v) = ex_dividend_date_gt {
                query.push(("ex_dividend_date.gt", v.to_string()));
            }
            if let Some(v) = ex_dividend_date_gte {
                query.push(("ex_dividend_date.gte", v.to_string()));
            }
            if let Some(v) = ex_dividend_date_lt {
                query.push(("ex_dividend_date.lt", v.to_string()));
            }
            if let Some(v) = ex_dividend_date_lte {
                query.push(("ex_dividend_date.lte", v.to_string()));
            }
            if let Some(v) = frequency {
                query.push(("frequency", v.to_string()));
            }
            if let Some(v) = frequency_gt {
                query.push(("frequency.gt", v.to_string()));
            }
            if let Some(v) = frequency_gte {
                query.push(("frequency.gte", v.to_string()));
            }
            if let Some(v) = frequency_lt {
                query.push(("frequency.lt", v.to_string()));
            }
            if let Some(v) = frequency_lte {
                query.push(("frequency.lte", v.to_string()));
            }
            if let Some(v) = distribution_type {
                query.push(("distribution_type", v.to_string()));
            }
            if let Some(v) = distribution_type_any_of {
                query.push(("distribution_type.any_of", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            if self.pagination {
                self.paginate::<StockDividend>(&path, Some(&query), options)
            } else {
                self.single_page::<StockDividend>(&path, Some(&query), options)
            }
        })
    }

    fn list_stocks_filings_risk_factors<'a>(
        &'a self,
        filing_date: Option<&'a str>,
        filing_date_any_of: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        cik: Option<&'a str>,
        cik_any_of: Option<&'a str>,
        cik_gt: Option<&'a str>,
        cik_gte: Option<&'a str>,
        cik_lt: Option<&'a str>,
        cik_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, RiskFactor> {
        self.list_stocks_filings_risk_factors_with_params(ListStocksFilingsRiskFactorsParams {
            filing_date: filing_date.map(String::from),
            filing_date_any_of: filing_date_any_of.map(String::from),
            filing_date_gt: filing_date_gt.map(String::from),
            filing_date_gte: filing_date_gte.map(String::from),
            filing_date_lt: filing_date_lt.map(String::from),
            filing_date_lte: filing_date_lte.map(String::from),
            ticker: ticker.map(String::from),
            ticker_any_of: ticker_any_of.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            cik: cik.map(String::from),
            cik_any_of: cik_any_of.map(String::from),
            cik_gt: cik_gt.map(String::from),
            cik_gte: cik_gte.map(String::from),
            cik_lt: cik_lt.map(String::from),
            cik_lte: cik_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_stocks_filings_risk_factors_with_params<'a>(
        &'a self,
        params: ListStocksFilingsRiskFactorsParams,
    ) -> BoxStream<'a, RiskFactor> {
        Box::pin({
            let ListStocksFilingsRiskFactorsParams {
                filing_date,
                filing_date_any_of,
                filing_date_gt,
                filing_date_gte,
                filing_date_lt,
                filing_date_lte,
                ticker,
                ticker_any_of,
                ticker_gt,
                ticker_gte,
                ticker_lt,
                ticker_lte,
                cik,
                cik_any_of,
                cik_gt,
                cik_gte,
                cik_lt,
                cik_lte,
                limit,
                sort,
                options,
            } = params;
            let filing_date = filing_date.as_deref();
            let filing_date_any_of = filing_date_any_of.as_deref();
            let filing_date_gt = filing_date_gt.as_deref();
            let filing_date_gte = filing_date_gte.as_deref();
            let filing_date_lt = filing_date_lt.as_deref();
            let filing_date_lte = filing_date_lte.as_deref();
            let ticker = ticker.as_deref();
            let ticker_any_of = ticker_any_of.as_deref();
            let ticker_gt = ticker_gt.as_deref();
            let ticker_gte = ticker_gte.as_deref();
            let ticker_lt = ticker_lt.as_deref();
            let ticker_lte = ticker_lte.as_deref();
            let cik = cik.as_deref();
            let cik_any_of = cik_any_of.as_deref();
            let cik_gt = cik_gt.as_deref();
            let cik_gte = cik_gte.as_deref();
            let cik_lt = cik_lt.as_deref();
            let cik_lte = cik_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/stocks/filings/vX/risk-factors".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = filing_date {
                query.push(("filing_date", v.to_string()));
            }
            if let Some(v) = filing_date_any_of {
                query.push(("filing_date.any_of", v.to_string()));
            }
            if let Some(v) = filing_date_gt {
                query.push(("filing_date.gt", v.to_string()));
            }
            if let Some(v) = filing_date_gte {
                query.push(("filing_date.gte", v.to_string()));
            }
            if let Some(v) = filing_date_lt {
                query.push(("filing_date.lt", v.to_string()));
            }
            if let Some(v) = filing_date_lte {
                query.push(("filing_date.lte", v.to_string()));
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
            if let Some(v) = cik {
                query.push(("cik", v.to_string()));
            }
            if let Some(v) = cik_any_of {
                query.push(("cik.any_of", v.to_string()));
            }
            if let Some(v) = cik_gt {
                query.push(("cik.gt", v.to_string()));
            }
            if let Some(v) = cik_gte {
                query.push(("cik.gte", v.to_string()));
            }
            if let Some(v) = cik_lt {
                query.push(("cik.lt", v.to_string()));
            }
            if let Some(v) = cik_lte {
                query.push(("cik.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            if self.pagination {
                self.paginate::<RiskFactor>(&path, Some(&query), options)
            } else {
                self.single_page::<RiskFactor>(&path, Some(&query), options)
            }
        })
    }

    fn list_stocks_taxonomies_risk_factors<'a>(
        &'a self,
        taxonomy: Option<f64>,
        taxonomy_gt: Option<f64>,
        taxonomy_gte: Option<f64>,
        taxonomy_lt: Option<f64>,
        taxonomy_lte: Option<f64>,
        primary_category: Option<&'a str>,
        primary_category_any_of: Option<&'a str>,
        primary_category_gt: Option<&'a str>,
        primary_category_gte: Option<&'a str>,
        primary_category_lt: Option<&'a str>,
        primary_category_lte: Option<&'a str>,
        secondary_category: Option<&'a str>,
        secondary_category_any_of: Option<&'a str>,
        secondary_category_gt: Option<&'a str>,
        secondary_category_gte: Option<&'a str>,
        secondary_category_lt: Option<&'a str>,
        secondary_category_lte: Option<&'a str>,
        tertiary_category: Option<&'a str>,
        tertiary_category_any_of: Option<&'a str>,
        tertiary_category_gt: Option<&'a str>,
        tertiary_category_gte: Option<&'a str>,
        tertiary_category_lt: Option<&'a str>,
        tertiary_category_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, RiskFactorTaxonomy> {
        self.list_stocks_taxonomies_risk_factors_with_params(
            ListStocksTaxonomiesRiskFactorsParams {
                taxonomy,
                taxonomy_gt,
                taxonomy_gte,
                taxonomy_lt,
                taxonomy_lte,
                primary_category: primary_category.map(String::from),
                primary_category_any_of: primary_category_any_of.map(String::from),
                primary_category_gt: primary_category_gt.map(String::from),
                primary_category_gte: primary_category_gte.map(String::from),
                primary_category_lt: primary_category_lt.map(String::from),
                primary_category_lte: primary_category_lte.map(String::from),
                secondary_category: secondary_category.map(String::from),
                secondary_category_any_of: secondary_category_any_of.map(String::from),
                secondary_category_gt: secondary_category_gt.map(String::from),
                secondary_category_gte: secondary_category_gte.map(String::from),
                secondary_category_lt: secondary_category_lt.map(String::from),
                secondary_category_lte: secondary_category_lte.map(String::from),
                tertiary_category: tertiary_category.map(String::from),
                tertiary_category_any_of: tertiary_category_any_of.map(String::from),
                tertiary_category_gt: tertiary_category_gt.map(String::from),
                tertiary_category_gte: tertiary_category_gte.map(String::from),
                tertiary_category_lt: tertiary_category_lt.map(String::from),
                tertiary_category_lte: tertiary_category_lte.map(String::from),
                limit,
                sort: sort.map(String::from),
                options: options.cloned(),
            },
        )
    }

    fn list_stocks_taxonomies_risk_factors_with_params<'a>(
        &'a self,
        params: ListStocksTaxonomiesRiskFactorsParams,
    ) -> BoxStream<'a, RiskFactorTaxonomy> {
        Box::pin({
            let ListStocksTaxonomiesRiskFactorsParams {
                taxonomy,
                taxonomy_gt,
                taxonomy_gte,
                taxonomy_lt,
                taxonomy_lte,
                primary_category,
                primary_category_any_of,
                primary_category_gt,
                primary_category_gte,
                primary_category_lt,
                primary_category_lte,
                secondary_category,
                secondary_category_any_of,
                secondary_category_gt,
                secondary_category_gte,
                secondary_category_lt,
                secondary_category_lte,
                tertiary_category,
                tertiary_category_any_of,
                tertiary_category_gt,
                tertiary_category_gte,
                tertiary_category_lt,
                tertiary_category_lte,
                limit,
                sort,
                options,
            } = params;
            let primary_category = primary_category.as_deref();
            let primary_category_any_of = primary_category_any_of.as_deref();
            let primary_category_gt = primary_category_gt.as_deref();
            let primary_category_gte = primary_category_gte.as_deref();
            let primary_category_lt = primary_category_lt.as_deref();
            let primary_category_lte = primary_category_lte.as_deref();
            let secondary_category = secondary_category.as_deref();
            let secondary_category_any_of = secondary_category_any_of.as_deref();
            let secondary_category_gt = secondary_category_gt.as_deref();
            let secondary_category_gte = secondary_category_gte.as_deref();
            let secondary_category_lt = secondary_category_lt.as_deref();
            let secondary_category_lte = secondary_category_lte.as_deref();
            let tertiary_category = tertiary_category.as_deref();
            let tertiary_category_any_of = tertiary_category_any_of.as_deref();
            let tertiary_category_gt = tertiary_category_gt.as_deref();
            let tertiary_category_gte = tertiary_category_gte.as_deref();
            let tertiary_category_lt = tertiary_category_lt.as_deref();
            let tertiary_category_lte = tertiary_category_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/stocks/taxonomies/vX/risk-factors".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = taxonomy {
                query.push(("taxonomy", v.to_string()));
            }
            if let Some(v) = taxonomy_gt {
                query.push(("taxonomy.gt", v.to_string()));
            }
            if let Some(v) = taxonomy_gte {
                query.push(("taxonomy.gte", v.to_string()));
            }
            if let Some(v) = taxonomy_lt {
                query.push(("taxonomy.lt", v.to_string()));
            }
            if let Some(v) = taxonomy_lte {
                query.push(("taxonomy.lte", v.to_string()));
            }
            if let Some(v) = primary_category {
                query.push(("primary_category", v.to_string()));
            }
            if let Some(v) = primary_category_any_of {
                query.push(("primary_category.any_of", v.to_string()));
            }
            if let Some(v) = primary_category_gt {
                query.push(("primary_category.gt", v.to_string()));
            }
            if let Some(v) = primary_category_gte {
                query.push(("primary_category.gte", v.to_string()));
            }
            if let Some(v) = primary_category_lt {
                query.push(("primary_category.lt", v.to_string()));
            }
            if let Some(v) = primary_category_lte {
                query.push(("primary_category.lte", v.to_string()));
            }
            if let Some(v) = secondary_category {
                query.push(("secondary_category", v.to_string()));
            }
            if let Some(v) = secondary_category_any_of {
                query.push(("secondary_category.any_of", v.to_string()));
            }
            if let Some(v) = secondary_category_gt {
                query.push(("secondary_category.gt", v.to_string()));
            }
            if let Some(v) = secondary_category_gte {
                query.push(("secondary_category.gte", v.to_string()));
            }
            if let Some(v) = secondary_category_lt {
                query.push(("secondary_category.lt", v.to_string()));
            }
            if let Some(v) = secondary_category_lte {
                query.push(("secondary_category.lte", v.to_string()));
            }
            if let Some(v) = tertiary_category {
                query.push(("tertiary_category", v.to_string()));
            }
            if let Some(v) = tertiary_category_any_of {
                query.push(("tertiary_category.any_of", v.to_string()));
            }
            if let Some(v) = tertiary_category_gt {
                query.push(("tertiary_category.gt", v.to_string()));
            }
            if let Some(v) = tertiary_category_gte {
                query.push(("tertiary_category.gte", v.to_string()));
            }
            if let Some(v) = tertiary_category_lt {
                query.push(("tertiary_category.lt", v.to_string()));
            }
            if let Some(v) = tertiary_category_lte {
                query.push(("tertiary_category.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            if self.pagination {
                self.paginate::<RiskFactorTaxonomy>(&path, Some(&query), options)
            } else {
                self.single_page::<RiskFactorTaxonomy>(&path, Some(&query), options)
            }
        })
    }

    fn list_stocks_filings_8k_disclosures<'a>(
        &'a self,
        cik: Option<&'a str>,
        cik_any_of: Option<&'a str>,
        tickers: Option<&'a str>,
        tickers_all_of: Option<&'a str>,
        tickers_any_of: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_any_of: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        tertiary_category: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, Disclosure> {
        self.list_stocks_filings_8k_disclosures_with_params(ListStocksFilings8kDisclosuresParams {
            cik: cik.map(String::from),
            cik_any_of: cik_any_of.map(String::from),
            tickers: tickers.map(String::from),
            tickers_all_of: tickers_all_of.map(String::from),
            tickers_any_of: tickers_any_of.map(String::from),
            filing_date: filing_date.map(String::from),
            filing_date_any_of: filing_date_any_of.map(String::from),
            filing_date_gt: filing_date_gt.map(String::from),
            filing_date_gte: filing_date_gte.map(String::from),
            filing_date_lt: filing_date_lt.map(String::from),
            filing_date_lte: filing_date_lte.map(String::from),
            tertiary_category: tertiary_category.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_stocks_filings_8k_disclosures_with_params<'a>(
        &'a self,
        params: ListStocksFilings8kDisclosuresParams,
    ) -> BoxStream<'a, Disclosure> {
        Box::pin({
            let ListStocksFilings8kDisclosuresParams {
                cik,
                cik_any_of,
                tickers,
                tickers_all_of,
                tickers_any_of,
                filing_date,
                filing_date_any_of,
                filing_date_gt,
                filing_date_gte,
                filing_date_lt,
                filing_date_lte,
                tertiary_category,
                limit,
                sort,
                options,
            } = params;
            let cik = cik.as_deref();
            let cik_any_of = cik_any_of.as_deref();
            let tickers = tickers.as_deref();
            let tickers_all_of = tickers_all_of.as_deref();
            let tickers_any_of = tickers_any_of.as_deref();
            let filing_date = filing_date.as_deref();
            let filing_date_any_of = filing_date_any_of.as_deref();
            let filing_date_gt = filing_date_gt.as_deref();
            let filing_date_gte = filing_date_gte.as_deref();
            let filing_date_lt = filing_date_lt.as_deref();
            let filing_date_lte = filing_date_lte.as_deref();
            let tertiary_category = tertiary_category.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/stocks/filings/8-K/vX/disclosures".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = cik {
                query.push(("cik", v.to_string()));
            }
            if let Some(v) = cik_any_of {
                query.push(("cik.any_of", v.to_string()));
            }
            if let Some(v) = tickers {
                query.push(("tickers", v.to_string()));
            }
            if let Some(v) = tickers_all_of {
                query.push(("tickers.all_of", v.to_string()));
            }
            if let Some(v) = tickers_any_of {
                query.push(("tickers.any_of", v.to_string()));
            }
            if let Some(v) = filing_date {
                query.push(("filing_date", v.to_string()));
            }
            if let Some(v) = filing_date_any_of {
                query.push(("filing_date.any_of", v.to_string()));
            }
            if let Some(v) = filing_date_gt {
                query.push(("filing_date.gt", v.to_string()));
            }
            if let Some(v) = filing_date_gte {
                query.push(("filing_date.gte", v.to_string()));
            }
            if let Some(v) = filing_date_lt {
                query.push(("filing_date.lt", v.to_string()));
            }
            if let Some(v) = filing_date_lte {
                query.push(("filing_date.lte", v.to_string()));
            }
            if let Some(v) = tertiary_category {
                query.push(("tertiary_category", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            if self.pagination {
                self.paginate::<Disclosure>(&path, Some(&query), options)
            } else {
                self.single_page::<Disclosure>(&path, Some(&query), options)
            }
        })
    }

    fn list_stocks_taxonomies_disclosures<'a>(
        &'a self,
        taxonomy: Option<&'a str>,
        taxonomy_any_of: Option<&'a str>,
        taxonomy_gt: Option<&'a str>,
        taxonomy_gte: Option<&'a str>,
        taxonomy_lt: Option<&'a str>,
        taxonomy_lte: Option<&'a str>,
        primary_category: Option<&'a str>,
        primary_category_any_of: Option<&'a str>,
        primary_category_gt: Option<&'a str>,
        primary_category_gte: Option<&'a str>,
        primary_category_lt: Option<&'a str>,
        primary_category_lte: Option<&'a str>,
        secondary_category: Option<&'a str>,
        secondary_category_any_of: Option<&'a str>,
        secondary_category_gt: Option<&'a str>,
        secondary_category_gte: Option<&'a str>,
        secondary_category_lt: Option<&'a str>,
        secondary_category_lte: Option<&'a str>,
        tertiary_category: Option<&'a str>,
        tertiary_category_any_of: Option<&'a str>,
        tertiary_category_gt: Option<&'a str>,
        tertiary_category_gte: Option<&'a str>,
        tertiary_category_lt: Option<&'a str>,
        tertiary_category_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, DisclosureTaxonomy> {
        self.list_stocks_taxonomies_disclosures_with_params(ListStocksTaxonomiesDisclosuresParams {
            taxonomy: taxonomy.map(String::from),
            taxonomy_any_of: taxonomy_any_of.map(String::from),
            taxonomy_gt: taxonomy_gt.map(String::from),
            taxonomy_gte: taxonomy_gte.map(String::from),
            taxonomy_lt: taxonomy_lt.map(String::from),
            taxonomy_lte: taxonomy_lte.map(String::from),
            primary_category: primary_category.map(String::from),
            primary_category_any_of: primary_category_any_of.map(String::from),
            primary_category_gt: primary_category_gt.map(String::from),
            primary_category_gte: primary_category_gte.map(String::from),
            primary_category_lt: primary_category_lt.map(String::from),
            primary_category_lte: primary_category_lte.map(String::from),
            secondary_category: secondary_category.map(String::from),
            secondary_category_any_of: secondary_category_any_of.map(String::from),
            secondary_category_gt: secondary_category_gt.map(String::from),
            secondary_category_gte: secondary_category_gte.map(String::from),
            secondary_category_lt: secondary_category_lt.map(String::from),
            secondary_category_lte: secondary_category_lte.map(String::from),
            tertiary_category: tertiary_category.map(String::from),
            tertiary_category_any_of: tertiary_category_any_of.map(String::from),
            tertiary_category_gt: tertiary_category_gt.map(String::from),
            tertiary_category_gte: tertiary_category_gte.map(String::from),
            tertiary_category_lt: tertiary_category_lt.map(String::from),
            tertiary_category_lte: tertiary_category_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_stocks_taxonomies_disclosures_with_params<'a>(
        &'a self,
        params: ListStocksTaxonomiesDisclosuresParams,
    ) -> BoxStream<'a, DisclosureTaxonomy> {
        Box::pin({
            let ListStocksTaxonomiesDisclosuresParams {
                taxonomy,
                taxonomy_any_of,
                taxonomy_gt,
                taxonomy_gte,
                taxonomy_lt,
                taxonomy_lte,
                primary_category,
                primary_category_any_of,
                primary_category_gt,
                primary_category_gte,
                primary_category_lt,
                primary_category_lte,
                secondary_category,
                secondary_category_any_of,
                secondary_category_gt,
                secondary_category_gte,
                secondary_category_lt,
                secondary_category_lte,
                tertiary_category,
                tertiary_category_any_of,
                tertiary_category_gt,
                tertiary_category_gte,
                tertiary_category_lt,
                tertiary_category_lte,
                limit,
                sort,
                options,
            } = params;
            let taxonomy = taxonomy.as_deref();
            let taxonomy_any_of = taxonomy_any_of.as_deref();
            let taxonomy_gt = taxonomy_gt.as_deref();
            let taxonomy_gte = taxonomy_gte.as_deref();
            let taxonomy_lt = taxonomy_lt.as_deref();
            let taxonomy_lte = taxonomy_lte.as_deref();
            let primary_category = primary_category.as_deref();
            let primary_category_any_of = primary_category_any_of.as_deref();
            let primary_category_gt = primary_category_gt.as_deref();
            let primary_category_gte = primary_category_gte.as_deref();
            let primary_category_lt = primary_category_lt.as_deref();
            let primary_category_lte = primary_category_lte.as_deref();
            let secondary_category = secondary_category.as_deref();
            let secondary_category_any_of = secondary_category_any_of.as_deref();
            let secondary_category_gt = secondary_category_gt.as_deref();
            let secondary_category_gte = secondary_category_gte.as_deref();
            let secondary_category_lt = secondary_category_lt.as_deref();
            let secondary_category_lte = secondary_category_lte.as_deref();
            let tertiary_category = tertiary_category.as_deref();
            let tertiary_category_any_of = tertiary_category_any_of.as_deref();
            let tertiary_category_gt = tertiary_category_gt.as_deref();
            let tertiary_category_gte = tertiary_category_gte.as_deref();
            let tertiary_category_lt = tertiary_category_lt.as_deref();
            let tertiary_category_lte = tertiary_category_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/stocks/taxonomies/vX/disclosures".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = taxonomy {
                query.push(("taxonomy", v.to_string()));
            }
            if let Some(v) = taxonomy_any_of {
                query.push(("taxonomy.any_of", v.to_string()));
            }
            if let Some(v) = taxonomy_gt {
                query.push(("taxonomy.gt", v.to_string()));
            }
            if let Some(v) = taxonomy_gte {
                query.push(("taxonomy.gte", v.to_string()));
            }
            if let Some(v) = taxonomy_lt {
                query.push(("taxonomy.lt", v.to_string()));
            }
            if let Some(v) = taxonomy_lte {
                query.push(("taxonomy.lte", v.to_string()));
            }
            if let Some(v) = primary_category {
                query.push(("primary_category", v.to_string()));
            }
            if let Some(v) = primary_category_any_of {
                query.push(("primary_category.any_of", v.to_string()));
            }
            if let Some(v) = primary_category_gt {
                query.push(("primary_category.gt", v.to_string()));
            }
            if let Some(v) = primary_category_gte {
                query.push(("primary_category.gte", v.to_string()));
            }
            if let Some(v) = primary_category_lt {
                query.push(("primary_category.lt", v.to_string()));
            }
            if let Some(v) = primary_category_lte {
                query.push(("primary_category.lte", v.to_string()));
            }
            if let Some(v) = secondary_category {
                query.push(("secondary_category", v.to_string()));
            }
            if let Some(v) = secondary_category_any_of {
                query.push(("secondary_category.any_of", v.to_string()));
            }
            if let Some(v) = secondary_category_gt {
                query.push(("secondary_category.gt", v.to_string()));
            }
            if let Some(v) = secondary_category_gte {
                query.push(("secondary_category.gte", v.to_string()));
            }
            if let Some(v) = secondary_category_lt {
                query.push(("secondary_category.lt", v.to_string()));
            }
            if let Some(v) = secondary_category_lte {
                query.push(("secondary_category.lte", v.to_string()));
            }
            if let Some(v) = tertiary_category {
                query.push(("tertiary_category", v.to_string()));
            }
            if let Some(v) = tertiary_category_any_of {
                query.push(("tertiary_category.any_of", v.to_string()));
            }
            if let Some(v) = tertiary_category_gt {
                query.push(("tertiary_category.gt", v.to_string()));
            }
            if let Some(v) = tertiary_category_gte {
                query.push(("tertiary_category.gte", v.to_string()));
            }
            if let Some(v) = tertiary_category_lt {
                query.push(("tertiary_category.lt", v.to_string()));
            }
            if let Some(v) = tertiary_category_lte {
                query.push(("tertiary_category.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            if self.pagination {
                self.paginate::<DisclosureTaxonomy>(&path, Some(&query), options)
            } else {
                self.single_page::<DisclosureTaxonomy>(&path, Some(&query), options)
            }
        })
    }

    fn list_stocks_filings_10k_sections<'a>(
        &'a self,
        cik: Option<&'a str>,
        cik_any_of: Option<&'a str>,
        cik_gt: Option<&'a str>,
        cik_gte: Option<&'a str>,
        cik_lt: Option<&'a str>,
        cik_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        section: Option<&'a str>,
        section_any_of: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        period_end: Option<&'a str>,
        period_end_gt: Option<&'a str>,
        period_end_gte: Option<&'a str>,
        period_end_lt: Option<&'a str>,
        period_end_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FilingSection> {
        self.list_stocks_filings_10k_sections_with_params(ListStocksFilings10kSectionsParams {
            cik: cik.map(String::from),
            cik_any_of: cik_any_of.map(String::from),
            cik_gt: cik_gt.map(String::from),
            cik_gte: cik_gte.map(String::from),
            cik_lt: cik_lt.map(String::from),
            cik_lte: cik_lte.map(String::from),
            ticker: ticker.map(String::from),
            ticker_any_of: ticker_any_of.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            section: section.map(String::from),
            section_any_of: section_any_of.map(String::from),
            filing_date: filing_date.map(String::from),
            filing_date_gt: filing_date_gt.map(String::from),
            filing_date_gte: filing_date_gte.map(String::from),
            filing_date_lt: filing_date_lt.map(String::from),
            filing_date_lte: filing_date_lte.map(String::from),
            period_end: period_end.map(String::from),
            period_end_gt: period_end_gt.map(String::from),
            period_end_gte: period_end_gte.map(String::from),
            period_end_lt: period_end_lt.map(String::from),
            period_end_lte: period_end_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_stocks_filings_10k_sections_with_params<'a>(
        &'a self,
        params: ListStocksFilings10kSectionsParams,
    ) -> BoxStream<'a, FilingSection> {
        Box::pin({
            let ListStocksFilings10kSectionsParams {
                cik,
                cik_any_of,
                cik_gt,
                cik_gte,
                cik_lt,
                cik_lte,
                ticker,
                ticker_any_of,
                ticker_gt,
                ticker_gte,
                ticker_lt,
                ticker_lte,
                section,
                section_any_of,
                filing_date,
                filing_date_gt,
                filing_date_gte,
                filing_date_lt,
                filing_date_lte,
                period_end,
                period_end_gt,
                period_end_gte,
                period_end_lt,
                period_end_lte,
                limit,
                sort,
                options,
            } = params;
            let cik = cik.as_deref();
            let cik_any_of = cik_any_of.as_deref();
            let cik_gt = cik_gt.as_deref();
            let cik_gte = cik_gte.as_deref();
            let cik_lt = cik_lt.as_deref();
            let cik_lte = cik_lte.as_deref();
            let ticker = ticker.as_deref();
            let ticker_any_of = ticker_any_of.as_deref();
            let ticker_gt = ticker_gt.as_deref();
            let ticker_gte = ticker_gte.as_deref();
            let ticker_lt = ticker_lt.as_deref();
            let ticker_lte = ticker_lte.as_deref();
            let section = section.as_deref();
            let section_any_of = section_any_of.as_deref();
            let filing_date = filing_date.as_deref();
            let filing_date_gt = filing_date_gt.as_deref();
            let filing_date_gte = filing_date_gte.as_deref();
            let filing_date_lt = filing_date_lt.as_deref();
            let filing_date_lte = filing_date_lte.as_deref();
            let period_end = period_end.as_deref();
            let period_end_gt = period_end_gt.as_deref();
            let period_end_gte = period_end_gte.as_deref();
            let period_end_lt = period_end_lt.as_deref();
            let period_end_lte = period_end_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/stocks/filings/10-K/vX/sections".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = cik {
                query.push(("cik", v.to_string()));
            }
            if let Some(v) = cik_any_of {
                query.push(("cik.any_of", v.to_string()));
            }
            if let Some(v) = cik_gt {
                query.push(("cik.gt", v.to_string()));
            }
            if let Some(v) = cik_gte {
                query.push(("cik.gte", v.to_string()));
            }
            if let Some(v) = cik_lt {
                query.push(("cik.lt", v.to_string()));
            }
            if let Some(v) = cik_lte {
                query.push(("cik.lte", v.to_string()));
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
            if let Some(v) = section {
                query.push(("section", v.to_string()));
            }
            if let Some(v) = section_any_of {
                query.push(("section.any_of", v.to_string()));
            }
            if let Some(v) = filing_date {
                query.push(("filing_date", v.to_string()));
            }
            if let Some(v) = filing_date_gt {
                query.push(("filing_date.gt", v.to_string()));
            }
            if let Some(v) = filing_date_gte {
                query.push(("filing_date.gte", v.to_string()));
            }
            if let Some(v) = filing_date_lt {
                query.push(("filing_date.lt", v.to_string()));
            }
            if let Some(v) = filing_date_lte {
                query.push(("filing_date.lte", v.to_string()));
            }
            if let Some(v) = period_end {
                query.push(("period_end", v.to_string()));
            }
            if let Some(v) = period_end_gt {
                query.push(("period_end.gt", v.to_string()));
            }
            if let Some(v) = period_end_gte {
                query.push(("period_end.gte", v.to_string()));
            }
            if let Some(v) = period_end_lt {
                query.push(("period_end.lt", v.to_string()));
            }
            if let Some(v) = period_end_lte {
                query.push(("period_end.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            if self.pagination {
                self.paginate::<FilingSection>(&path, Some(&query), options)
            } else {
                self.single_page::<FilingSection>(&path, Some(&query), options)
            }
        })
    }

    fn list_stocks_filings_8k_text<'a>(
        &'a self,
        cik: Option<&'a str>,
        cik_any_of: Option<&'a str>,
        cik_gt: Option<&'a str>,
        cik_gte: Option<&'a str>,
        cik_lt: Option<&'a str>,
        cik_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        form_type: Option<&'a str>,
        form_type_any_of: Option<&'a str>,
        form_type_gt: Option<&'a str>,
        form_type_gte: Option<&'a str>,
        form_type_lt: Option<&'a str>,
        form_type_lte: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, Filing8K> {
        self.list_stocks_filings_8k_text_with_params(ListStocksFilings8kTextParams {
            cik: cik.map(String::from),
            cik_any_of: cik_any_of.map(String::from),
            cik_gt: cik_gt.map(String::from),
            cik_gte: cik_gte.map(String::from),
            cik_lt: cik_lt.map(String::from),
            cik_lte: cik_lte.map(String::from),
            ticker: ticker.map(String::from),
            ticker_any_of: ticker_any_of.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            form_type: form_type.map(String::from),
            form_type_any_of: form_type_any_of.map(String::from),
            form_type_gt: form_type_gt.map(String::from),
            form_type_gte: form_type_gte.map(String::from),
            form_type_lt: form_type_lt.map(String::from),
            form_type_lte: form_type_lte.map(String::from),
            filing_date: filing_date.map(String::from),
            filing_date_gt: filing_date_gt.map(String::from),
            filing_date_gte: filing_date_gte.map(String::from),
            filing_date_lt: filing_date_lt.map(String::from),
            filing_date_lte: filing_date_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_stocks_filings_8k_text_with_params<'a>(
        &'a self,
        params: ListStocksFilings8kTextParams,
    ) -> BoxStream<'a, Filing8K> {
        Box::pin({
            let ListStocksFilings8kTextParams {
                cik,
                cik_any_of,
                cik_gt,
                cik_gte,
                cik_lt,
                cik_lte,
                ticker,
                ticker_any_of,
                ticker_gt,
                ticker_gte,
                ticker_lt,
                ticker_lte,
                form_type,
                form_type_any_of,
                form_type_gt,
                form_type_gte,
                form_type_lt,
                form_type_lte,
                filing_date,
                filing_date_gt,
                filing_date_gte,
                filing_date_lt,
                filing_date_lte,
                limit,
                sort,
                options,
            } = params;
            let cik = cik.as_deref();
            let cik_any_of = cik_any_of.as_deref();
            let cik_gt = cik_gt.as_deref();
            let cik_gte = cik_gte.as_deref();
            let cik_lt = cik_lt.as_deref();
            let cik_lte = cik_lte.as_deref();
            let ticker = ticker.as_deref();
            let ticker_any_of = ticker_any_of.as_deref();
            let ticker_gt = ticker_gt.as_deref();
            let ticker_gte = ticker_gte.as_deref();
            let ticker_lt = ticker_lt.as_deref();
            let ticker_lte = ticker_lte.as_deref();
            let form_type = form_type.as_deref();
            let form_type_any_of = form_type_any_of.as_deref();
            let form_type_gt = form_type_gt.as_deref();
            let form_type_gte = form_type_gte.as_deref();
            let form_type_lt = form_type_lt.as_deref();
            let form_type_lte = form_type_lte.as_deref();
            let filing_date = filing_date.as_deref();
            let filing_date_gt = filing_date_gt.as_deref();
            let filing_date_gte = filing_date_gte.as_deref();
            let filing_date_lt = filing_date_lt.as_deref();
            let filing_date_lte = filing_date_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/stocks/filings/8-K/vX/text".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = cik {
                query.push(("cik", v.to_string()));
            }
            if let Some(v) = cik_any_of {
                query.push(("cik.any_of", v.to_string()));
            }
            if let Some(v) = cik_gt {
                query.push(("cik.gt", v.to_string()));
            }
            if let Some(v) = cik_gte {
                query.push(("cik.gte", v.to_string()));
            }
            if let Some(v) = cik_lt {
                query.push(("cik.lt", v.to_string()));
            }
            if let Some(v) = cik_lte {
                query.push(("cik.lte", v.to_string()));
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
            if let Some(v) = form_type {
                query.push(("form_type", v.to_string()));
            }
            if let Some(v) = form_type_any_of {
                query.push(("form_type.any_of", v.to_string()));
            }
            if let Some(v) = form_type_gt {
                query.push(("form_type.gt", v.to_string()));
            }
            if let Some(v) = form_type_gte {
                query.push(("form_type.gte", v.to_string()));
            }
            if let Some(v) = form_type_lt {
                query.push(("form_type.lt", v.to_string()));
            }
            if let Some(v) = form_type_lte {
                query.push(("form_type.lte", v.to_string()));
            }
            if let Some(v) = filing_date {
                query.push(("filing_date", v.to_string()));
            }
            if let Some(v) = filing_date_gt {
                query.push(("filing_date.gt", v.to_string()));
            }
            if let Some(v) = filing_date_gte {
                query.push(("filing_date.gte", v.to_string()));
            }
            if let Some(v) = filing_date_lt {
                query.push(("filing_date.lt", v.to_string()));
            }
            if let Some(v) = filing_date_lte {
                query.push(("filing_date.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            if self.pagination {
                self.paginate::<Filing8K>(&path, Some(&query), options)
            } else {
                self.single_page::<Filing8K>(&path, Some(&query), options)
            }
        })
    }

    fn list_stocks_filings_index<'a>(
        &'a self,
        cik: Option<&'a str>,
        cik_any_of: Option<&'a str>,
        cik_gt: Option<&'a str>,
        cik_gte: Option<&'a str>,
        cik_lt: Option<&'a str>,
        cik_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        form_type: Option<&'a str>,
        form_type_any_of: Option<&'a str>,
        form_type_gt: Option<&'a str>,
        form_type_gte: Option<&'a str>,
        form_type_lt: Option<&'a str>,
        form_type_lte: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FilingIndex> {
        self.list_stocks_filings_index_with_params(ListStocksFilingsIndexParams {
            cik: cik.map(String::from),
            cik_any_of: cik_any_of.map(String::from),
            cik_gt: cik_gt.map(String::from),
            cik_gte: cik_gte.map(String::from),
            cik_lt: cik_lt.map(String::from),
            cik_lte: cik_lte.map(String::from),
            ticker: ticker.map(String::from),
            ticker_any_of: ticker_any_of.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            form_type: form_type.map(String::from),
            form_type_any_of: form_type_any_of.map(String::from),
            form_type_gt: form_type_gt.map(String::from),
            form_type_gte: form_type_gte.map(String::from),
            form_type_lt: form_type_lt.map(String::from),
            form_type_lte: form_type_lte.map(String::from),
            filing_date: filing_date.map(String::from),
            filing_date_gt: filing_date_gt.map(String::from),
            filing_date_gte: filing_date_gte.map(String::from),
            filing_date_lt: filing_date_lt.map(String::from),
            filing_date_lte: filing_date_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_stocks_filings_index_with_params<'a>(
        &'a self,
        params: ListStocksFilingsIndexParams,
    ) -> BoxStream<'a, FilingIndex> {
        Box::pin({
            let ListStocksFilingsIndexParams {
                cik,
                cik_any_of,
                cik_gt,
                cik_gte,
                cik_lt,
                cik_lte,
                ticker,
                ticker_any_of,
                ticker_gt,
                ticker_gte,
                ticker_lt,
                ticker_lte,
                form_type,
                form_type_any_of,
                form_type_gt,
                form_type_gte,
                form_type_lt,
                form_type_lte,
                filing_date,
                filing_date_gt,
                filing_date_gte,
                filing_date_lt,
                filing_date_lte,
                limit,
                sort,
                options,
            } = params;
            let cik = cik.as_deref();
            let cik_any_of = cik_any_of.as_deref();
            let cik_gt = cik_gt.as_deref();
            let cik_gte = cik_gte.as_deref();
            let cik_lt = cik_lt.as_deref();
            let cik_lte = cik_lte.as_deref();
            let ticker = ticker.as_deref();
            let ticker_any_of = ticker_any_of.as_deref();
            let ticker_gt = ticker_gt.as_deref();
            let ticker_gte = ticker_gte.as_deref();
            let ticker_lt = ticker_lt.as_deref();
            let ticker_lte = ticker_lte.as_deref();
            let form_type = form_type.as_deref();
            let form_type_any_of = form_type_any_of.as_deref();
            let form_type_gt = form_type_gt.as_deref();
            let form_type_gte = form_type_gte.as_deref();
            let form_type_lt = form_type_lt.as_deref();
            let form_type_lte = form_type_lte.as_deref();
            let filing_date = filing_date.as_deref();
            let filing_date_gt = filing_date_gt.as_deref();
            let filing_date_gte = filing_date_gte.as_deref();
            let filing_date_lt = filing_date_lt.as_deref();
            let filing_date_lte = filing_date_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/stocks/filings/vX/index".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = cik {
                query.push(("cik", v.to_string()));
            }
            if let Some(v) = cik_any_of {
                query.push(("cik.any_of", v.to_string()));
            }
            if let Some(v) = cik_gt {
                query.push(("cik.gt", v.to_string()));
            }
            if let Some(v) = cik_gte {
                query.push(("cik.gte", v.to_string()));
            }
            if let Some(v) = cik_lt {
                query.push(("cik.lt", v.to_string()));
            }
            if let Some(v) = cik_lte {
                query.push(("cik.lte", v.to_string()));
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
            if let Some(v) = form_type {
                query.push(("form_type", v.to_string()));
            }
            if let Some(v) = form_type_any_of {
                query.push(("form_type.any_of", v.to_string()));
            }
            if let Some(v) = form_type_gt {
                query.push(("form_type.gt", v.to_string()));
            }
            if let Some(v) = form_type_gte {
                query.push(("form_type.gte", v.to_string()));
            }
            if let Some(v) = form_type_lt {
                query.push(("form_type.lt", v.to_string()));
            }
            if let Some(v) = form_type_lte {
                query.push(("form_type.lte", v.to_string()));
            }
            if let Some(v) = filing_date {
                query.push(("filing_date", v.to_string()));
            }
            if let Some(v) = filing_date_gt {
                query.push(("filing_date.gt", v.to_string()));
            }
            if let Some(v) = filing_date_gte {
                query.push(("filing_date.gte", v.to_string()));
            }
            if let Some(v) = filing_date_lt {
                query.push(("filing_date.lt", v.to_string()));
            }
            if let Some(v) = filing_date_lte {
                query.push(("filing_date.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            if self.pagination {
                self.paginate::<FilingIndex>(&path, Some(&query), options)
            } else {
                self.single_page::<FilingIndex>(&path, Some(&query), options)
            }
        })
    }

    fn list_stocks_filings_13f<'a>(
        &'a self,
        filer_cik: Option<&'a str>,
        filer_cik_any_of: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, Filing13F> {
        self.list_stocks_filings_13f_with_params(ListStocksFilings13fParams {
            filer_cik: filer_cik.map(String::from),
            filer_cik_any_of: filer_cik_any_of.map(String::from),
            filing_date: filing_date.map(String::from),
            filing_date_gt: filing_date_gt.map(String::from),
            filing_date_gte: filing_date_gte.map(String::from),
            filing_date_lt: filing_date_lt.map(String::from),
            filing_date_lte: filing_date_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_stocks_filings_13f_with_params<'a>(
        &'a self,
        params: ListStocksFilings13fParams,
    ) -> BoxStream<'a, Filing13F> {
        Box::pin({
            let ListStocksFilings13fParams {
                filer_cik,
                filer_cik_any_of,
                filing_date,
                filing_date_gt,
                filing_date_gte,
                filing_date_lt,
                filing_date_lte,
                limit,
                sort,
                options,
            } = params;
            let filer_cik = filer_cik.as_deref();
            let filer_cik_any_of = filer_cik_any_of.as_deref();
            let filing_date = filing_date.as_deref();
            let filing_date_gt = filing_date_gt.as_deref();
            let filing_date_gte = filing_date_gte.as_deref();
            let filing_date_lt = filing_date_lt.as_deref();
            let filing_date_lte = filing_date_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/stocks/filings/vX/13-F".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = filer_cik {
                query.push(("filer_cik", v.to_string()));
            }
            if let Some(v) = filer_cik_any_of {
                query.push(("filer_cik.any_of", v.to_string()));
            }
            if let Some(v) = filing_date {
                query.push(("filing_date", v.to_string()));
            }
            if let Some(v) = filing_date_gt {
                query.push(("filing_date.gt", v.to_string()));
            }
            if let Some(v) = filing_date_gte {
                query.push(("filing_date.gte", v.to_string()));
            }
            if let Some(v) = filing_date_lt {
                query.push(("filing_date.lt", v.to_string()));
            }
            if let Some(v) = filing_date_lte {
                query.push(("filing_date.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            if self.pagination {
                self.paginate::<Filing13F>(&path, Some(&query), options)
            } else {
                self.single_page::<Filing13F>(&path, Some(&query), options)
            }
        })
    }

    fn list_stocks_filings_form_3<'a>(
        &'a self,
        issuer_cik: Option<&'a str>,
        issuer_cik_any_of: Option<&'a str>,
        owner_cik: Option<&'a str>,
        owner_cik_any_of: Option<&'a str>,
        tickers: Option<&'a str>,
        tickers_all_of: Option<&'a str>,
        tickers_any_of: Option<&'a str>,
        form_type: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        max_ticker: Option<&'a str>,
        max_ticker_any_of: Option<&'a str>,
        max_ticker_gt: Option<&'a str>,
        max_ticker_gte: Option<&'a str>,
        max_ticker_lt: Option<&'a str>,
        max_ticker_lte: Option<&'a str>,
        min_ticker: Option<&'a str>,
        min_ticker_any_of: Option<&'a str>,
        min_ticker_gt: Option<&'a str>,
        min_ticker_gte: Option<&'a str>,
        min_ticker_lt: Option<&'a str>,
        min_ticker_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FilingForm3> {
        self.list_stocks_filings_form_3_with_params(ListStocksFilingsForm3Params {
            issuer_cik: issuer_cik.map(String::from),
            issuer_cik_any_of: issuer_cik_any_of.map(String::from),
            owner_cik: owner_cik.map(String::from),
            owner_cik_any_of: owner_cik_any_of.map(String::from),
            tickers: tickers.map(String::from),
            tickers_all_of: tickers_all_of.map(String::from),
            tickers_any_of: tickers_any_of.map(String::from),
            form_type: form_type.map(String::from),
            filing_date: filing_date.map(String::from),
            filing_date_gt: filing_date_gt.map(String::from),
            filing_date_gte: filing_date_gte.map(String::from),
            filing_date_lt: filing_date_lt.map(String::from),
            filing_date_lte: filing_date_lte.map(String::from),
            max_ticker: max_ticker.map(String::from),
            max_ticker_any_of: max_ticker_any_of.map(String::from),
            max_ticker_gt: max_ticker_gt.map(String::from),
            max_ticker_gte: max_ticker_gte.map(String::from),
            max_ticker_lt: max_ticker_lt.map(String::from),
            max_ticker_lte: max_ticker_lte.map(String::from),
            min_ticker: min_ticker.map(String::from),
            min_ticker_any_of: min_ticker_any_of.map(String::from),
            min_ticker_gt: min_ticker_gt.map(String::from),
            min_ticker_gte: min_ticker_gte.map(String::from),
            min_ticker_lt: min_ticker_lt.map(String::from),
            min_ticker_lte: min_ticker_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_stocks_filings_form_3_with_params<'a>(
        &'a self,
        params: ListStocksFilingsForm3Params,
    ) -> BoxStream<'a, FilingForm3> {
        Box::pin({
            let ListStocksFilingsForm3Params {
                issuer_cik,
                issuer_cik_any_of,
                owner_cik,
                owner_cik_any_of,
                tickers,
                tickers_all_of,
                tickers_any_of,
                form_type,
                filing_date,
                filing_date_gt,
                filing_date_gte,
                filing_date_lt,
                filing_date_lte,
                max_ticker,
                max_ticker_any_of,
                max_ticker_gt,
                max_ticker_gte,
                max_ticker_lt,
                max_ticker_lte,
                min_ticker,
                min_ticker_any_of,
                min_ticker_gt,
                min_ticker_gte,
                min_ticker_lt,
                min_ticker_lte,
                limit,
                sort,
                options,
            } = params;
            let issuer_cik = issuer_cik.as_deref();
            let issuer_cik_any_of = issuer_cik_any_of.as_deref();
            let owner_cik = owner_cik.as_deref();
            let owner_cik_any_of = owner_cik_any_of.as_deref();
            let tickers = tickers.as_deref();
            let tickers_all_of = tickers_all_of.as_deref();
            let tickers_any_of = tickers_any_of.as_deref();
            let form_type = form_type.as_deref();
            let filing_date = filing_date.as_deref();
            let filing_date_gt = filing_date_gt.as_deref();
            let filing_date_gte = filing_date_gte.as_deref();
            let filing_date_lt = filing_date_lt.as_deref();
            let filing_date_lte = filing_date_lte.as_deref();
            let max_ticker = max_ticker.as_deref();
            let max_ticker_any_of = max_ticker_any_of.as_deref();
            let max_ticker_gt = max_ticker_gt.as_deref();
            let max_ticker_gte = max_ticker_gte.as_deref();
            let max_ticker_lt = max_ticker_lt.as_deref();
            let max_ticker_lte = max_ticker_lte.as_deref();
            let min_ticker = min_ticker.as_deref();
            let min_ticker_any_of = min_ticker_any_of.as_deref();
            let min_ticker_gt = min_ticker_gt.as_deref();
            let min_ticker_gte = min_ticker_gte.as_deref();
            let min_ticker_lt = min_ticker_lt.as_deref();
            let min_ticker_lte = min_ticker_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/stocks/filings/vX/form-3".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = issuer_cik {
                query.push(("issuer_cik", v.to_string()));
            }
            if let Some(v) = issuer_cik_any_of {
                query.push(("issuer_cik.any_of", v.to_string()));
            }
            if let Some(v) = owner_cik {
                query.push(("owner_cik", v.to_string()));
            }
            if let Some(v) = owner_cik_any_of {
                query.push(("owner_cik.any_of", v.to_string()));
            }
            if let Some(v) = tickers {
                query.push(("tickers", v.to_string()));
            }
            if let Some(v) = tickers_all_of {
                query.push(("tickers.all_of", v.to_string()));
            }
            if let Some(v) = tickers_any_of {
                query.push(("tickers.any_of", v.to_string()));
            }
            if let Some(v) = form_type {
                query.push(("form_type", v.to_string()));
            }
            if let Some(v) = filing_date {
                query.push(("filing_date", v.to_string()));
            }
            if let Some(v) = filing_date_gt {
                query.push(("filing_date.gt", v.to_string()));
            }
            if let Some(v) = filing_date_gte {
                query.push(("filing_date.gte", v.to_string()));
            }
            if let Some(v) = filing_date_lt {
                query.push(("filing_date.lt", v.to_string()));
            }
            if let Some(v) = filing_date_lte {
                query.push(("filing_date.lte", v.to_string()));
            }
            if let Some(v) = max_ticker {
                query.push(("max_ticker", v.to_string()));
            }
            if let Some(v) = max_ticker_any_of {
                query.push(("max_ticker.any_of", v.to_string()));
            }
            if let Some(v) = max_ticker_gt {
                query.push(("max_ticker.gt", v.to_string()));
            }
            if let Some(v) = max_ticker_gte {
                query.push(("max_ticker.gte", v.to_string()));
            }
            if let Some(v) = max_ticker_lt {
                query.push(("max_ticker.lt", v.to_string()));
            }
            if let Some(v) = max_ticker_lte {
                query.push(("max_ticker.lte", v.to_string()));
            }
            if let Some(v) = min_ticker {
                query.push(("min_ticker", v.to_string()));
            }
            if let Some(v) = min_ticker_any_of {
                query.push(("min_ticker.any_of", v.to_string()));
            }
            if let Some(v) = min_ticker_gt {
                query.push(("min_ticker.gt", v.to_string()));
            }
            if let Some(v) = min_ticker_gte {
                query.push(("min_ticker.gte", v.to_string()));
            }
            if let Some(v) = min_ticker_lt {
                query.push(("min_ticker.lt", v.to_string()));
            }
            if let Some(v) = min_ticker_lte {
                query.push(("min_ticker.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            if self.pagination {
                self.paginate::<FilingForm3>(&path, Some(&query), options)
            } else {
                self.single_page::<FilingForm3>(&path, Some(&query), options)
            }
        })
    }

    fn list_stocks_filings_form_4<'a>(
        &'a self,
        issuer_cik: Option<&'a str>,
        issuer_cik_any_of: Option<&'a str>,
        owner_cik: Option<&'a str>,
        owner_cik_any_of: Option<&'a str>,
        tickers: Option<&'a str>,
        tickers_all_of: Option<&'a str>,
        tickers_any_of: Option<&'a str>,
        form_type: Option<&'a str>,
        transaction_code: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        max_ticker: Option<&'a str>,
        max_ticker_any_of: Option<&'a str>,
        max_ticker_gt: Option<&'a str>,
        max_ticker_gte: Option<&'a str>,
        max_ticker_lt: Option<&'a str>,
        max_ticker_lte: Option<&'a str>,
        min_ticker: Option<&'a str>,
        min_ticker_any_of: Option<&'a str>,
        min_ticker_gt: Option<&'a str>,
        min_ticker_gte: Option<&'a str>,
        min_ticker_lt: Option<&'a str>,
        min_ticker_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FilingForm4> {
        self.list_stocks_filings_form_4_with_params(ListStocksFilingsForm4Params {
            issuer_cik: issuer_cik.map(String::from),
            issuer_cik_any_of: issuer_cik_any_of.map(String::from),
            owner_cik: owner_cik.map(String::from),
            owner_cik_any_of: owner_cik_any_of.map(String::from),
            tickers: tickers.map(String::from),
            tickers_all_of: tickers_all_of.map(String::from),
            tickers_any_of: tickers_any_of.map(String::from),
            form_type: form_type.map(String::from),
            transaction_code: transaction_code.map(String::from),
            filing_date: filing_date.map(String::from),
            filing_date_gt: filing_date_gt.map(String::from),
            filing_date_gte: filing_date_gte.map(String::from),
            filing_date_lt: filing_date_lt.map(String::from),
            filing_date_lte: filing_date_lte.map(String::from),
            max_ticker: max_ticker.map(String::from),
            max_ticker_any_of: max_ticker_any_of.map(String::from),
            max_ticker_gt: max_ticker_gt.map(String::from),
            max_ticker_gte: max_ticker_gte.map(String::from),
            max_ticker_lt: max_ticker_lt.map(String::from),
            max_ticker_lte: max_ticker_lte.map(String::from),
            min_ticker: min_ticker.map(String::from),
            min_ticker_any_of: min_ticker_any_of.map(String::from),
            min_ticker_gt: min_ticker_gt.map(String::from),
            min_ticker_gte: min_ticker_gte.map(String::from),
            min_ticker_lt: min_ticker_lt.map(String::from),
            min_ticker_lte: min_ticker_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_stocks_filings_form_4_with_params<'a>(
        &'a self,
        params: ListStocksFilingsForm4Params,
    ) -> BoxStream<'a, FilingForm4> {
        Box::pin({
            let ListStocksFilingsForm4Params {
                issuer_cik,
                issuer_cik_any_of,
                owner_cik,
                owner_cik_any_of,
                tickers,
                tickers_all_of,
                tickers_any_of,
                form_type,
                transaction_code,
                filing_date,
                filing_date_gt,
                filing_date_gte,
                filing_date_lt,
                filing_date_lte,
                max_ticker,
                max_ticker_any_of,
                max_ticker_gt,
                max_ticker_gte,
                max_ticker_lt,
                max_ticker_lte,
                min_ticker,
                min_ticker_any_of,
                min_ticker_gt,
                min_ticker_gte,
                min_ticker_lt,
                min_ticker_lte,
                limit,
                sort,
                options,
            } = params;
            let issuer_cik = issuer_cik.as_deref();
            let issuer_cik_any_of = issuer_cik_any_of.as_deref();
            let owner_cik = owner_cik.as_deref();
            let owner_cik_any_of = owner_cik_any_of.as_deref();
            let tickers = tickers.as_deref();
            let tickers_all_of = tickers_all_of.as_deref();
            let tickers_any_of = tickers_any_of.as_deref();
            let form_type = form_type.as_deref();
            let transaction_code = transaction_code.as_deref();
            let filing_date = filing_date.as_deref();
            let filing_date_gt = filing_date_gt.as_deref();
            let filing_date_gte = filing_date_gte.as_deref();
            let filing_date_lt = filing_date_lt.as_deref();
            let filing_date_lte = filing_date_lte.as_deref();
            let max_ticker = max_ticker.as_deref();
            let max_ticker_any_of = max_ticker_any_of.as_deref();
            let max_ticker_gt = max_ticker_gt.as_deref();
            let max_ticker_gte = max_ticker_gte.as_deref();
            let max_ticker_lt = max_ticker_lt.as_deref();
            let max_ticker_lte = max_ticker_lte.as_deref();
            let min_ticker = min_ticker.as_deref();
            let min_ticker_any_of = min_ticker_any_of.as_deref();
            let min_ticker_gt = min_ticker_gt.as_deref();
            let min_ticker_gte = min_ticker_gte.as_deref();
            let min_ticker_lt = min_ticker_lt.as_deref();
            let min_ticker_lte = min_ticker_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/stocks/filings/vX/form-4".to_string();
            let mut query: Vec<(&str, String)> = Vec::new();
            if let Some(v) = issuer_cik {
                query.push(("issuer_cik", v.to_string()));
            }
            if let Some(v) = issuer_cik_any_of {
                query.push(("issuer_cik.any_of", v.to_string()));
            }
            if let Some(v) = owner_cik {
                query.push(("owner_cik", v.to_string()));
            }
            if let Some(v) = owner_cik_any_of {
                query.push(("owner_cik.any_of", v.to_string()));
            }
            if let Some(v) = tickers {
                query.push(("tickers", v.to_string()));
            }
            if let Some(v) = tickers_all_of {
                query.push(("tickers.all_of", v.to_string()));
            }
            if let Some(v) = tickers_any_of {
                query.push(("tickers.any_of", v.to_string()));
            }
            if let Some(v) = form_type {
                query.push(("form_type", v.to_string()));
            }
            if let Some(v) = transaction_code {
                query.push(("transaction_code", v.to_string()));
            }
            if let Some(v) = filing_date {
                query.push(("filing_date", v.to_string()));
            }
            if let Some(v) = filing_date_gt {
                query.push(("filing_date.gt", v.to_string()));
            }
            if let Some(v) = filing_date_gte {
                query.push(("filing_date.gte", v.to_string()));
            }
            if let Some(v) = filing_date_lt {
                query.push(("filing_date.lt", v.to_string()));
            }
            if let Some(v) = filing_date_lte {
                query.push(("filing_date.lte", v.to_string()));
            }
            if let Some(v) = max_ticker {
                query.push(("max_ticker", v.to_string()));
            }
            if let Some(v) = max_ticker_any_of {
                query.push(("max_ticker.any_of", v.to_string()));
            }
            if let Some(v) = max_ticker_gt {
                query.push(("max_ticker.gt", v.to_string()));
            }
            if let Some(v) = max_ticker_gte {
                query.push(("max_ticker.gte", v.to_string()));
            }
            if let Some(v) = max_ticker_lt {
                query.push(("max_ticker.lt", v.to_string()));
            }
            if let Some(v) = max_ticker_lte {
                query.push(("max_ticker.lte", v.to_string()));
            }
            if let Some(v) = min_ticker {
                query.push(("min_ticker", v.to_string()));
            }
            if let Some(v) = min_ticker_any_of {
                query.push(("min_ticker.any_of", v.to_string()));
            }
            if let Some(v) = min_ticker_gt {
                query.push(("min_ticker.gt", v.to_string()));
            }
            if let Some(v) = min_ticker_gte {
                query.push(("min_ticker.gte", v.to_string()));
            }
            if let Some(v) = min_ticker_lt {
                query.push(("min_ticker.lt", v.to_string()));
            }
            if let Some(v) = min_ticker_lte {
                query.push(("min_ticker.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            if self.pagination {
                self.paginate::<FilingForm4>(&path, Some(&query), options)
            } else {
                self.single_page::<FilingForm4>(&path, Some(&query), options)
            }
        })
    }
}

// --- Params structs (additive builder API) ---

/// Optional arguments for [`ReferenceApi::get_market_holidays`].
#[derive(Debug, Default, Clone)]
pub struct GetMarketHolidaysParams {
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetMarketHolidaysParams {
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

/// Optional arguments for [`ReferenceApi::get_market_status`].
#[derive(Debug, Default, Clone)]
pub struct GetMarketStatusParams {
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetMarketStatusParams {
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

/// Optional arguments for [`ReferenceApi::list_tickers`].
#[derive(Debug, Default, Clone)]
pub struct ListTickersParams {
    /// The `ticker` argument.
    pub ticker: Option<String>,
    /// The `ticker_lt` argument.
    pub ticker_lt: Option<String>,
    /// The `ticker_lte` argument.
    pub ticker_lte: Option<String>,
    /// The `ticker_gt` argument.
    pub ticker_gt: Option<String>,
    /// The `ticker_gte` argument.
    pub ticker_gte: Option<String>,
    /// The `type` argument.
    pub r#type: Option<String>,
    /// The `market` argument.
    pub market: Option<String>,
    /// The `exchange` argument.
    pub exchange: Option<String>,
    /// The `cusip` argument.
    pub cusip: Option<i64>,
    /// The `cik` argument.
    pub cik: Option<i64>,
    /// The `date` argument.
    pub date: Option<String>,
    /// The `active` argument.
    pub active: Option<bool>,
    /// The `search` argument.
    pub search: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `order` argument.
    pub order: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListTickersParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `ticker` argument.
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
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

    /// Set the `type` argument.
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Set the `market` argument.
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    /// Set the `exchange` argument.
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }

    /// Set the `cusip` argument.
    pub fn cusip(mut self, cusip: i64) -> Self {
        self.cusip = Some(cusip);
        self
    }

    /// Set the `cik` argument.
    pub fn cik(mut self, cik: i64) -> Self {
        self.cik = Some(cik);
        self
    }

    /// Set the `date` argument.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }

    /// Set the `active` argument.
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    /// Set the `search` argument.
    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.search = Some(search.into());
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

/// Optional arguments for [`ReferenceApi::get_ticker_details`].
#[derive(Debug, Default, Clone)]
pub struct GetTickerDetailsParams {
    /// The `date` argument.
    pub date: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetTickerDetailsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `date` argument.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`ReferenceApi::get_ticker_events`].
#[derive(Debug, Default, Clone)]
pub struct GetTickerEventsParams {
    /// The `types` argument.
    pub types: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetTickerEventsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `types` argument.
    pub fn types(mut self, types: impl Into<String>) -> Self {
        self.types = Some(types.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`ReferenceApi::list_ticker_news`].
#[derive(Debug, Default, Clone)]
pub struct ListTickerNewsParams {
    /// The `ticker` argument.
    pub ticker: Option<String>,
    /// The `ticker_lt` argument.
    pub ticker_lt: Option<String>,
    /// The `ticker_lte` argument.
    pub ticker_lte: Option<String>,
    /// The `ticker_gt` argument.
    pub ticker_gt: Option<String>,
    /// The `ticker_gte` argument.
    pub ticker_gte: Option<String>,
    /// The `published_utc` argument.
    pub published_utc: Option<String>,
    /// The `published_utc_lt` argument.
    pub published_utc_lt: Option<String>,
    /// The `published_utc_lte` argument.
    pub published_utc_lte: Option<String>,
    /// The `published_utc_gt` argument.
    pub published_utc_gt: Option<String>,
    /// The `published_utc_gte` argument.
    pub published_utc_gte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `order` argument.
    pub order: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListTickerNewsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `ticker` argument.
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
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

    /// Set the `published_utc` argument.
    pub fn published_utc(mut self, published_utc: impl Into<String>) -> Self {
        self.published_utc = Some(published_utc.into());
        self
    }

    /// Set the `published_utc_lt` argument.
    pub fn published_utc_lt(mut self, published_utc_lt: impl Into<String>) -> Self {
        self.published_utc_lt = Some(published_utc_lt.into());
        self
    }

    /// Set the `published_utc_lte` argument.
    pub fn published_utc_lte(mut self, published_utc_lte: impl Into<String>) -> Self {
        self.published_utc_lte = Some(published_utc_lte.into());
        self
    }

    /// Set the `published_utc_gt` argument.
    pub fn published_utc_gt(mut self, published_utc_gt: impl Into<String>) -> Self {
        self.published_utc_gt = Some(published_utc_gt.into());
        self
    }

    /// Set the `published_utc_gte` argument.
    pub fn published_utc_gte(mut self, published_utc_gte: impl Into<String>) -> Self {
        self.published_utc_gte = Some(published_utc_gte.into());
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

/// Optional arguments for [`ReferenceApi::get_ticker_types`].
#[derive(Debug, Default, Clone)]
pub struct GetTickerTypesParams {
    /// The `asset_class` argument.
    pub asset_class: Option<String>,
    /// The `locale` argument.
    pub locale: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetTickerTypesParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `asset_class` argument.
    pub fn asset_class(mut self, asset_class: impl Into<String>) -> Self {
        self.asset_class = Some(asset_class.into());
        self
    }

    /// Set the `locale` argument.
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`ReferenceApi::get_related_companies`].
#[derive(Debug, Default, Clone)]
pub struct GetRelatedCompaniesParams {
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetRelatedCompaniesParams {
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

/// Optional arguments for [`ReferenceApi::list_splits`].
#[derive(Debug, Default, Clone)]
pub struct ListSplitsParams {
    /// The `ticker` argument.
    pub ticker: Option<String>,
    /// The `ticker_lt` argument.
    pub ticker_lt: Option<String>,
    /// The `ticker_lte` argument.
    pub ticker_lte: Option<String>,
    /// The `ticker_gt` argument.
    pub ticker_gt: Option<String>,
    /// The `ticker_gte` argument.
    pub ticker_gte: Option<String>,
    /// The `execution_date` argument.
    pub execution_date: Option<String>,
    /// The `execution_date_lt` argument.
    pub execution_date_lt: Option<String>,
    /// The `execution_date_lte` argument.
    pub execution_date_lte: Option<String>,
    /// The `execution_date_gt` argument.
    pub execution_date_gt: Option<String>,
    /// The `execution_date_gte` argument.
    pub execution_date_gte: Option<String>,
    /// The `reverse_split` argument.
    pub reverse_split: Option<bool>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `order` argument.
    pub order: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListSplitsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `ticker` argument.
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
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

    /// Set the `execution_date` argument.
    pub fn execution_date(mut self, execution_date: impl Into<String>) -> Self {
        self.execution_date = Some(execution_date.into());
        self
    }

    /// Set the `execution_date_lt` argument.
    pub fn execution_date_lt(mut self, execution_date_lt: impl Into<String>) -> Self {
        self.execution_date_lt = Some(execution_date_lt.into());
        self
    }

    /// Set the `execution_date_lte` argument.
    pub fn execution_date_lte(mut self, execution_date_lte: impl Into<String>) -> Self {
        self.execution_date_lte = Some(execution_date_lte.into());
        self
    }

    /// Set the `execution_date_gt` argument.
    pub fn execution_date_gt(mut self, execution_date_gt: impl Into<String>) -> Self {
        self.execution_date_gt = Some(execution_date_gt.into());
        self
    }

    /// Set the `execution_date_gte` argument.
    pub fn execution_date_gte(mut self, execution_date_gte: impl Into<String>) -> Self {
        self.execution_date_gte = Some(execution_date_gte.into());
        self
    }

    /// Set the `reverse_split` argument.
    pub fn reverse_split(mut self, reverse_split: bool) -> Self {
        self.reverse_split = Some(reverse_split);
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

/// Optional arguments for [`ReferenceApi::list_dividends`].
#[derive(Debug, Default, Clone)]
pub struct ListDividendsParams {
    /// The `ticker` argument.
    pub ticker: Option<String>,
    /// The `ticker_lt` argument.
    pub ticker_lt: Option<String>,
    /// The `ticker_lte` argument.
    pub ticker_lte: Option<String>,
    /// The `ticker_gt` argument.
    pub ticker_gt: Option<String>,
    /// The `ticker_gte` argument.
    pub ticker_gte: Option<String>,
    /// The `ex_dividend_date` argument.
    pub ex_dividend_date: Option<String>,
    /// The `ex_dividend_date_lt` argument.
    pub ex_dividend_date_lt: Option<String>,
    /// The `ex_dividend_date_lte` argument.
    pub ex_dividend_date_lte: Option<String>,
    /// The `ex_dividend_date_gt` argument.
    pub ex_dividend_date_gt: Option<String>,
    /// The `ex_dividend_date_gte` argument.
    pub ex_dividend_date_gte: Option<String>,
    /// The `record_date` argument.
    pub record_date: Option<String>,
    /// The `record_date_lt` argument.
    pub record_date_lt: Option<String>,
    /// The `record_date_lte` argument.
    pub record_date_lte: Option<String>,
    /// The `record_date_gt` argument.
    pub record_date_gt: Option<String>,
    /// The `record_date_gte` argument.
    pub record_date_gte: Option<String>,
    /// The `declaration_date` argument.
    pub declaration_date: Option<String>,
    /// The `declaration_date_lt` argument.
    pub declaration_date_lt: Option<String>,
    /// The `declaration_date_lte` argument.
    pub declaration_date_lte: Option<String>,
    /// The `declaration_date_gt` argument.
    pub declaration_date_gt: Option<String>,
    /// The `declaration_date_gte` argument.
    pub declaration_date_gte: Option<String>,
    /// The `pay_date` argument.
    pub pay_date: Option<String>,
    /// The `pay_date_lt` argument.
    pub pay_date_lt: Option<String>,
    /// The `pay_date_lte` argument.
    pub pay_date_lte: Option<String>,
    /// The `pay_date_gt` argument.
    pub pay_date_gt: Option<String>,
    /// The `pay_date_gte` argument.
    pub pay_date_gte: Option<String>,
    /// The `frequency` argument.
    pub frequency: Option<i64>,
    /// The `cash_amount` argument.
    pub cash_amount: Option<f64>,
    /// The `cash_amount_lt` argument.
    pub cash_amount_lt: Option<f64>,
    /// The `cash_amount_lte` argument.
    pub cash_amount_lte: Option<f64>,
    /// The `cash_amount_gt` argument.
    pub cash_amount_gt: Option<f64>,
    /// The `cash_amount_gte` argument.
    pub cash_amount_gte: Option<f64>,
    /// The `dividend_type` argument.
    pub dividend_type: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `order` argument.
    pub order: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListDividendsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `ticker` argument.
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
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

    /// Set the `ex_dividend_date` argument.
    pub fn ex_dividend_date(mut self, ex_dividend_date: impl Into<String>) -> Self {
        self.ex_dividend_date = Some(ex_dividend_date.into());
        self
    }

    /// Set the `ex_dividend_date_lt` argument.
    pub fn ex_dividend_date_lt(mut self, ex_dividend_date_lt: impl Into<String>) -> Self {
        self.ex_dividend_date_lt = Some(ex_dividend_date_lt.into());
        self
    }

    /// Set the `ex_dividend_date_lte` argument.
    pub fn ex_dividend_date_lte(mut self, ex_dividend_date_lte: impl Into<String>) -> Self {
        self.ex_dividend_date_lte = Some(ex_dividend_date_lte.into());
        self
    }

    /// Set the `ex_dividend_date_gt` argument.
    pub fn ex_dividend_date_gt(mut self, ex_dividend_date_gt: impl Into<String>) -> Self {
        self.ex_dividend_date_gt = Some(ex_dividend_date_gt.into());
        self
    }

    /// Set the `ex_dividend_date_gte` argument.
    pub fn ex_dividend_date_gte(mut self, ex_dividend_date_gte: impl Into<String>) -> Self {
        self.ex_dividend_date_gte = Some(ex_dividend_date_gte.into());
        self
    }

    /// Set the `record_date` argument.
    pub fn record_date(mut self, record_date: impl Into<String>) -> Self {
        self.record_date = Some(record_date.into());
        self
    }

    /// Set the `record_date_lt` argument.
    pub fn record_date_lt(mut self, record_date_lt: impl Into<String>) -> Self {
        self.record_date_lt = Some(record_date_lt.into());
        self
    }

    /// Set the `record_date_lte` argument.
    pub fn record_date_lte(mut self, record_date_lte: impl Into<String>) -> Self {
        self.record_date_lte = Some(record_date_lte.into());
        self
    }

    /// Set the `record_date_gt` argument.
    pub fn record_date_gt(mut self, record_date_gt: impl Into<String>) -> Self {
        self.record_date_gt = Some(record_date_gt.into());
        self
    }

    /// Set the `record_date_gte` argument.
    pub fn record_date_gte(mut self, record_date_gte: impl Into<String>) -> Self {
        self.record_date_gte = Some(record_date_gte.into());
        self
    }

    /// Set the `declaration_date` argument.
    pub fn declaration_date(mut self, declaration_date: impl Into<String>) -> Self {
        self.declaration_date = Some(declaration_date.into());
        self
    }

    /// Set the `declaration_date_lt` argument.
    pub fn declaration_date_lt(mut self, declaration_date_lt: impl Into<String>) -> Self {
        self.declaration_date_lt = Some(declaration_date_lt.into());
        self
    }

    /// Set the `declaration_date_lte` argument.
    pub fn declaration_date_lte(mut self, declaration_date_lte: impl Into<String>) -> Self {
        self.declaration_date_lte = Some(declaration_date_lte.into());
        self
    }

    /// Set the `declaration_date_gt` argument.
    pub fn declaration_date_gt(mut self, declaration_date_gt: impl Into<String>) -> Self {
        self.declaration_date_gt = Some(declaration_date_gt.into());
        self
    }

    /// Set the `declaration_date_gte` argument.
    pub fn declaration_date_gte(mut self, declaration_date_gte: impl Into<String>) -> Self {
        self.declaration_date_gte = Some(declaration_date_gte.into());
        self
    }

    /// Set the `pay_date` argument.
    pub fn pay_date(mut self, pay_date: impl Into<String>) -> Self {
        self.pay_date = Some(pay_date.into());
        self
    }

    /// Set the `pay_date_lt` argument.
    pub fn pay_date_lt(mut self, pay_date_lt: impl Into<String>) -> Self {
        self.pay_date_lt = Some(pay_date_lt.into());
        self
    }

    /// Set the `pay_date_lte` argument.
    pub fn pay_date_lte(mut self, pay_date_lte: impl Into<String>) -> Self {
        self.pay_date_lte = Some(pay_date_lte.into());
        self
    }

    /// Set the `pay_date_gt` argument.
    pub fn pay_date_gt(mut self, pay_date_gt: impl Into<String>) -> Self {
        self.pay_date_gt = Some(pay_date_gt.into());
        self
    }

    /// Set the `pay_date_gte` argument.
    pub fn pay_date_gte(mut self, pay_date_gte: impl Into<String>) -> Self {
        self.pay_date_gte = Some(pay_date_gte.into());
        self
    }

    /// Set the `frequency` argument.
    pub fn frequency(mut self, frequency: i64) -> Self {
        self.frequency = Some(frequency);
        self
    }

    /// Set the `cash_amount` argument.
    pub fn cash_amount(mut self, cash_amount: f64) -> Self {
        self.cash_amount = Some(cash_amount);
        self
    }

    /// Set the `cash_amount_lt` argument.
    pub fn cash_amount_lt(mut self, cash_amount_lt: f64) -> Self {
        self.cash_amount_lt = Some(cash_amount_lt);
        self
    }

    /// Set the `cash_amount_lte` argument.
    pub fn cash_amount_lte(mut self, cash_amount_lte: f64) -> Self {
        self.cash_amount_lte = Some(cash_amount_lte);
        self
    }

    /// Set the `cash_amount_gt` argument.
    pub fn cash_amount_gt(mut self, cash_amount_gt: f64) -> Self {
        self.cash_amount_gt = Some(cash_amount_gt);
        self
    }

    /// Set the `cash_amount_gte` argument.
    pub fn cash_amount_gte(mut self, cash_amount_gte: f64) -> Self {
        self.cash_amount_gte = Some(cash_amount_gte);
        self
    }

    /// Set the `dividend_type` argument.
    pub fn dividend_type(mut self, dividend_type: impl Into<String>) -> Self {
        self.dividend_type = Some(dividend_type.into());
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

/// Optional arguments for [`ReferenceApi::list_conditions`].
#[derive(Debug, Default, Clone)]
pub struct ListConditionsParams {
    /// The `asset_class` argument.
    pub asset_class: Option<String>,
    /// The `data_type` argument.
    pub data_type: Option<String>,
    /// The `id` argument.
    pub id: Option<i64>,
    /// The `sip` argument.
    pub sip: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `order` argument.
    pub order: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListConditionsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `asset_class` argument.
    pub fn asset_class(mut self, asset_class: impl Into<String>) -> Self {
        self.asset_class = Some(asset_class.into());
        self
    }

    /// Set the `data_type` argument.
    pub fn data_type(mut self, data_type: impl Into<String>) -> Self {
        self.data_type = Some(data_type.into());
        self
    }

    /// Set the `id` argument.
    pub fn id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the `sip` argument.
    pub fn sip(mut self, sip: impl Into<String>) -> Self {
        self.sip = Some(sip.into());
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

/// Optional arguments for [`ReferenceApi::get_exchanges`].
#[derive(Debug, Default, Clone)]
pub struct GetExchangesParams {
    /// The `asset_class` argument.
    pub asset_class: Option<String>,
    /// The `locale` argument.
    pub locale: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetExchangesParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `asset_class` argument.
    pub fn asset_class(mut self, asset_class: impl Into<String>) -> Self {
        self.asset_class = Some(asset_class.into());
        self
    }

    /// Set the `locale` argument.
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`ReferenceApi::get_options_contract`].
#[derive(Debug, Default, Clone)]
pub struct GetOptionsContractParams {
    /// The `as_of` argument.
    pub as_of: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl GetOptionsContractParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `as_of` argument.
    pub fn as_of(mut self, as_of: impl Into<String>) -> Self {
        self.as_of = Some(as_of.into());
        self
    }

    /// Set per-request options (e.g. Launchpad edge headers).
    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// Optional arguments for [`ReferenceApi::list_options_contracts`].
#[derive(Debug, Default, Clone)]
pub struct ListOptionsContractsParams {
    /// The `underlying_ticker` argument.
    pub underlying_ticker: Option<String>,
    /// The `underlying_ticker_lt` argument.
    pub underlying_ticker_lt: Option<String>,
    /// The `underlying_ticker_lte` argument.
    pub underlying_ticker_lte: Option<String>,
    /// The `underlying_ticker_gt` argument.
    pub underlying_ticker_gt: Option<String>,
    /// The `underlying_ticker_gte` argument.
    pub underlying_ticker_gte: Option<String>,
    /// The `contract_type` argument.
    pub contract_type: Option<String>,
    /// The `expiration_date` argument.
    pub expiration_date: Option<String>,
    /// The `expiration_date_lt` argument.
    pub expiration_date_lt: Option<String>,
    /// The `expiration_date_lte` argument.
    pub expiration_date_lte: Option<String>,
    /// The `expiration_date_gt` argument.
    pub expiration_date_gt: Option<String>,
    /// The `expiration_date_gte` argument.
    pub expiration_date_gte: Option<String>,
    /// The `as_of` argument.
    pub as_of: Option<String>,
    /// The `strike_price` argument.
    pub strike_price: Option<f64>,
    /// The `strike_price_lt` argument.
    pub strike_price_lt: Option<f64>,
    /// The `strike_price_lte` argument.
    pub strike_price_lte: Option<f64>,
    /// The `strike_price_gt` argument.
    pub strike_price_gt: Option<f64>,
    /// The `strike_price_gte` argument.
    pub strike_price_gte: Option<f64>,
    /// The `expired` argument.
    pub expired: Option<bool>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `order` argument.
    pub order: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListOptionsContractsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `underlying_ticker` argument.
    pub fn underlying_ticker(mut self, underlying_ticker: impl Into<String>) -> Self {
        self.underlying_ticker = Some(underlying_ticker.into());
        self
    }

    /// Set the `underlying_ticker_lt` argument.
    pub fn underlying_ticker_lt(mut self, underlying_ticker_lt: impl Into<String>) -> Self {
        self.underlying_ticker_lt = Some(underlying_ticker_lt.into());
        self
    }

    /// Set the `underlying_ticker_lte` argument.
    pub fn underlying_ticker_lte(mut self, underlying_ticker_lte: impl Into<String>) -> Self {
        self.underlying_ticker_lte = Some(underlying_ticker_lte.into());
        self
    }

    /// Set the `underlying_ticker_gt` argument.
    pub fn underlying_ticker_gt(mut self, underlying_ticker_gt: impl Into<String>) -> Self {
        self.underlying_ticker_gt = Some(underlying_ticker_gt.into());
        self
    }

    /// Set the `underlying_ticker_gte` argument.
    pub fn underlying_ticker_gte(mut self, underlying_ticker_gte: impl Into<String>) -> Self {
        self.underlying_ticker_gte = Some(underlying_ticker_gte.into());
        self
    }

    /// Set the `contract_type` argument.
    pub fn contract_type(mut self, contract_type: impl Into<String>) -> Self {
        self.contract_type = Some(contract_type.into());
        self
    }

    /// Set the `expiration_date` argument.
    pub fn expiration_date(mut self, expiration_date: impl Into<String>) -> Self {
        self.expiration_date = Some(expiration_date.into());
        self
    }

    /// Set the `expiration_date_lt` argument.
    pub fn expiration_date_lt(mut self, expiration_date_lt: impl Into<String>) -> Self {
        self.expiration_date_lt = Some(expiration_date_lt.into());
        self
    }

    /// Set the `expiration_date_lte` argument.
    pub fn expiration_date_lte(mut self, expiration_date_lte: impl Into<String>) -> Self {
        self.expiration_date_lte = Some(expiration_date_lte.into());
        self
    }

    /// Set the `expiration_date_gt` argument.
    pub fn expiration_date_gt(mut self, expiration_date_gt: impl Into<String>) -> Self {
        self.expiration_date_gt = Some(expiration_date_gt.into());
        self
    }

    /// Set the `expiration_date_gte` argument.
    pub fn expiration_date_gte(mut self, expiration_date_gte: impl Into<String>) -> Self {
        self.expiration_date_gte = Some(expiration_date_gte.into());
        self
    }

    /// Set the `as_of` argument.
    pub fn as_of(mut self, as_of: impl Into<String>) -> Self {
        self.as_of = Some(as_of.into());
        self
    }

    /// Set the `strike_price` argument.
    pub fn strike_price(mut self, strike_price: f64) -> Self {
        self.strike_price = Some(strike_price);
        self
    }

    /// Set the `strike_price_lt` argument.
    pub fn strike_price_lt(mut self, strike_price_lt: f64) -> Self {
        self.strike_price_lt = Some(strike_price_lt);
        self
    }

    /// Set the `strike_price_lte` argument.
    pub fn strike_price_lte(mut self, strike_price_lte: f64) -> Self {
        self.strike_price_lte = Some(strike_price_lte);
        self
    }

    /// Set the `strike_price_gt` argument.
    pub fn strike_price_gt(mut self, strike_price_gt: f64) -> Self {
        self.strike_price_gt = Some(strike_price_gt);
        self
    }

    /// Set the `strike_price_gte` argument.
    pub fn strike_price_gte(mut self, strike_price_gte: f64) -> Self {
        self.strike_price_gte = Some(strike_price_gte);
        self
    }

    /// Set the `expired` argument.
    pub fn expired(mut self, expired: bool) -> Self {
        self.expired = Some(expired);
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

/// Optional arguments for [`ReferenceApi::list_short_interest`].
#[derive(Debug, Default, Clone)]
pub struct ListShortInterestParams {
    /// The `ticker` argument.
    pub ticker: Option<String>,
    /// The `days_to_cover` argument.
    pub days_to_cover: Option<String>,
    /// The `days_to_cover_lt` argument.
    pub days_to_cover_lt: Option<String>,
    /// The `days_to_cover_lte` argument.
    pub days_to_cover_lte: Option<String>,
    /// The `days_to_cover_gt` argument.
    pub days_to_cover_gt: Option<String>,
    /// The `days_to_cover_gte` argument.
    pub days_to_cover_gte: Option<String>,
    /// The `settlement_date` argument.
    pub settlement_date: Option<String>,
    /// The `settlement_date_lt` argument.
    pub settlement_date_lt: Option<String>,
    /// The `settlement_date_lte` argument.
    pub settlement_date_lte: Option<String>,
    /// The `settlement_date_gt` argument.
    pub settlement_date_gt: Option<String>,
    /// The `settlement_date_gte` argument.
    pub settlement_date_gte: Option<String>,
    /// The `avg_daily_volume` argument.
    pub avg_daily_volume: Option<String>,
    /// The `avg_daily_volume_lt` argument.
    pub avg_daily_volume_lt: Option<String>,
    /// The `avg_daily_volume_lte` argument.
    pub avg_daily_volume_lte: Option<String>,
    /// The `avg_daily_volume_gt` argument.
    pub avg_daily_volume_gt: Option<String>,
    /// The `avg_daily_volume_gte` argument.
    pub avg_daily_volume_gte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `order` argument.
    pub order: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListShortInterestParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `ticker` argument.
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    /// Set the `days_to_cover` argument.
    pub fn days_to_cover(mut self, days_to_cover: impl Into<String>) -> Self {
        self.days_to_cover = Some(days_to_cover.into());
        self
    }

    /// Set the `days_to_cover_lt` argument.
    pub fn days_to_cover_lt(mut self, days_to_cover_lt: impl Into<String>) -> Self {
        self.days_to_cover_lt = Some(days_to_cover_lt.into());
        self
    }

    /// Set the `days_to_cover_lte` argument.
    pub fn days_to_cover_lte(mut self, days_to_cover_lte: impl Into<String>) -> Self {
        self.days_to_cover_lte = Some(days_to_cover_lte.into());
        self
    }

    /// Set the `days_to_cover_gt` argument.
    pub fn days_to_cover_gt(mut self, days_to_cover_gt: impl Into<String>) -> Self {
        self.days_to_cover_gt = Some(days_to_cover_gt.into());
        self
    }

    /// Set the `days_to_cover_gte` argument.
    pub fn days_to_cover_gte(mut self, days_to_cover_gte: impl Into<String>) -> Self {
        self.days_to_cover_gte = Some(days_to_cover_gte.into());
        self
    }

    /// Set the `settlement_date` argument.
    pub fn settlement_date(mut self, settlement_date: impl Into<String>) -> Self {
        self.settlement_date = Some(settlement_date.into());
        self
    }

    /// Set the `settlement_date_lt` argument.
    pub fn settlement_date_lt(mut self, settlement_date_lt: impl Into<String>) -> Self {
        self.settlement_date_lt = Some(settlement_date_lt.into());
        self
    }

    /// Set the `settlement_date_lte` argument.
    pub fn settlement_date_lte(mut self, settlement_date_lte: impl Into<String>) -> Self {
        self.settlement_date_lte = Some(settlement_date_lte.into());
        self
    }

    /// Set the `settlement_date_gt` argument.
    pub fn settlement_date_gt(mut self, settlement_date_gt: impl Into<String>) -> Self {
        self.settlement_date_gt = Some(settlement_date_gt.into());
        self
    }

    /// Set the `settlement_date_gte` argument.
    pub fn settlement_date_gte(mut self, settlement_date_gte: impl Into<String>) -> Self {
        self.settlement_date_gte = Some(settlement_date_gte.into());
        self
    }

    /// Set the `avg_daily_volume` argument.
    pub fn avg_daily_volume(mut self, avg_daily_volume: impl Into<String>) -> Self {
        self.avg_daily_volume = Some(avg_daily_volume.into());
        self
    }

    /// Set the `avg_daily_volume_lt` argument.
    pub fn avg_daily_volume_lt(mut self, avg_daily_volume_lt: impl Into<String>) -> Self {
        self.avg_daily_volume_lt = Some(avg_daily_volume_lt.into());
        self
    }

    /// Set the `avg_daily_volume_lte` argument.
    pub fn avg_daily_volume_lte(mut self, avg_daily_volume_lte: impl Into<String>) -> Self {
        self.avg_daily_volume_lte = Some(avg_daily_volume_lte.into());
        self
    }

    /// Set the `avg_daily_volume_gt` argument.
    pub fn avg_daily_volume_gt(mut self, avg_daily_volume_gt: impl Into<String>) -> Self {
        self.avg_daily_volume_gt = Some(avg_daily_volume_gt.into());
        self
    }

    /// Set the `avg_daily_volume_gte` argument.
    pub fn avg_daily_volume_gte(mut self, avg_daily_volume_gte: impl Into<String>) -> Self {
        self.avg_daily_volume_gte = Some(avg_daily_volume_gte.into());
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

/// Optional arguments for [`ReferenceApi::list_short_volume`].
#[derive(Debug, Default, Clone)]
pub struct ListShortVolumeParams {
    /// The `ticker` argument.
    pub ticker: Option<String>,
    /// The `date` argument.
    pub date: Option<String>,
    /// The `date_lt` argument.
    pub date_lt: Option<String>,
    /// The `date_lte` argument.
    pub date_lte: Option<String>,
    /// The `date_gt` argument.
    pub date_gt: Option<String>,
    /// The `date_gte` argument.
    pub date_gte: Option<String>,
    /// The `short_volume_ratio` argument.
    pub short_volume_ratio: Option<String>,
    /// The `short_volume_ratio_lt` argument.
    pub short_volume_ratio_lt: Option<String>,
    /// The `short_volume_ratio_lte` argument.
    pub short_volume_ratio_lte: Option<String>,
    /// The `short_volume_ratio_gt` argument.
    pub short_volume_ratio_gt: Option<String>,
    /// The `short_volume_ratio_gte` argument.
    pub short_volume_ratio_gte: Option<String>,
    /// The `total_volume` argument.
    pub total_volume: Option<String>,
    /// The `total_volume_lt` argument.
    pub total_volume_lt: Option<String>,
    /// The `total_volume_lte` argument.
    pub total_volume_lte: Option<String>,
    /// The `total_volume_gt` argument.
    pub total_volume_gt: Option<String>,
    /// The `total_volume_gte` argument.
    pub total_volume_gte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `order` argument.
    pub order: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListShortVolumeParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `ticker` argument.
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    /// Set the `date` argument.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
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

    /// Set the `short_volume_ratio` argument.
    pub fn short_volume_ratio(mut self, short_volume_ratio: impl Into<String>) -> Self {
        self.short_volume_ratio = Some(short_volume_ratio.into());
        self
    }

    /// Set the `short_volume_ratio_lt` argument.
    pub fn short_volume_ratio_lt(mut self, short_volume_ratio_lt: impl Into<String>) -> Self {
        self.short_volume_ratio_lt = Some(short_volume_ratio_lt.into());
        self
    }

    /// Set the `short_volume_ratio_lte` argument.
    pub fn short_volume_ratio_lte(mut self, short_volume_ratio_lte: impl Into<String>) -> Self {
        self.short_volume_ratio_lte = Some(short_volume_ratio_lte.into());
        self
    }

    /// Set the `short_volume_ratio_gt` argument.
    pub fn short_volume_ratio_gt(mut self, short_volume_ratio_gt: impl Into<String>) -> Self {
        self.short_volume_ratio_gt = Some(short_volume_ratio_gt.into());
        self
    }

    /// Set the `short_volume_ratio_gte` argument.
    pub fn short_volume_ratio_gte(mut self, short_volume_ratio_gte: impl Into<String>) -> Self {
        self.short_volume_ratio_gte = Some(short_volume_ratio_gte.into());
        self
    }

    /// Set the `total_volume` argument.
    pub fn total_volume(mut self, total_volume: impl Into<String>) -> Self {
        self.total_volume = Some(total_volume.into());
        self
    }

    /// Set the `total_volume_lt` argument.
    pub fn total_volume_lt(mut self, total_volume_lt: impl Into<String>) -> Self {
        self.total_volume_lt = Some(total_volume_lt.into());
        self
    }

    /// Set the `total_volume_lte` argument.
    pub fn total_volume_lte(mut self, total_volume_lte: impl Into<String>) -> Self {
        self.total_volume_lte = Some(total_volume_lte.into());
        self
    }

    /// Set the `total_volume_gt` argument.
    pub fn total_volume_gt(mut self, total_volume_gt: impl Into<String>) -> Self {
        self.total_volume_gt = Some(total_volume_gt.into());
        self
    }

    /// Set the `total_volume_gte` argument.
    pub fn total_volume_gte(mut self, total_volume_gte: impl Into<String>) -> Self {
        self.total_volume_gte = Some(total_volume_gte.into());
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

/// Optional arguments for [`ReferenceApi::list_stocks_splits`].
#[derive(Debug, Default, Clone)]
pub struct ListStocksSplitsParams {
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
    /// The `execution_date` argument.
    pub execution_date: Option<String>,
    /// The `execution_date_gt` argument.
    pub execution_date_gt: Option<String>,
    /// The `execution_date_gte` argument.
    pub execution_date_gte: Option<String>,
    /// The `execution_date_lt` argument.
    pub execution_date_lt: Option<String>,
    /// The `execution_date_lte` argument.
    pub execution_date_lte: Option<String>,
    /// The `adjustment_type` argument.
    pub adjustment_type: Option<String>,
    /// The `adjustment_type_any_of` argument.
    pub adjustment_type_any_of: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListStocksSplitsParams {
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

    /// Set the `execution_date` argument.
    pub fn execution_date(mut self, execution_date: impl Into<String>) -> Self {
        self.execution_date = Some(execution_date.into());
        self
    }

    /// Set the `execution_date_gt` argument.
    pub fn execution_date_gt(mut self, execution_date_gt: impl Into<String>) -> Self {
        self.execution_date_gt = Some(execution_date_gt.into());
        self
    }

    /// Set the `execution_date_gte` argument.
    pub fn execution_date_gte(mut self, execution_date_gte: impl Into<String>) -> Self {
        self.execution_date_gte = Some(execution_date_gte.into());
        self
    }

    /// Set the `execution_date_lt` argument.
    pub fn execution_date_lt(mut self, execution_date_lt: impl Into<String>) -> Self {
        self.execution_date_lt = Some(execution_date_lt.into());
        self
    }

    /// Set the `execution_date_lte` argument.
    pub fn execution_date_lte(mut self, execution_date_lte: impl Into<String>) -> Self {
        self.execution_date_lte = Some(execution_date_lte.into());
        self
    }

    /// Set the `adjustment_type` argument.
    pub fn adjustment_type(mut self, adjustment_type: impl Into<String>) -> Self {
        self.adjustment_type = Some(adjustment_type.into());
        self
    }

    /// Set the `adjustment_type_any_of` argument.
    pub fn adjustment_type_any_of(mut self, adjustment_type_any_of: impl Into<String>) -> Self {
        self.adjustment_type_any_of = Some(adjustment_type_any_of.into());
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

/// Optional arguments for [`ReferenceApi::list_stocks_dividends`].
#[derive(Debug, Default, Clone)]
pub struct ListStocksDividendsParams {
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
    /// The `ex_dividend_date` argument.
    pub ex_dividend_date: Option<String>,
    /// The `ex_dividend_date_gt` argument.
    pub ex_dividend_date_gt: Option<String>,
    /// The `ex_dividend_date_gte` argument.
    pub ex_dividend_date_gte: Option<String>,
    /// The `ex_dividend_date_lt` argument.
    pub ex_dividend_date_lt: Option<String>,
    /// The `ex_dividend_date_lte` argument.
    pub ex_dividend_date_lte: Option<String>,
    /// The `frequency` argument.
    pub frequency: Option<i64>,
    /// The `frequency_gt` argument.
    pub frequency_gt: Option<i64>,
    /// The `frequency_gte` argument.
    pub frequency_gte: Option<i64>,
    /// The `frequency_lt` argument.
    pub frequency_lt: Option<i64>,
    /// The `frequency_lte` argument.
    pub frequency_lte: Option<i64>,
    /// The `distribution_type` argument.
    pub distribution_type: Option<String>,
    /// The `distribution_type_any_of` argument.
    pub distribution_type_any_of: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListStocksDividendsParams {
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

    /// Set the `ex_dividend_date` argument.
    pub fn ex_dividend_date(mut self, ex_dividend_date: impl Into<String>) -> Self {
        self.ex_dividend_date = Some(ex_dividend_date.into());
        self
    }

    /// Set the `ex_dividend_date_gt` argument.
    pub fn ex_dividend_date_gt(mut self, ex_dividend_date_gt: impl Into<String>) -> Self {
        self.ex_dividend_date_gt = Some(ex_dividend_date_gt.into());
        self
    }

    /// Set the `ex_dividend_date_gte` argument.
    pub fn ex_dividend_date_gte(mut self, ex_dividend_date_gte: impl Into<String>) -> Self {
        self.ex_dividend_date_gte = Some(ex_dividend_date_gte.into());
        self
    }

    /// Set the `ex_dividend_date_lt` argument.
    pub fn ex_dividend_date_lt(mut self, ex_dividend_date_lt: impl Into<String>) -> Self {
        self.ex_dividend_date_lt = Some(ex_dividend_date_lt.into());
        self
    }

    /// Set the `ex_dividend_date_lte` argument.
    pub fn ex_dividend_date_lte(mut self, ex_dividend_date_lte: impl Into<String>) -> Self {
        self.ex_dividend_date_lte = Some(ex_dividend_date_lte.into());
        self
    }

    /// Set the `frequency` argument.
    pub fn frequency(mut self, frequency: i64) -> Self {
        self.frequency = Some(frequency);
        self
    }

    /// Set the `frequency_gt` argument.
    pub fn frequency_gt(mut self, frequency_gt: i64) -> Self {
        self.frequency_gt = Some(frequency_gt);
        self
    }

    /// Set the `frequency_gte` argument.
    pub fn frequency_gte(mut self, frequency_gte: i64) -> Self {
        self.frequency_gte = Some(frequency_gte);
        self
    }

    /// Set the `frequency_lt` argument.
    pub fn frequency_lt(mut self, frequency_lt: i64) -> Self {
        self.frequency_lt = Some(frequency_lt);
        self
    }

    /// Set the `frequency_lte` argument.
    pub fn frequency_lte(mut self, frequency_lte: i64) -> Self {
        self.frequency_lte = Some(frequency_lte);
        self
    }

    /// Set the `distribution_type` argument.
    pub fn distribution_type(mut self, distribution_type: impl Into<String>) -> Self {
        self.distribution_type = Some(distribution_type.into());
        self
    }

    /// Set the `distribution_type_any_of` argument.
    pub fn distribution_type_any_of(mut self, distribution_type_any_of: impl Into<String>) -> Self {
        self.distribution_type_any_of = Some(distribution_type_any_of.into());
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

/// Optional arguments for [`ReferenceApi::list_stocks_filings_risk_factors`].
#[derive(Debug, Default, Clone)]
pub struct ListStocksFilingsRiskFactorsParams {
    /// The `filing_date` argument.
    pub filing_date: Option<String>,
    /// The `filing_date_any_of` argument.
    pub filing_date_any_of: Option<String>,
    /// The `filing_date_gt` argument.
    pub filing_date_gt: Option<String>,
    /// The `filing_date_gte` argument.
    pub filing_date_gte: Option<String>,
    /// The `filing_date_lt` argument.
    pub filing_date_lt: Option<String>,
    /// The `filing_date_lte` argument.
    pub filing_date_lte: Option<String>,
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
    /// The `cik` argument.
    pub cik: Option<String>,
    /// The `cik_any_of` argument.
    pub cik_any_of: Option<String>,
    /// The `cik_gt` argument.
    pub cik_gt: Option<String>,
    /// The `cik_gte` argument.
    pub cik_gte: Option<String>,
    /// The `cik_lt` argument.
    pub cik_lt: Option<String>,
    /// The `cik_lte` argument.
    pub cik_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListStocksFilingsRiskFactorsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `filing_date` argument.
    pub fn filing_date(mut self, filing_date: impl Into<String>) -> Self {
        self.filing_date = Some(filing_date.into());
        self
    }

    /// Set the `filing_date_any_of` argument.
    pub fn filing_date_any_of(mut self, filing_date_any_of: impl Into<String>) -> Self {
        self.filing_date_any_of = Some(filing_date_any_of.into());
        self
    }

    /// Set the `filing_date_gt` argument.
    pub fn filing_date_gt(mut self, filing_date_gt: impl Into<String>) -> Self {
        self.filing_date_gt = Some(filing_date_gt.into());
        self
    }

    /// Set the `filing_date_gte` argument.
    pub fn filing_date_gte(mut self, filing_date_gte: impl Into<String>) -> Self {
        self.filing_date_gte = Some(filing_date_gte.into());
        self
    }

    /// Set the `filing_date_lt` argument.
    pub fn filing_date_lt(mut self, filing_date_lt: impl Into<String>) -> Self {
        self.filing_date_lt = Some(filing_date_lt.into());
        self
    }

    /// Set the `filing_date_lte` argument.
    pub fn filing_date_lte(mut self, filing_date_lte: impl Into<String>) -> Self {
        self.filing_date_lte = Some(filing_date_lte.into());
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

    /// Set the `cik` argument.
    pub fn cik(mut self, cik: impl Into<String>) -> Self {
        self.cik = Some(cik.into());
        self
    }

    /// Set the `cik_any_of` argument.
    pub fn cik_any_of(mut self, cik_any_of: impl Into<String>) -> Self {
        self.cik_any_of = Some(cik_any_of.into());
        self
    }

    /// Set the `cik_gt` argument.
    pub fn cik_gt(mut self, cik_gt: impl Into<String>) -> Self {
        self.cik_gt = Some(cik_gt.into());
        self
    }

    /// Set the `cik_gte` argument.
    pub fn cik_gte(mut self, cik_gte: impl Into<String>) -> Self {
        self.cik_gte = Some(cik_gte.into());
        self
    }

    /// Set the `cik_lt` argument.
    pub fn cik_lt(mut self, cik_lt: impl Into<String>) -> Self {
        self.cik_lt = Some(cik_lt.into());
        self
    }

    /// Set the `cik_lte` argument.
    pub fn cik_lte(mut self, cik_lte: impl Into<String>) -> Self {
        self.cik_lte = Some(cik_lte.into());
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

/// Optional arguments for [`ReferenceApi::list_stocks_taxonomies_risk_factors`].
#[derive(Debug, Default, Clone)]
pub struct ListStocksTaxonomiesRiskFactorsParams {
    /// The `taxonomy` argument.
    pub taxonomy: Option<f64>,
    /// The `taxonomy_gt` argument.
    pub taxonomy_gt: Option<f64>,
    /// The `taxonomy_gte` argument.
    pub taxonomy_gte: Option<f64>,
    /// The `taxonomy_lt` argument.
    pub taxonomy_lt: Option<f64>,
    /// The `taxonomy_lte` argument.
    pub taxonomy_lte: Option<f64>,
    /// The `primary_category` argument.
    pub primary_category: Option<String>,
    /// The `primary_category_any_of` argument.
    pub primary_category_any_of: Option<String>,
    /// The `primary_category_gt` argument.
    pub primary_category_gt: Option<String>,
    /// The `primary_category_gte` argument.
    pub primary_category_gte: Option<String>,
    /// The `primary_category_lt` argument.
    pub primary_category_lt: Option<String>,
    /// The `primary_category_lte` argument.
    pub primary_category_lte: Option<String>,
    /// The `secondary_category` argument.
    pub secondary_category: Option<String>,
    /// The `secondary_category_any_of` argument.
    pub secondary_category_any_of: Option<String>,
    /// The `secondary_category_gt` argument.
    pub secondary_category_gt: Option<String>,
    /// The `secondary_category_gte` argument.
    pub secondary_category_gte: Option<String>,
    /// The `secondary_category_lt` argument.
    pub secondary_category_lt: Option<String>,
    /// The `secondary_category_lte` argument.
    pub secondary_category_lte: Option<String>,
    /// The `tertiary_category` argument.
    pub tertiary_category: Option<String>,
    /// The `tertiary_category_any_of` argument.
    pub tertiary_category_any_of: Option<String>,
    /// The `tertiary_category_gt` argument.
    pub tertiary_category_gt: Option<String>,
    /// The `tertiary_category_gte` argument.
    pub tertiary_category_gte: Option<String>,
    /// The `tertiary_category_lt` argument.
    pub tertiary_category_lt: Option<String>,
    /// The `tertiary_category_lte` argument.
    pub tertiary_category_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListStocksTaxonomiesRiskFactorsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `taxonomy` argument.
    pub fn taxonomy(mut self, taxonomy: f64) -> Self {
        self.taxonomy = Some(taxonomy);
        self
    }

    /// Set the `taxonomy_gt` argument.
    pub fn taxonomy_gt(mut self, taxonomy_gt: f64) -> Self {
        self.taxonomy_gt = Some(taxonomy_gt);
        self
    }

    /// Set the `taxonomy_gte` argument.
    pub fn taxonomy_gte(mut self, taxonomy_gte: f64) -> Self {
        self.taxonomy_gte = Some(taxonomy_gte);
        self
    }

    /// Set the `taxonomy_lt` argument.
    pub fn taxonomy_lt(mut self, taxonomy_lt: f64) -> Self {
        self.taxonomy_lt = Some(taxonomy_lt);
        self
    }

    /// Set the `taxonomy_lte` argument.
    pub fn taxonomy_lte(mut self, taxonomy_lte: f64) -> Self {
        self.taxonomy_lte = Some(taxonomy_lte);
        self
    }

    /// Set the `primary_category` argument.
    pub fn primary_category(mut self, primary_category: impl Into<String>) -> Self {
        self.primary_category = Some(primary_category.into());
        self
    }

    /// Set the `primary_category_any_of` argument.
    pub fn primary_category_any_of(mut self, primary_category_any_of: impl Into<String>) -> Self {
        self.primary_category_any_of = Some(primary_category_any_of.into());
        self
    }

    /// Set the `primary_category_gt` argument.
    pub fn primary_category_gt(mut self, primary_category_gt: impl Into<String>) -> Self {
        self.primary_category_gt = Some(primary_category_gt.into());
        self
    }

    /// Set the `primary_category_gte` argument.
    pub fn primary_category_gte(mut self, primary_category_gte: impl Into<String>) -> Self {
        self.primary_category_gte = Some(primary_category_gte.into());
        self
    }

    /// Set the `primary_category_lt` argument.
    pub fn primary_category_lt(mut self, primary_category_lt: impl Into<String>) -> Self {
        self.primary_category_lt = Some(primary_category_lt.into());
        self
    }

    /// Set the `primary_category_lte` argument.
    pub fn primary_category_lte(mut self, primary_category_lte: impl Into<String>) -> Self {
        self.primary_category_lte = Some(primary_category_lte.into());
        self
    }

    /// Set the `secondary_category` argument.
    pub fn secondary_category(mut self, secondary_category: impl Into<String>) -> Self {
        self.secondary_category = Some(secondary_category.into());
        self
    }

    /// Set the `secondary_category_any_of` argument.
    pub fn secondary_category_any_of(
        mut self,
        secondary_category_any_of: impl Into<String>,
    ) -> Self {
        self.secondary_category_any_of = Some(secondary_category_any_of.into());
        self
    }

    /// Set the `secondary_category_gt` argument.
    pub fn secondary_category_gt(mut self, secondary_category_gt: impl Into<String>) -> Self {
        self.secondary_category_gt = Some(secondary_category_gt.into());
        self
    }

    /// Set the `secondary_category_gte` argument.
    pub fn secondary_category_gte(mut self, secondary_category_gte: impl Into<String>) -> Self {
        self.secondary_category_gte = Some(secondary_category_gte.into());
        self
    }

    /// Set the `secondary_category_lt` argument.
    pub fn secondary_category_lt(mut self, secondary_category_lt: impl Into<String>) -> Self {
        self.secondary_category_lt = Some(secondary_category_lt.into());
        self
    }

    /// Set the `secondary_category_lte` argument.
    pub fn secondary_category_lte(mut self, secondary_category_lte: impl Into<String>) -> Self {
        self.secondary_category_lte = Some(secondary_category_lte.into());
        self
    }

    /// Set the `tertiary_category` argument.
    pub fn tertiary_category(mut self, tertiary_category: impl Into<String>) -> Self {
        self.tertiary_category = Some(tertiary_category.into());
        self
    }

    /// Set the `tertiary_category_any_of` argument.
    pub fn tertiary_category_any_of(mut self, tertiary_category_any_of: impl Into<String>) -> Self {
        self.tertiary_category_any_of = Some(tertiary_category_any_of.into());
        self
    }

    /// Set the `tertiary_category_gt` argument.
    pub fn tertiary_category_gt(mut self, tertiary_category_gt: impl Into<String>) -> Self {
        self.tertiary_category_gt = Some(tertiary_category_gt.into());
        self
    }

    /// Set the `tertiary_category_gte` argument.
    pub fn tertiary_category_gte(mut self, tertiary_category_gte: impl Into<String>) -> Self {
        self.tertiary_category_gte = Some(tertiary_category_gte.into());
        self
    }

    /// Set the `tertiary_category_lt` argument.
    pub fn tertiary_category_lt(mut self, tertiary_category_lt: impl Into<String>) -> Self {
        self.tertiary_category_lt = Some(tertiary_category_lt.into());
        self
    }

    /// Set the `tertiary_category_lte` argument.
    pub fn tertiary_category_lte(mut self, tertiary_category_lte: impl Into<String>) -> Self {
        self.tertiary_category_lte = Some(tertiary_category_lte.into());
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

/// Optional arguments for [`ReferenceApi::list_stocks_filings_8k_disclosures`].
#[derive(Debug, Default, Clone)]
pub struct ListStocksFilings8kDisclosuresParams {
    /// The `cik` argument.
    pub cik: Option<String>,
    /// The `cik_any_of` argument.
    pub cik_any_of: Option<String>,
    /// The `tickers` argument.
    pub tickers: Option<String>,
    /// The `tickers_all_of` argument.
    pub tickers_all_of: Option<String>,
    /// The `tickers_any_of` argument.
    pub tickers_any_of: Option<String>,
    /// The `filing_date` argument.
    pub filing_date: Option<String>,
    /// The `filing_date_any_of` argument.
    pub filing_date_any_of: Option<String>,
    /// The `filing_date_gt` argument.
    pub filing_date_gt: Option<String>,
    /// The `filing_date_gte` argument.
    pub filing_date_gte: Option<String>,
    /// The `filing_date_lt` argument.
    pub filing_date_lt: Option<String>,
    /// The `filing_date_lte` argument.
    pub filing_date_lte: Option<String>,
    /// The `tertiary_category` argument.
    pub tertiary_category: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListStocksFilings8kDisclosuresParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `cik` argument.
    pub fn cik(mut self, cik: impl Into<String>) -> Self {
        self.cik = Some(cik.into());
        self
    }

    /// Set the `cik_any_of` argument.
    pub fn cik_any_of(mut self, cik_any_of: impl Into<String>) -> Self {
        self.cik_any_of = Some(cik_any_of.into());
        self
    }

    /// Set the `tickers` argument.
    pub fn tickers(mut self, tickers: impl Into<String>) -> Self {
        self.tickers = Some(tickers.into());
        self
    }

    /// Set the `tickers_all_of` argument.
    pub fn tickers_all_of(mut self, tickers_all_of: impl Into<String>) -> Self {
        self.tickers_all_of = Some(tickers_all_of.into());
        self
    }

    /// Set the `tickers_any_of` argument.
    pub fn tickers_any_of(mut self, tickers_any_of: impl Into<String>) -> Self {
        self.tickers_any_of = Some(tickers_any_of.into());
        self
    }

    /// Set the `filing_date` argument.
    pub fn filing_date(mut self, filing_date: impl Into<String>) -> Self {
        self.filing_date = Some(filing_date.into());
        self
    }

    /// Set the `filing_date_any_of` argument.
    pub fn filing_date_any_of(mut self, filing_date_any_of: impl Into<String>) -> Self {
        self.filing_date_any_of = Some(filing_date_any_of.into());
        self
    }

    /// Set the `filing_date_gt` argument.
    pub fn filing_date_gt(mut self, filing_date_gt: impl Into<String>) -> Self {
        self.filing_date_gt = Some(filing_date_gt.into());
        self
    }

    /// Set the `filing_date_gte` argument.
    pub fn filing_date_gte(mut self, filing_date_gte: impl Into<String>) -> Self {
        self.filing_date_gte = Some(filing_date_gte.into());
        self
    }

    /// Set the `filing_date_lt` argument.
    pub fn filing_date_lt(mut self, filing_date_lt: impl Into<String>) -> Self {
        self.filing_date_lt = Some(filing_date_lt.into());
        self
    }

    /// Set the `filing_date_lte` argument.
    pub fn filing_date_lte(mut self, filing_date_lte: impl Into<String>) -> Self {
        self.filing_date_lte = Some(filing_date_lte.into());
        self
    }

    /// Set the `tertiary_category` argument.
    pub fn tertiary_category(mut self, tertiary_category: impl Into<String>) -> Self {
        self.tertiary_category = Some(tertiary_category.into());
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

/// Optional arguments for [`ReferenceApi::list_stocks_taxonomies_disclosures`].
#[derive(Debug, Default, Clone)]
pub struct ListStocksTaxonomiesDisclosuresParams {
    /// The `taxonomy` argument.
    pub taxonomy: Option<String>,
    /// The `taxonomy_any_of` argument.
    pub taxonomy_any_of: Option<String>,
    /// The `taxonomy_gt` argument.
    pub taxonomy_gt: Option<String>,
    /// The `taxonomy_gte` argument.
    pub taxonomy_gte: Option<String>,
    /// The `taxonomy_lt` argument.
    pub taxonomy_lt: Option<String>,
    /// The `taxonomy_lte` argument.
    pub taxonomy_lte: Option<String>,
    /// The `primary_category` argument.
    pub primary_category: Option<String>,
    /// The `primary_category_any_of` argument.
    pub primary_category_any_of: Option<String>,
    /// The `primary_category_gt` argument.
    pub primary_category_gt: Option<String>,
    /// The `primary_category_gte` argument.
    pub primary_category_gte: Option<String>,
    /// The `primary_category_lt` argument.
    pub primary_category_lt: Option<String>,
    /// The `primary_category_lte` argument.
    pub primary_category_lte: Option<String>,
    /// The `secondary_category` argument.
    pub secondary_category: Option<String>,
    /// The `secondary_category_any_of` argument.
    pub secondary_category_any_of: Option<String>,
    /// The `secondary_category_gt` argument.
    pub secondary_category_gt: Option<String>,
    /// The `secondary_category_gte` argument.
    pub secondary_category_gte: Option<String>,
    /// The `secondary_category_lt` argument.
    pub secondary_category_lt: Option<String>,
    /// The `secondary_category_lte` argument.
    pub secondary_category_lte: Option<String>,
    /// The `tertiary_category` argument.
    pub tertiary_category: Option<String>,
    /// The `tertiary_category_any_of` argument.
    pub tertiary_category_any_of: Option<String>,
    /// The `tertiary_category_gt` argument.
    pub tertiary_category_gt: Option<String>,
    /// The `tertiary_category_gte` argument.
    pub tertiary_category_gte: Option<String>,
    /// The `tertiary_category_lt` argument.
    pub tertiary_category_lt: Option<String>,
    /// The `tertiary_category_lte` argument.
    pub tertiary_category_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListStocksTaxonomiesDisclosuresParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `taxonomy` argument.
    pub fn taxonomy(mut self, taxonomy: impl Into<String>) -> Self {
        self.taxonomy = Some(taxonomy.into());
        self
    }

    /// Set the `taxonomy_any_of` argument.
    pub fn taxonomy_any_of(mut self, taxonomy_any_of: impl Into<String>) -> Self {
        self.taxonomy_any_of = Some(taxonomy_any_of.into());
        self
    }

    /// Set the `taxonomy_gt` argument.
    pub fn taxonomy_gt(mut self, taxonomy_gt: impl Into<String>) -> Self {
        self.taxonomy_gt = Some(taxonomy_gt.into());
        self
    }

    /// Set the `taxonomy_gte` argument.
    pub fn taxonomy_gte(mut self, taxonomy_gte: impl Into<String>) -> Self {
        self.taxonomy_gte = Some(taxonomy_gte.into());
        self
    }

    /// Set the `taxonomy_lt` argument.
    pub fn taxonomy_lt(mut self, taxonomy_lt: impl Into<String>) -> Self {
        self.taxonomy_lt = Some(taxonomy_lt.into());
        self
    }

    /// Set the `taxonomy_lte` argument.
    pub fn taxonomy_lte(mut self, taxonomy_lte: impl Into<String>) -> Self {
        self.taxonomy_lte = Some(taxonomy_lte.into());
        self
    }

    /// Set the `primary_category` argument.
    pub fn primary_category(mut self, primary_category: impl Into<String>) -> Self {
        self.primary_category = Some(primary_category.into());
        self
    }

    /// Set the `primary_category_any_of` argument.
    pub fn primary_category_any_of(mut self, primary_category_any_of: impl Into<String>) -> Self {
        self.primary_category_any_of = Some(primary_category_any_of.into());
        self
    }

    /// Set the `primary_category_gt` argument.
    pub fn primary_category_gt(mut self, primary_category_gt: impl Into<String>) -> Self {
        self.primary_category_gt = Some(primary_category_gt.into());
        self
    }

    /// Set the `primary_category_gte` argument.
    pub fn primary_category_gte(mut self, primary_category_gte: impl Into<String>) -> Self {
        self.primary_category_gte = Some(primary_category_gte.into());
        self
    }

    /// Set the `primary_category_lt` argument.
    pub fn primary_category_lt(mut self, primary_category_lt: impl Into<String>) -> Self {
        self.primary_category_lt = Some(primary_category_lt.into());
        self
    }

    /// Set the `primary_category_lte` argument.
    pub fn primary_category_lte(mut self, primary_category_lte: impl Into<String>) -> Self {
        self.primary_category_lte = Some(primary_category_lte.into());
        self
    }

    /// Set the `secondary_category` argument.
    pub fn secondary_category(mut self, secondary_category: impl Into<String>) -> Self {
        self.secondary_category = Some(secondary_category.into());
        self
    }

    /// Set the `secondary_category_any_of` argument.
    pub fn secondary_category_any_of(
        mut self,
        secondary_category_any_of: impl Into<String>,
    ) -> Self {
        self.secondary_category_any_of = Some(secondary_category_any_of.into());
        self
    }

    /// Set the `secondary_category_gt` argument.
    pub fn secondary_category_gt(mut self, secondary_category_gt: impl Into<String>) -> Self {
        self.secondary_category_gt = Some(secondary_category_gt.into());
        self
    }

    /// Set the `secondary_category_gte` argument.
    pub fn secondary_category_gte(mut self, secondary_category_gte: impl Into<String>) -> Self {
        self.secondary_category_gte = Some(secondary_category_gte.into());
        self
    }

    /// Set the `secondary_category_lt` argument.
    pub fn secondary_category_lt(mut self, secondary_category_lt: impl Into<String>) -> Self {
        self.secondary_category_lt = Some(secondary_category_lt.into());
        self
    }

    /// Set the `secondary_category_lte` argument.
    pub fn secondary_category_lte(mut self, secondary_category_lte: impl Into<String>) -> Self {
        self.secondary_category_lte = Some(secondary_category_lte.into());
        self
    }

    /// Set the `tertiary_category` argument.
    pub fn tertiary_category(mut self, tertiary_category: impl Into<String>) -> Self {
        self.tertiary_category = Some(tertiary_category.into());
        self
    }

    /// Set the `tertiary_category_any_of` argument.
    pub fn tertiary_category_any_of(mut self, tertiary_category_any_of: impl Into<String>) -> Self {
        self.tertiary_category_any_of = Some(tertiary_category_any_of.into());
        self
    }

    /// Set the `tertiary_category_gt` argument.
    pub fn tertiary_category_gt(mut self, tertiary_category_gt: impl Into<String>) -> Self {
        self.tertiary_category_gt = Some(tertiary_category_gt.into());
        self
    }

    /// Set the `tertiary_category_gte` argument.
    pub fn tertiary_category_gte(mut self, tertiary_category_gte: impl Into<String>) -> Self {
        self.tertiary_category_gte = Some(tertiary_category_gte.into());
        self
    }

    /// Set the `tertiary_category_lt` argument.
    pub fn tertiary_category_lt(mut self, tertiary_category_lt: impl Into<String>) -> Self {
        self.tertiary_category_lt = Some(tertiary_category_lt.into());
        self
    }

    /// Set the `tertiary_category_lte` argument.
    pub fn tertiary_category_lte(mut self, tertiary_category_lte: impl Into<String>) -> Self {
        self.tertiary_category_lte = Some(tertiary_category_lte.into());
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

/// Optional arguments for [`ReferenceApi::list_stocks_filings_10k_sections`].
#[derive(Debug, Default, Clone)]
pub struct ListStocksFilings10kSectionsParams {
    /// The `cik` argument.
    pub cik: Option<String>,
    /// The `cik_any_of` argument.
    pub cik_any_of: Option<String>,
    /// The `cik_gt` argument.
    pub cik_gt: Option<String>,
    /// The `cik_gte` argument.
    pub cik_gte: Option<String>,
    /// The `cik_lt` argument.
    pub cik_lt: Option<String>,
    /// The `cik_lte` argument.
    pub cik_lte: Option<String>,
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
    /// The `section` argument.
    pub section: Option<String>,
    /// The `section_any_of` argument.
    pub section_any_of: Option<String>,
    /// The `filing_date` argument.
    pub filing_date: Option<String>,
    /// The `filing_date_gt` argument.
    pub filing_date_gt: Option<String>,
    /// The `filing_date_gte` argument.
    pub filing_date_gte: Option<String>,
    /// The `filing_date_lt` argument.
    pub filing_date_lt: Option<String>,
    /// The `filing_date_lte` argument.
    pub filing_date_lte: Option<String>,
    /// The `period_end` argument.
    pub period_end: Option<String>,
    /// The `period_end_gt` argument.
    pub period_end_gt: Option<String>,
    /// The `period_end_gte` argument.
    pub period_end_gte: Option<String>,
    /// The `period_end_lt` argument.
    pub period_end_lt: Option<String>,
    /// The `period_end_lte` argument.
    pub period_end_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListStocksFilings10kSectionsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `cik` argument.
    pub fn cik(mut self, cik: impl Into<String>) -> Self {
        self.cik = Some(cik.into());
        self
    }

    /// Set the `cik_any_of` argument.
    pub fn cik_any_of(mut self, cik_any_of: impl Into<String>) -> Self {
        self.cik_any_of = Some(cik_any_of.into());
        self
    }

    /// Set the `cik_gt` argument.
    pub fn cik_gt(mut self, cik_gt: impl Into<String>) -> Self {
        self.cik_gt = Some(cik_gt.into());
        self
    }

    /// Set the `cik_gte` argument.
    pub fn cik_gte(mut self, cik_gte: impl Into<String>) -> Self {
        self.cik_gte = Some(cik_gte.into());
        self
    }

    /// Set the `cik_lt` argument.
    pub fn cik_lt(mut self, cik_lt: impl Into<String>) -> Self {
        self.cik_lt = Some(cik_lt.into());
        self
    }

    /// Set the `cik_lte` argument.
    pub fn cik_lte(mut self, cik_lte: impl Into<String>) -> Self {
        self.cik_lte = Some(cik_lte.into());
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

    /// Set the `section` argument.
    pub fn section(mut self, section: impl Into<String>) -> Self {
        self.section = Some(section.into());
        self
    }

    /// Set the `section_any_of` argument.
    pub fn section_any_of(mut self, section_any_of: impl Into<String>) -> Self {
        self.section_any_of = Some(section_any_of.into());
        self
    }

    /// Set the `filing_date` argument.
    pub fn filing_date(mut self, filing_date: impl Into<String>) -> Self {
        self.filing_date = Some(filing_date.into());
        self
    }

    /// Set the `filing_date_gt` argument.
    pub fn filing_date_gt(mut self, filing_date_gt: impl Into<String>) -> Self {
        self.filing_date_gt = Some(filing_date_gt.into());
        self
    }

    /// Set the `filing_date_gte` argument.
    pub fn filing_date_gte(mut self, filing_date_gte: impl Into<String>) -> Self {
        self.filing_date_gte = Some(filing_date_gte.into());
        self
    }

    /// Set the `filing_date_lt` argument.
    pub fn filing_date_lt(mut self, filing_date_lt: impl Into<String>) -> Self {
        self.filing_date_lt = Some(filing_date_lt.into());
        self
    }

    /// Set the `filing_date_lte` argument.
    pub fn filing_date_lte(mut self, filing_date_lte: impl Into<String>) -> Self {
        self.filing_date_lte = Some(filing_date_lte.into());
        self
    }

    /// Set the `period_end` argument.
    pub fn period_end(mut self, period_end: impl Into<String>) -> Self {
        self.period_end = Some(period_end.into());
        self
    }

    /// Set the `period_end_gt` argument.
    pub fn period_end_gt(mut self, period_end_gt: impl Into<String>) -> Self {
        self.period_end_gt = Some(period_end_gt.into());
        self
    }

    /// Set the `period_end_gte` argument.
    pub fn period_end_gte(mut self, period_end_gte: impl Into<String>) -> Self {
        self.period_end_gte = Some(period_end_gte.into());
        self
    }

    /// Set the `period_end_lt` argument.
    pub fn period_end_lt(mut self, period_end_lt: impl Into<String>) -> Self {
        self.period_end_lt = Some(period_end_lt.into());
        self
    }

    /// Set the `period_end_lte` argument.
    pub fn period_end_lte(mut self, period_end_lte: impl Into<String>) -> Self {
        self.period_end_lte = Some(period_end_lte.into());
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

/// Optional arguments for [`ReferenceApi::list_stocks_filings_8k_text`].
#[derive(Debug, Default, Clone)]
pub struct ListStocksFilings8kTextParams {
    /// The `cik` argument.
    pub cik: Option<String>,
    /// The `cik_any_of` argument.
    pub cik_any_of: Option<String>,
    /// The `cik_gt` argument.
    pub cik_gt: Option<String>,
    /// The `cik_gte` argument.
    pub cik_gte: Option<String>,
    /// The `cik_lt` argument.
    pub cik_lt: Option<String>,
    /// The `cik_lte` argument.
    pub cik_lte: Option<String>,
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
    /// The `form_type` argument.
    pub form_type: Option<String>,
    /// The `form_type_any_of` argument.
    pub form_type_any_of: Option<String>,
    /// The `form_type_gt` argument.
    pub form_type_gt: Option<String>,
    /// The `form_type_gte` argument.
    pub form_type_gte: Option<String>,
    /// The `form_type_lt` argument.
    pub form_type_lt: Option<String>,
    /// The `form_type_lte` argument.
    pub form_type_lte: Option<String>,
    /// The `filing_date` argument.
    pub filing_date: Option<String>,
    /// The `filing_date_gt` argument.
    pub filing_date_gt: Option<String>,
    /// The `filing_date_gte` argument.
    pub filing_date_gte: Option<String>,
    /// The `filing_date_lt` argument.
    pub filing_date_lt: Option<String>,
    /// The `filing_date_lte` argument.
    pub filing_date_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListStocksFilings8kTextParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `cik` argument.
    pub fn cik(mut self, cik: impl Into<String>) -> Self {
        self.cik = Some(cik.into());
        self
    }

    /// Set the `cik_any_of` argument.
    pub fn cik_any_of(mut self, cik_any_of: impl Into<String>) -> Self {
        self.cik_any_of = Some(cik_any_of.into());
        self
    }

    /// Set the `cik_gt` argument.
    pub fn cik_gt(mut self, cik_gt: impl Into<String>) -> Self {
        self.cik_gt = Some(cik_gt.into());
        self
    }

    /// Set the `cik_gte` argument.
    pub fn cik_gte(mut self, cik_gte: impl Into<String>) -> Self {
        self.cik_gte = Some(cik_gte.into());
        self
    }

    /// Set the `cik_lt` argument.
    pub fn cik_lt(mut self, cik_lt: impl Into<String>) -> Self {
        self.cik_lt = Some(cik_lt.into());
        self
    }

    /// Set the `cik_lte` argument.
    pub fn cik_lte(mut self, cik_lte: impl Into<String>) -> Self {
        self.cik_lte = Some(cik_lte.into());
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

    /// Set the `form_type` argument.
    pub fn form_type(mut self, form_type: impl Into<String>) -> Self {
        self.form_type = Some(form_type.into());
        self
    }

    /// Set the `form_type_any_of` argument.
    pub fn form_type_any_of(mut self, form_type_any_of: impl Into<String>) -> Self {
        self.form_type_any_of = Some(form_type_any_of.into());
        self
    }

    /// Set the `form_type_gt` argument.
    pub fn form_type_gt(mut self, form_type_gt: impl Into<String>) -> Self {
        self.form_type_gt = Some(form_type_gt.into());
        self
    }

    /// Set the `form_type_gte` argument.
    pub fn form_type_gte(mut self, form_type_gte: impl Into<String>) -> Self {
        self.form_type_gte = Some(form_type_gte.into());
        self
    }

    /// Set the `form_type_lt` argument.
    pub fn form_type_lt(mut self, form_type_lt: impl Into<String>) -> Self {
        self.form_type_lt = Some(form_type_lt.into());
        self
    }

    /// Set the `form_type_lte` argument.
    pub fn form_type_lte(mut self, form_type_lte: impl Into<String>) -> Self {
        self.form_type_lte = Some(form_type_lte.into());
        self
    }

    /// Set the `filing_date` argument.
    pub fn filing_date(mut self, filing_date: impl Into<String>) -> Self {
        self.filing_date = Some(filing_date.into());
        self
    }

    /// Set the `filing_date_gt` argument.
    pub fn filing_date_gt(mut self, filing_date_gt: impl Into<String>) -> Self {
        self.filing_date_gt = Some(filing_date_gt.into());
        self
    }

    /// Set the `filing_date_gte` argument.
    pub fn filing_date_gte(mut self, filing_date_gte: impl Into<String>) -> Self {
        self.filing_date_gte = Some(filing_date_gte.into());
        self
    }

    /// Set the `filing_date_lt` argument.
    pub fn filing_date_lt(mut self, filing_date_lt: impl Into<String>) -> Self {
        self.filing_date_lt = Some(filing_date_lt.into());
        self
    }

    /// Set the `filing_date_lte` argument.
    pub fn filing_date_lte(mut self, filing_date_lte: impl Into<String>) -> Self {
        self.filing_date_lte = Some(filing_date_lte.into());
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

/// Optional arguments for [`ReferenceApi::list_stocks_filings_index`].
#[derive(Debug, Default, Clone)]
pub struct ListStocksFilingsIndexParams {
    /// The `cik` argument.
    pub cik: Option<String>,
    /// The `cik_any_of` argument.
    pub cik_any_of: Option<String>,
    /// The `cik_gt` argument.
    pub cik_gt: Option<String>,
    /// The `cik_gte` argument.
    pub cik_gte: Option<String>,
    /// The `cik_lt` argument.
    pub cik_lt: Option<String>,
    /// The `cik_lte` argument.
    pub cik_lte: Option<String>,
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
    /// The `form_type` argument.
    pub form_type: Option<String>,
    /// The `form_type_any_of` argument.
    pub form_type_any_of: Option<String>,
    /// The `form_type_gt` argument.
    pub form_type_gt: Option<String>,
    /// The `form_type_gte` argument.
    pub form_type_gte: Option<String>,
    /// The `form_type_lt` argument.
    pub form_type_lt: Option<String>,
    /// The `form_type_lte` argument.
    pub form_type_lte: Option<String>,
    /// The `filing_date` argument.
    pub filing_date: Option<String>,
    /// The `filing_date_gt` argument.
    pub filing_date_gt: Option<String>,
    /// The `filing_date_gte` argument.
    pub filing_date_gte: Option<String>,
    /// The `filing_date_lt` argument.
    pub filing_date_lt: Option<String>,
    /// The `filing_date_lte` argument.
    pub filing_date_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListStocksFilingsIndexParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `cik` argument.
    pub fn cik(mut self, cik: impl Into<String>) -> Self {
        self.cik = Some(cik.into());
        self
    }

    /// Set the `cik_any_of` argument.
    pub fn cik_any_of(mut self, cik_any_of: impl Into<String>) -> Self {
        self.cik_any_of = Some(cik_any_of.into());
        self
    }

    /// Set the `cik_gt` argument.
    pub fn cik_gt(mut self, cik_gt: impl Into<String>) -> Self {
        self.cik_gt = Some(cik_gt.into());
        self
    }

    /// Set the `cik_gte` argument.
    pub fn cik_gte(mut self, cik_gte: impl Into<String>) -> Self {
        self.cik_gte = Some(cik_gte.into());
        self
    }

    /// Set the `cik_lt` argument.
    pub fn cik_lt(mut self, cik_lt: impl Into<String>) -> Self {
        self.cik_lt = Some(cik_lt.into());
        self
    }

    /// Set the `cik_lte` argument.
    pub fn cik_lte(mut self, cik_lte: impl Into<String>) -> Self {
        self.cik_lte = Some(cik_lte.into());
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

    /// Set the `form_type` argument.
    pub fn form_type(mut self, form_type: impl Into<String>) -> Self {
        self.form_type = Some(form_type.into());
        self
    }

    /// Set the `form_type_any_of` argument.
    pub fn form_type_any_of(mut self, form_type_any_of: impl Into<String>) -> Self {
        self.form_type_any_of = Some(form_type_any_of.into());
        self
    }

    /// Set the `form_type_gt` argument.
    pub fn form_type_gt(mut self, form_type_gt: impl Into<String>) -> Self {
        self.form_type_gt = Some(form_type_gt.into());
        self
    }

    /// Set the `form_type_gte` argument.
    pub fn form_type_gte(mut self, form_type_gte: impl Into<String>) -> Self {
        self.form_type_gte = Some(form_type_gte.into());
        self
    }

    /// Set the `form_type_lt` argument.
    pub fn form_type_lt(mut self, form_type_lt: impl Into<String>) -> Self {
        self.form_type_lt = Some(form_type_lt.into());
        self
    }

    /// Set the `form_type_lte` argument.
    pub fn form_type_lte(mut self, form_type_lte: impl Into<String>) -> Self {
        self.form_type_lte = Some(form_type_lte.into());
        self
    }

    /// Set the `filing_date` argument.
    pub fn filing_date(mut self, filing_date: impl Into<String>) -> Self {
        self.filing_date = Some(filing_date.into());
        self
    }

    /// Set the `filing_date_gt` argument.
    pub fn filing_date_gt(mut self, filing_date_gt: impl Into<String>) -> Self {
        self.filing_date_gt = Some(filing_date_gt.into());
        self
    }

    /// Set the `filing_date_gte` argument.
    pub fn filing_date_gte(mut self, filing_date_gte: impl Into<String>) -> Self {
        self.filing_date_gte = Some(filing_date_gte.into());
        self
    }

    /// Set the `filing_date_lt` argument.
    pub fn filing_date_lt(mut self, filing_date_lt: impl Into<String>) -> Self {
        self.filing_date_lt = Some(filing_date_lt.into());
        self
    }

    /// Set the `filing_date_lte` argument.
    pub fn filing_date_lte(mut self, filing_date_lte: impl Into<String>) -> Self {
        self.filing_date_lte = Some(filing_date_lte.into());
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

/// Optional arguments for [`ReferenceApi::list_stocks_filings_13f`].
#[derive(Debug, Default, Clone)]
pub struct ListStocksFilings13fParams {
    /// The `filer_cik` argument.
    pub filer_cik: Option<String>,
    /// The `filer_cik_any_of` argument.
    pub filer_cik_any_of: Option<String>,
    /// The `filing_date` argument.
    pub filing_date: Option<String>,
    /// The `filing_date_gt` argument.
    pub filing_date_gt: Option<String>,
    /// The `filing_date_gte` argument.
    pub filing_date_gte: Option<String>,
    /// The `filing_date_lt` argument.
    pub filing_date_lt: Option<String>,
    /// The `filing_date_lte` argument.
    pub filing_date_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListStocksFilings13fParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `filer_cik` argument.
    pub fn filer_cik(mut self, filer_cik: impl Into<String>) -> Self {
        self.filer_cik = Some(filer_cik.into());
        self
    }

    /// Set the `filer_cik_any_of` argument.
    pub fn filer_cik_any_of(mut self, filer_cik_any_of: impl Into<String>) -> Self {
        self.filer_cik_any_of = Some(filer_cik_any_of.into());
        self
    }

    /// Set the `filing_date` argument.
    pub fn filing_date(mut self, filing_date: impl Into<String>) -> Self {
        self.filing_date = Some(filing_date.into());
        self
    }

    /// Set the `filing_date_gt` argument.
    pub fn filing_date_gt(mut self, filing_date_gt: impl Into<String>) -> Self {
        self.filing_date_gt = Some(filing_date_gt.into());
        self
    }

    /// Set the `filing_date_gte` argument.
    pub fn filing_date_gte(mut self, filing_date_gte: impl Into<String>) -> Self {
        self.filing_date_gte = Some(filing_date_gte.into());
        self
    }

    /// Set the `filing_date_lt` argument.
    pub fn filing_date_lt(mut self, filing_date_lt: impl Into<String>) -> Self {
        self.filing_date_lt = Some(filing_date_lt.into());
        self
    }

    /// Set the `filing_date_lte` argument.
    pub fn filing_date_lte(mut self, filing_date_lte: impl Into<String>) -> Self {
        self.filing_date_lte = Some(filing_date_lte.into());
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

/// Optional arguments for [`ReferenceApi::list_stocks_filings_form_3`].
#[derive(Debug, Default, Clone)]
pub struct ListStocksFilingsForm3Params {
    /// The `issuer_cik` argument.
    pub issuer_cik: Option<String>,
    /// The `issuer_cik_any_of` argument.
    pub issuer_cik_any_of: Option<String>,
    /// The `owner_cik` argument.
    pub owner_cik: Option<String>,
    /// The `owner_cik_any_of` argument.
    pub owner_cik_any_of: Option<String>,
    /// The `tickers` argument.
    pub tickers: Option<String>,
    /// The `tickers_all_of` argument.
    pub tickers_all_of: Option<String>,
    /// The `tickers_any_of` argument.
    pub tickers_any_of: Option<String>,
    /// The `form_type` argument.
    pub form_type: Option<String>,
    /// The `filing_date` argument.
    pub filing_date: Option<String>,
    /// The `filing_date_gt` argument.
    pub filing_date_gt: Option<String>,
    /// The `filing_date_gte` argument.
    pub filing_date_gte: Option<String>,
    /// The `filing_date_lt` argument.
    pub filing_date_lt: Option<String>,
    /// The `filing_date_lte` argument.
    pub filing_date_lte: Option<String>,
    /// The `max_ticker` argument.
    pub max_ticker: Option<String>,
    /// The `max_ticker_any_of` argument.
    pub max_ticker_any_of: Option<String>,
    /// The `max_ticker_gt` argument.
    pub max_ticker_gt: Option<String>,
    /// The `max_ticker_gte` argument.
    pub max_ticker_gte: Option<String>,
    /// The `max_ticker_lt` argument.
    pub max_ticker_lt: Option<String>,
    /// The `max_ticker_lte` argument.
    pub max_ticker_lte: Option<String>,
    /// The `min_ticker` argument.
    pub min_ticker: Option<String>,
    /// The `min_ticker_any_of` argument.
    pub min_ticker_any_of: Option<String>,
    /// The `min_ticker_gt` argument.
    pub min_ticker_gt: Option<String>,
    /// The `min_ticker_gte` argument.
    pub min_ticker_gte: Option<String>,
    /// The `min_ticker_lt` argument.
    pub min_ticker_lt: Option<String>,
    /// The `min_ticker_lte` argument.
    pub min_ticker_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListStocksFilingsForm3Params {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `issuer_cik` argument.
    pub fn issuer_cik(mut self, issuer_cik: impl Into<String>) -> Self {
        self.issuer_cik = Some(issuer_cik.into());
        self
    }

    /// Set the `issuer_cik_any_of` argument.
    pub fn issuer_cik_any_of(mut self, issuer_cik_any_of: impl Into<String>) -> Self {
        self.issuer_cik_any_of = Some(issuer_cik_any_of.into());
        self
    }

    /// Set the `owner_cik` argument.
    pub fn owner_cik(mut self, owner_cik: impl Into<String>) -> Self {
        self.owner_cik = Some(owner_cik.into());
        self
    }

    /// Set the `owner_cik_any_of` argument.
    pub fn owner_cik_any_of(mut self, owner_cik_any_of: impl Into<String>) -> Self {
        self.owner_cik_any_of = Some(owner_cik_any_of.into());
        self
    }

    /// Set the `tickers` argument.
    pub fn tickers(mut self, tickers: impl Into<String>) -> Self {
        self.tickers = Some(tickers.into());
        self
    }

    /// Set the `tickers_all_of` argument.
    pub fn tickers_all_of(mut self, tickers_all_of: impl Into<String>) -> Self {
        self.tickers_all_of = Some(tickers_all_of.into());
        self
    }

    /// Set the `tickers_any_of` argument.
    pub fn tickers_any_of(mut self, tickers_any_of: impl Into<String>) -> Self {
        self.tickers_any_of = Some(tickers_any_of.into());
        self
    }

    /// Set the `form_type` argument.
    pub fn form_type(mut self, form_type: impl Into<String>) -> Self {
        self.form_type = Some(form_type.into());
        self
    }

    /// Set the `filing_date` argument.
    pub fn filing_date(mut self, filing_date: impl Into<String>) -> Self {
        self.filing_date = Some(filing_date.into());
        self
    }

    /// Set the `filing_date_gt` argument.
    pub fn filing_date_gt(mut self, filing_date_gt: impl Into<String>) -> Self {
        self.filing_date_gt = Some(filing_date_gt.into());
        self
    }

    /// Set the `filing_date_gte` argument.
    pub fn filing_date_gte(mut self, filing_date_gte: impl Into<String>) -> Self {
        self.filing_date_gte = Some(filing_date_gte.into());
        self
    }

    /// Set the `filing_date_lt` argument.
    pub fn filing_date_lt(mut self, filing_date_lt: impl Into<String>) -> Self {
        self.filing_date_lt = Some(filing_date_lt.into());
        self
    }

    /// Set the `filing_date_lte` argument.
    pub fn filing_date_lte(mut self, filing_date_lte: impl Into<String>) -> Self {
        self.filing_date_lte = Some(filing_date_lte.into());
        self
    }

    /// Set the `max_ticker` argument.
    pub fn max_ticker(mut self, max_ticker: impl Into<String>) -> Self {
        self.max_ticker = Some(max_ticker.into());
        self
    }

    /// Set the `max_ticker_any_of` argument.
    pub fn max_ticker_any_of(mut self, max_ticker_any_of: impl Into<String>) -> Self {
        self.max_ticker_any_of = Some(max_ticker_any_of.into());
        self
    }

    /// Set the `max_ticker_gt` argument.
    pub fn max_ticker_gt(mut self, max_ticker_gt: impl Into<String>) -> Self {
        self.max_ticker_gt = Some(max_ticker_gt.into());
        self
    }

    /// Set the `max_ticker_gte` argument.
    pub fn max_ticker_gte(mut self, max_ticker_gte: impl Into<String>) -> Self {
        self.max_ticker_gte = Some(max_ticker_gte.into());
        self
    }

    /// Set the `max_ticker_lt` argument.
    pub fn max_ticker_lt(mut self, max_ticker_lt: impl Into<String>) -> Self {
        self.max_ticker_lt = Some(max_ticker_lt.into());
        self
    }

    /// Set the `max_ticker_lte` argument.
    pub fn max_ticker_lte(mut self, max_ticker_lte: impl Into<String>) -> Self {
        self.max_ticker_lte = Some(max_ticker_lte.into());
        self
    }

    /// Set the `min_ticker` argument.
    pub fn min_ticker(mut self, min_ticker: impl Into<String>) -> Self {
        self.min_ticker = Some(min_ticker.into());
        self
    }

    /// Set the `min_ticker_any_of` argument.
    pub fn min_ticker_any_of(mut self, min_ticker_any_of: impl Into<String>) -> Self {
        self.min_ticker_any_of = Some(min_ticker_any_of.into());
        self
    }

    /// Set the `min_ticker_gt` argument.
    pub fn min_ticker_gt(mut self, min_ticker_gt: impl Into<String>) -> Self {
        self.min_ticker_gt = Some(min_ticker_gt.into());
        self
    }

    /// Set the `min_ticker_gte` argument.
    pub fn min_ticker_gte(mut self, min_ticker_gte: impl Into<String>) -> Self {
        self.min_ticker_gte = Some(min_ticker_gte.into());
        self
    }

    /// Set the `min_ticker_lt` argument.
    pub fn min_ticker_lt(mut self, min_ticker_lt: impl Into<String>) -> Self {
        self.min_ticker_lt = Some(min_ticker_lt.into());
        self
    }

    /// Set the `min_ticker_lte` argument.
    pub fn min_ticker_lte(mut self, min_ticker_lte: impl Into<String>) -> Self {
        self.min_ticker_lte = Some(min_ticker_lte.into());
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

/// Optional arguments for [`ReferenceApi::list_stocks_filings_form_4`].
#[derive(Debug, Default, Clone)]
pub struct ListStocksFilingsForm4Params {
    /// The `issuer_cik` argument.
    pub issuer_cik: Option<String>,
    /// The `issuer_cik_any_of` argument.
    pub issuer_cik_any_of: Option<String>,
    /// The `owner_cik` argument.
    pub owner_cik: Option<String>,
    /// The `owner_cik_any_of` argument.
    pub owner_cik_any_of: Option<String>,
    /// The `tickers` argument.
    pub tickers: Option<String>,
    /// The `tickers_all_of` argument.
    pub tickers_all_of: Option<String>,
    /// The `tickers_any_of` argument.
    pub tickers_any_of: Option<String>,
    /// The `form_type` argument.
    pub form_type: Option<String>,
    /// The `transaction_code` argument.
    pub transaction_code: Option<String>,
    /// The `filing_date` argument.
    pub filing_date: Option<String>,
    /// The `filing_date_gt` argument.
    pub filing_date_gt: Option<String>,
    /// The `filing_date_gte` argument.
    pub filing_date_gte: Option<String>,
    /// The `filing_date_lt` argument.
    pub filing_date_lt: Option<String>,
    /// The `filing_date_lte` argument.
    pub filing_date_lte: Option<String>,
    /// The `max_ticker` argument.
    pub max_ticker: Option<String>,
    /// The `max_ticker_any_of` argument.
    pub max_ticker_any_of: Option<String>,
    /// The `max_ticker_gt` argument.
    pub max_ticker_gt: Option<String>,
    /// The `max_ticker_gte` argument.
    pub max_ticker_gte: Option<String>,
    /// The `max_ticker_lt` argument.
    pub max_ticker_lt: Option<String>,
    /// The `max_ticker_lte` argument.
    pub max_ticker_lte: Option<String>,
    /// The `min_ticker` argument.
    pub min_ticker: Option<String>,
    /// The `min_ticker_any_of` argument.
    pub min_ticker_any_of: Option<String>,
    /// The `min_ticker_gt` argument.
    pub min_ticker_gt: Option<String>,
    /// The `min_ticker_gte` argument.
    pub min_ticker_gte: Option<String>,
    /// The `min_ticker_lt` argument.
    pub min_ticker_lt: Option<String>,
    /// The `min_ticker_lte` argument.
    pub min_ticker_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListStocksFilingsForm4Params {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `issuer_cik` argument.
    pub fn issuer_cik(mut self, issuer_cik: impl Into<String>) -> Self {
        self.issuer_cik = Some(issuer_cik.into());
        self
    }

    /// Set the `issuer_cik_any_of` argument.
    pub fn issuer_cik_any_of(mut self, issuer_cik_any_of: impl Into<String>) -> Self {
        self.issuer_cik_any_of = Some(issuer_cik_any_of.into());
        self
    }

    /// Set the `owner_cik` argument.
    pub fn owner_cik(mut self, owner_cik: impl Into<String>) -> Self {
        self.owner_cik = Some(owner_cik.into());
        self
    }

    /// Set the `owner_cik_any_of` argument.
    pub fn owner_cik_any_of(mut self, owner_cik_any_of: impl Into<String>) -> Self {
        self.owner_cik_any_of = Some(owner_cik_any_of.into());
        self
    }

    /// Set the `tickers` argument.
    pub fn tickers(mut self, tickers: impl Into<String>) -> Self {
        self.tickers = Some(tickers.into());
        self
    }

    /// Set the `tickers_all_of` argument.
    pub fn tickers_all_of(mut self, tickers_all_of: impl Into<String>) -> Self {
        self.tickers_all_of = Some(tickers_all_of.into());
        self
    }

    /// Set the `tickers_any_of` argument.
    pub fn tickers_any_of(mut self, tickers_any_of: impl Into<String>) -> Self {
        self.tickers_any_of = Some(tickers_any_of.into());
        self
    }

    /// Set the `form_type` argument.
    pub fn form_type(mut self, form_type: impl Into<String>) -> Self {
        self.form_type = Some(form_type.into());
        self
    }

    /// Set the `transaction_code` argument.
    pub fn transaction_code(mut self, transaction_code: impl Into<String>) -> Self {
        self.transaction_code = Some(transaction_code.into());
        self
    }

    /// Set the `filing_date` argument.
    pub fn filing_date(mut self, filing_date: impl Into<String>) -> Self {
        self.filing_date = Some(filing_date.into());
        self
    }

    /// Set the `filing_date_gt` argument.
    pub fn filing_date_gt(mut self, filing_date_gt: impl Into<String>) -> Self {
        self.filing_date_gt = Some(filing_date_gt.into());
        self
    }

    /// Set the `filing_date_gte` argument.
    pub fn filing_date_gte(mut self, filing_date_gte: impl Into<String>) -> Self {
        self.filing_date_gte = Some(filing_date_gte.into());
        self
    }

    /// Set the `filing_date_lt` argument.
    pub fn filing_date_lt(mut self, filing_date_lt: impl Into<String>) -> Self {
        self.filing_date_lt = Some(filing_date_lt.into());
        self
    }

    /// Set the `filing_date_lte` argument.
    pub fn filing_date_lte(mut self, filing_date_lte: impl Into<String>) -> Self {
        self.filing_date_lte = Some(filing_date_lte.into());
        self
    }

    /// Set the `max_ticker` argument.
    pub fn max_ticker(mut self, max_ticker: impl Into<String>) -> Self {
        self.max_ticker = Some(max_ticker.into());
        self
    }

    /// Set the `max_ticker_any_of` argument.
    pub fn max_ticker_any_of(mut self, max_ticker_any_of: impl Into<String>) -> Self {
        self.max_ticker_any_of = Some(max_ticker_any_of.into());
        self
    }

    /// Set the `max_ticker_gt` argument.
    pub fn max_ticker_gt(mut self, max_ticker_gt: impl Into<String>) -> Self {
        self.max_ticker_gt = Some(max_ticker_gt.into());
        self
    }

    /// Set the `max_ticker_gte` argument.
    pub fn max_ticker_gte(mut self, max_ticker_gte: impl Into<String>) -> Self {
        self.max_ticker_gte = Some(max_ticker_gte.into());
        self
    }

    /// Set the `max_ticker_lt` argument.
    pub fn max_ticker_lt(mut self, max_ticker_lt: impl Into<String>) -> Self {
        self.max_ticker_lt = Some(max_ticker_lt.into());
        self
    }

    /// Set the `max_ticker_lte` argument.
    pub fn max_ticker_lte(mut self, max_ticker_lte: impl Into<String>) -> Self {
        self.max_ticker_lte = Some(max_ticker_lte.into());
        self
    }

    /// Set the `min_ticker` argument.
    pub fn min_ticker(mut self, min_ticker: impl Into<String>) -> Self {
        self.min_ticker = Some(min_ticker.into());
        self
    }

    /// Set the `min_ticker_any_of` argument.
    pub fn min_ticker_any_of(mut self, min_ticker_any_of: impl Into<String>) -> Self {
        self.min_ticker_any_of = Some(min_ticker_any_of.into());
        self
    }

    /// Set the `min_ticker_gt` argument.
    pub fn min_ticker_gt(mut self, min_ticker_gt: impl Into<String>) -> Self {
        self.min_ticker_gt = Some(min_ticker_gt.into());
        self
    }

    /// Set the `min_ticker_gte` argument.
    pub fn min_ticker_gte(mut self, min_ticker_gte: impl Into<String>) -> Self {
        self.min_ticker_gte = Some(min_ticker_gte.into());
        self
    }

    /// Set the `min_ticker_lt` argument.
    pub fn min_ticker_lt(mut self, min_ticker_lt: impl Into<String>) -> Self {
        self.min_ticker_lt = Some(min_ticker_lt.into());
        self
    }

    /// Set the `min_ticker_lte` argument.
    pub fn min_ticker_lte(mut self, min_ticker_lte: impl Into<String>) -> Self {
        self.min_ticker_lte = Some(min_ticker_lte.into());
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
