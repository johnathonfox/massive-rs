use super::{encode_query, BoxStream};
use crate::client::{Client, RequestOptions};
use crate::models::{IPOListing, StockFinancial};

/// Experimental (vX) API: stock financials and IPO listings.
pub trait VxApi {
    /// Get historical financial data for a stock ticker (paginated stream).
    fn list_stock_financials<'a>(
        &'a self,
        ticker: Option<&'a str>,
        cik: Option<&'a str>,
        company_name: Option<&'a str>,
        company_name_search: Option<&'a str>,
        sic: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        period_of_report_date: Option<&'a str>,
        period_of_report_date_lt: Option<&'a str>,
        period_of_report_date_lte: Option<&'a str>,
        period_of_report_date_gt: Option<&'a str>,
        period_of_report_date_gte: Option<&'a str>,
        timeframe: Option<&'a str>,
        include_sources: Option<bool>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, StockFinancial>;

    /// Same as [`Self::list_stock_financials`], but takes the optional arguments as a
    /// chainable [`ListStockFinancialsParams`] struct.
    fn list_stock_financials_with_params<'a>(
        &'a self,
        params: ListStockFinancialsParams,
    ) -> BoxStream<'a, StockFinancial>;

    /// Retrieve upcoming or historical IPOs (paginated stream).
    fn list_ipos<'a>(
        &'a self,
        ticker: Option<&'a str>,
        us_code: Option<&'a str>,
        isin: Option<&'a str>,
        listing_date: Option<&'a str>,
        listing_date_lt: Option<&'a str>,
        listing_date_lte: Option<&'a str>,
        listing_date_gt: Option<&'a str>,
        listing_date_gte: Option<&'a str>,
        ipo_status: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, IPOListing>;

    /// Same as [`Self::list_ipos`], but takes the optional arguments as a
    /// chainable [`ListIposParams`] struct.
    fn list_ipos_with_params<'a>(&'a self, params: ListIposParams) -> BoxStream<'a, IPOListing>;
}

impl VxApi for Client {
    fn list_stock_financials<'a>(
        &'a self,
        ticker: Option<&'a str>,
        cik: Option<&'a str>,
        company_name: Option<&'a str>,
        company_name_search: Option<&'a str>,
        sic: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        period_of_report_date: Option<&'a str>,
        period_of_report_date_lt: Option<&'a str>,
        period_of_report_date_lte: Option<&'a str>,
        period_of_report_date_gt: Option<&'a str>,
        period_of_report_date_gte: Option<&'a str>,
        timeframe: Option<&'a str>,
        include_sources: Option<bool>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, StockFinancial> {
        self.list_stock_financials_with_params(ListStockFinancialsParams {
            ticker: ticker.map(String::from),
            cik: cik.map(String::from),
            company_name: company_name.map(String::from),
            company_name_search: company_name_search.map(String::from),
            sic: sic.map(String::from),
            filing_date: filing_date.map(String::from),
            filing_date_lt: filing_date_lt.map(String::from),
            filing_date_lte: filing_date_lte.map(String::from),
            filing_date_gt: filing_date_gt.map(String::from),
            filing_date_gte: filing_date_gte.map(String::from),
            period_of_report_date: period_of_report_date.map(String::from),
            period_of_report_date_lt: period_of_report_date_lt.map(String::from),
            period_of_report_date_lte: period_of_report_date_lte.map(String::from),
            period_of_report_date_gt: period_of_report_date_gt.map(String::from),
            period_of_report_date_gte: period_of_report_date_gte.map(String::from),
            timeframe: timeframe.map(String::from),
            include_sources,
            limit,
            sort: sort.map(String::from),
            order: order.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_stock_financials_with_params<'a>(
        &'a self,
        params: ListStockFinancialsParams,
    ) -> BoxStream<'a, StockFinancial> {
        Box::pin({
            let path = "/vX/reference/financials".to_string();
            let query = encode_query(&params);
            self.list::<StockFinancial>(&path, &query, params.options.as_ref())
        })
    }

    fn list_ipos<'a>(
        &'a self,
        ticker: Option<&'a str>,
        us_code: Option<&'a str>,
        isin: Option<&'a str>,
        listing_date: Option<&'a str>,
        listing_date_lt: Option<&'a str>,
        listing_date_lte: Option<&'a str>,
        listing_date_gt: Option<&'a str>,
        listing_date_gte: Option<&'a str>,
        ipo_status: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        order: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, IPOListing> {
        self.list_ipos_with_params(ListIposParams {
            ticker: ticker.map(String::from),
            us_code: us_code.map(String::from),
            isin: isin.map(String::from),
            listing_date: listing_date.map(String::from),
            listing_date_lt: listing_date_lt.map(String::from),
            listing_date_lte: listing_date_lte.map(String::from),
            listing_date_gt: listing_date_gt.map(String::from),
            listing_date_gte: listing_date_gte.map(String::from),
            ipo_status: ipo_status.map(String::from),
            limit,
            sort: sort.map(String::from),
            order: order.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_ipos_with_params<'a>(&'a self, params: ListIposParams) -> BoxStream<'a, IPOListing> {
        Box::pin({
            let path = "/vX/reference/ipos".to_string();
            let query = encode_query(&params);
            self.list::<IPOListing>(&path, &query, params.options.as_ref())
        })
    }
}

// --- Params structs (additive builder API) ---

/// Optional arguments for [`VxApi::list_stock_financials`].
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ListStockFinancialsParams {
    /// The `ticker` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// The `cik` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cik: Option<String>,
    /// The `company_name` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// The `company_name_search` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name_search: Option<String>,
    /// The `sic` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sic: Option<String>,
    /// The `filing_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filing_date: Option<String>,
    /// The `filing_date_lt` argument.
    #[serde(rename = "filing_date.lt", skip_serializing_if = "Option::is_none")]
    pub filing_date_lt: Option<String>,
    /// The `filing_date_lte` argument.
    #[serde(rename = "filing_date.lte", skip_serializing_if = "Option::is_none")]
    pub filing_date_lte: Option<String>,
    /// The `filing_date_gt` argument.
    #[serde(rename = "filing_date.gt", skip_serializing_if = "Option::is_none")]
    pub filing_date_gt: Option<String>,
    /// The `filing_date_gte` argument.
    #[serde(rename = "filing_date.gte", skip_serializing_if = "Option::is_none")]
    pub filing_date_gte: Option<String>,
    /// The `period_of_report_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_of_report_date: Option<String>,
    /// The `period_of_report_date_lt` argument.
    #[serde(rename = "period_of_report_date.lt", skip_serializing_if = "Option::is_none")]
    pub period_of_report_date_lt: Option<String>,
    /// The `period_of_report_date_lte` argument.
    #[serde(rename = "period_of_report_date.lte", skip_serializing_if = "Option::is_none")]
    pub period_of_report_date_lte: Option<String>,
    /// The `period_of_report_date_gt` argument.
    #[serde(rename = "period_of_report_date.gt", skip_serializing_if = "Option::is_none")]
    pub period_of_report_date_gt: Option<String>,
    /// The `period_of_report_date_gte` argument.
    #[serde(rename = "period_of_report_date.gte", skip_serializing_if = "Option::is_none")]
    pub period_of_report_date_gte: Option<String>,
    /// The `timeframe` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<String>,
    /// The `include_sources` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_sources: Option<bool>,
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

impl ListStockFinancialsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `ticker` argument.
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    /// Set the `cik` argument.
    pub fn cik(mut self, cik: impl Into<String>) -> Self {
        self.cik = Some(cik.into());
        self
    }

    /// Set the `company_name` argument.
    pub fn company_name(mut self, company_name: impl Into<String>) -> Self {
        self.company_name = Some(company_name.into());
        self
    }

    /// Set the `company_name_search` argument.
    pub fn company_name_search(mut self, company_name_search: impl Into<String>) -> Self {
        self.company_name_search = Some(company_name_search.into());
        self
    }

    /// Set the `sic` argument.
    pub fn sic(mut self, sic: impl Into<String>) -> Self {
        self.sic = Some(sic.into());
        self
    }

    /// Set the `filing_date` argument.
    pub fn filing_date(mut self, filing_date: impl Into<String>) -> Self {
        self.filing_date = Some(filing_date.into());
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

    /// Set the `period_of_report_date` argument.
    pub fn period_of_report_date(mut self, period_of_report_date: impl Into<String>) -> Self {
        self.period_of_report_date = Some(period_of_report_date.into());
        self
    }

    /// Set the `period_of_report_date_lt` argument.
    pub fn period_of_report_date_lt(mut self, period_of_report_date_lt: impl Into<String>) -> Self {
        self.period_of_report_date_lt = Some(period_of_report_date_lt.into());
        self
    }

    /// Set the `period_of_report_date_lte` argument.
    pub fn period_of_report_date_lte(
        mut self,
        period_of_report_date_lte: impl Into<String>,
    ) -> Self {
        self.period_of_report_date_lte = Some(period_of_report_date_lte.into());
        self
    }

    /// Set the `period_of_report_date_gt` argument.
    pub fn period_of_report_date_gt(mut self, period_of_report_date_gt: impl Into<String>) -> Self {
        self.period_of_report_date_gt = Some(period_of_report_date_gt.into());
        self
    }

    /// Set the `period_of_report_date_gte` argument.
    pub fn period_of_report_date_gte(
        mut self,
        period_of_report_date_gte: impl Into<String>,
    ) -> Self {
        self.period_of_report_date_gte = Some(period_of_report_date_gte.into());
        self
    }

    /// Set the `timeframe` argument.
    pub fn timeframe(mut self, timeframe: impl Into<String>) -> Self {
        self.timeframe = Some(timeframe.into());
        self
    }

    /// Set the `include_sources` argument.
    pub fn include_sources(mut self, include_sources: bool) -> Self {
        self.include_sources = Some(include_sources);
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

/// Optional arguments for [`VxApi::list_ipos`].
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ListIposParams {
    /// The `ticker` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// The `us_code` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_code: Option<String>,
    /// The `isin` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isin: Option<String>,
    /// The `listing_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listing_date: Option<String>,
    /// The `listing_date_lt` argument.
    #[serde(rename = "listing_date.lt", skip_serializing_if = "Option::is_none")]
    pub listing_date_lt: Option<String>,
    /// The `listing_date_lte` argument.
    #[serde(rename = "listing_date.lte", skip_serializing_if = "Option::is_none")]
    pub listing_date_lte: Option<String>,
    /// The `listing_date_gt` argument.
    #[serde(rename = "listing_date.gt", skip_serializing_if = "Option::is_none")]
    pub listing_date_gt: Option<String>,
    /// The `listing_date_gte` argument.
    #[serde(rename = "listing_date.gte", skip_serializing_if = "Option::is_none")]
    pub listing_date_gte: Option<String>,
    /// The `ipo_status` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipo_status: Option<String>,
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

impl ListIposParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `ticker` argument.
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    /// Set the `us_code` argument.
    pub fn us_code(mut self, us_code: impl Into<String>) -> Self {
        self.us_code = Some(us_code.into());
        self
    }

    /// Set the `isin` argument.
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }

    /// Set the `listing_date` argument.
    pub fn listing_date(mut self, listing_date: impl Into<String>) -> Self {
        self.listing_date = Some(listing_date.into());
        self
    }

    /// Set the `listing_date_lt` argument.
    pub fn listing_date_lt(mut self, listing_date_lt: impl Into<String>) -> Self {
        self.listing_date_lt = Some(listing_date_lt.into());
        self
    }

    /// Set the `listing_date_lte` argument.
    pub fn listing_date_lte(mut self, listing_date_lte: impl Into<String>) -> Self {
        self.listing_date_lte = Some(listing_date_lte.into());
        self
    }

    /// Set the `listing_date_gt` argument.
    pub fn listing_date_gt(mut self, listing_date_gt: impl Into<String>) -> Self {
        self.listing_date_gt = Some(listing_date_gt.into());
        self
    }

    /// Set the `listing_date_gte` argument.
    pub fn listing_date_gte(mut self, listing_date_gte: impl Into<String>) -> Self {
        self.listing_date_gte = Some(listing_date_gte.into());
        self
    }

    /// Set the `ipo_status` argument.
    pub fn ipo_status(mut self, ipo_status: impl Into<String>) -> Self {
        self.ipo_status = Some(ipo_status.into());
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
