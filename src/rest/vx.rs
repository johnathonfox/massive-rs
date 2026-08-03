use crate::client::{Client, RequestOptions};
use crate::error::Result;
use crate::models::{IPOListing, StockFinancial};
use futures::Stream;

/// Experimental (vX) API: stock financials and IPO listings.
pub trait VxApi {
    /// Get historical financial data for a stock ticker (paginated stream).
    fn list_stock_financials(
        &self,
        ticker: Option<&str>,
        cik: Option<&str>,
        company_name: Option<&str>,
        company_name_search: Option<&str>,
        sic: Option<&str>,
        filing_date: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        period_of_report_date: Option<&str>,
        period_of_report_date_lt: Option<&str>,
        period_of_report_date_lte: Option<&str>,
        period_of_report_date_gt: Option<&str>,
        period_of_report_date_gte: Option<&str>,
        timeframe: Option<&str>,
        include_sources: Option<bool>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<StockFinancial>>;

    /// Retrieve upcoming or historical IPOs (paginated stream).
    fn list_ipos(
        &self,
        ticker: Option<&str>,
        us_code: Option<&str>,
        isin: Option<&str>,
        listing_date: Option<&str>,
        listing_date_lt: Option<&str>,
        listing_date_lte: Option<&str>,
        listing_date_gt: Option<&str>,
        listing_date_gte: Option<&str>,
        ipo_status: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<IPOListing>>;
}

impl VxApi for Client {
    fn list_stock_financials(
        &self,
        ticker: Option<&str>,
        cik: Option<&str>,
        company_name: Option<&str>,
        company_name_search: Option<&str>,
        sic: Option<&str>,
        filing_date: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        period_of_report_date: Option<&str>,
        period_of_report_date_lt: Option<&str>,
        period_of_report_date_lte: Option<&str>,
        period_of_report_date_gt: Option<&str>,
        period_of_report_date_gte: Option<&str>,
        timeframe: Option<&str>,
        include_sources: Option<bool>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<StockFinancial>> {
        let path = "/vX/reference/financials".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(t) = ticker {
            params.push(("ticker", t.to_string()));
        }
        if let Some(c) = cik {
            params.push(("cik", c.to_string()));
        }
        if let Some(c) = company_name {
            params.push(("company_name", c.to_string()));
        }
        if let Some(c) = company_name_search {
            params.push(("company_name_search", c.to_string()));
        }
        if let Some(s) = sic {
            params.push(("sic", s.to_string()));
        }
        if let Some(f) = filing_date {
            params.push(("filing_date", f.to_string()));
        }
        if let Some(f) = filing_date_lt {
            params.push(("filing_date.lt", f.to_string()));
        }
        if let Some(f) = filing_date_lte {
            params.push(("filing_date.lte", f.to_string()));
        }
        if let Some(f) = filing_date_gt {
            params.push(("filing_date.gt", f.to_string()));
        }
        if let Some(f) = filing_date_gte {
            params.push(("filing_date.gte", f.to_string()));
        }
        if let Some(p) = period_of_report_date {
            params.push(("period_of_report_date", p.to_string()));
        }
        if let Some(p) = period_of_report_date_lt {
            params.push(("period_of_report_date.lt", p.to_string()));
        }
        if let Some(p) = period_of_report_date_lte {
            params.push(("period_of_report_date.lte", p.to_string()));
        }
        if let Some(p) = period_of_report_date_gt {
            params.push(("period_of_report_date.gt", p.to_string()));
        }
        if let Some(p) = period_of_report_date_gte {
            params.push(("period_of_report_date.gte", p.to_string()));
        }
        if let Some(t) = timeframe {
            params.push(("timeframe", t.to_string()));
        }
        if let Some(i) = include_sources {
            params.push(("include_sources", i.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if let Some(s) = sort {
            params.push(("sort", s.to_string()));
        }
        if let Some(o) = order {
            params.push(("order", o.to_string()));
        }
        if self.pagination {
            self.paginate::<StockFinancial>(&path, Some(&params), options)
        } else {
            self.single_page::<StockFinancial>(&path, Some(&params), options)
        }
    }

    fn list_ipos(
        &self,
        ticker: Option<&str>,
        us_code: Option<&str>,
        isin: Option<&str>,
        listing_date: Option<&str>,
        listing_date_lt: Option<&str>,
        listing_date_lte: Option<&str>,
        listing_date_gt: Option<&str>,
        listing_date_gte: Option<&str>,
        ipo_status: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<IPOListing>> {
        let path = "/vX/reference/ipos".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(t) = ticker {
            params.push(("ticker", t.to_string()));
        }
        if let Some(u) = us_code {
            params.push(("us_code", u.to_string()));
        }
        if let Some(i) = isin {
            params.push(("isin", i.to_string()));
        }
        if let Some(l) = listing_date {
            params.push(("listing_date", l.to_string()));
        }
        if let Some(l) = listing_date_lt {
            params.push(("listing_date.lt", l.to_string()));
        }
        if let Some(l) = listing_date_lte {
            params.push(("listing_date.lte", l.to_string()));
        }
        if let Some(l) = listing_date_gt {
            params.push(("listing_date.gt", l.to_string()));
        }
        if let Some(l) = listing_date_gte {
            params.push(("listing_date.gte", l.to_string()));
        }
        if let Some(i) = ipo_status {
            params.push(("ipo_status", i.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if let Some(s) = sort {
            params.push(("sort", s.to_string()));
        }
        if let Some(o) = order {
            params.push(("order", o.to_string()));
        }
        if self.pagination {
            self.paginate::<IPOListing>(&path, Some(&params), options)
        } else {
            self.single_page::<IPOListing>(&path, Some(&params), options)
        }
    }
}
