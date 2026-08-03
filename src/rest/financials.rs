use crate::client::{Client, RequestOptions};
use crate::error::Result;
use crate::models::{
    FinancialBalanceSheet, FinancialCashFlowStatement, FinancialFloat, FinancialIncomeStatement,
    FinancialRatio,
};
use futures::Stream;

/// Stocks Financials API.
pub trait FinancialsApi {
    /// List balance sheets (GET /stocks/financials/v1/balance-sheets).
    #[allow(clippy::too_many_arguments)]
    fn list_financials_balance_sheets(
        &self,
        cik: Option<&str>,
        cik_any_of: Option<&str>,
        cik_gt: Option<&str>,
        cik_gte: Option<&str>,
        cik_lt: Option<&str>,
        cik_lte: Option<&str>,
        tickers: Option<&str>,
        tickers_all_of: Option<&str>,
        tickers_any_of: Option<&str>,
        period_end: Option<&str>,
        period_end_gt: Option<&str>,
        period_end_gte: Option<&str>,
        period_end_lt: Option<&str>,
        period_end_lte: Option<&str>,
        filing_date: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        fiscal_year: Option<f64>,
        fiscal_year_gt: Option<f64>,
        fiscal_year_gte: Option<f64>,
        fiscal_year_lt: Option<f64>,
        fiscal_year_lte: Option<f64>,
        fiscal_quarter: Option<f64>,
        fiscal_quarter_gt: Option<f64>,
        fiscal_quarter_gte: Option<f64>,
        fiscal_quarter_lt: Option<f64>,
        fiscal_quarter_lte: Option<f64>,
        timeframe: Option<&str>,
        timeframe_any_of: Option<&str>,
        timeframe_gt: Option<&str>,
        timeframe_gte: Option<&str>,
        timeframe_lt: Option<&str>,
        timeframe_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FinancialBalanceSheet>>;

    /// List cash flow statements (GET /stocks/financials/v1/cash-flow-statements).
    #[allow(clippy::too_many_arguments)]
    fn list_financials_cash_flow_statements(
        &self,
        cik: Option<&str>,
        cik_any_of: Option<&str>,
        cik_gt: Option<&str>,
        cik_gte: Option<&str>,
        cik_lt: Option<&str>,
        cik_lte: Option<&str>,
        period_end: Option<&str>,
        period_end_gt: Option<&str>,
        period_end_gte: Option<&str>,
        period_end_lt: Option<&str>,
        period_end_lte: Option<&str>,
        filing_date: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        tickers: Option<&str>,
        tickers_all_of: Option<&str>,
        tickers_any_of: Option<&str>,
        fiscal_year: Option<f64>,
        fiscal_year_gt: Option<f64>,
        fiscal_year_gte: Option<f64>,
        fiscal_year_lt: Option<f64>,
        fiscal_year_lte: Option<f64>,
        fiscal_quarter: Option<f64>,
        fiscal_quarter_gt: Option<f64>,
        fiscal_quarter_gte: Option<f64>,
        fiscal_quarter_lt: Option<f64>,
        fiscal_quarter_lte: Option<f64>,
        timeframe: Option<&str>,
        timeframe_any_of: Option<&str>,
        timeframe_gt: Option<&str>,
        timeframe_gte: Option<&str>,
        timeframe_lt: Option<&str>,
        timeframe_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FinancialCashFlowStatement>>;

    /// List income statements (GET /stocks/financials/v1/income-statements).
    #[allow(clippy::too_many_arguments)]
    fn list_financials_income_statements(
        &self,
        cik: Option<&str>,
        cik_any_of: Option<&str>,
        cik_gt: Option<&str>,
        cik_gte: Option<&str>,
        cik_lt: Option<&str>,
        cik_lte: Option<&str>,
        tickers: Option<&str>,
        tickers_all_of: Option<&str>,
        tickers_any_of: Option<&str>,
        period_end: Option<&str>,
        period_end_gt: Option<&str>,
        period_end_gte: Option<&str>,
        period_end_lt: Option<&str>,
        period_end_lte: Option<&str>,
        filing_date: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        fiscal_year: Option<f64>,
        fiscal_year_gt: Option<f64>,
        fiscal_year_gte: Option<f64>,
        fiscal_year_lt: Option<f64>,
        fiscal_year_lte: Option<f64>,
        fiscal_quarter: Option<f64>,
        fiscal_quarter_gt: Option<f64>,
        fiscal_quarter_gte: Option<f64>,
        fiscal_quarter_lt: Option<f64>,
        fiscal_quarter_lte: Option<f64>,
        timeframe: Option<&str>,
        timeframe_any_of: Option<&str>,
        timeframe_gt: Option<&str>,
        timeframe_gte: Option<&str>,
        timeframe_lt: Option<&str>,
        timeframe_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FinancialIncomeStatement>>;

    /// List financial ratios (GET /stocks/financials/v1/ratios).
    #[allow(clippy::too_many_arguments)]
    fn list_financials_ratios(
        &self,
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
        price: Option<f64>,
        price_gt: Option<f64>,
        price_gte: Option<f64>,
        price_lt: Option<f64>,
        price_lte: Option<f64>,
        average_volume: Option<f64>,
        average_volume_gt: Option<f64>,
        average_volume_gte: Option<f64>,
        average_volume_lt: Option<f64>,
        average_volume_lte: Option<f64>,
        market_cap: Option<f64>,
        market_cap_gt: Option<f64>,
        market_cap_gte: Option<f64>,
        market_cap_lt: Option<f64>,
        market_cap_lte: Option<f64>,
        earnings_per_share: Option<f64>,
        earnings_per_share_gt: Option<f64>,
        earnings_per_share_gte: Option<f64>,
        earnings_per_share_lt: Option<f64>,
        earnings_per_share_lte: Option<f64>,
        price_to_earnings: Option<f64>,
        price_to_earnings_gt: Option<f64>,
        price_to_earnings_gte: Option<f64>,
        price_to_earnings_lt: Option<f64>,
        price_to_earnings_lte: Option<f64>,
        price_to_book: Option<f64>,
        price_to_book_gt: Option<f64>,
        price_to_book_gte: Option<f64>,
        price_to_book_lt: Option<f64>,
        price_to_book_lte: Option<f64>,
        price_to_sales: Option<f64>,
        price_to_sales_gt: Option<f64>,
        price_to_sales_gte: Option<f64>,
        price_to_sales_lt: Option<f64>,
        price_to_sales_lte: Option<f64>,
        price_to_cash_flow: Option<f64>,
        price_to_cash_flow_gt: Option<f64>,
        price_to_cash_flow_gte: Option<f64>,
        price_to_cash_flow_lt: Option<f64>,
        price_to_cash_flow_lte: Option<f64>,
        price_to_free_cash_flow: Option<f64>,
        price_to_free_cash_flow_gt: Option<f64>,
        price_to_free_cash_flow_gte: Option<f64>,
        price_to_free_cash_flow_lt: Option<f64>,
        price_to_free_cash_flow_lte: Option<f64>,
        dividend_yield: Option<f64>,
        dividend_yield_gt: Option<f64>,
        dividend_yield_gte: Option<f64>,
        dividend_yield_lt: Option<f64>,
        dividend_yield_lte: Option<f64>,
        return_on_assets: Option<f64>,
        return_on_assets_gt: Option<f64>,
        return_on_assets_gte: Option<f64>,
        return_on_assets_lt: Option<f64>,
        return_on_assets_lte: Option<f64>,
        return_on_equity: Option<f64>,
        return_on_equity_gt: Option<f64>,
        return_on_equity_gte: Option<f64>,
        return_on_equity_lt: Option<f64>,
        return_on_equity_lte: Option<f64>,
        debt_to_equity: Option<f64>,
        debt_to_equity_gt: Option<f64>,
        debt_to_equity_gte: Option<f64>,
        debt_to_equity_lt: Option<f64>,
        debt_to_equity_lte: Option<f64>,
        current: Option<f64>,
        current_gt: Option<f64>,
        current_gte: Option<f64>,
        current_lt: Option<f64>,
        current_lte: Option<f64>,
        quick: Option<f64>,
        quick_gt: Option<f64>,
        quick_gte: Option<f64>,
        quick_lt: Option<f64>,
        quick_lte: Option<f64>,
        cash: Option<f64>,
        cash_gt: Option<f64>,
        cash_gte: Option<f64>,
        cash_lt: Option<f64>,
        cash_lte: Option<f64>,
        ev_to_sales: Option<f64>,
        ev_to_sales_gt: Option<f64>,
        ev_to_sales_gte: Option<f64>,
        ev_to_sales_lt: Option<f64>,
        ev_to_sales_lte: Option<f64>,
        ev_to_ebitda: Option<f64>,
        ev_to_ebitda_gt: Option<f64>,
        ev_to_ebitda_gte: Option<f64>,
        ev_to_ebitda_lt: Option<f64>,
        ev_to_ebitda_lte: Option<f64>,
        enterprise_value: Option<f64>,
        enterprise_value_gt: Option<f64>,
        enterprise_value_gte: Option<f64>,
        enterprise_value_lt: Option<f64>,
        enterprise_value_lte: Option<f64>,
        free_cash_flow: Option<f64>,
        free_cash_flow_gt: Option<f64>,
        free_cash_flow_gte: Option<f64>,
        free_cash_flow_lt: Option<f64>,
        free_cash_flow_lte: Option<f64>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FinancialRatio>>;

    /// List stocks float data (GET /stocks/vX/float).
    fn list_stocks_floats(
        &self,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        free_float_percent: Option<f64>,
        free_float_percent_gt: Option<f64>,
        free_float_percent_gte: Option<f64>,
        free_float_percent_lt: Option<f64>,
        free_float_percent_lte: Option<f64>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FinancialFloat>>;
}

impl FinancialsApi for Client {
    fn list_financials_balance_sheets(
        &self,
        cik: Option<&str>,
        cik_any_of: Option<&str>,
        cik_gt: Option<&str>,
        cik_gte: Option<&str>,
        cik_lt: Option<&str>,
        cik_lte: Option<&str>,
        tickers: Option<&str>,
        tickers_all_of: Option<&str>,
        tickers_any_of: Option<&str>,
        period_end: Option<&str>,
        period_end_gt: Option<&str>,
        period_end_gte: Option<&str>,
        period_end_lt: Option<&str>,
        period_end_lte: Option<&str>,
        filing_date: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        fiscal_year: Option<f64>,
        fiscal_year_gt: Option<f64>,
        fiscal_year_gte: Option<f64>,
        fiscal_year_lt: Option<f64>,
        fiscal_year_lte: Option<f64>,
        fiscal_quarter: Option<f64>,
        fiscal_quarter_gt: Option<f64>,
        fiscal_quarter_gte: Option<f64>,
        fiscal_quarter_lt: Option<f64>,
        fiscal_quarter_lte: Option<f64>,
        timeframe: Option<&str>,
        timeframe_any_of: Option<&str>,
        timeframe_gt: Option<&str>,
        timeframe_gte: Option<&str>,
        timeframe_lt: Option<&str>,
        timeframe_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FinancialBalanceSheet>> {
        let path = "/stocks/financials/v1/balance-sheets".to_string();
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
        if let Some(v) = tickers {
            params.push(("tickers", v.to_string()));
        }
        if let Some(v) = tickers_all_of {
            params.push(("tickers_all_of", v.to_string()));
        }
        if let Some(v) = tickers_any_of {
            params.push(("tickers.any_of", v.to_string()));
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
        if let Some(v) = fiscal_year {
            params.push(("fiscal_year", v.to_string()));
        }
        if let Some(v) = fiscal_year_gt {
            params.push(("fiscal_year.gt", v.to_string()));
        }
        if let Some(v) = fiscal_year_gte {
            params.push(("fiscal_year.gte", v.to_string()));
        }
        if let Some(v) = fiscal_year_lt {
            params.push(("fiscal_year.lt", v.to_string()));
        }
        if let Some(v) = fiscal_year_lte {
            params.push(("fiscal_year.lte", v.to_string()));
        }
        if let Some(v) = fiscal_quarter {
            params.push(("fiscal_quarter", v.to_string()));
        }
        if let Some(v) = fiscal_quarter_gt {
            params.push(("fiscal_quarter.gt", v.to_string()));
        }
        if let Some(v) = fiscal_quarter_gte {
            params.push(("fiscal_quarter.gte", v.to_string()));
        }
        if let Some(v) = fiscal_quarter_lt {
            params.push(("fiscal_quarter.lt", v.to_string()));
        }
        if let Some(v) = fiscal_quarter_lte {
            params.push(("fiscal_quarter.lte", v.to_string()));
        }
        if let Some(v) = timeframe {
            params.push(("timeframe", v.to_string()));
        }
        if let Some(v) = timeframe_any_of {
            params.push(("timeframe.any_of", v.to_string()));
        }
        if let Some(v) = timeframe_gt {
            params.push(("timeframe.gt", v.to_string()));
        }
        if let Some(v) = timeframe_gte {
            params.push(("timeframe.gte", v.to_string()));
        }
        if let Some(v) = timeframe_lt {
            params.push(("timeframe.lt", v.to_string()));
        }
        if let Some(v) = timeframe_lte {
            params.push(("timeframe.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<FinancialBalanceSheet>(&path, Some(&params), options)
        } else {
            self.single_page::<FinancialBalanceSheet>(&path, Some(&params), options)
        }
    }

    fn list_financials_cash_flow_statements(
        &self,
        cik: Option<&str>,
        cik_any_of: Option<&str>,
        cik_gt: Option<&str>,
        cik_gte: Option<&str>,
        cik_lt: Option<&str>,
        cik_lte: Option<&str>,
        period_end: Option<&str>,
        period_end_gt: Option<&str>,
        period_end_gte: Option<&str>,
        period_end_lt: Option<&str>,
        period_end_lte: Option<&str>,
        filing_date: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        tickers: Option<&str>,
        tickers_all_of: Option<&str>,
        tickers_any_of: Option<&str>,
        fiscal_year: Option<f64>,
        fiscal_year_gt: Option<f64>,
        fiscal_year_gte: Option<f64>,
        fiscal_year_lt: Option<f64>,
        fiscal_year_lte: Option<f64>,
        fiscal_quarter: Option<f64>,
        fiscal_quarter_gt: Option<f64>,
        fiscal_quarter_gte: Option<f64>,
        fiscal_quarter_lt: Option<f64>,
        fiscal_quarter_lte: Option<f64>,
        timeframe: Option<&str>,
        timeframe_any_of: Option<&str>,
        timeframe_gt: Option<&str>,
        timeframe_gte: Option<&str>,
        timeframe_lt: Option<&str>,
        timeframe_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FinancialCashFlowStatement>> {
        let path = "/stocks/financials/v1/cash-flow-statements".to_string();
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
        if let Some(v) = tickers {
            params.push(("tickers", v.to_string()));
        }
        if let Some(v) = tickers_all_of {
            params.push(("tickers_all_of", v.to_string()));
        }
        if let Some(v) = tickers_any_of {
            params.push(("tickers.any_of", v.to_string()));
        }
        if let Some(v) = fiscal_year {
            params.push(("fiscal_year", v.to_string()));
        }
        if let Some(v) = fiscal_year_gt {
            params.push(("fiscal_year.gt", v.to_string()));
        }
        if let Some(v) = fiscal_year_gte {
            params.push(("fiscal_year.gte", v.to_string()));
        }
        if let Some(v) = fiscal_year_lt {
            params.push(("fiscal_year.lt", v.to_string()));
        }
        if let Some(v) = fiscal_year_lte {
            params.push(("fiscal_year.lte", v.to_string()));
        }
        if let Some(v) = fiscal_quarter {
            params.push(("fiscal_quarter", v.to_string()));
        }
        if let Some(v) = fiscal_quarter_gt {
            params.push(("fiscal_quarter.gt", v.to_string()));
        }
        if let Some(v) = fiscal_quarter_gte {
            params.push(("fiscal_quarter.gte", v.to_string()));
        }
        if let Some(v) = fiscal_quarter_lt {
            params.push(("fiscal_quarter.lt", v.to_string()));
        }
        if let Some(v) = fiscal_quarter_lte {
            params.push(("fiscal_quarter.lte", v.to_string()));
        }
        if let Some(v) = timeframe {
            params.push(("timeframe", v.to_string()));
        }
        if let Some(v) = timeframe_any_of {
            params.push(("timeframe.any_of", v.to_string()));
        }
        if let Some(v) = timeframe_gt {
            params.push(("timeframe.gt", v.to_string()));
        }
        if let Some(v) = timeframe_gte {
            params.push(("timeframe.gte", v.to_string()));
        }
        if let Some(v) = timeframe_lt {
            params.push(("timeframe.lt", v.to_string()));
        }
        if let Some(v) = timeframe_lte {
            params.push(("timeframe.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<FinancialCashFlowStatement>(&path, Some(&params), options)
        } else {
            self.single_page::<FinancialCashFlowStatement>(&path, Some(&params), options)
        }
    }

    fn list_financials_income_statements(
        &self,
        cik: Option<&str>,
        cik_any_of: Option<&str>,
        cik_gt: Option<&str>,
        cik_gte: Option<&str>,
        cik_lt: Option<&str>,
        cik_lte: Option<&str>,
        tickers: Option<&str>,
        tickers_all_of: Option<&str>,
        tickers_any_of: Option<&str>,
        period_end: Option<&str>,
        period_end_gt: Option<&str>,
        period_end_gte: Option<&str>,
        period_end_lt: Option<&str>,
        period_end_lte: Option<&str>,
        filing_date: Option<&str>,
        filing_date_gt: Option<&str>,
        filing_date_gte: Option<&str>,
        filing_date_lt: Option<&str>,
        filing_date_lte: Option<&str>,
        fiscal_year: Option<f64>,
        fiscal_year_gt: Option<f64>,
        fiscal_year_gte: Option<f64>,
        fiscal_year_lt: Option<f64>,
        fiscal_year_lte: Option<f64>,
        fiscal_quarter: Option<f64>,
        fiscal_quarter_gt: Option<f64>,
        fiscal_quarter_gte: Option<f64>,
        fiscal_quarter_lt: Option<f64>,
        fiscal_quarter_lte: Option<f64>,
        timeframe: Option<&str>,
        timeframe_any_of: Option<&str>,
        timeframe_gt: Option<&str>,
        timeframe_gte: Option<&str>,
        timeframe_lt: Option<&str>,
        timeframe_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FinancialIncomeStatement>> {
        let path = "/stocks/financials/v1/income-statements".to_string();
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
        if let Some(v) = tickers {
            params.push(("tickers", v.to_string()));
        }
        if let Some(v) = tickers_all_of {
            params.push(("tickers_all_of", v.to_string()));
        }
        if let Some(v) = tickers_any_of {
            params.push(("tickers.any_of", v.to_string()));
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
        if let Some(v) = fiscal_year {
            params.push(("fiscal_year", v.to_string()));
        }
        if let Some(v) = fiscal_year_gt {
            params.push(("fiscal_year.gt", v.to_string()));
        }
        if let Some(v) = fiscal_year_gte {
            params.push(("fiscal_year.gte", v.to_string()));
        }
        if let Some(v) = fiscal_year_lt {
            params.push(("fiscal_year.lt", v.to_string()));
        }
        if let Some(v) = fiscal_year_lte {
            params.push(("fiscal_year.lte", v.to_string()));
        }
        if let Some(v) = fiscal_quarter {
            params.push(("fiscal_quarter", v.to_string()));
        }
        if let Some(v) = fiscal_quarter_gt {
            params.push(("fiscal_quarter.gt", v.to_string()));
        }
        if let Some(v) = fiscal_quarter_gte {
            params.push(("fiscal_quarter.gte", v.to_string()));
        }
        if let Some(v) = fiscal_quarter_lt {
            params.push(("fiscal_quarter.lt", v.to_string()));
        }
        if let Some(v) = fiscal_quarter_lte {
            params.push(("fiscal_quarter.lte", v.to_string()));
        }
        if let Some(v) = timeframe {
            params.push(("timeframe", v.to_string()));
        }
        if let Some(v) = timeframe_any_of {
            params.push(("timeframe.any_of", v.to_string()));
        }
        if let Some(v) = timeframe_gt {
            params.push(("timeframe.gt", v.to_string()));
        }
        if let Some(v) = timeframe_gte {
            params.push(("timeframe.gte", v.to_string()));
        }
        if let Some(v) = timeframe_lt {
            params.push(("timeframe.lt", v.to_string()));
        }
        if let Some(v) = timeframe_lte {
            params.push(("timeframe.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<FinancialIncomeStatement>(&path, Some(&params), options)
        } else {
            self.single_page::<FinancialIncomeStatement>(&path, Some(&params), options)
        }
    }

    fn list_financials_ratios(
        &self,
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
        price: Option<f64>,
        price_gt: Option<f64>,
        price_gte: Option<f64>,
        price_lt: Option<f64>,
        price_lte: Option<f64>,
        average_volume: Option<f64>,
        average_volume_gt: Option<f64>,
        average_volume_gte: Option<f64>,
        average_volume_lt: Option<f64>,
        average_volume_lte: Option<f64>,
        market_cap: Option<f64>,
        market_cap_gt: Option<f64>,
        market_cap_gte: Option<f64>,
        market_cap_lt: Option<f64>,
        market_cap_lte: Option<f64>,
        earnings_per_share: Option<f64>,
        earnings_per_share_gt: Option<f64>,
        earnings_per_share_gte: Option<f64>,
        earnings_per_share_lt: Option<f64>,
        earnings_per_share_lte: Option<f64>,
        price_to_earnings: Option<f64>,
        price_to_earnings_gt: Option<f64>,
        price_to_earnings_gte: Option<f64>,
        price_to_earnings_lt: Option<f64>,
        price_to_earnings_lte: Option<f64>,
        price_to_book: Option<f64>,
        price_to_book_gt: Option<f64>,
        price_to_book_gte: Option<f64>,
        price_to_book_lt: Option<f64>,
        price_to_book_lte: Option<f64>,
        price_to_sales: Option<f64>,
        price_to_sales_gt: Option<f64>,
        price_to_sales_gte: Option<f64>,
        price_to_sales_lt: Option<f64>,
        price_to_sales_lte: Option<f64>,
        price_to_cash_flow: Option<f64>,
        price_to_cash_flow_gt: Option<f64>,
        price_to_cash_flow_gte: Option<f64>,
        price_to_cash_flow_lt: Option<f64>,
        price_to_cash_flow_lte: Option<f64>,
        price_to_free_cash_flow: Option<f64>,
        price_to_free_cash_flow_gt: Option<f64>,
        price_to_free_cash_flow_gte: Option<f64>,
        price_to_free_cash_flow_lt: Option<f64>,
        price_to_free_cash_flow_lte: Option<f64>,
        dividend_yield: Option<f64>,
        dividend_yield_gt: Option<f64>,
        dividend_yield_gte: Option<f64>,
        dividend_yield_lt: Option<f64>,
        dividend_yield_lte: Option<f64>,
        return_on_assets: Option<f64>,
        return_on_assets_gt: Option<f64>,
        return_on_assets_gte: Option<f64>,
        return_on_assets_lt: Option<f64>,
        return_on_assets_lte: Option<f64>,
        return_on_equity: Option<f64>,
        return_on_equity_gt: Option<f64>,
        return_on_equity_gte: Option<f64>,
        return_on_equity_lt: Option<f64>,
        return_on_equity_lte: Option<f64>,
        debt_to_equity: Option<f64>,
        debt_to_equity_gt: Option<f64>,
        debt_to_equity_gte: Option<f64>,
        debt_to_equity_lt: Option<f64>,
        debt_to_equity_lte: Option<f64>,
        current: Option<f64>,
        current_gt: Option<f64>,
        current_gte: Option<f64>,
        current_lt: Option<f64>,
        current_lte: Option<f64>,
        quick: Option<f64>,
        quick_gt: Option<f64>,
        quick_gte: Option<f64>,
        quick_lt: Option<f64>,
        quick_lte: Option<f64>,
        cash: Option<f64>,
        cash_gt: Option<f64>,
        cash_gte: Option<f64>,
        cash_lt: Option<f64>,
        cash_lte: Option<f64>,
        ev_to_sales: Option<f64>,
        ev_to_sales_gt: Option<f64>,
        ev_to_sales_gte: Option<f64>,
        ev_to_sales_lt: Option<f64>,
        ev_to_sales_lte: Option<f64>,
        ev_to_ebitda: Option<f64>,
        ev_to_ebitda_gt: Option<f64>,
        ev_to_ebitda_gte: Option<f64>,
        ev_to_ebitda_lt: Option<f64>,
        ev_to_ebitda_lte: Option<f64>,
        enterprise_value: Option<f64>,
        enterprise_value_gt: Option<f64>,
        enterprise_value_gte: Option<f64>,
        enterprise_value_lt: Option<f64>,
        enterprise_value_lte: Option<f64>,
        free_cash_flow: Option<f64>,
        free_cash_flow_gt: Option<f64>,
        free_cash_flow_gte: Option<f64>,
        free_cash_flow_lt: Option<f64>,
        free_cash_flow_lte: Option<f64>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FinancialRatio>> {
        let path = "/stocks/financials/v1/ratios".to_string();
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
        if let Some(v) = price {
            params.push(("price", v.to_string()));
        }
        if let Some(v) = price_gt {
            params.push(("price.gt", v.to_string()));
        }
        if let Some(v) = price_gte {
            params.push(("price.gte", v.to_string()));
        }
        if let Some(v) = price_lt {
            params.push(("price.lt", v.to_string()));
        }
        if let Some(v) = price_lte {
            params.push(("price.lte", v.to_string()));
        }
        if let Some(v) = average_volume {
            params.push(("average_volume", v.to_string()));
        }
        if let Some(v) = average_volume_gt {
            params.push(("average_volume.gt", v.to_string()));
        }
        if let Some(v) = average_volume_gte {
            params.push(("average_volume.gte", v.to_string()));
        }
        if let Some(v) = average_volume_lt {
            params.push(("average_volume.lt", v.to_string()));
        }
        if let Some(v) = average_volume_lte {
            params.push(("average_volume.lte", v.to_string()));
        }
        if let Some(v) = market_cap {
            params.push(("market_cap", v.to_string()));
        }
        if let Some(v) = market_cap_gt {
            params.push(("market_cap.gt", v.to_string()));
        }
        if let Some(v) = market_cap_gte {
            params.push(("market_cap.gte", v.to_string()));
        }
        if let Some(v) = market_cap_lt {
            params.push(("market_cap.lt", v.to_string()));
        }
        if let Some(v) = market_cap_lte {
            params.push(("market_cap.lte", v.to_string()));
        }
        if let Some(v) = earnings_per_share {
            params.push(("earnings_per_share", v.to_string()));
        }
        if let Some(v) = earnings_per_share_gt {
            params.push(("earnings_per_share.gt", v.to_string()));
        }
        if let Some(v) = earnings_per_share_gte {
            params.push(("earnings_per_share.gte", v.to_string()));
        }
        if let Some(v) = earnings_per_share_lt {
            params.push(("earnings_per_share.lt", v.to_string()));
        }
        if let Some(v) = earnings_per_share_lte {
            params.push(("earnings_per_share.lte", v.to_string()));
        }
        if let Some(v) = price_to_earnings {
            params.push(("price_to_earnings", v.to_string()));
        }
        if let Some(v) = price_to_earnings_gt {
            params.push(("price_to_earnings.gt", v.to_string()));
        }
        if let Some(v) = price_to_earnings_gte {
            params.push(("price_to_earnings.gte", v.to_string()));
        }
        if let Some(v) = price_to_earnings_lt {
            params.push(("price_to_earnings.lt", v.to_string()));
        }
        if let Some(v) = price_to_earnings_lte {
            params.push(("price_to_earnings.lte", v.to_string()));
        }
        if let Some(v) = price_to_book {
            params.push(("price_to_book", v.to_string()));
        }
        if let Some(v) = price_to_book_gt {
            params.push(("price_to_book.gt", v.to_string()));
        }
        if let Some(v) = price_to_book_gte {
            params.push(("price_to_book.gte", v.to_string()));
        }
        if let Some(v) = price_to_book_lt {
            params.push(("price_to_book.lt", v.to_string()));
        }
        if let Some(v) = price_to_book_lte {
            params.push(("price_to_book.lte", v.to_string()));
        }
        if let Some(v) = price_to_sales {
            params.push(("price_to_sales", v.to_string()));
        }
        if let Some(v) = price_to_sales_gt {
            params.push(("price_to_sales.gt", v.to_string()));
        }
        if let Some(v) = price_to_sales_gte {
            params.push(("price_to_sales.gte", v.to_string()));
        }
        if let Some(v) = price_to_sales_lt {
            params.push(("price_to_sales.lt", v.to_string()));
        }
        if let Some(v) = price_to_sales_lte {
            params.push(("price_to_sales.lte", v.to_string()));
        }
        if let Some(v) = price_to_cash_flow {
            params.push(("price_to_cash_flow", v.to_string()));
        }
        if let Some(v) = price_to_cash_flow_gt {
            params.push(("price_to_cash_flow.gt", v.to_string()));
        }
        if let Some(v) = price_to_cash_flow_gte {
            params.push(("price_to_cash_flow.gte", v.to_string()));
        }
        if let Some(v) = price_to_cash_flow_lt {
            params.push(("price_to_cash_flow.lt", v.to_string()));
        }
        if let Some(v) = price_to_cash_flow_lte {
            params.push(("price_to_cash_flow.lte", v.to_string()));
        }
        if let Some(v) = price_to_free_cash_flow {
            params.push(("price_to_free_cash_flow", v.to_string()));
        }
        if let Some(v) = price_to_free_cash_flow_gt {
            params.push(("price_to_free_cash_flow.gt", v.to_string()));
        }
        if let Some(v) = price_to_free_cash_flow_gte {
            params.push(("price_to_free_cash_flow.gte", v.to_string()));
        }
        if let Some(v) = price_to_free_cash_flow_lt {
            params.push(("price_to_free_cash_flow.lt", v.to_string()));
        }
        if let Some(v) = price_to_free_cash_flow_lte {
            params.push(("price_to_free_cash_flow.lte", v.to_string()));
        }
        if let Some(v) = dividend_yield {
            params.push(("dividend_yield", v.to_string()));
        }
        if let Some(v) = dividend_yield_gt {
            params.push(("dividend_yield.gt", v.to_string()));
        }
        if let Some(v) = dividend_yield_gte {
            params.push(("dividend_yield.gte", v.to_string()));
        }
        if let Some(v) = dividend_yield_lt {
            params.push(("dividend_yield.lt", v.to_string()));
        }
        if let Some(v) = dividend_yield_lte {
            params.push(("dividend_yield.lte", v.to_string()));
        }
        if let Some(v) = return_on_assets {
            params.push(("return_on_assets", v.to_string()));
        }
        if let Some(v) = return_on_assets_gt {
            params.push(("return_on_assets.gt", v.to_string()));
        }
        if let Some(v) = return_on_assets_gte {
            params.push(("return_on_assets.gte", v.to_string()));
        }
        if let Some(v) = return_on_assets_lt {
            params.push(("return_on_assets.lt", v.to_string()));
        }
        if let Some(v) = return_on_assets_lte {
            params.push(("return_on_assets.lte", v.to_string()));
        }
        if let Some(v) = return_on_equity {
            params.push(("return_on_equity", v.to_string()));
        }
        if let Some(v) = return_on_equity_gt {
            params.push(("return_on_equity.gt", v.to_string()));
        }
        if let Some(v) = return_on_equity_gte {
            params.push(("return_on_equity.gte", v.to_string()));
        }
        if let Some(v) = return_on_equity_lt {
            params.push(("return_on_equity.lt", v.to_string()));
        }
        if let Some(v) = return_on_equity_lte {
            params.push(("return_on_equity.lte", v.to_string()));
        }
        if let Some(v) = debt_to_equity {
            params.push(("debt_to_equity", v.to_string()));
        }
        if let Some(v) = debt_to_equity_gt {
            params.push(("debt_to_equity.gt", v.to_string()));
        }
        if let Some(v) = debt_to_equity_gte {
            params.push(("debt_to_equity.gte", v.to_string()));
        }
        if let Some(v) = debt_to_equity_lt {
            params.push(("debt_to_equity.lt", v.to_string()));
        }
        if let Some(v) = debt_to_equity_lte {
            params.push(("debt_to_equity.lte", v.to_string()));
        }
        if let Some(v) = current {
            params.push(("current", v.to_string()));
        }
        if let Some(v) = current_gt {
            params.push(("current.gt", v.to_string()));
        }
        if let Some(v) = current_gte {
            params.push(("current.gte", v.to_string()));
        }
        if let Some(v) = current_lt {
            params.push(("current.lt", v.to_string()));
        }
        if let Some(v) = current_lte {
            params.push(("current.lte", v.to_string()));
        }
        if let Some(v) = quick {
            params.push(("quick", v.to_string()));
        }
        if let Some(v) = quick_gt {
            params.push(("quick.gt", v.to_string()));
        }
        if let Some(v) = quick_gte {
            params.push(("quick.gte", v.to_string()));
        }
        if let Some(v) = quick_lt {
            params.push(("quick.lt", v.to_string()));
        }
        if let Some(v) = quick_lte {
            params.push(("quick.lte", v.to_string()));
        }
        if let Some(v) = cash {
            params.push(("cash", v.to_string()));
        }
        if let Some(v) = cash_gt {
            params.push(("cash.gt", v.to_string()));
        }
        if let Some(v) = cash_gte {
            params.push(("cash.gte", v.to_string()));
        }
        if let Some(v) = cash_lt {
            params.push(("cash.lt", v.to_string()));
        }
        if let Some(v) = cash_lte {
            params.push(("cash.lte", v.to_string()));
        }
        if let Some(v) = ev_to_sales {
            params.push(("ev_to_sales", v.to_string()));
        }
        if let Some(v) = ev_to_sales_gt {
            params.push(("ev_to_sales.gt", v.to_string()));
        }
        if let Some(v) = ev_to_sales_gte {
            params.push(("ev_to_sales.gte", v.to_string()));
        }
        if let Some(v) = ev_to_sales_lt {
            params.push(("ev_to_sales.lt", v.to_string()));
        }
        if let Some(v) = ev_to_sales_lte {
            params.push(("ev_to_sales.lte", v.to_string()));
        }
        if let Some(v) = ev_to_ebitda {
            params.push(("ev_to_ebitda", v.to_string()));
        }
        if let Some(v) = ev_to_ebitda_gt {
            params.push(("ev_to_ebitda.gt", v.to_string()));
        }
        if let Some(v) = ev_to_ebitda_gte {
            params.push(("ev_to_ebitda.gte", v.to_string()));
        }
        if let Some(v) = ev_to_ebitda_lt {
            params.push(("ev_to_ebitda.lt", v.to_string()));
        }
        if let Some(v) = ev_to_ebitda_lte {
            params.push(("ev_to_ebitda.lte", v.to_string()));
        }
        if let Some(v) = enterprise_value {
            params.push(("enterprise_value", v.to_string()));
        }
        if let Some(v) = enterprise_value_gt {
            params.push(("enterprise_value.gt", v.to_string()));
        }
        if let Some(v) = enterprise_value_gte {
            params.push(("enterprise_value.gte", v.to_string()));
        }
        if let Some(v) = enterprise_value_lt {
            params.push(("enterprise_value.lt", v.to_string()));
        }
        if let Some(v) = enterprise_value_lte {
            params.push(("enterprise_value.lte", v.to_string()));
        }
        if let Some(v) = free_cash_flow {
            params.push(("free_cash_flow", v.to_string()));
        }
        if let Some(v) = free_cash_flow_gt {
            params.push(("free_cash_flow.gt", v.to_string()));
        }
        if let Some(v) = free_cash_flow_gte {
            params.push(("free_cash_flow.gte", v.to_string()));
        }
        if let Some(v) = free_cash_flow_lt {
            params.push(("free_cash_flow.lt", v.to_string()));
        }
        if let Some(v) = free_cash_flow_lte {
            params.push(("free_cash_flow.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<FinancialRatio>(&path, Some(&params), options)
        } else {
            self.single_page::<FinancialRatio>(&path, Some(&params), options)
        }
    }

    fn list_stocks_floats(
        &self,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        free_float_percent: Option<f64>,
        free_float_percent_gt: Option<f64>,
        free_float_percent_gte: Option<f64>,
        free_float_percent_lt: Option<f64>,
        free_float_percent_lte: Option<f64>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<FinancialFloat>> {
        let path = "/stocks/vX/float".to_string();
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
        if let Some(v) = free_float_percent {
            params.push(("free_float_percent", v.to_string()));
        }
        if let Some(v) = free_float_percent_gt {
            params.push(("free_float_percent.gt", v.to_string()));
        }
        if let Some(v) = free_float_percent_gte {
            params.push(("free_float_percent.gte", v.to_string()));
        }
        if let Some(v) = free_float_percent_lt {
            params.push(("free_float_percent.lt", v.to_string()));
        }
        if let Some(v) = free_float_percent_lte {
            params.push(("free_float_percent.lte", v.to_string()));
        }
        if let Some(v) = limit {
            params.push(("limit", v.to_string()));
        }
        if let Some(v) = sort {
            params.push(("sort", v.to_string()));
        }
        if self.pagination {
            self.paginate::<FinancialFloat>(&path, Some(&params), options)
        } else {
            self.single_page::<FinancialFloat>(&path, Some(&params), options)
        }
    }
}
