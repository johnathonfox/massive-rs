use crate::client::{Client, RequestOptions};
use crate::error::Result;
use crate::models::{
    Condition, Disclosure, DisclosureTaxonomy, Dividend, Exchange, Filing13F, Filing8K,
    FilingForm3, FilingForm4, FilingIndex, FilingSection, MarketHoliday, MarketStatus,
    OptionsContract, RelatedCompany, RiskFactor, RiskFactorTaxonomy, ShortInterest, ShortVolume,
    Split, StockDividend, StockSplit, Ticker, TickerChangeResults, TickerDetails, TickerNews,
    TickerTypes,
};
use futures::Stream;

/// Reference data API.
pub trait ReferenceApi {
    /// Get upcoming market holidays and their open/close times.
    async fn get_market_holidays(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<MarketHoliday>>;

    /// Get the current trading status of the exchanges and overall financial markets.
    async fn get_market_status(&self, options: Option<&RequestOptions>) -> Result<MarketStatus>;

    /// Query all ticker symbols supported by Massive.com (stocks, indices, forex, crypto).
    fn list_tickers(
        &self,
        ticker: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        r#type: Option<&str>,
        market: Option<&str>,
        exchange: Option<&str>,
        cusip: Option<i64>,
        cik: Option<i64>,
        date: Option<&str>,
        active: Option<bool>,
        search: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Ticker>>;

    /// Get detailed information about a single ticker and the company behind it.
    async fn get_ticker_details(
        &self,
        ticker: &str,
        date: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<TickerDetails>;

    /// Get event history of a ticker given a particular point in time.
    async fn get_ticker_events(
        &self,
        ticker: &str,
        types: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<TickerChangeResults>;

    /// Get the most recent news articles relating to a stock ticker symbol.
    fn list_ticker_news(
        &self,
        ticker: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        published_utc: Option<&str>,
        published_utc_lt: Option<&str>,
        published_utc_lte: Option<&str>,
        published_utc_gt: Option<&str>,
        published_utc_gte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<TickerNews>>;

    /// List all ticker types that Massive.com has.
    async fn get_ticker_types(
        &self,
        asset_class: Option<&str>,
        locale: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<TickerTypes>>;

    /// Get a list of tickers related to the queried ticker based on News and Returns data.
    async fn get_related_companies(
        &self,
        ticker: &str,
        options: Option<&RequestOptions>,
    ) -> Result<RelatedCompany>;

    /// Get a list of historical stock splits.
    fn list_splits(
        &self,
        ticker: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        execution_date: Option<&str>,
        execution_date_lt: Option<&str>,
        execution_date_lte: Option<&str>,
        execution_date_gt: Option<&str>,
        execution_date_gte: Option<&str>,
        reverse_split: Option<bool>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Split>>;

    /// Get a list of historical cash dividends.
    fn list_dividends(
        &self,
        ticker: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ex_dividend_date: Option<&str>,
        ex_dividend_date_lt: Option<&str>,
        ex_dividend_date_lte: Option<&str>,
        ex_dividend_date_gt: Option<&str>,
        ex_dividend_date_gte: Option<&str>,
        record_date: Option<&str>,
        record_date_lt: Option<&str>,
        record_date_lte: Option<&str>,
        record_date_gt: Option<&str>,
        record_date_gte: Option<&str>,
        declaration_date: Option<&str>,
        declaration_date_lt: Option<&str>,
        declaration_date_lte: Option<&str>,
        declaration_date_gt: Option<&str>,
        declaration_date_gte: Option<&str>,
        pay_date: Option<&str>,
        pay_date_lt: Option<&str>,
        pay_date_lte: Option<&str>,
        pay_date_gt: Option<&str>,
        pay_date_gte: Option<&str>,
        frequency: Option<i64>,
        cash_amount: Option<f64>,
        cash_amount_lt: Option<f64>,
        cash_amount_lte: Option<f64>,
        cash_amount_gt: Option<f64>,
        cash_amount_gte: Option<f64>,
        dividend_type: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Dividend>>;

    /// List all conditions that Massive.com uses.
    fn list_conditions(
        &self,
        asset_class: Option<&str>,
        data_type: Option<&str>,
        id: Option<i64>,
        sip: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Condition>>;

    /// List all exchanges that Massive.com knows about.
    async fn get_exchanges(
        &self,
        asset_class: Option<&str>,
        locale: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<Exchange>>;

    /// Get a single options contract by ticker.
    async fn get_options_contract(
        &self,
        ticker: &str,
        as_of: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<OptionsContract>;

    /// List historical options contracts.
    fn list_options_contracts(
        &self,
        underlying_ticker: Option<&str>,
        underlying_ticker_lt: Option<&str>,
        underlying_ticker_lte: Option<&str>,
        underlying_ticker_gt: Option<&str>,
        underlying_ticker_gte: Option<&str>,
        contract_type: Option<&str>,
        expiration_date: Option<&str>,
        expiration_date_lt: Option<&str>,
        expiration_date_lte: Option<&str>,
        expiration_date_gt: Option<&str>,
        expiration_date_gte: Option<&str>,
        as_of: Option<&str>,
        strike_price: Option<f64>,
        strike_price_lt: Option<f64>,
        strike_price_lte: Option<f64>,
        strike_price_gt: Option<f64>,
        strike_price_gte: Option<f64>,
        expired: Option<bool>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<OptionsContract>>;

    /// Retrieve short interest data for stocks.
    fn list_short_interest(
        &self,
        ticker: Option<&str>,
        days_to_cover: Option<&str>,
        days_to_cover_lt: Option<&str>,
        days_to_cover_lte: Option<&str>,
        days_to_cover_gt: Option<&str>,
        days_to_cover_gte: Option<&str>,
        settlement_date: Option<&str>,
        settlement_date_lt: Option<&str>,
        settlement_date_lte: Option<&str>,
        settlement_date_gt: Option<&str>,
        settlement_date_gte: Option<&str>,
        avg_daily_volume: Option<&str>,
        avg_daily_volume_lt: Option<&str>,
        avg_daily_volume_lte: Option<&str>,
        avg_daily_volume_gt: Option<&str>,
        avg_daily_volume_gte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<ShortInterest>>;

    /// Retrieve short volume data for stocks.
    fn list_short_volume(
        &self,
        ticker: Option<&str>,
        date: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        short_volume_ratio: Option<&str>,
        short_volume_ratio_lt: Option<&str>,
        short_volume_ratio_lte: Option<&str>,
        short_volume_ratio_gt: Option<&str>,
        short_volume_ratio_gte: Option<&str>,
        total_volume: Option<&str>,
        total_volume_lt: Option<&str>,
        total_volume_lte: Option<&str>,
        total_volume_gt: Option<&str>,
        total_volume_gte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<ShortVolume>>;

    /// List stock splits (GET /stocks/v1/splits).
    fn list_stocks_splits(
        &self,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        execution_date: Option<&str>,
        execution_date_gt: Option<&str>,
        execution_date_gte: Option<&str>,
        execution_date_lt: Option<&str>,
        execution_date_lte: Option<&str>,
        adjustment_type: Option<&str>,
        adjustment_type_any_of: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<StockSplit>>;

    /// List stock dividends (GET /stocks/v1/dividends).
    fn list_stocks_dividends(
        &self,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        ex_dividend_date: Option<&str>,
        ex_dividend_date_gt: Option<&str>,
        ex_dividend_date_gte: Option<&str>,
        ex_dividend_date_lt: Option<&str>,
        ex_dividend_date_lte: Option<&str>,
        frequency: Option<i64>,
        frequency_gt: Option<i64>,
        frequency_gte: Option<i64>,
        frequency_lt: Option<i64>,
        frequency_lte: Option<i64>,
        distribution_type: Option<&str>,
        distribution_type_any_of: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<StockDividend>>;

    /// Get categorized risk factors extracted from 10-K filings (with supporting_text).
    fn list_stocks_filings_risk_factors(
        &self,
        filing_date: Option<&str>,
        filing_date_any_of: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        cik: Option<&str>,
        cik_any_of: Option<&str>,
        cik_gt: Option<&str>,
        cik_gte: Option<&str>,
        cik_lt: Option<&str>,
        cik_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<RiskFactor>>;

    /// Get the taxonomy/categories used to classify risk factors.
    fn list_stocks_taxonomies_risk_factors(
        &self,
        taxonomy: Option<f64>,
        taxonomy_gt: Option<f64>,
        taxonomy_gte: Option<f64>,
        taxonomy_lt: Option<f64>,
        taxonomy_lte: Option<f64>,
        primary_category: Option<&str>,
        primary_category_any_of: Option<&str>,
        primary_category_gt: Option<&str>,
        primary_category_gte: Option<&str>,
        primary_category_lt: Option<&str>,
        primary_category_lte: Option<&str>,
        secondary_category: Option<&str>,
        secondary_category_any_of: Option<&str>,
        secondary_category_gt: Option<&str>,
        secondary_category_gte: Option<&str>,
        secondary_category_lt: Option<&str>,
        secondary_category_lte: Option<&str>,
        tertiary_category: Option<&str>,
        tertiary_category_any_of: Option<&str>,
        tertiary_category_gt: Option<&str>,
        tertiary_category_gte: Option<&str>,
        tertiary_category_lt: Option<&str>,
        tertiary_category_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<RiskFactorTaxonomy>>;

    /// SEC 8-K filing disclosure categorization.
    fn list_stocks_filings_8k_disclosures(
        &self,
        cik: Option<&str>,
        cik_any_of: Option<&str>,
        tickers: Option<&str>,
        tickers_all_of: Option<&str>,
        tickers_any_of: Option<&str>,
        filing_date: Option<&str>,
        filing_date_any_of: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        tertiary_category: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Disclosure>>;

    /// The complete list of 8-K disclosure classifications.
    fn list_stocks_taxonomies_disclosures(
        &self,
        taxonomy: Option<&str>,
        taxonomy_any_of: Option<&str>,
        taxonomy_gt: Option<&str>,
        taxonomy_gte: Option<&str>,
        taxonomy_lt: Option<&str>,
        taxonomy_lte: Option<&str>,
        primary_category: Option<&str>,
        primary_category_any_of: Option<&str>,
        primary_category_gt: Option<&str>,
        primary_category_gte: Option<&str>,
        primary_category_lt: Option<&str>,
        primary_category_lte: Option<&str>,
        secondary_category: Option<&str>,
        secondary_category_any_of: Option<&str>,
        secondary_category_gt: Option<&str>,
        secondary_category_gte: Option<&str>,
        secondary_category_lt: Option<&str>,
        secondary_category_lte: Option<&str>,
        tertiary_category: Option<&str>,
        tertiary_category_any_of: Option<&str>,
        tertiary_category_gt: Option<&str>,
        tertiary_category_gte: Option<&str>,
        tertiary_category_lt: Option<&str>,
        tertiary_category_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<DisclosureTaxonomy>>;

    /// Get raw text sections from 10-K/10-Q filings (business, risk_factors, etc.).
    fn list_stocks_filings_10k_sections(
        &self,
        cik: Option<&str>,
        cik_any_of: Option<&str>,
        cik_gt: Option<&str>,
        cik_gte: Option<&str>,
        cik_lt: Option<&str>,
        cik_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        section: Option<&str>,
        section_any_of: Option<&str>,
        filing_date: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        period_end: Option<&str>,
        period_end_gt: Option<&str>,
        period_end_gte: Option<&str>,
        period_end_lt: Option<&str>,
        period_end_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FilingSection>>;

    /// Get parsed 8-K filings (earnings, acquisitions, executive changes, etc.).
    fn list_stocks_filings_8k_text(
        &self,
        cik: Option<&str>,
        cik_any_of: Option<&str>,
        cik_gt: Option<&str>,
        cik_gte: Option<&str>,
        cik_lt: Option<&str>,
        cik_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        form_type: Option<&str>,
        form_type_any_of: Option<&str>,
        form_type_gt: Option<&str>,
        form_type_gte: Option<&str>,
        form_type_lt: Option<&str>,
        form_type_lte: Option<&str>,
        filing_date: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Filing8K>>;

    /// Get the master index of all SEC filings (10-K, 8-K, 10-Q, S-1, 4, etc.).
    fn list_stocks_filings_index(
        &self,
        cik: Option<&str>,
        cik_any_of: Option<&str>,
        cik_gt: Option<&str>,
        cik_gte: Option<&str>,
        cik_lt: Option<&str>,
        cik_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        form_type: Option<&str>,
        form_type_any_of: Option<&str>,
        form_type_gt: Option<&str>,
        form_type_gte: Option<&str>,
        form_type_lt: Option<&str>,
        form_type_lte: Option<&str>,
        filing_date: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FilingIndex>>;

    /// SEC Form 13F filings data showing institutional investment manager holdings.
    fn list_stocks_filings_13f(
        &self,
        filer_cik: Option<&str>,
        filer_cik_any_of: Option<&str>,
        filing_date: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Filing13F>>;

    /// SEC Form 3 filings reporting initial statements of beneficial ownership of securities.
    fn list_stocks_filings_form_3(
        &self,
        issuer_cik: Option<&str>,
        issuer_cik_any_of: Option<&str>,
        owner_cik: Option<&str>,
        owner_cik_any_of: Option<&str>,
        tickers: Option<&str>,
        tickers_all_of: Option<&str>,
        tickers_any_of: Option<&str>,
        form_type: Option<&str>,
        filing_date: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        max_ticker: Option<&str>,
        max_ticker_any_of: Option<&str>,
        max_ticker_gt: Option<&str>,
        max_ticker_gte: Option<&str>,
        max_ticker_lt: Option<&str>,
        max_ticker_lte: Option<&str>,
        min_ticker: Option<&str>,
        min_ticker_any_of: Option<&str>,
        min_ticker_gt: Option<&str>,
        min_ticker_gte: Option<&str>,
        min_ticker_lt: Option<&str>,
        min_ticker_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FilingForm3>>;

    /// SEC Form 4 filings reporting changes in beneficial ownership of securities.
    fn list_stocks_filings_form_4(
        &self,
        issuer_cik: Option<&str>,
        issuer_cik_any_of: Option<&str>,
        owner_cik: Option<&str>,
        owner_cik_any_of: Option<&str>,
        tickers: Option<&str>,
        tickers_all_of: Option<&str>,
        tickers_any_of: Option<&str>,
        form_type: Option<&str>,
        transaction_code: Option<&str>,
        filing_date: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        max_ticker: Option<&str>,
        max_ticker_any_of: Option<&str>,
        max_ticker_gt: Option<&str>,
        max_ticker_gte: Option<&str>,
        max_ticker_lt: Option<&str>,
        max_ticker_lte: Option<&str>,
        min_ticker: Option<&str>,
        min_ticker_any_of: Option<&str>,
        min_ticker_gt: Option<&str>,
        min_ticker_gte: Option<&str>,
        min_ticker_lt: Option<&str>,
        min_ticker_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FilingForm4>>;
}

impl ReferenceApi for Client {
    async fn get_market_holidays(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<MarketHoliday>> {
        let path = "/v1/marketstatus/upcoming".to_string();
        self.get(&path, None, options).await
    }

    async fn get_market_status(&self, options: Option<&RequestOptions>) -> Result<MarketStatus> {
        let path = "/v1/marketstatus/now".to_string();
        self.get(&path, None, options).await
    }

    fn list_tickers(
        &self,
        ticker: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        r#type: Option<&str>,
        market: Option<&str>,
        exchange: Option<&str>,
        cusip: Option<i64>,
        cik: Option<i64>,
        date: Option<&str>,
        active: Option<bool>,
        search: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Ticker>> {
        let path = "/v3/reference/tickers".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = ticker {
            params.push(("ticker", v.to_string()));
        }
        if let Some(v) = ticker_lt {
            params.push(("ticker.lt", v.to_string()));
        }
        if let Some(v) = ticker_lte {
            params.push(("ticker.lte", v.to_string()));
        }
        if let Some(v) = ticker_gt {
            params.push(("ticker.gt", v.to_string()));
        }
        if let Some(v) = ticker_gte {
            params.push(("ticker.gte", v.to_string()));
        }
        if let Some(v) = r#type {
            params.push(("type", v.to_string()));
        }
        if let Some(v) = market {
            params.push(("market", v.to_string()));
        }
        if let Some(v) = exchange {
            params.push(("exchange", v.to_string()));
        }
        if let Some(v) = cusip {
            params.push(("cusip", v.to_string()));
        }
        if let Some(v) = cik {
            params.push(("cik", v.to_string()));
        }
        if let Some(v) = date {
            params.push(("date", v.to_string()));
        }
        if let Some(v) = active {
            params.push(("active", v.to_string()));
        }
        if let Some(v) = search {
            params.push(("search", v.to_string()));
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
            self.paginate::<Ticker>(&path, Some(&params), options)
        } else {
            self.single_page::<Ticker>(&path, Some(&params), options)
        }
    }

    async fn get_ticker_details(
        &self,
        ticker: &str,
        date: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<TickerDetails> {
        let path = format!("/v3/reference/tickers/{}", ticker);
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = date {
            params.push(("date", v.to_string()));
        }
        #[derive(serde::Deserialize)]
        struct Resp {
            results: TickerDetails,
        }
        let resp: Resp = self.get(&path, Some(&params), options).await?;
        Ok(resp.results)
    }

    async fn get_ticker_events(
        &self,
        ticker: &str,
        types: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<TickerChangeResults> {
        let path = format!("/vX/reference/tickers/{}/events", ticker);
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = types {
            params.push(("types", v.to_string()));
        }
        #[derive(serde::Deserialize)]
        struct Resp {
            results: TickerChangeResults,
        }
        let resp: Resp = self.get(&path, Some(&params), options).await?;
        Ok(resp.results)
    }

    fn list_ticker_news(
        &self,
        ticker: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        published_utc: Option<&str>,
        published_utc_lt: Option<&str>,
        published_utc_lte: Option<&str>,
        published_utc_gt: Option<&str>,
        published_utc_gte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<TickerNews>> {
        let path = "/v2/reference/news".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = ticker {
            params.push(("ticker", v.to_string()));
        }
        if let Some(v) = ticker_lt {
            params.push(("ticker.lt", v.to_string()));
        }
        if let Some(v) = ticker_lte {
            params.push(("ticker.lte", v.to_string()));
        }
        if let Some(v) = ticker_gt {
            params.push(("ticker.gt", v.to_string()));
        }
        if let Some(v) = ticker_gte {
            params.push(("ticker.gte", v.to_string()));
        }
        if let Some(v) = published_utc {
            params.push(("published_utc", v.to_string()));
        }
        if let Some(v) = published_utc_lt {
            params.push(("published_utc.lt", v.to_string()));
        }
        if let Some(v) = published_utc_lte {
            params.push(("published_utc.lte", v.to_string()));
        }
        if let Some(v) = published_utc_gt {
            params.push(("published_utc.gt", v.to_string()));
        }
        if let Some(v) = published_utc_gte {
            params.push(("published_utc.gte", v.to_string()));
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
            self.paginate::<TickerNews>(&path, Some(&params), options)
        } else {
            self.single_page::<TickerNews>(&path, Some(&params), options)
        }
    }

    async fn get_ticker_types(
        &self,
        asset_class: Option<&str>,
        locale: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<TickerTypes>> {
        let path = "/v3/reference/tickers/types".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = asset_class {
            params.push(("asset_class", v.to_string()));
        }
        if let Some(v) = locale {
            params.push(("locale", v.to_string()));
        }
        #[derive(serde::Deserialize)]
        struct Resp {
            results: Option<Vec<TickerTypes>>,
        }
        let resp: Resp = self.get(&path, Some(&params), options).await?;
        Ok(resp.results.unwrap_or_default())
    }

    async fn get_related_companies(
        &self,
        ticker: &str,
        options: Option<&RequestOptions>,
    ) -> Result<RelatedCompany> {
        let path = format!("/v1/related-companies/{}", ticker);
        #[derive(serde::Deserialize)]
        struct Resp {
            results: RelatedCompany,
        }
        let resp: Resp = self.get(&path, None, options).await?;
        Ok(resp.results)
    }

    fn list_splits(
        &self,
        ticker: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        execution_date: Option<&str>,
        execution_date_lt: Option<&str>,
        execution_date_lte: Option<&str>,
        execution_date_gt: Option<&str>,
        execution_date_gte: Option<&str>,
        reverse_split: Option<bool>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Split>> {
        let path = "/v3/reference/splits".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = ticker {
            params.push(("ticker", v.to_string()));
        }
        if let Some(v) = ticker_lt {
            params.push(("ticker.lt", v.to_string()));
        }
        if let Some(v) = ticker_lte {
            params.push(("ticker.lte", v.to_string()));
        }
        if let Some(v) = ticker_gt {
            params.push(("ticker.gt", v.to_string()));
        }
        if let Some(v) = ticker_gte {
            params.push(("ticker.gte", v.to_string()));
        }
        if let Some(v) = execution_date {
            params.push(("execution_date", v.to_string()));
        }
        if let Some(v) = execution_date_lt {
            params.push(("execution_date.lt", v.to_string()));
        }
        if let Some(v) = execution_date_lte {
            params.push(("execution_date.lte", v.to_string()));
        }
        if let Some(v) = execution_date_gt {
            params.push(("execution_date.gt", v.to_string()));
        }
        if let Some(v) = execution_date_gte {
            params.push(("execution_date.gte", v.to_string()));
        }
        if let Some(v) = reverse_split {
            params.push(("reverse_split", v.to_string()));
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
            self.paginate::<Split>(&path, Some(&params), options)
        } else {
            self.single_page::<Split>(&path, Some(&params), options)
        }
    }

    fn list_dividends(
        &self,
        ticker: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ex_dividend_date: Option<&str>,
        ex_dividend_date_lt: Option<&str>,
        ex_dividend_date_lte: Option<&str>,
        ex_dividend_date_gt: Option<&str>,
        ex_dividend_date_gte: Option<&str>,
        record_date: Option<&str>,
        record_date_lt: Option<&str>,
        record_date_lte: Option<&str>,
        record_date_gt: Option<&str>,
        record_date_gte: Option<&str>,
        declaration_date: Option<&str>,
        declaration_date_lt: Option<&str>,
        declaration_date_lte: Option<&str>,
        declaration_date_gt: Option<&str>,
        declaration_date_gte: Option<&str>,
        pay_date: Option<&str>,
        pay_date_lt: Option<&str>,
        pay_date_lte: Option<&str>,
        pay_date_gt: Option<&str>,
        pay_date_gte: Option<&str>,
        frequency: Option<i64>,
        cash_amount: Option<f64>,
        cash_amount_lt: Option<f64>,
        cash_amount_lte: Option<f64>,
        cash_amount_gt: Option<f64>,
        cash_amount_gte: Option<f64>,
        dividend_type: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Dividend>> {
        let path = "/v3/reference/dividends".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = ticker {
            params.push(("ticker", v.to_string()));
        }
        if let Some(v) = ticker_lt {
            params.push(("ticker.lt", v.to_string()));
        }
        if let Some(v) = ticker_lte {
            params.push(("ticker.lte", v.to_string()));
        }
        if let Some(v) = ticker_gt {
            params.push(("ticker.gt", v.to_string()));
        }
        if let Some(v) = ticker_gte {
            params.push(("ticker.gte", v.to_string()));
        }
        if let Some(v) = ex_dividend_date {
            params.push(("ex_dividend_date", v.to_string()));
        }
        if let Some(v) = ex_dividend_date_lt {
            params.push(("ex_dividend_date.lt", v.to_string()));
        }
        if let Some(v) = ex_dividend_date_lte {
            params.push(("ex_dividend_date.lte", v.to_string()));
        }
        if let Some(v) = ex_dividend_date_gt {
            params.push(("ex_dividend_date.gt", v.to_string()));
        }
        if let Some(v) = ex_dividend_date_gte {
            params.push(("ex_dividend_date.gte", v.to_string()));
        }
        if let Some(v) = record_date {
            params.push(("record_date", v.to_string()));
        }
        if let Some(v) = record_date_lt {
            params.push(("record_date.lt", v.to_string()));
        }
        if let Some(v) = record_date_lte {
            params.push(("record_date.lte", v.to_string()));
        }
        if let Some(v) = record_date_gt {
            params.push(("record_date.gt", v.to_string()));
        }
        if let Some(v) = record_date_gte {
            params.push(("record_date.gte", v.to_string()));
        }
        if let Some(v) = declaration_date {
            params.push(("declaration_date", v.to_string()));
        }
        if let Some(v) = declaration_date_lt {
            params.push(("declaration_date.lt", v.to_string()));
        }
        if let Some(v) = declaration_date_lte {
            params.push(("declaration_date.lte", v.to_string()));
        }
        if let Some(v) = declaration_date_gt {
            params.push(("declaration_date.gt", v.to_string()));
        }
        if let Some(v) = declaration_date_gte {
            params.push(("declaration_date.gte", v.to_string()));
        }
        if let Some(v) = pay_date {
            params.push(("pay_date", v.to_string()));
        }
        if let Some(v) = pay_date_lt {
            params.push(("pay_date.lt", v.to_string()));
        }
        if let Some(v) = pay_date_lte {
            params.push(("pay_date.lte", v.to_string()));
        }
        if let Some(v) = pay_date_gt {
            params.push(("pay_date.gt", v.to_string()));
        }
        if let Some(v) = pay_date_gte {
            params.push(("pay_date.gte", v.to_string()));
        }
        if let Some(v) = frequency {
            params.push(("frequency", v.to_string()));
        }
        if let Some(v) = cash_amount {
            params.push(("cash_amount", v.to_string()));
        }
        if let Some(v) = cash_amount_lt {
            params.push(("cash_amount.lt", v.to_string()));
        }
        if let Some(v) = cash_amount_lte {
            params.push(("cash_amount.lte", v.to_string()));
        }
        if let Some(v) = cash_amount_gt {
            params.push(("cash_amount.gt", v.to_string()));
        }
        if let Some(v) = cash_amount_gte {
            params.push(("cash_amount.gte", v.to_string()));
        }
        if let Some(v) = dividend_type {
            params.push(("dividend_type", v.to_string()));
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
            self.paginate::<Dividend>(&path, Some(&params), options)
        } else {
            self.single_page::<Dividend>(&path, Some(&params), options)
        }
    }

    fn list_conditions(
        &self,
        asset_class: Option<&str>,
        data_type: Option<&str>,
        id: Option<i64>,
        sip: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Condition>> {
        let path = "/v3/reference/conditions".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = asset_class {
            params.push(("asset_class", v.to_string()));
        }
        if let Some(v) = data_type {
            params.push(("data_type", v.to_string()));
        }
        if let Some(v) = id {
            params.push(("id", v.to_string()));
        }
        if let Some(v) = sip {
            params.push(("sip", v.to_string()));
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
            self.paginate::<Condition>(&path, Some(&params), options)
        } else {
            self.single_page::<Condition>(&path, Some(&params), options)
        }
    }

    async fn get_exchanges(
        &self,
        asset_class: Option<&str>,
        locale: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<Exchange>> {
        let path = "/v3/reference/exchanges".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = asset_class {
            params.push(("asset_class", v.to_string()));
        }
        if let Some(v) = locale {
            params.push(("locale", v.to_string()));
        }
        #[derive(serde::Deserialize)]
        struct Resp {
            results: Option<Vec<Exchange>>,
        }
        let resp: Resp = self.get(&path, Some(&params), options).await?;
        Ok(resp.results.unwrap_or_default())
    }

    async fn get_options_contract(
        &self,
        ticker: &str,
        as_of: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<OptionsContract> {
        let path = format!("/v3/reference/options/contracts/{}", ticker);
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = as_of {
            params.push(("as_of", v.to_string()));
        }
        #[derive(serde::Deserialize)]
        struct Resp {
            results: OptionsContract,
        }
        let resp: Resp = self.get(&path, Some(&params), options).await?;
        Ok(resp.results)
    }

    fn list_options_contracts(
        &self,
        underlying_ticker: Option<&str>,
        underlying_ticker_lt: Option<&str>,
        underlying_ticker_lte: Option<&str>,
        underlying_ticker_gt: Option<&str>,
        underlying_ticker_gte: Option<&str>,
        contract_type: Option<&str>,
        expiration_date: Option<&str>,
        expiration_date_lt: Option<&str>,
        expiration_date_lte: Option<&str>,
        expiration_date_gt: Option<&str>,
        expiration_date_gte: Option<&str>,
        as_of: Option<&str>,
        strike_price: Option<f64>,
        strike_price_lt: Option<f64>,
        strike_price_lte: Option<f64>,
        strike_price_gt: Option<f64>,
        strike_price_gte: Option<f64>,
        expired: Option<bool>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<OptionsContract>> {
        let path = "/v3/reference/options/contracts".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = underlying_ticker {
            params.push(("underlying_ticker", v.to_string()));
        }
        if let Some(v) = underlying_ticker_lt {
            params.push(("underlying_ticker.lt", v.to_string()));
        }
        if let Some(v) = underlying_ticker_lte {
            params.push(("underlying_ticker.lte", v.to_string()));
        }
        if let Some(v) = underlying_ticker_gt {
            params.push(("underlying_ticker.gt", v.to_string()));
        }
        if let Some(v) = underlying_ticker_gte {
            params.push(("underlying_ticker.gte", v.to_string()));
        }
        if let Some(v) = contract_type {
            params.push(("contract_type", v.to_string()));
        }
        if let Some(v) = expiration_date {
            params.push(("expiration_date", v.to_string()));
        }
        if let Some(v) = expiration_date_lt {
            params.push(("expiration_date.lt", v.to_string()));
        }
        if let Some(v) = expiration_date_lte {
            params.push(("expiration_date.lte", v.to_string()));
        }
        if let Some(v) = expiration_date_gt {
            params.push(("expiration_date.gt", v.to_string()));
        }
        if let Some(v) = expiration_date_gte {
            params.push(("expiration_date.gte", v.to_string()));
        }
        if let Some(v) = as_of {
            params.push(("as_of", v.to_string()));
        }
        if let Some(v) = strike_price {
            params.push(("strike_price", v.to_string()));
        }
        if let Some(v) = strike_price_lt {
            params.push(("strike_price.lt", v.to_string()));
        }
        if let Some(v) = strike_price_lte {
            params.push(("strike_price.lte", v.to_string()));
        }
        if let Some(v) = strike_price_gt {
            params.push(("strike_price.gt", v.to_string()));
        }
        if let Some(v) = strike_price_gte {
            params.push(("strike_price.gte", v.to_string()));
        }
        if let Some(v) = expired {
            params.push(("expired", v.to_string()));
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
            self.paginate::<OptionsContract>(&path, Some(&params), options)
        } else {
            self.single_page::<OptionsContract>(&path, Some(&params), options)
        }
    }

    fn list_short_interest(
        &self,
        ticker: Option<&str>,
        days_to_cover: Option<&str>,
        days_to_cover_lt: Option<&str>,
        days_to_cover_lte: Option<&str>,
        days_to_cover_gt: Option<&str>,
        days_to_cover_gte: Option<&str>,
        settlement_date: Option<&str>,
        settlement_date_lt: Option<&str>,
        settlement_date_lte: Option<&str>,
        settlement_date_gt: Option<&str>,
        settlement_date_gte: Option<&str>,
        avg_daily_volume: Option<&str>,
        avg_daily_volume_lt: Option<&str>,
        avg_daily_volume_lte: Option<&str>,
        avg_daily_volume_gt: Option<&str>,
        avg_daily_volume_gte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<ShortInterest>> {
        let path = "/stocks/v1/short-interest".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = ticker {
            params.push(("ticker", v.to_string()));
        }
        if let Some(v) = days_to_cover {
            params.push(("days_to_cover", v.to_string()));
        }
        if let Some(v) = days_to_cover_lt {
            params.push(("days_to_cover.lt", v.to_string()));
        }
        if let Some(v) = days_to_cover_lte {
            params.push(("days_to_cover.lte", v.to_string()));
        }
        if let Some(v) = days_to_cover_gt {
            params.push(("days_to_cover.gt", v.to_string()));
        }
        if let Some(v) = days_to_cover_gte {
            params.push(("days_to_cover.gte", v.to_string()));
        }
        if let Some(v) = settlement_date {
            params.push(("settlement_date", v.to_string()));
        }
        if let Some(v) = settlement_date_lt {
            params.push(("settlement_date.lt", v.to_string()));
        }
        if let Some(v) = settlement_date_lte {
            params.push(("settlement_date.lte", v.to_string()));
        }
        if let Some(v) = settlement_date_gt {
            params.push(("settlement_date.gt", v.to_string()));
        }
        if let Some(v) = settlement_date_gte {
            params.push(("settlement_date.gte", v.to_string()));
        }
        if let Some(v) = avg_daily_volume {
            params.push(("avg_daily_volume", v.to_string()));
        }
        if let Some(v) = avg_daily_volume_lt {
            params.push(("avg_daily_volume.lt", v.to_string()));
        }
        if let Some(v) = avg_daily_volume_lte {
            params.push(("avg_daily_volume.lte", v.to_string()));
        }
        if let Some(v) = avg_daily_volume_gt {
            params.push(("avg_daily_volume.gt", v.to_string()));
        }
        if let Some(v) = avg_daily_volume_gte {
            params.push(("avg_daily_volume.gte", v.to_string()));
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
            self.paginate::<ShortInterest>(&path, Some(&params), options)
        } else {
            self.single_page::<ShortInterest>(&path, Some(&params), options)
        }
    }

    fn list_short_volume(
        &self,
        ticker: Option<&str>,
        date: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        short_volume_ratio: Option<&str>,
        short_volume_ratio_lt: Option<&str>,
        short_volume_ratio_lte: Option<&str>,
        short_volume_ratio_gt: Option<&str>,
        short_volume_ratio_gte: Option<&str>,
        total_volume: Option<&str>,
        total_volume_lt: Option<&str>,
        total_volume_lte: Option<&str>,
        total_volume_gt: Option<&str>,
        total_volume_gte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<ShortVolume>> {
        let path = "/stocks/v1/short-volume".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = ticker {
            params.push(("ticker", v.to_string()));
        }
        if let Some(v) = date {
            params.push(("date", v.to_string()));
        }
        if let Some(v) = date_lt {
            params.push(("date.lt", v.to_string()));
        }
        if let Some(v) = date_lte {
            params.push(("date.lte", v.to_string()));
        }
        if let Some(v) = date_gt {
            params.push(("date.gt", v.to_string()));
        }
        if let Some(v) = date_gte {
            params.push(("date.gte", v.to_string()));
        }
        if let Some(v) = short_volume_ratio {
            params.push(("short_volume_ratio", v.to_string()));
        }
        if let Some(v) = short_volume_ratio_lt {
            params.push(("short_volume_ratio.lt", v.to_string()));
        }
        if let Some(v) = short_volume_ratio_lte {
            params.push(("short_volume_ratio.lte", v.to_string()));
        }
        if let Some(v) = short_volume_ratio_gt {
            params.push(("short_volume_ratio.gt", v.to_string()));
        }
        if let Some(v) = short_volume_ratio_gte {
            params.push(("short_volume_ratio.gte", v.to_string()));
        }
        if let Some(v) = total_volume {
            params.push(("total_volume", v.to_string()));
        }
        if let Some(v) = total_volume_lt {
            params.push(("total_volume.lt", v.to_string()));
        }
        if let Some(v) = total_volume_lte {
            params.push(("total_volume.lte", v.to_string()));
        }
        if let Some(v) = total_volume_gt {
            params.push(("total_volume.gt", v.to_string()));
        }
        if let Some(v) = total_volume_gte {
            params.push(("total_volume.gte", v.to_string()));
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
            self.paginate::<ShortVolume>(&path, Some(&params), options)
        } else {
            self.single_page::<ShortVolume>(&path, Some(&params), options)
        }
    }

    fn list_stocks_splits(
        &self,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        execution_date: Option<&str>,
        execution_date_gt: Option<&str>,
        execution_date_gte: Option<&str>,
        execution_date_lt: Option<&str>,
        execution_date_lte: Option<&str>,
        adjustment_type: Option<&str>,
        adjustment_type_any_of: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<StockSplit>> {
        let path = "/stocks/v1/splits".to_string();
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
        if let Some(v) = execution_date {
            params.push(("execution_date", v.to_string()));
        }
        if let Some(v) = execution_date_gt {
            params.push(("execution_date.gt", v.to_string()));
        }
        if let Some(v) = execution_date_gte {
            params.push(("execution_date.gte", v.to_string()));
        }
        if let Some(v) = execution_date_lt {
            params.push(("execution_date.lt", v.to_string()));
        }
        if let Some(v) = execution_date_lte {
            params.push(("execution_date.lte", v.to_string()));
        }
        if let Some(v) = adjustment_type {
            params.push(("adjustment_type", v.to_string()));
        }
        if let Some(v) = adjustment_type_any_of {
            params.push(("adjustment_type.any_of", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<StockSplit>(&path, Some(&params), options)
        } else {
            self.single_page::<StockSplit>(&path, Some(&params), options)
        }
    }

    fn list_stocks_dividends(
        &self,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        ex_dividend_date: Option<&str>,
        ex_dividend_date_gt: Option<&str>,
        ex_dividend_date_gte: Option<&str>,
        ex_dividend_date_lt: Option<&str>,
        ex_dividend_date_lte: Option<&str>,
        frequency: Option<i64>,
        frequency_gt: Option<i64>,
        frequency_gte: Option<i64>,
        frequency_lt: Option<i64>,
        frequency_lte: Option<i64>,
        distribution_type: Option<&str>,
        distribution_type_any_of: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<StockDividend>> {
        let path = "/stocks/v1/dividends".to_string();
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
        if let Some(v) = ex_dividend_date {
            params.push(("ex_dividend_date", v.to_string()));
        }
        if let Some(v) = ex_dividend_date_gt {
            params.push(("ex_dividend_date.gt", v.to_string()));
        }
        if let Some(v) = ex_dividend_date_gte {
            params.push(("ex_dividend_date.gte", v.to_string()));
        }
        if let Some(v) = ex_dividend_date_lt {
            params.push(("ex_dividend_date.lt", v.to_string()));
        }
        if let Some(v) = ex_dividend_date_lte {
            params.push(("ex_dividend_date.lte", v.to_string()));
        }
        if let Some(v) = frequency {
            params.push(("frequency", v.to_string()));
        }
        if let Some(v) = frequency_gt {
            params.push(("frequency.gt", v.to_string()));
        }
        if let Some(v) = frequency_gte {
            params.push(("frequency.gte", v.to_string()));
        }
        if let Some(v) = frequency_lt {
            params.push(("frequency.lt", v.to_string()));
        }
        if let Some(v) = frequency_lte {
            params.push(("frequency.lte", v.to_string()));
        }
        if let Some(v) = distribution_type {
            params.push(("distribution_type", v.to_string()));
        }
        if let Some(v) = distribution_type_any_of {
            params.push(("distribution_type.any_of", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<StockDividend>(&path, Some(&params), options)
        } else {
            self.single_page::<StockDividend>(&path, Some(&params), options)
        }
    }

    fn list_stocks_filings_risk_factors(
        &self,
        filing_date: Option<&str>,
        filing_date_any_of: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        cik: Option<&str>,
        cik_any_of: Option<&str>,
        cik_gt: Option<&str>,
        cik_gte: Option<&str>,
        cik_lt: Option<&str>,
        cik_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<RiskFactor>> {
        let path = "/stocks/filings/vX/risk-factors".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = filing_date {
            params.push(("filing_date", v.to_string()));
        }
        if let Some(v) = filing_date_any_of {
            params.push(("filing_date.any_of", v.to_string()));
        }
        if let Some(v) = filing_date_gt {
            params.push(("filing_date.gt", v.to_string()));
        }
        if let Some(v) = filing_date_gte {
            params.push(("filing_date.gte", v.to_string()));
        }
        if let Some(v) = filing_date_lt {
            params.push(("filing_date.lt", v.to_string()));
        }
        if let Some(v) = filing_date_lte {
            params.push(("filing_date.lte", v.to_string()));
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
        if let Some(v) = cik {
            params.push(("cik", v.to_string()));
        }
        if let Some(v) = cik_any_of {
            params.push(("cik.any_of", v.to_string()));
        }
        if let Some(v) = cik_gt {
            params.push(("cik.gt", v.to_string()));
        }
        if let Some(v) = cik_gte {
            params.push(("cik.gte", v.to_string()));
        }
        if let Some(v) = cik_lt {
            params.push(("cik.lt", v.to_string()));
        }
        if let Some(v) = cik_lte {
            params.push(("cik.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<RiskFactor>(&path, Some(&params), options)
        } else {
            self.single_page::<RiskFactor>(&path, Some(&params), options)
        }
    }

    fn list_stocks_taxonomies_risk_factors(
        &self,
        taxonomy: Option<f64>,
        taxonomy_gt: Option<f64>,
        taxonomy_gte: Option<f64>,
        taxonomy_lt: Option<f64>,
        taxonomy_lte: Option<f64>,
        primary_category: Option<&str>,
        primary_category_any_of: Option<&str>,
        primary_category_gt: Option<&str>,
        primary_category_gte: Option<&str>,
        primary_category_lt: Option<&str>,
        primary_category_lte: Option<&str>,
        secondary_category: Option<&str>,
        secondary_category_any_of: Option<&str>,
        secondary_category_gt: Option<&str>,
        secondary_category_gte: Option<&str>,
        secondary_category_lt: Option<&str>,
        secondary_category_lte: Option<&str>,
        tertiary_category: Option<&str>,
        tertiary_category_any_of: Option<&str>,
        tertiary_category_gt: Option<&str>,
        tertiary_category_gte: Option<&str>,
        tertiary_category_lt: Option<&str>,
        tertiary_category_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<RiskFactorTaxonomy>> {
        let path = "/stocks/taxonomies/vX/risk-factors".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = taxonomy {
            params.push(("taxonomy", v.to_string()));
        }
        if let Some(v) = taxonomy_gt {
            params.push(("taxonomy.gt", v.to_string()));
        }
        if let Some(v) = taxonomy_gte {
            params.push(("taxonomy.gte", v.to_string()));
        }
        if let Some(v) = taxonomy_lt {
            params.push(("taxonomy.lt", v.to_string()));
        }
        if let Some(v) = taxonomy_lte {
            params.push(("taxonomy.lte", v.to_string()));
        }
        if let Some(v) = primary_category {
            params.push(("primary_category", v.to_string()));
        }
        if let Some(v) = primary_category_any_of {
            params.push(("primary_category.any_of", v.to_string()));
        }
        if let Some(v) = primary_category_gt {
            params.push(("primary_category.gt", v.to_string()));
        }
        if let Some(v) = primary_category_gte {
            params.push(("primary_category.gte", v.to_string()));
        }
        if let Some(v) = primary_category_lt {
            params.push(("primary_category.lt", v.to_string()));
        }
        if let Some(v) = primary_category_lte {
            params.push(("primary_category.lte", v.to_string()));
        }
        if let Some(v) = secondary_category {
            params.push(("secondary_category", v.to_string()));
        }
        if let Some(v) = secondary_category_any_of {
            params.push(("secondary_category.any_of", v.to_string()));
        }
        if let Some(v) = secondary_category_gt {
            params.push(("secondary_category.gt", v.to_string()));
        }
        if let Some(v) = secondary_category_gte {
            params.push(("secondary_category.gte", v.to_string()));
        }
        if let Some(v) = secondary_category_lt {
            params.push(("secondary_category.lt", v.to_string()));
        }
        if let Some(v) = secondary_category_lte {
            params.push(("secondary_category.lte", v.to_string()));
        }
        if let Some(v) = tertiary_category {
            params.push(("tertiary_category", v.to_string()));
        }
        if let Some(v) = tertiary_category_any_of {
            params.push(("tertiary_category.any_of", v.to_string()));
        }
        if let Some(v) = tertiary_category_gt {
            params.push(("tertiary_category.gt", v.to_string()));
        }
        if let Some(v) = tertiary_category_gte {
            params.push(("tertiary_category.gte", v.to_string()));
        }
        if let Some(v) = tertiary_category_lt {
            params.push(("tertiary_category.lt", v.to_string()));
        }
        if let Some(v) = tertiary_category_lte {
            params.push(("tertiary_category.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<RiskFactorTaxonomy>(&path, Some(&params), options)
        } else {
            self.single_page::<RiskFactorTaxonomy>(&path, Some(&params), options)
        }
    }

    fn list_stocks_filings_8k_disclosures(
        &self,
        cik: Option<&str>,
        cik_any_of: Option<&str>,
        tickers: Option<&str>,
        tickers_all_of: Option<&str>,
        tickers_any_of: Option<&str>,
        filing_date: Option<&str>,
        filing_date_any_of: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        tertiary_category: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Disclosure>> {
        let path = "/stocks/filings/8-K/vX/disclosures".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = cik {
            params.push(("cik", v.to_string()));
        }
        if let Some(v) = cik_any_of {
            params.push(("cik.any_of", v.to_string()));
        }
        if let Some(v) = tickers {
            params.push(("tickers", v.to_string()));
        }
        if let Some(v) = tickers_all_of {
            params.push(("tickers.all_of", v.to_string()));
        }
        if let Some(v) = tickers_any_of {
            params.push(("tickers.any_of", v.to_string()));
        }
        if let Some(v) = filing_date {
            params.push(("filing_date", v.to_string()));
        }
        if let Some(v) = filing_date_any_of {
            params.push(("filing_date.any_of", v.to_string()));
        }
        if let Some(v) = filing_date_gt {
            params.push(("filing_date.gt", v.to_string()));
        }
        if let Some(v) = filing_date_gte {
            params.push(("filing_date.gte", v.to_string()));
        }
        if let Some(v) = filing_date_lt {
            params.push(("filing_date.lt", v.to_string()));
        }
        if let Some(v) = filing_date_lte {
            params.push(("filing_date.lte", v.to_string()));
        }
        if let Some(v) = tertiary_category {
            params.push(("tertiary_category", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<Disclosure>(&path, Some(&params), options)
        } else {
            self.single_page::<Disclosure>(&path, Some(&params), options)
        }
    }

    fn list_stocks_taxonomies_disclosures(
        &self,
        taxonomy: Option<&str>,
        taxonomy_any_of: Option<&str>,
        taxonomy_gt: Option<&str>,
        taxonomy_gte: Option<&str>,
        taxonomy_lt: Option<&str>,
        taxonomy_lte: Option<&str>,
        primary_category: Option<&str>,
        primary_category_any_of: Option<&str>,
        primary_category_gt: Option<&str>,
        primary_category_gte: Option<&str>,
        primary_category_lt: Option<&str>,
        primary_category_lte: Option<&str>,
        secondary_category: Option<&str>,
        secondary_category_any_of: Option<&str>,
        secondary_category_gt: Option<&str>,
        secondary_category_gte: Option<&str>,
        secondary_category_lt: Option<&str>,
        secondary_category_lte: Option<&str>,
        tertiary_category: Option<&str>,
        tertiary_category_any_of: Option<&str>,
        tertiary_category_gt: Option<&str>,
        tertiary_category_gte: Option<&str>,
        tertiary_category_lt: Option<&str>,
        tertiary_category_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<DisclosureTaxonomy>> {
        let path = "/stocks/taxonomies/vX/disclosures".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = taxonomy {
            params.push(("taxonomy", v.to_string()));
        }
        if let Some(v) = taxonomy_any_of {
            params.push(("taxonomy.any_of", v.to_string()));
        }
        if let Some(v) = taxonomy_gt {
            params.push(("taxonomy.gt", v.to_string()));
        }
        if let Some(v) = taxonomy_gte {
            params.push(("taxonomy.gte", v.to_string()));
        }
        if let Some(v) = taxonomy_lt {
            params.push(("taxonomy.lt", v.to_string()));
        }
        if let Some(v) = taxonomy_lte {
            params.push(("taxonomy.lte", v.to_string()));
        }
        if let Some(v) = primary_category {
            params.push(("primary_category", v.to_string()));
        }
        if let Some(v) = primary_category_any_of {
            params.push(("primary_category.any_of", v.to_string()));
        }
        if let Some(v) = primary_category_gt {
            params.push(("primary_category.gt", v.to_string()));
        }
        if let Some(v) = primary_category_gte {
            params.push(("primary_category.gte", v.to_string()));
        }
        if let Some(v) = primary_category_lt {
            params.push(("primary_category.lt", v.to_string()));
        }
        if let Some(v) = primary_category_lte {
            params.push(("primary_category.lte", v.to_string()));
        }
        if let Some(v) = secondary_category {
            params.push(("secondary_category", v.to_string()));
        }
        if let Some(v) = secondary_category_any_of {
            params.push(("secondary_category.any_of", v.to_string()));
        }
        if let Some(v) = secondary_category_gt {
            params.push(("secondary_category.gt", v.to_string()));
        }
        if let Some(v) = secondary_category_gte {
            params.push(("secondary_category.gte", v.to_string()));
        }
        if let Some(v) = secondary_category_lt {
            params.push(("secondary_category.lt", v.to_string()));
        }
        if let Some(v) = secondary_category_lte {
            params.push(("secondary_category.lte", v.to_string()));
        }
        if let Some(v) = tertiary_category {
            params.push(("tertiary_category", v.to_string()));
        }
        if let Some(v) = tertiary_category_any_of {
            params.push(("tertiary_category.any_of", v.to_string()));
        }
        if let Some(v) = tertiary_category_gt {
            params.push(("tertiary_category.gt", v.to_string()));
        }
        if let Some(v) = tertiary_category_gte {
            params.push(("tertiary_category.gte", v.to_string()));
        }
        if let Some(v) = tertiary_category_lt {
            params.push(("tertiary_category.lt", v.to_string()));
        }
        if let Some(v) = tertiary_category_lte {
            params.push(("tertiary_category.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<DisclosureTaxonomy>(&path, Some(&params), options)
        } else {
            self.single_page::<DisclosureTaxonomy>(&path, Some(&params), options)
        }
    }

    fn list_stocks_filings_10k_sections(
        &self,
        cik: Option<&str>,
        cik_any_of: Option<&str>,
        cik_gt: Option<&str>,
        cik_gte: Option<&str>,
        cik_lt: Option<&str>,
        cik_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        section: Option<&str>,
        section_any_of: Option<&str>,
        filing_date: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        period_end: Option<&str>,
        period_end_gt: Option<&str>,
        period_end_gte: Option<&str>,
        period_end_lt: Option<&str>,
        period_end_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FilingSection>> {
        let path = "/stocks/filings/10-K/vX/sections".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = cik {
            params.push(("cik", v.to_string()));
        }
        if let Some(v) = cik_any_of {
            params.push(("cik.any_of", v.to_string()));
        }
        if let Some(v) = cik_gt {
            params.push(("cik.gt", v.to_string()));
        }
        if let Some(v) = cik_gte {
            params.push(("cik.gte", v.to_string()));
        }
        if let Some(v) = cik_lt {
            params.push(("cik.lt", v.to_string()));
        }
        if let Some(v) = cik_lte {
            params.push(("cik.lte", v.to_string()));
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
        if let Some(v) = section {
            params.push(("section", v.to_string()));
        }
        if let Some(v) = section_any_of {
            params.push(("section.any_of", v.to_string()));
        }
        if let Some(v) = filing_date {
            params.push(("filing_date", v.to_string()));
        }
        if let Some(v) = filing_date_gt {
            params.push(("filing_date.gt", v.to_string()));
        }
        if let Some(v) = filing_date_gte {
            params.push(("filing_date.gte", v.to_string()));
        }
        if let Some(v) = filing_date_lt {
            params.push(("filing_date.lt", v.to_string()));
        }
        if let Some(v) = filing_date_lte {
            params.push(("filing_date.lte", v.to_string()));
        }
        if let Some(v) = period_end {
            params.push(("period_end", v.to_string()));
        }
        if let Some(v) = period_end_gt {
            params.push(("period_end.gt", v.to_string()));
        }
        if let Some(v) = period_end_gte {
            params.push(("period_end.gte", v.to_string()));
        }
        if let Some(v) = period_end_lt {
            params.push(("period_end.lt", v.to_string()));
        }
        if let Some(v) = period_end_lte {
            params.push(("period_end.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<FilingSection>(&path, Some(&params), options)
        } else {
            self.single_page::<FilingSection>(&path, Some(&params), options)
        }
    }

    fn list_stocks_filings_8k_text(
        &self,
        cik: Option<&str>,
        cik_any_of: Option<&str>,
        cik_gt: Option<&str>,
        cik_gte: Option<&str>,
        cik_lt: Option<&str>,
        cik_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        form_type: Option<&str>,
        form_type_any_of: Option<&str>,
        form_type_gt: Option<&str>,
        form_type_gte: Option<&str>,
        form_type_lt: Option<&str>,
        form_type_lte: Option<&str>,
        filing_date: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Filing8K>> {
        let path = "/stocks/filings/8-K/vX/text".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = cik {
            params.push(("cik", v.to_string()));
        }
        if let Some(v) = cik_any_of {
            params.push(("cik.any_of", v.to_string()));
        }
        if let Some(v) = cik_gt {
            params.push(("cik.gt", v.to_string()));
        }
        if let Some(v) = cik_gte {
            params.push(("cik.gte", v.to_string()));
        }
        if let Some(v) = cik_lt {
            params.push(("cik.lt", v.to_string()));
        }
        if let Some(v) = cik_lte {
            params.push(("cik.lte", v.to_string()));
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
        if let Some(v) = form_type {
            params.push(("form_type", v.to_string()));
        }
        if let Some(v) = form_type_any_of {
            params.push(("form_type.any_of", v.to_string()));
        }
        if let Some(v) = form_type_gt {
            params.push(("form_type.gt", v.to_string()));
        }
        if let Some(v) = form_type_gte {
            params.push(("form_type.gte", v.to_string()));
        }
        if let Some(v) = form_type_lt {
            params.push(("form_type.lt", v.to_string()));
        }
        if let Some(v) = form_type_lte {
            params.push(("form_type.lte", v.to_string()));
        }
        if let Some(v) = filing_date {
            params.push(("filing_date", v.to_string()));
        }
        if let Some(v) = filing_date_gt {
            params.push(("filing_date.gt", v.to_string()));
        }
        if let Some(v) = filing_date_gte {
            params.push(("filing_date.gte", v.to_string()));
        }
        if let Some(v) = filing_date_lt {
            params.push(("filing_date.lt", v.to_string()));
        }
        if let Some(v) = filing_date_lte {
            params.push(("filing_date.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<Filing8K>(&path, Some(&params), options)
        } else {
            self.single_page::<Filing8K>(&path, Some(&params), options)
        }
    }

    fn list_stocks_filings_index(
        &self,
        cik: Option<&str>,
        cik_any_of: Option<&str>,
        cik_gt: Option<&str>,
        cik_gte: Option<&str>,
        cik_lt: Option<&str>,
        cik_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        form_type: Option<&str>,
        form_type_any_of: Option<&str>,
        form_type_gt: Option<&str>,
        form_type_gte: Option<&str>,
        form_type_lt: Option<&str>,
        form_type_lte: Option<&str>,
        filing_date: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FilingIndex>> {
        let path = "/stocks/filings/vX/index".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = cik {
            params.push(("cik", v.to_string()));
        }
        if let Some(v) = cik_any_of {
            params.push(("cik.any_of", v.to_string()));
        }
        if let Some(v) = cik_gt {
            params.push(("cik.gt", v.to_string()));
        }
        if let Some(v) = cik_gte {
            params.push(("cik.gte", v.to_string()));
        }
        if let Some(v) = cik_lt {
            params.push(("cik.lt", v.to_string()));
        }
        if let Some(v) = cik_lte {
            params.push(("cik.lte", v.to_string()));
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
        if let Some(v) = form_type {
            params.push(("form_type", v.to_string()));
        }
        if let Some(v) = form_type_any_of {
            params.push(("form_type.any_of", v.to_string()));
        }
        if let Some(v) = form_type_gt {
            params.push(("form_type.gt", v.to_string()));
        }
        if let Some(v) = form_type_gte {
            params.push(("form_type.gte", v.to_string()));
        }
        if let Some(v) = form_type_lt {
            params.push(("form_type.lt", v.to_string()));
        }
        if let Some(v) = form_type_lte {
            params.push(("form_type.lte", v.to_string()));
        }
        if let Some(v) = filing_date {
            params.push(("filing_date", v.to_string()));
        }
        if let Some(v) = filing_date_gt {
            params.push(("filing_date.gt", v.to_string()));
        }
        if let Some(v) = filing_date_gte {
            params.push(("filing_date.gte", v.to_string()));
        }
        if let Some(v) = filing_date_lt {
            params.push(("filing_date.lt", v.to_string()));
        }
        if let Some(v) = filing_date_lte {
            params.push(("filing_date.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<FilingIndex>(&path, Some(&params), options)
        } else {
            self.single_page::<FilingIndex>(&path, Some(&params), options)
        }
    }

    fn list_stocks_filings_13f(
        &self,
        filer_cik: Option<&str>,
        filer_cik_any_of: Option<&str>,
        filing_date: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Filing13F>> {
        let path = "/stocks/filings/vX/13-F".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = filer_cik {
            params.push(("filer_cik", v.to_string()));
        }
        if let Some(v) = filer_cik_any_of {
            params.push(("filer_cik.any_of", v.to_string()));
        }
        if let Some(v) = filing_date {
            params.push(("filing_date", v.to_string()));
        }
        if let Some(v) = filing_date_gt {
            params.push(("filing_date.gt", v.to_string()));
        }
        if let Some(v) = filing_date_gte {
            params.push(("filing_date.gte", v.to_string()));
        }
        if let Some(v) = filing_date_lt {
            params.push(("filing_date.lt", v.to_string()));
        }
        if let Some(v) = filing_date_lte {
            params.push(("filing_date.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<Filing13F>(&path, Some(&params), options)
        } else {
            self.single_page::<Filing13F>(&path, Some(&params), options)
        }
    }

    fn list_stocks_filings_form_3(
        &self,
        issuer_cik: Option<&str>,
        issuer_cik_any_of: Option<&str>,
        owner_cik: Option<&str>,
        owner_cik_any_of: Option<&str>,
        tickers: Option<&str>,
        tickers_all_of: Option<&str>,
        tickers_any_of: Option<&str>,
        form_type: Option<&str>,
        filing_date: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        max_ticker: Option<&str>,
        max_ticker_any_of: Option<&str>,
        max_ticker_gt: Option<&str>,
        max_ticker_gte: Option<&str>,
        max_ticker_lt: Option<&str>,
        max_ticker_lte: Option<&str>,
        min_ticker: Option<&str>,
        min_ticker_any_of: Option<&str>,
        min_ticker_gt: Option<&str>,
        min_ticker_gte: Option<&str>,
        min_ticker_lt: Option<&str>,
        min_ticker_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FilingForm3>> {
        let path = "/stocks/filings/vX/form-3".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = issuer_cik {
            params.push(("issuer_cik", v.to_string()));
        }
        if let Some(v) = issuer_cik_any_of {
            params.push(("issuer_cik.any_of", v.to_string()));
        }
        if let Some(v) = owner_cik {
            params.push(("owner_cik", v.to_string()));
        }
        if let Some(v) = owner_cik_any_of {
            params.push(("owner_cik.any_of", v.to_string()));
        }
        if let Some(v) = tickers {
            params.push(("tickers", v.to_string()));
        }
        if let Some(v) = tickers_all_of {
            params.push(("tickers.all_of", v.to_string()));
        }
        if let Some(v) = tickers_any_of {
            params.push(("tickers.any_of", v.to_string()));
        }
        if let Some(v) = form_type {
            params.push(("form_type", v.to_string()));
        }
        if let Some(v) = filing_date {
            params.push(("filing_date", v.to_string()));
        }
        if let Some(v) = filing_date_gt {
            params.push(("filing_date.gt", v.to_string()));
        }
        if let Some(v) = filing_date_gte {
            params.push(("filing_date.gte", v.to_string()));
        }
        if let Some(v) = filing_date_lt {
            params.push(("filing_date.lt", v.to_string()));
        }
        if let Some(v) = filing_date_lte {
            params.push(("filing_date.lte", v.to_string()));
        }
        if let Some(v) = max_ticker {
            params.push(("max_ticker", v.to_string()));
        }
        if let Some(v) = max_ticker_any_of {
            params.push(("max_ticker.any_of", v.to_string()));
        }
        if let Some(v) = max_ticker_gt {
            params.push(("max_ticker.gt", v.to_string()));
        }
        if let Some(v) = max_ticker_gte {
            params.push(("max_ticker.gte", v.to_string()));
        }
        if let Some(v) = max_ticker_lt {
            params.push(("max_ticker.lt", v.to_string()));
        }
        if let Some(v) = max_ticker_lte {
            params.push(("max_ticker.lte", v.to_string()));
        }
        if let Some(v) = min_ticker {
            params.push(("min_ticker", v.to_string()));
        }
        if let Some(v) = min_ticker_any_of {
            params.push(("min_ticker.any_of", v.to_string()));
        }
        if let Some(v) = min_ticker_gt {
            params.push(("min_ticker.gt", v.to_string()));
        }
        if let Some(v) = min_ticker_gte {
            params.push(("min_ticker.gte", v.to_string()));
        }
        if let Some(v) = min_ticker_lt {
            params.push(("min_ticker.lt", v.to_string()));
        }
        if let Some(v) = min_ticker_lte {
            params.push(("min_ticker.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<FilingForm3>(&path, Some(&params), options)
        } else {
            self.single_page::<FilingForm3>(&path, Some(&params), options)
        }
    }

    fn list_stocks_filings_form_4(
        &self,
        issuer_cik: Option<&str>,
        issuer_cik_any_of: Option<&str>,
        owner_cik: Option<&str>,
        owner_cik_any_of: Option<&str>,
        tickers: Option<&str>,
        tickers_all_of: Option<&str>,
        tickers_any_of: Option<&str>,
        form_type: Option<&str>,
        transaction_code: Option<&str>,
        filing_date: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        max_ticker: Option<&str>,
        max_ticker_any_of: Option<&str>,
        max_ticker_gt: Option<&str>,
        max_ticker_gte: Option<&str>,
        max_ticker_lt: Option<&str>,
        max_ticker_lte: Option<&str>,
        min_ticker: Option<&str>,
        min_ticker_any_of: Option<&str>,
        min_ticker_gt: Option<&str>,
        min_ticker_gte: Option<&str>,
        min_ticker_lt: Option<&str>,
        min_ticker_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FilingForm4>> {
        let path = "/stocks/filings/vX/form-4".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(v) = issuer_cik {
            params.push(("issuer_cik", v.to_string()));
        }
        if let Some(v) = issuer_cik_any_of {
            params.push(("issuer_cik.any_of", v.to_string()));
        }
        if let Some(v) = owner_cik {
            params.push(("owner_cik", v.to_string()));
        }
        if let Some(v) = owner_cik_any_of {
            params.push(("owner_cik.any_of", v.to_string()));
        }
        if let Some(v) = tickers {
            params.push(("tickers", v.to_string()));
        }
        if let Some(v) = tickers_all_of {
            params.push(("tickers.all_of", v.to_string()));
        }
        if let Some(v) = tickers_any_of {
            params.push(("tickers.any_of", v.to_string()));
        }
        if let Some(v) = form_type {
            params.push(("form_type", v.to_string()));
        }
        if let Some(v) = transaction_code {
            params.push(("transaction_code", v.to_string()));
        }
        if let Some(v) = filing_date {
            params.push(("filing_date", v.to_string()));
        }
        if let Some(v) = filing_date_gt {
            params.push(("filing_date.gt", v.to_string()));
        }
        if let Some(v) = filing_date_gte {
            params.push(("filing_date.gte", v.to_string()));
        }
        if let Some(v) = filing_date_lt {
            params.push(("filing_date.lt", v.to_string()));
        }
        if let Some(v) = filing_date_lte {
            params.push(("filing_date.lte", v.to_string()));
        }
        if let Some(v) = max_ticker {
            params.push(("max_ticker", v.to_string()));
        }
        if let Some(v) = max_ticker_any_of {
            params.push(("max_ticker.any_of", v.to_string()));
        }
        if let Some(v) = max_ticker_gt {
            params.push(("max_ticker.gt", v.to_string()));
        }
        if let Some(v) = max_ticker_gte {
            params.push(("max_ticker.gte", v.to_string()));
        }
        if let Some(v) = max_ticker_lt {
            params.push(("max_ticker.lt", v.to_string()));
        }
        if let Some(v) = max_ticker_lte {
            params.push(("max_ticker.lte", v.to_string()));
        }
        if let Some(v) = min_ticker {
            params.push(("min_ticker", v.to_string()));
        }
        if let Some(v) = min_ticker_any_of {
            params.push(("min_ticker.any_of", v.to_string()));
        }
        if let Some(v) = min_ticker_gt {
            params.push(("min_ticker.gt", v.to_string()));
        }
        if let Some(v) = min_ticker_gte {
            params.push(("min_ticker.gte", v.to_string()));
        }
        if let Some(v) = min_ticker_lt {
            params.push(("min_ticker.lt", v.to_string()));
        }
        if let Some(v) = min_ticker_lte {
            params.push(("min_ticker.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<FilingForm4>(&path, Some(&params), options)
        } else {
            self.single_page::<FilingForm4>(&path, Some(&params), options)
        }
    }
}
