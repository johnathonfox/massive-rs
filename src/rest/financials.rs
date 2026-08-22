use super::BoxStream;
use crate::client::{Client, RequestOptions};
use crate::models::{
    FinancialBalanceSheet, FinancialCashFlowStatement, FinancialFloat, FinancialIncomeStatement,
    FinancialRatio,
};

/// Stocks Financials API.
pub trait FinancialsApi {
    /// List balance sheets (GET /stocks/financials/v1/balance-sheets).
    #[allow(clippy::too_many_arguments)]
    fn list_financials_balance_sheets<'a>(
        &'a self,
        cik: Option<&'a str>,
        cik_any_of: Option<&'a str>,
        cik_gt: Option<&'a str>,
        cik_gte: Option<&'a str>,
        cik_lt: Option<&'a str>,
        cik_lte: Option<&'a str>,
        tickers: Option<&'a str>,
        tickers_all_of: Option<&'a str>,
        tickers_any_of: Option<&'a str>,
        period_end: Option<&'a str>,
        period_end_gt: Option<&'a str>,
        period_end_gte: Option<&'a str>,
        period_end_lt: Option<&'a str>,
        period_end_lte: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
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
        timeframe: Option<&'a str>,
        timeframe_any_of: Option<&'a str>,
        timeframe_gt: Option<&'a str>,
        timeframe_gte: Option<&'a str>,
        timeframe_lt: Option<&'a str>,
        timeframe_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FinancialBalanceSheet>;

    /// Same as [`Self::list_financials_balance_sheets`], but takes the optional arguments as a
    /// chainable [`ListFinancialsBalanceSheetsParams`] struct.
    fn list_financials_balance_sheets_with_params<'a>(
        &'a self,
        params: ListFinancialsBalanceSheetsParams,
    ) -> BoxStream<'a, FinancialBalanceSheet>;

    /// List cash flow statements (GET /stocks/financials/v1/cash-flow-statements).
    #[allow(clippy::too_many_arguments)]
    fn list_financials_cash_flow_statements<'a>(
        &'a self,
        cik: Option<&'a str>,
        cik_any_of: Option<&'a str>,
        cik_gt: Option<&'a str>,
        cik_gte: Option<&'a str>,
        cik_lt: Option<&'a str>,
        cik_lte: Option<&'a str>,
        period_end: Option<&'a str>,
        period_end_gt: Option<&'a str>,
        period_end_gte: Option<&'a str>,
        period_end_lt: Option<&'a str>,
        period_end_lte: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        tickers: Option<&'a str>,
        tickers_all_of: Option<&'a str>,
        tickers_any_of: Option<&'a str>,
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
        timeframe: Option<&'a str>,
        timeframe_any_of: Option<&'a str>,
        timeframe_gt: Option<&'a str>,
        timeframe_gte: Option<&'a str>,
        timeframe_lt: Option<&'a str>,
        timeframe_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FinancialCashFlowStatement>;

    /// Same as [`Self::list_financials_cash_flow_statements`], but takes the optional arguments as a
    /// chainable [`ListFinancialsCashFlowStatementsParams`] struct.
    fn list_financials_cash_flow_statements_with_params<'a>(
        &'a self,
        params: ListFinancialsCashFlowStatementsParams,
    ) -> BoxStream<'a, FinancialCashFlowStatement>;

    /// List income statements (GET /stocks/financials/v1/income-statements).
    #[allow(clippy::too_many_arguments)]
    fn list_financials_income_statements<'a>(
        &'a self,
        cik: Option<&'a str>,
        cik_any_of: Option<&'a str>,
        cik_gt: Option<&'a str>,
        cik_gte: Option<&'a str>,
        cik_lt: Option<&'a str>,
        cik_lte: Option<&'a str>,
        tickers: Option<&'a str>,
        tickers_all_of: Option<&'a str>,
        tickers_any_of: Option<&'a str>,
        period_end: Option<&'a str>,
        period_end_gt: Option<&'a str>,
        period_end_gte: Option<&'a str>,
        period_end_lt: Option<&'a str>,
        period_end_lte: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
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
        timeframe: Option<&'a str>,
        timeframe_any_of: Option<&'a str>,
        timeframe_gt: Option<&'a str>,
        timeframe_gte: Option<&'a str>,
        timeframe_lt: Option<&'a str>,
        timeframe_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FinancialIncomeStatement>;

    /// Same as [`Self::list_financials_income_statements`], but takes the optional arguments as a
    /// chainable [`ListFinancialsIncomeStatementsParams`] struct.
    fn list_financials_income_statements_with_params<'a>(
        &'a self,
        params: ListFinancialsIncomeStatementsParams,
    ) -> BoxStream<'a, FinancialIncomeStatement>;

    /// List financial ratios (GET /stocks/financials/v1/ratios).
    #[allow(clippy::too_many_arguments)]
    fn list_financials_ratios<'a>(
        &'a self,
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
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FinancialRatio>;

    /// Same as [`Self::list_financials_ratios`], but takes the optional arguments as a
    /// chainable [`ListFinancialsRatiosParams`] struct.
    fn list_financials_ratios_with_params<'a>(
        &'a self,
        params: ListFinancialsRatiosParams,
    ) -> BoxStream<'a, FinancialRatio>;

    /// List stocks float data (GET /stocks/vX/float).
    fn list_stocks_floats<'a>(
        &'a self,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        free_float_percent: Option<f64>,
        free_float_percent_gt: Option<f64>,
        free_float_percent_gte: Option<f64>,
        free_float_percent_lt: Option<f64>,
        free_float_percent_lte: Option<f64>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FinancialFloat>;

    /// Same as [`Self::list_stocks_floats`], but takes the optional arguments as a
    /// chainable [`ListStocksFloatsParams`] struct.
    fn list_stocks_floats_with_params<'a>(
        &'a self,
        params: ListStocksFloatsParams,
    ) -> BoxStream<'a, FinancialFloat>;
}

impl FinancialsApi for Client {
    fn list_financials_balance_sheets<'a>(
        &'a self,
        cik: Option<&'a str>,
        cik_any_of: Option<&'a str>,
        cik_gt: Option<&'a str>,
        cik_gte: Option<&'a str>,
        cik_lt: Option<&'a str>,
        cik_lte: Option<&'a str>,
        tickers: Option<&'a str>,
        tickers_all_of: Option<&'a str>,
        tickers_any_of: Option<&'a str>,
        period_end: Option<&'a str>,
        period_end_gt: Option<&'a str>,
        period_end_gte: Option<&'a str>,
        period_end_lt: Option<&'a str>,
        period_end_lte: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
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
        timeframe: Option<&'a str>,
        timeframe_any_of: Option<&'a str>,
        timeframe_gt: Option<&'a str>,
        timeframe_gte: Option<&'a str>,
        timeframe_lt: Option<&'a str>,
        timeframe_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FinancialBalanceSheet> {
        self.list_financials_balance_sheets_with_params(ListFinancialsBalanceSheetsParams {
            cik: cik.map(String::from),
            cik_any_of: cik_any_of.map(String::from),
            cik_gt: cik_gt.map(String::from),
            cik_gte: cik_gte.map(String::from),
            cik_lt: cik_lt.map(String::from),
            cik_lte: cik_lte.map(String::from),
            tickers: tickers.map(String::from),
            tickers_all_of: tickers_all_of.map(String::from),
            tickers_any_of: tickers_any_of.map(String::from),
            period_end: period_end.map(String::from),
            period_end_gt: period_end_gt.map(String::from),
            period_end_gte: period_end_gte.map(String::from),
            period_end_lt: period_end_lt.map(String::from),
            period_end_lte: period_end_lte.map(String::from),
            filing_date: filing_date.map(String::from),
            filing_date_gt: filing_date_gt.map(String::from),
            filing_date_gte: filing_date_gte.map(String::from),
            filing_date_lt: filing_date_lt.map(String::from),
            filing_date_lte: filing_date_lte.map(String::from),
            fiscal_year,
            fiscal_year_gt,
            fiscal_year_gte,
            fiscal_year_lt,
            fiscal_year_lte,
            fiscal_quarter,
            fiscal_quarter_gt,
            fiscal_quarter_gte,
            fiscal_quarter_lt,
            fiscal_quarter_lte,
            timeframe: timeframe.map(String::from),
            timeframe_any_of: timeframe_any_of.map(String::from),
            timeframe_gt: timeframe_gt.map(String::from),
            timeframe_gte: timeframe_gte.map(String::from),
            timeframe_lt: timeframe_lt.map(String::from),
            timeframe_lte: timeframe_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_financials_balance_sheets_with_params<'a>(
        &'a self,
        params: ListFinancialsBalanceSheetsParams,
    ) -> BoxStream<'a, FinancialBalanceSheet> {
        Box::pin({
            let ListFinancialsBalanceSheetsParams {
                cik,
                cik_any_of,
                cik_gt,
                cik_gte,
                cik_lt,
                cik_lte,
                tickers,
                tickers_all_of,
                tickers_any_of,
                period_end,
                period_end_gt,
                period_end_gte,
                period_end_lt,
                period_end_lte,
                filing_date,
                filing_date_gt,
                filing_date_gte,
                filing_date_lt,
                filing_date_lte,
                fiscal_year,
                fiscal_year_gt,
                fiscal_year_gte,
                fiscal_year_lt,
                fiscal_year_lte,
                fiscal_quarter,
                fiscal_quarter_gt,
                fiscal_quarter_gte,
                fiscal_quarter_lt,
                fiscal_quarter_lte,
                timeframe,
                timeframe_any_of,
                timeframe_gt,
                timeframe_gte,
                timeframe_lt,
                timeframe_lte,
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
            let tickers = tickers.as_deref();
            let tickers_all_of = tickers_all_of.as_deref();
            let tickers_any_of = tickers_any_of.as_deref();
            let period_end = period_end.as_deref();
            let period_end_gt = period_end_gt.as_deref();
            let period_end_gte = period_end_gte.as_deref();
            let period_end_lt = period_end_lt.as_deref();
            let period_end_lte = period_end_lte.as_deref();
            let filing_date = filing_date.as_deref();
            let filing_date_gt = filing_date_gt.as_deref();
            let filing_date_gte = filing_date_gte.as_deref();
            let filing_date_lt = filing_date_lt.as_deref();
            let filing_date_lte = filing_date_lte.as_deref();
            let timeframe = timeframe.as_deref();
            let timeframe_any_of = timeframe_any_of.as_deref();
            let timeframe_gt = timeframe_gt.as_deref();
            let timeframe_gte = timeframe_gte.as_deref();
            let timeframe_lt = timeframe_lt.as_deref();
            let timeframe_lte = timeframe_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/stocks/financials/v1/balance-sheets".to_string();
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
            if let Some(v) = tickers {
                query.push(("tickers", v.to_string()));
            }
            if let Some(v) = tickers_all_of {
                query.push(("tickers_all_of", v.to_string()));
            }
            if let Some(v) = tickers_any_of {
                query.push(("tickers.any_of", v.to_string()));
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
            if let Some(v) = fiscal_year {
                query.push(("fiscal_year", v.to_string()));
            }
            if let Some(v) = fiscal_year_gt {
                query.push(("fiscal_year.gt", v.to_string()));
            }
            if let Some(v) = fiscal_year_gte {
                query.push(("fiscal_year.gte", v.to_string()));
            }
            if let Some(v) = fiscal_year_lt {
                query.push(("fiscal_year.lt", v.to_string()));
            }
            if let Some(v) = fiscal_year_lte {
                query.push(("fiscal_year.lte", v.to_string()));
            }
            if let Some(v) = fiscal_quarter {
                query.push(("fiscal_quarter", v.to_string()));
            }
            if let Some(v) = fiscal_quarter_gt {
                query.push(("fiscal_quarter.gt", v.to_string()));
            }
            if let Some(v) = fiscal_quarter_gte {
                query.push(("fiscal_quarter.gte", v.to_string()));
            }
            if let Some(v) = fiscal_quarter_lt {
                query.push(("fiscal_quarter.lt", v.to_string()));
            }
            if let Some(v) = fiscal_quarter_lte {
                query.push(("fiscal_quarter.lte", v.to_string()));
            }
            if let Some(v) = timeframe {
                query.push(("timeframe", v.to_string()));
            }
            if let Some(v) = timeframe_any_of {
                query.push(("timeframe.any_of", v.to_string()));
            }
            if let Some(v) = timeframe_gt {
                query.push(("timeframe.gt", v.to_string()));
            }
            if let Some(v) = timeframe_gte {
                query.push(("timeframe.gte", v.to_string()));
            }
            if let Some(v) = timeframe_lt {
                query.push(("timeframe.lt", v.to_string()));
            }
            if let Some(v) = timeframe_lte {
                query.push(("timeframe.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<FinancialBalanceSheet>(&path, Some(&query), options)
        })
    }

    fn list_financials_cash_flow_statements<'a>(
        &'a self,
        cik: Option<&'a str>,
        cik_any_of: Option<&'a str>,
        cik_gt: Option<&'a str>,
        cik_gte: Option<&'a str>,
        cik_lt: Option<&'a str>,
        cik_lte: Option<&'a str>,
        period_end: Option<&'a str>,
        period_end_gt: Option<&'a str>,
        period_end_gte: Option<&'a str>,
        period_end_lt: Option<&'a str>,
        period_end_lte: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
        tickers: Option<&'a str>,
        tickers_all_of: Option<&'a str>,
        tickers_any_of: Option<&'a str>,
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
        timeframe: Option<&'a str>,
        timeframe_any_of: Option<&'a str>,
        timeframe_gt: Option<&'a str>,
        timeframe_gte: Option<&'a str>,
        timeframe_lt: Option<&'a str>,
        timeframe_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FinancialCashFlowStatement> {
        self.list_financials_cash_flow_statements_with_params(
            ListFinancialsCashFlowStatementsParams {
                cik: cik.map(String::from),
                cik_any_of: cik_any_of.map(String::from),
                cik_gt: cik_gt.map(String::from),
                cik_gte: cik_gte.map(String::from),
                cik_lt: cik_lt.map(String::from),
                cik_lte: cik_lte.map(String::from),
                period_end: period_end.map(String::from),
                period_end_gt: period_end_gt.map(String::from),
                period_end_gte: period_end_gte.map(String::from),
                period_end_lt: period_end_lt.map(String::from),
                period_end_lte: period_end_lte.map(String::from),
                filing_date: filing_date.map(String::from),
                filing_date_gt: filing_date_gt.map(String::from),
                filing_date_gte: filing_date_gte.map(String::from),
                filing_date_lt: filing_date_lt.map(String::from),
                filing_date_lte: filing_date_lte.map(String::from),
                tickers: tickers.map(String::from),
                tickers_all_of: tickers_all_of.map(String::from),
                tickers_any_of: tickers_any_of.map(String::from),
                fiscal_year,
                fiscal_year_gt,
                fiscal_year_gte,
                fiscal_year_lt,
                fiscal_year_lte,
                fiscal_quarter,
                fiscal_quarter_gt,
                fiscal_quarter_gte,
                fiscal_quarter_lt,
                fiscal_quarter_lte,
                timeframe: timeframe.map(String::from),
                timeframe_any_of: timeframe_any_of.map(String::from),
                timeframe_gt: timeframe_gt.map(String::from),
                timeframe_gte: timeframe_gte.map(String::from),
                timeframe_lt: timeframe_lt.map(String::from),
                timeframe_lte: timeframe_lte.map(String::from),
                limit,
                sort: sort.map(String::from),
                options: options.cloned(),
            },
        )
    }

    fn list_financials_cash_flow_statements_with_params<'a>(
        &'a self,
        params: ListFinancialsCashFlowStatementsParams,
    ) -> BoxStream<'a, FinancialCashFlowStatement> {
        Box::pin({
            let ListFinancialsCashFlowStatementsParams {
                cik,
                cik_any_of,
                cik_gt,
                cik_gte,
                cik_lt,
                cik_lte,
                period_end,
                period_end_gt,
                period_end_gte,
                period_end_lt,
                period_end_lte,
                filing_date,
                filing_date_gt,
                filing_date_gte,
                filing_date_lt,
                filing_date_lte,
                tickers,
                tickers_all_of,
                tickers_any_of,
                fiscal_year,
                fiscal_year_gt,
                fiscal_year_gte,
                fiscal_year_lt,
                fiscal_year_lte,
                fiscal_quarter,
                fiscal_quarter_gt,
                fiscal_quarter_gte,
                fiscal_quarter_lt,
                fiscal_quarter_lte,
                timeframe,
                timeframe_any_of,
                timeframe_gt,
                timeframe_gte,
                timeframe_lt,
                timeframe_lte,
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
            let period_end = period_end.as_deref();
            let period_end_gt = period_end_gt.as_deref();
            let period_end_gte = period_end_gte.as_deref();
            let period_end_lt = period_end_lt.as_deref();
            let period_end_lte = period_end_lte.as_deref();
            let filing_date = filing_date.as_deref();
            let filing_date_gt = filing_date_gt.as_deref();
            let filing_date_gte = filing_date_gte.as_deref();
            let filing_date_lt = filing_date_lt.as_deref();
            let filing_date_lte = filing_date_lte.as_deref();
            let tickers = tickers.as_deref();
            let tickers_all_of = tickers_all_of.as_deref();
            let tickers_any_of = tickers_any_of.as_deref();
            let timeframe = timeframe.as_deref();
            let timeframe_any_of = timeframe_any_of.as_deref();
            let timeframe_gt = timeframe_gt.as_deref();
            let timeframe_gte = timeframe_gte.as_deref();
            let timeframe_lt = timeframe_lt.as_deref();
            let timeframe_lte = timeframe_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/stocks/financials/v1/cash-flow-statements".to_string();
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
            if let Some(v) = tickers {
                query.push(("tickers", v.to_string()));
            }
            if let Some(v) = tickers_all_of {
                query.push(("tickers_all_of", v.to_string()));
            }
            if let Some(v) = tickers_any_of {
                query.push(("tickers.any_of", v.to_string()));
            }
            if let Some(v) = fiscal_year {
                query.push(("fiscal_year", v.to_string()));
            }
            if let Some(v) = fiscal_year_gt {
                query.push(("fiscal_year.gt", v.to_string()));
            }
            if let Some(v) = fiscal_year_gte {
                query.push(("fiscal_year.gte", v.to_string()));
            }
            if let Some(v) = fiscal_year_lt {
                query.push(("fiscal_year.lt", v.to_string()));
            }
            if let Some(v) = fiscal_year_lte {
                query.push(("fiscal_year.lte", v.to_string()));
            }
            if let Some(v) = fiscal_quarter {
                query.push(("fiscal_quarter", v.to_string()));
            }
            if let Some(v) = fiscal_quarter_gt {
                query.push(("fiscal_quarter.gt", v.to_string()));
            }
            if let Some(v) = fiscal_quarter_gte {
                query.push(("fiscal_quarter.gte", v.to_string()));
            }
            if let Some(v) = fiscal_quarter_lt {
                query.push(("fiscal_quarter.lt", v.to_string()));
            }
            if let Some(v) = fiscal_quarter_lte {
                query.push(("fiscal_quarter.lte", v.to_string()));
            }
            if let Some(v) = timeframe {
                query.push(("timeframe", v.to_string()));
            }
            if let Some(v) = timeframe_any_of {
                query.push(("timeframe.any_of", v.to_string()));
            }
            if let Some(v) = timeframe_gt {
                query.push(("timeframe.gt", v.to_string()));
            }
            if let Some(v) = timeframe_gte {
                query.push(("timeframe.gte", v.to_string()));
            }
            if let Some(v) = timeframe_lt {
                query.push(("timeframe.lt", v.to_string()));
            }
            if let Some(v) = timeframe_lte {
                query.push(("timeframe.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<FinancialCashFlowStatement>(&path, Some(&query), options)
        })
    }

    fn list_financials_income_statements<'a>(
        &'a self,
        cik: Option<&'a str>,
        cik_any_of: Option<&'a str>,
        cik_gt: Option<&'a str>,
        cik_gte: Option<&'a str>,
        cik_lt: Option<&'a str>,
        cik_lte: Option<&'a str>,
        tickers: Option<&'a str>,
        tickers_all_of: Option<&'a str>,
        tickers_any_of: Option<&'a str>,
        period_end: Option<&'a str>,
        period_end_gt: Option<&'a str>,
        period_end_gte: Option<&'a str>,
        period_end_lt: Option<&'a str>,
        period_end_lte: Option<&'a str>,
        filing_date: Option<&'a str>,
        filing_date_gt: Option<&'a str>,
        filing_date_gte: Option<&'a str>,
        filing_date_lt: Option<&'a str>,
        filing_date_lte: Option<&'a str>,
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
        timeframe: Option<&'a str>,
        timeframe_any_of: Option<&'a str>,
        timeframe_gt: Option<&'a str>,
        timeframe_gte: Option<&'a str>,
        timeframe_lt: Option<&'a str>,
        timeframe_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FinancialIncomeStatement> {
        self.list_financials_income_statements_with_params(ListFinancialsIncomeStatementsParams {
            cik: cik.map(String::from),
            cik_any_of: cik_any_of.map(String::from),
            cik_gt: cik_gt.map(String::from),
            cik_gte: cik_gte.map(String::from),
            cik_lt: cik_lt.map(String::from),
            cik_lte: cik_lte.map(String::from),
            tickers: tickers.map(String::from),
            tickers_all_of: tickers_all_of.map(String::from),
            tickers_any_of: tickers_any_of.map(String::from),
            period_end: period_end.map(String::from),
            period_end_gt: period_end_gt.map(String::from),
            period_end_gte: period_end_gte.map(String::from),
            period_end_lt: period_end_lt.map(String::from),
            period_end_lte: period_end_lte.map(String::from),
            filing_date: filing_date.map(String::from),
            filing_date_gt: filing_date_gt.map(String::from),
            filing_date_gte: filing_date_gte.map(String::from),
            filing_date_lt: filing_date_lt.map(String::from),
            filing_date_lte: filing_date_lte.map(String::from),
            fiscal_year,
            fiscal_year_gt,
            fiscal_year_gte,
            fiscal_year_lt,
            fiscal_year_lte,
            fiscal_quarter,
            fiscal_quarter_gt,
            fiscal_quarter_gte,
            fiscal_quarter_lt,
            fiscal_quarter_lte,
            timeframe: timeframe.map(String::from),
            timeframe_any_of: timeframe_any_of.map(String::from),
            timeframe_gt: timeframe_gt.map(String::from),
            timeframe_gte: timeframe_gte.map(String::from),
            timeframe_lt: timeframe_lt.map(String::from),
            timeframe_lte: timeframe_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_financials_income_statements_with_params<'a>(
        &'a self,
        params: ListFinancialsIncomeStatementsParams,
    ) -> BoxStream<'a, FinancialIncomeStatement> {
        Box::pin({
            let ListFinancialsIncomeStatementsParams {
                cik,
                cik_any_of,
                cik_gt,
                cik_gte,
                cik_lt,
                cik_lte,
                tickers,
                tickers_all_of,
                tickers_any_of,
                period_end,
                period_end_gt,
                period_end_gte,
                period_end_lt,
                period_end_lte,
                filing_date,
                filing_date_gt,
                filing_date_gte,
                filing_date_lt,
                filing_date_lte,
                fiscal_year,
                fiscal_year_gt,
                fiscal_year_gte,
                fiscal_year_lt,
                fiscal_year_lte,
                fiscal_quarter,
                fiscal_quarter_gt,
                fiscal_quarter_gte,
                fiscal_quarter_lt,
                fiscal_quarter_lte,
                timeframe,
                timeframe_any_of,
                timeframe_gt,
                timeframe_gte,
                timeframe_lt,
                timeframe_lte,
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
            let tickers = tickers.as_deref();
            let tickers_all_of = tickers_all_of.as_deref();
            let tickers_any_of = tickers_any_of.as_deref();
            let period_end = period_end.as_deref();
            let period_end_gt = period_end_gt.as_deref();
            let period_end_gte = period_end_gte.as_deref();
            let period_end_lt = period_end_lt.as_deref();
            let period_end_lte = period_end_lte.as_deref();
            let filing_date = filing_date.as_deref();
            let filing_date_gt = filing_date_gt.as_deref();
            let filing_date_gte = filing_date_gte.as_deref();
            let filing_date_lt = filing_date_lt.as_deref();
            let filing_date_lte = filing_date_lte.as_deref();
            let timeframe = timeframe.as_deref();
            let timeframe_any_of = timeframe_any_of.as_deref();
            let timeframe_gt = timeframe_gt.as_deref();
            let timeframe_gte = timeframe_gte.as_deref();
            let timeframe_lt = timeframe_lt.as_deref();
            let timeframe_lte = timeframe_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/stocks/financials/v1/income-statements".to_string();
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
            if let Some(v) = tickers {
                query.push(("tickers", v.to_string()));
            }
            if let Some(v) = tickers_all_of {
                query.push(("tickers_all_of", v.to_string()));
            }
            if let Some(v) = tickers_any_of {
                query.push(("tickers.any_of", v.to_string()));
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
            if let Some(v) = fiscal_year {
                query.push(("fiscal_year", v.to_string()));
            }
            if let Some(v) = fiscal_year_gt {
                query.push(("fiscal_year.gt", v.to_string()));
            }
            if let Some(v) = fiscal_year_gte {
                query.push(("fiscal_year.gte", v.to_string()));
            }
            if let Some(v) = fiscal_year_lt {
                query.push(("fiscal_year.lt", v.to_string()));
            }
            if let Some(v) = fiscal_year_lte {
                query.push(("fiscal_year.lte", v.to_string()));
            }
            if let Some(v) = fiscal_quarter {
                query.push(("fiscal_quarter", v.to_string()));
            }
            if let Some(v) = fiscal_quarter_gt {
                query.push(("fiscal_quarter.gt", v.to_string()));
            }
            if let Some(v) = fiscal_quarter_gte {
                query.push(("fiscal_quarter.gte", v.to_string()));
            }
            if let Some(v) = fiscal_quarter_lt {
                query.push(("fiscal_quarter.lt", v.to_string()));
            }
            if let Some(v) = fiscal_quarter_lte {
                query.push(("fiscal_quarter.lte", v.to_string()));
            }
            if let Some(v) = timeframe {
                query.push(("timeframe", v.to_string()));
            }
            if let Some(v) = timeframe_any_of {
                query.push(("timeframe.any_of", v.to_string()));
            }
            if let Some(v) = timeframe_gt {
                query.push(("timeframe.gt", v.to_string()));
            }
            if let Some(v) = timeframe_gte {
                query.push(("timeframe.gte", v.to_string()));
            }
            if let Some(v) = timeframe_lt {
                query.push(("timeframe.lt", v.to_string()));
            }
            if let Some(v) = timeframe_lte {
                query.push(("timeframe.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<FinancialIncomeStatement>(&path, Some(&query), options)
        })
    }

    fn list_financials_ratios<'a>(
        &'a self,
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
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FinancialRatio> {
        self.list_financials_ratios_with_params(ListFinancialsRatiosParams {
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
            price,
            price_gt,
            price_gte,
            price_lt,
            price_lte,
            average_volume,
            average_volume_gt,
            average_volume_gte,
            average_volume_lt,
            average_volume_lte,
            market_cap,
            market_cap_gt,
            market_cap_gte,
            market_cap_lt,
            market_cap_lte,
            earnings_per_share,
            earnings_per_share_gt,
            earnings_per_share_gte,
            earnings_per_share_lt,
            earnings_per_share_lte,
            price_to_earnings,
            price_to_earnings_gt,
            price_to_earnings_gte,
            price_to_earnings_lt,
            price_to_earnings_lte,
            price_to_book,
            price_to_book_gt,
            price_to_book_gte,
            price_to_book_lt,
            price_to_book_lte,
            price_to_sales,
            price_to_sales_gt,
            price_to_sales_gte,
            price_to_sales_lt,
            price_to_sales_lte,
            price_to_cash_flow,
            price_to_cash_flow_gt,
            price_to_cash_flow_gte,
            price_to_cash_flow_lt,
            price_to_cash_flow_lte,
            price_to_free_cash_flow,
            price_to_free_cash_flow_gt,
            price_to_free_cash_flow_gte,
            price_to_free_cash_flow_lt,
            price_to_free_cash_flow_lte,
            dividend_yield,
            dividend_yield_gt,
            dividend_yield_gte,
            dividend_yield_lt,
            dividend_yield_lte,
            return_on_assets,
            return_on_assets_gt,
            return_on_assets_gte,
            return_on_assets_lt,
            return_on_assets_lte,
            return_on_equity,
            return_on_equity_gt,
            return_on_equity_gte,
            return_on_equity_lt,
            return_on_equity_lte,
            debt_to_equity,
            debt_to_equity_gt,
            debt_to_equity_gte,
            debt_to_equity_lt,
            debt_to_equity_lte,
            current,
            current_gt,
            current_gte,
            current_lt,
            current_lte,
            quick,
            quick_gt,
            quick_gte,
            quick_lt,
            quick_lte,
            cash,
            cash_gt,
            cash_gte,
            cash_lt,
            cash_lte,
            ev_to_sales,
            ev_to_sales_gt,
            ev_to_sales_gte,
            ev_to_sales_lt,
            ev_to_sales_lte,
            ev_to_ebitda,
            ev_to_ebitda_gt,
            ev_to_ebitda_gte,
            ev_to_ebitda_lt,
            ev_to_ebitda_lte,
            enterprise_value,
            enterprise_value_gt,
            enterprise_value_gte,
            enterprise_value_lt,
            enterprise_value_lte,
            free_cash_flow,
            free_cash_flow_gt,
            free_cash_flow_gte,
            free_cash_flow_lt,
            free_cash_flow_lte,
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_financials_ratios_with_params<'a>(
        &'a self,
        params: ListFinancialsRatiosParams,
    ) -> BoxStream<'a, FinancialRatio> {
        Box::pin({
            let ListFinancialsRatiosParams {
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
                price,
                price_gt,
                price_gte,
                price_lt,
                price_lte,
                average_volume,
                average_volume_gt,
                average_volume_gte,
                average_volume_lt,
                average_volume_lte,
                market_cap,
                market_cap_gt,
                market_cap_gte,
                market_cap_lt,
                market_cap_lte,
                earnings_per_share,
                earnings_per_share_gt,
                earnings_per_share_gte,
                earnings_per_share_lt,
                earnings_per_share_lte,
                price_to_earnings,
                price_to_earnings_gt,
                price_to_earnings_gte,
                price_to_earnings_lt,
                price_to_earnings_lte,
                price_to_book,
                price_to_book_gt,
                price_to_book_gte,
                price_to_book_lt,
                price_to_book_lte,
                price_to_sales,
                price_to_sales_gt,
                price_to_sales_gte,
                price_to_sales_lt,
                price_to_sales_lte,
                price_to_cash_flow,
                price_to_cash_flow_gt,
                price_to_cash_flow_gte,
                price_to_cash_flow_lt,
                price_to_cash_flow_lte,
                price_to_free_cash_flow,
                price_to_free_cash_flow_gt,
                price_to_free_cash_flow_gte,
                price_to_free_cash_flow_lt,
                price_to_free_cash_flow_lte,
                dividend_yield,
                dividend_yield_gt,
                dividend_yield_gte,
                dividend_yield_lt,
                dividend_yield_lte,
                return_on_assets,
                return_on_assets_gt,
                return_on_assets_gte,
                return_on_assets_lt,
                return_on_assets_lte,
                return_on_equity,
                return_on_equity_gt,
                return_on_equity_gte,
                return_on_equity_lt,
                return_on_equity_lte,
                debt_to_equity,
                debt_to_equity_gt,
                debt_to_equity_gte,
                debt_to_equity_lt,
                debt_to_equity_lte,
                current,
                current_gt,
                current_gte,
                current_lt,
                current_lte,
                quick,
                quick_gt,
                quick_gte,
                quick_lt,
                quick_lte,
                cash,
                cash_gt,
                cash_gte,
                cash_lt,
                cash_lte,
                ev_to_sales,
                ev_to_sales_gt,
                ev_to_sales_gte,
                ev_to_sales_lt,
                ev_to_sales_lte,
                ev_to_ebitda,
                ev_to_ebitda_gt,
                ev_to_ebitda_gte,
                ev_to_ebitda_lt,
                ev_to_ebitda_lte,
                enterprise_value,
                enterprise_value_gt,
                enterprise_value_gte,
                enterprise_value_lt,
                enterprise_value_lte,
                free_cash_flow,
                free_cash_flow_gt,
                free_cash_flow_gte,
                free_cash_flow_lt,
                free_cash_flow_lte,
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
            let cik = cik.as_deref();
            let cik_any_of = cik_any_of.as_deref();
            let cik_gt = cik_gt.as_deref();
            let cik_gte = cik_gte.as_deref();
            let cik_lt = cik_lt.as_deref();
            let cik_lte = cik_lte.as_deref();
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/stocks/financials/v1/ratios".to_string();
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
            if let Some(v) = price {
                query.push(("price", v.to_string()));
            }
            if let Some(v) = price_gt {
                query.push(("price.gt", v.to_string()));
            }
            if let Some(v) = price_gte {
                query.push(("price.gte", v.to_string()));
            }
            if let Some(v) = price_lt {
                query.push(("price.lt", v.to_string()));
            }
            if let Some(v) = price_lte {
                query.push(("price.lte", v.to_string()));
            }
            if let Some(v) = average_volume {
                query.push(("average_volume", v.to_string()));
            }
            if let Some(v) = average_volume_gt {
                query.push(("average_volume.gt", v.to_string()));
            }
            if let Some(v) = average_volume_gte {
                query.push(("average_volume.gte", v.to_string()));
            }
            if let Some(v) = average_volume_lt {
                query.push(("average_volume.lt", v.to_string()));
            }
            if let Some(v) = average_volume_lte {
                query.push(("average_volume.lte", v.to_string()));
            }
            if let Some(v) = market_cap {
                query.push(("market_cap", v.to_string()));
            }
            if let Some(v) = market_cap_gt {
                query.push(("market_cap.gt", v.to_string()));
            }
            if let Some(v) = market_cap_gte {
                query.push(("market_cap.gte", v.to_string()));
            }
            if let Some(v) = market_cap_lt {
                query.push(("market_cap.lt", v.to_string()));
            }
            if let Some(v) = market_cap_lte {
                query.push(("market_cap.lte", v.to_string()));
            }
            if let Some(v) = earnings_per_share {
                query.push(("earnings_per_share", v.to_string()));
            }
            if let Some(v) = earnings_per_share_gt {
                query.push(("earnings_per_share.gt", v.to_string()));
            }
            if let Some(v) = earnings_per_share_gte {
                query.push(("earnings_per_share.gte", v.to_string()));
            }
            if let Some(v) = earnings_per_share_lt {
                query.push(("earnings_per_share.lt", v.to_string()));
            }
            if let Some(v) = earnings_per_share_lte {
                query.push(("earnings_per_share.lte", v.to_string()));
            }
            if let Some(v) = price_to_earnings {
                query.push(("price_to_earnings", v.to_string()));
            }
            if let Some(v) = price_to_earnings_gt {
                query.push(("price_to_earnings.gt", v.to_string()));
            }
            if let Some(v) = price_to_earnings_gte {
                query.push(("price_to_earnings.gte", v.to_string()));
            }
            if let Some(v) = price_to_earnings_lt {
                query.push(("price_to_earnings.lt", v.to_string()));
            }
            if let Some(v) = price_to_earnings_lte {
                query.push(("price_to_earnings.lte", v.to_string()));
            }
            if let Some(v) = price_to_book {
                query.push(("price_to_book", v.to_string()));
            }
            if let Some(v) = price_to_book_gt {
                query.push(("price_to_book.gt", v.to_string()));
            }
            if let Some(v) = price_to_book_gte {
                query.push(("price_to_book.gte", v.to_string()));
            }
            if let Some(v) = price_to_book_lt {
                query.push(("price_to_book.lt", v.to_string()));
            }
            if let Some(v) = price_to_book_lte {
                query.push(("price_to_book.lte", v.to_string()));
            }
            if let Some(v) = price_to_sales {
                query.push(("price_to_sales", v.to_string()));
            }
            if let Some(v) = price_to_sales_gt {
                query.push(("price_to_sales.gt", v.to_string()));
            }
            if let Some(v) = price_to_sales_gte {
                query.push(("price_to_sales.gte", v.to_string()));
            }
            if let Some(v) = price_to_sales_lt {
                query.push(("price_to_sales.lt", v.to_string()));
            }
            if let Some(v) = price_to_sales_lte {
                query.push(("price_to_sales.lte", v.to_string()));
            }
            if let Some(v) = price_to_cash_flow {
                query.push(("price_to_cash_flow", v.to_string()));
            }
            if let Some(v) = price_to_cash_flow_gt {
                query.push(("price_to_cash_flow.gt", v.to_string()));
            }
            if let Some(v) = price_to_cash_flow_gte {
                query.push(("price_to_cash_flow.gte", v.to_string()));
            }
            if let Some(v) = price_to_cash_flow_lt {
                query.push(("price_to_cash_flow.lt", v.to_string()));
            }
            if let Some(v) = price_to_cash_flow_lte {
                query.push(("price_to_cash_flow.lte", v.to_string()));
            }
            if let Some(v) = price_to_free_cash_flow {
                query.push(("price_to_free_cash_flow", v.to_string()));
            }
            if let Some(v) = price_to_free_cash_flow_gt {
                query.push(("price_to_free_cash_flow.gt", v.to_string()));
            }
            if let Some(v) = price_to_free_cash_flow_gte {
                query.push(("price_to_free_cash_flow.gte", v.to_string()));
            }
            if let Some(v) = price_to_free_cash_flow_lt {
                query.push(("price_to_free_cash_flow.lt", v.to_string()));
            }
            if let Some(v) = price_to_free_cash_flow_lte {
                query.push(("price_to_free_cash_flow.lte", v.to_string()));
            }
            if let Some(v) = dividend_yield {
                query.push(("dividend_yield", v.to_string()));
            }
            if let Some(v) = dividend_yield_gt {
                query.push(("dividend_yield.gt", v.to_string()));
            }
            if let Some(v) = dividend_yield_gte {
                query.push(("dividend_yield.gte", v.to_string()));
            }
            if let Some(v) = dividend_yield_lt {
                query.push(("dividend_yield.lt", v.to_string()));
            }
            if let Some(v) = dividend_yield_lte {
                query.push(("dividend_yield.lte", v.to_string()));
            }
            if let Some(v) = return_on_assets {
                query.push(("return_on_assets", v.to_string()));
            }
            if let Some(v) = return_on_assets_gt {
                query.push(("return_on_assets.gt", v.to_string()));
            }
            if let Some(v) = return_on_assets_gte {
                query.push(("return_on_assets.gte", v.to_string()));
            }
            if let Some(v) = return_on_assets_lt {
                query.push(("return_on_assets.lt", v.to_string()));
            }
            if let Some(v) = return_on_assets_lte {
                query.push(("return_on_assets.lte", v.to_string()));
            }
            if let Some(v) = return_on_equity {
                query.push(("return_on_equity", v.to_string()));
            }
            if let Some(v) = return_on_equity_gt {
                query.push(("return_on_equity.gt", v.to_string()));
            }
            if let Some(v) = return_on_equity_gte {
                query.push(("return_on_equity.gte", v.to_string()));
            }
            if let Some(v) = return_on_equity_lt {
                query.push(("return_on_equity.lt", v.to_string()));
            }
            if let Some(v) = return_on_equity_lte {
                query.push(("return_on_equity.lte", v.to_string()));
            }
            if let Some(v) = debt_to_equity {
                query.push(("debt_to_equity", v.to_string()));
            }
            if let Some(v) = debt_to_equity_gt {
                query.push(("debt_to_equity.gt", v.to_string()));
            }
            if let Some(v) = debt_to_equity_gte {
                query.push(("debt_to_equity.gte", v.to_string()));
            }
            if let Some(v) = debt_to_equity_lt {
                query.push(("debt_to_equity.lt", v.to_string()));
            }
            if let Some(v) = debt_to_equity_lte {
                query.push(("debt_to_equity.lte", v.to_string()));
            }
            if let Some(v) = current {
                query.push(("current", v.to_string()));
            }
            if let Some(v) = current_gt {
                query.push(("current.gt", v.to_string()));
            }
            if let Some(v) = current_gte {
                query.push(("current.gte", v.to_string()));
            }
            if let Some(v) = current_lt {
                query.push(("current.lt", v.to_string()));
            }
            if let Some(v) = current_lte {
                query.push(("current.lte", v.to_string()));
            }
            if let Some(v) = quick {
                query.push(("quick", v.to_string()));
            }
            if let Some(v) = quick_gt {
                query.push(("quick.gt", v.to_string()));
            }
            if let Some(v) = quick_gte {
                query.push(("quick.gte", v.to_string()));
            }
            if let Some(v) = quick_lt {
                query.push(("quick.lt", v.to_string()));
            }
            if let Some(v) = quick_lte {
                query.push(("quick.lte", v.to_string()));
            }
            if let Some(v) = cash {
                query.push(("cash", v.to_string()));
            }
            if let Some(v) = cash_gt {
                query.push(("cash.gt", v.to_string()));
            }
            if let Some(v) = cash_gte {
                query.push(("cash.gte", v.to_string()));
            }
            if let Some(v) = cash_lt {
                query.push(("cash.lt", v.to_string()));
            }
            if let Some(v) = cash_lte {
                query.push(("cash.lte", v.to_string()));
            }
            if let Some(v) = ev_to_sales {
                query.push(("ev_to_sales", v.to_string()));
            }
            if let Some(v) = ev_to_sales_gt {
                query.push(("ev_to_sales.gt", v.to_string()));
            }
            if let Some(v) = ev_to_sales_gte {
                query.push(("ev_to_sales.gte", v.to_string()));
            }
            if let Some(v) = ev_to_sales_lt {
                query.push(("ev_to_sales.lt", v.to_string()));
            }
            if let Some(v) = ev_to_sales_lte {
                query.push(("ev_to_sales.lte", v.to_string()));
            }
            if let Some(v) = ev_to_ebitda {
                query.push(("ev_to_ebitda", v.to_string()));
            }
            if let Some(v) = ev_to_ebitda_gt {
                query.push(("ev_to_ebitda.gt", v.to_string()));
            }
            if let Some(v) = ev_to_ebitda_gte {
                query.push(("ev_to_ebitda.gte", v.to_string()));
            }
            if let Some(v) = ev_to_ebitda_lt {
                query.push(("ev_to_ebitda.lt", v.to_string()));
            }
            if let Some(v) = ev_to_ebitda_lte {
                query.push(("ev_to_ebitda.lte", v.to_string()));
            }
            if let Some(v) = enterprise_value {
                query.push(("enterprise_value", v.to_string()));
            }
            if let Some(v) = enterprise_value_gt {
                query.push(("enterprise_value.gt", v.to_string()));
            }
            if let Some(v) = enterprise_value_gte {
                query.push(("enterprise_value.gte", v.to_string()));
            }
            if let Some(v) = enterprise_value_lt {
                query.push(("enterprise_value.lt", v.to_string()));
            }
            if let Some(v) = enterprise_value_lte {
                query.push(("enterprise_value.lte", v.to_string()));
            }
            if let Some(v) = free_cash_flow {
                query.push(("free_cash_flow", v.to_string()));
            }
            if let Some(v) = free_cash_flow_gt {
                query.push(("free_cash_flow.gt", v.to_string()));
            }
            if let Some(v) = free_cash_flow_gte {
                query.push(("free_cash_flow.gte", v.to_string()));
            }
            if let Some(v) = free_cash_flow_lt {
                query.push(("free_cash_flow.lt", v.to_string()));
            }
            if let Some(v) = free_cash_flow_lte {
                query.push(("free_cash_flow.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<FinancialRatio>(&path, Some(&query), options)
        })
    }

    fn list_stocks_floats<'a>(
        &'a self,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        free_float_percent: Option<f64>,
        free_float_percent_gt: Option<f64>,
        free_float_percent_gte: Option<f64>,
        free_float_percent_lt: Option<f64>,
        free_float_percent_lte: Option<f64>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, FinancialFloat> {
        self.list_stocks_floats_with_params(ListStocksFloatsParams {
            ticker: ticker.map(String::from),
            ticker_any_of: ticker_any_of.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            free_float_percent,
            free_float_percent_gt,
            free_float_percent_gte,
            free_float_percent_lt,
            free_float_percent_lte,
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_stocks_floats_with_params<'a>(
        &'a self,
        params: ListStocksFloatsParams,
    ) -> BoxStream<'a, FinancialFloat> {
        Box::pin({
            let ListStocksFloatsParams {
                ticker,
                ticker_any_of,
                ticker_gt,
                ticker_gte,
                ticker_lt,
                ticker_lte,
                free_float_percent,
                free_float_percent_gt,
                free_float_percent_gte,
                free_float_percent_lt,
                free_float_percent_lte,
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
            let sort = sort.as_deref();
            let options = options.as_ref();
            let path = "/stocks/vX/float".to_string();
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
            if let Some(v) = free_float_percent {
                query.push(("free_float_percent", v.to_string()));
            }
            if let Some(v) = free_float_percent_gt {
                query.push(("free_float_percent.gt", v.to_string()));
            }
            if let Some(v) = free_float_percent_gte {
                query.push(("free_float_percent.gte", v.to_string()));
            }
            if let Some(v) = free_float_percent_lt {
                query.push(("free_float_percent.lt", v.to_string()));
            }
            if let Some(v) = free_float_percent_lte {
                query.push(("free_float_percent.lte", v.to_string()));
            }
            if let Some(v) = limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = sort {
                query.push(("sort", v.to_string()));
            }
            self.list::<FinancialFloat>(&path, Some(&query), options)
        })
    }
}

// --- Params structs (additive builder API) ---

/// Optional arguments for [`FinancialsApi::list_financials_balance_sheets`].
#[derive(Debug, Default, Clone)]
pub struct ListFinancialsBalanceSheetsParams {
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
    /// The `tickers` argument.
    pub tickers: Option<String>,
    /// The `tickers_all_of` argument.
    pub tickers_all_of: Option<String>,
    /// The `tickers_any_of` argument.
    pub tickers_any_of: Option<String>,
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
    /// The `fiscal_year` argument.
    pub fiscal_year: Option<f64>,
    /// The `fiscal_year_gt` argument.
    pub fiscal_year_gt: Option<f64>,
    /// The `fiscal_year_gte` argument.
    pub fiscal_year_gte: Option<f64>,
    /// The `fiscal_year_lt` argument.
    pub fiscal_year_lt: Option<f64>,
    /// The `fiscal_year_lte` argument.
    pub fiscal_year_lte: Option<f64>,
    /// The `fiscal_quarter` argument.
    pub fiscal_quarter: Option<f64>,
    /// The `fiscal_quarter_gt` argument.
    pub fiscal_quarter_gt: Option<f64>,
    /// The `fiscal_quarter_gte` argument.
    pub fiscal_quarter_gte: Option<f64>,
    /// The `fiscal_quarter_lt` argument.
    pub fiscal_quarter_lt: Option<f64>,
    /// The `fiscal_quarter_lte` argument.
    pub fiscal_quarter_lte: Option<f64>,
    /// The `timeframe` argument.
    pub timeframe: Option<String>,
    /// The `timeframe_any_of` argument.
    pub timeframe_any_of: Option<String>,
    /// The `timeframe_gt` argument.
    pub timeframe_gt: Option<String>,
    /// The `timeframe_gte` argument.
    pub timeframe_gte: Option<String>,
    /// The `timeframe_lt` argument.
    pub timeframe_lt: Option<String>,
    /// The `timeframe_lte` argument.
    pub timeframe_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListFinancialsBalanceSheetsParams {
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

    /// Set the `fiscal_year` argument.
    pub fn fiscal_year(mut self, fiscal_year: f64) -> Self {
        self.fiscal_year = Some(fiscal_year);
        self
    }

    /// Set the `fiscal_year_gt` argument.
    pub fn fiscal_year_gt(mut self, fiscal_year_gt: f64) -> Self {
        self.fiscal_year_gt = Some(fiscal_year_gt);
        self
    }

    /// Set the `fiscal_year_gte` argument.
    pub fn fiscal_year_gte(mut self, fiscal_year_gte: f64) -> Self {
        self.fiscal_year_gte = Some(fiscal_year_gte);
        self
    }

    /// Set the `fiscal_year_lt` argument.
    pub fn fiscal_year_lt(mut self, fiscal_year_lt: f64) -> Self {
        self.fiscal_year_lt = Some(fiscal_year_lt);
        self
    }

    /// Set the `fiscal_year_lte` argument.
    pub fn fiscal_year_lte(mut self, fiscal_year_lte: f64) -> Self {
        self.fiscal_year_lte = Some(fiscal_year_lte);
        self
    }

    /// Set the `fiscal_quarter` argument.
    pub fn fiscal_quarter(mut self, fiscal_quarter: f64) -> Self {
        self.fiscal_quarter = Some(fiscal_quarter);
        self
    }

    /// Set the `fiscal_quarter_gt` argument.
    pub fn fiscal_quarter_gt(mut self, fiscal_quarter_gt: f64) -> Self {
        self.fiscal_quarter_gt = Some(fiscal_quarter_gt);
        self
    }

    /// Set the `fiscal_quarter_gte` argument.
    pub fn fiscal_quarter_gte(mut self, fiscal_quarter_gte: f64) -> Self {
        self.fiscal_quarter_gte = Some(fiscal_quarter_gte);
        self
    }

    /// Set the `fiscal_quarter_lt` argument.
    pub fn fiscal_quarter_lt(mut self, fiscal_quarter_lt: f64) -> Self {
        self.fiscal_quarter_lt = Some(fiscal_quarter_lt);
        self
    }

    /// Set the `fiscal_quarter_lte` argument.
    pub fn fiscal_quarter_lte(mut self, fiscal_quarter_lte: f64) -> Self {
        self.fiscal_quarter_lte = Some(fiscal_quarter_lte);
        self
    }

    /// Set the `timeframe` argument.
    pub fn timeframe(mut self, timeframe: impl Into<String>) -> Self {
        self.timeframe = Some(timeframe.into());
        self
    }

    /// Set the `timeframe_any_of` argument.
    pub fn timeframe_any_of(mut self, timeframe_any_of: impl Into<String>) -> Self {
        self.timeframe_any_of = Some(timeframe_any_of.into());
        self
    }

    /// Set the `timeframe_gt` argument.
    pub fn timeframe_gt(mut self, timeframe_gt: impl Into<String>) -> Self {
        self.timeframe_gt = Some(timeframe_gt.into());
        self
    }

    /// Set the `timeframe_gte` argument.
    pub fn timeframe_gte(mut self, timeframe_gte: impl Into<String>) -> Self {
        self.timeframe_gte = Some(timeframe_gte.into());
        self
    }

    /// Set the `timeframe_lt` argument.
    pub fn timeframe_lt(mut self, timeframe_lt: impl Into<String>) -> Self {
        self.timeframe_lt = Some(timeframe_lt.into());
        self
    }

    /// Set the `timeframe_lte` argument.
    pub fn timeframe_lte(mut self, timeframe_lte: impl Into<String>) -> Self {
        self.timeframe_lte = Some(timeframe_lte.into());
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

/// Optional arguments for [`FinancialsApi::list_financials_cash_flow_statements`].
#[derive(Debug, Default, Clone)]
pub struct ListFinancialsCashFlowStatementsParams {
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
    /// The `tickers` argument.
    pub tickers: Option<String>,
    /// The `tickers_all_of` argument.
    pub tickers_all_of: Option<String>,
    /// The `tickers_any_of` argument.
    pub tickers_any_of: Option<String>,
    /// The `fiscal_year` argument.
    pub fiscal_year: Option<f64>,
    /// The `fiscal_year_gt` argument.
    pub fiscal_year_gt: Option<f64>,
    /// The `fiscal_year_gte` argument.
    pub fiscal_year_gte: Option<f64>,
    /// The `fiscal_year_lt` argument.
    pub fiscal_year_lt: Option<f64>,
    /// The `fiscal_year_lte` argument.
    pub fiscal_year_lte: Option<f64>,
    /// The `fiscal_quarter` argument.
    pub fiscal_quarter: Option<f64>,
    /// The `fiscal_quarter_gt` argument.
    pub fiscal_quarter_gt: Option<f64>,
    /// The `fiscal_quarter_gte` argument.
    pub fiscal_quarter_gte: Option<f64>,
    /// The `fiscal_quarter_lt` argument.
    pub fiscal_quarter_lt: Option<f64>,
    /// The `fiscal_quarter_lte` argument.
    pub fiscal_quarter_lte: Option<f64>,
    /// The `timeframe` argument.
    pub timeframe: Option<String>,
    /// The `timeframe_any_of` argument.
    pub timeframe_any_of: Option<String>,
    /// The `timeframe_gt` argument.
    pub timeframe_gt: Option<String>,
    /// The `timeframe_gte` argument.
    pub timeframe_gte: Option<String>,
    /// The `timeframe_lt` argument.
    pub timeframe_lt: Option<String>,
    /// The `timeframe_lte` argument.
    pub timeframe_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListFinancialsCashFlowStatementsParams {
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

    /// Set the `fiscal_year` argument.
    pub fn fiscal_year(mut self, fiscal_year: f64) -> Self {
        self.fiscal_year = Some(fiscal_year);
        self
    }

    /// Set the `fiscal_year_gt` argument.
    pub fn fiscal_year_gt(mut self, fiscal_year_gt: f64) -> Self {
        self.fiscal_year_gt = Some(fiscal_year_gt);
        self
    }

    /// Set the `fiscal_year_gte` argument.
    pub fn fiscal_year_gte(mut self, fiscal_year_gte: f64) -> Self {
        self.fiscal_year_gte = Some(fiscal_year_gte);
        self
    }

    /// Set the `fiscal_year_lt` argument.
    pub fn fiscal_year_lt(mut self, fiscal_year_lt: f64) -> Self {
        self.fiscal_year_lt = Some(fiscal_year_lt);
        self
    }

    /// Set the `fiscal_year_lte` argument.
    pub fn fiscal_year_lte(mut self, fiscal_year_lte: f64) -> Self {
        self.fiscal_year_lte = Some(fiscal_year_lte);
        self
    }

    /// Set the `fiscal_quarter` argument.
    pub fn fiscal_quarter(mut self, fiscal_quarter: f64) -> Self {
        self.fiscal_quarter = Some(fiscal_quarter);
        self
    }

    /// Set the `fiscal_quarter_gt` argument.
    pub fn fiscal_quarter_gt(mut self, fiscal_quarter_gt: f64) -> Self {
        self.fiscal_quarter_gt = Some(fiscal_quarter_gt);
        self
    }

    /// Set the `fiscal_quarter_gte` argument.
    pub fn fiscal_quarter_gte(mut self, fiscal_quarter_gte: f64) -> Self {
        self.fiscal_quarter_gte = Some(fiscal_quarter_gte);
        self
    }

    /// Set the `fiscal_quarter_lt` argument.
    pub fn fiscal_quarter_lt(mut self, fiscal_quarter_lt: f64) -> Self {
        self.fiscal_quarter_lt = Some(fiscal_quarter_lt);
        self
    }

    /// Set the `fiscal_quarter_lte` argument.
    pub fn fiscal_quarter_lte(mut self, fiscal_quarter_lte: f64) -> Self {
        self.fiscal_quarter_lte = Some(fiscal_quarter_lte);
        self
    }

    /// Set the `timeframe` argument.
    pub fn timeframe(mut self, timeframe: impl Into<String>) -> Self {
        self.timeframe = Some(timeframe.into());
        self
    }

    /// Set the `timeframe_any_of` argument.
    pub fn timeframe_any_of(mut self, timeframe_any_of: impl Into<String>) -> Self {
        self.timeframe_any_of = Some(timeframe_any_of.into());
        self
    }

    /// Set the `timeframe_gt` argument.
    pub fn timeframe_gt(mut self, timeframe_gt: impl Into<String>) -> Self {
        self.timeframe_gt = Some(timeframe_gt.into());
        self
    }

    /// Set the `timeframe_gte` argument.
    pub fn timeframe_gte(mut self, timeframe_gte: impl Into<String>) -> Self {
        self.timeframe_gte = Some(timeframe_gte.into());
        self
    }

    /// Set the `timeframe_lt` argument.
    pub fn timeframe_lt(mut self, timeframe_lt: impl Into<String>) -> Self {
        self.timeframe_lt = Some(timeframe_lt.into());
        self
    }

    /// Set the `timeframe_lte` argument.
    pub fn timeframe_lte(mut self, timeframe_lte: impl Into<String>) -> Self {
        self.timeframe_lte = Some(timeframe_lte.into());
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

/// Optional arguments for [`FinancialsApi::list_financials_income_statements`].
#[derive(Debug, Default, Clone)]
pub struct ListFinancialsIncomeStatementsParams {
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
    /// The `tickers` argument.
    pub tickers: Option<String>,
    /// The `tickers_all_of` argument.
    pub tickers_all_of: Option<String>,
    /// The `tickers_any_of` argument.
    pub tickers_any_of: Option<String>,
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
    /// The `fiscal_year` argument.
    pub fiscal_year: Option<f64>,
    /// The `fiscal_year_gt` argument.
    pub fiscal_year_gt: Option<f64>,
    /// The `fiscal_year_gte` argument.
    pub fiscal_year_gte: Option<f64>,
    /// The `fiscal_year_lt` argument.
    pub fiscal_year_lt: Option<f64>,
    /// The `fiscal_year_lte` argument.
    pub fiscal_year_lte: Option<f64>,
    /// The `fiscal_quarter` argument.
    pub fiscal_quarter: Option<f64>,
    /// The `fiscal_quarter_gt` argument.
    pub fiscal_quarter_gt: Option<f64>,
    /// The `fiscal_quarter_gte` argument.
    pub fiscal_quarter_gte: Option<f64>,
    /// The `fiscal_quarter_lt` argument.
    pub fiscal_quarter_lt: Option<f64>,
    /// The `fiscal_quarter_lte` argument.
    pub fiscal_quarter_lte: Option<f64>,
    /// The `timeframe` argument.
    pub timeframe: Option<String>,
    /// The `timeframe_any_of` argument.
    pub timeframe_any_of: Option<String>,
    /// The `timeframe_gt` argument.
    pub timeframe_gt: Option<String>,
    /// The `timeframe_gte` argument.
    pub timeframe_gte: Option<String>,
    /// The `timeframe_lt` argument.
    pub timeframe_lt: Option<String>,
    /// The `timeframe_lte` argument.
    pub timeframe_lte: Option<String>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListFinancialsIncomeStatementsParams {
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

    /// Set the `fiscal_year` argument.
    pub fn fiscal_year(mut self, fiscal_year: f64) -> Self {
        self.fiscal_year = Some(fiscal_year);
        self
    }

    /// Set the `fiscal_year_gt` argument.
    pub fn fiscal_year_gt(mut self, fiscal_year_gt: f64) -> Self {
        self.fiscal_year_gt = Some(fiscal_year_gt);
        self
    }

    /// Set the `fiscal_year_gte` argument.
    pub fn fiscal_year_gte(mut self, fiscal_year_gte: f64) -> Self {
        self.fiscal_year_gte = Some(fiscal_year_gte);
        self
    }

    /// Set the `fiscal_year_lt` argument.
    pub fn fiscal_year_lt(mut self, fiscal_year_lt: f64) -> Self {
        self.fiscal_year_lt = Some(fiscal_year_lt);
        self
    }

    /// Set the `fiscal_year_lte` argument.
    pub fn fiscal_year_lte(mut self, fiscal_year_lte: f64) -> Self {
        self.fiscal_year_lte = Some(fiscal_year_lte);
        self
    }

    /// Set the `fiscal_quarter` argument.
    pub fn fiscal_quarter(mut self, fiscal_quarter: f64) -> Self {
        self.fiscal_quarter = Some(fiscal_quarter);
        self
    }

    /// Set the `fiscal_quarter_gt` argument.
    pub fn fiscal_quarter_gt(mut self, fiscal_quarter_gt: f64) -> Self {
        self.fiscal_quarter_gt = Some(fiscal_quarter_gt);
        self
    }

    /// Set the `fiscal_quarter_gte` argument.
    pub fn fiscal_quarter_gte(mut self, fiscal_quarter_gte: f64) -> Self {
        self.fiscal_quarter_gte = Some(fiscal_quarter_gte);
        self
    }

    /// Set the `fiscal_quarter_lt` argument.
    pub fn fiscal_quarter_lt(mut self, fiscal_quarter_lt: f64) -> Self {
        self.fiscal_quarter_lt = Some(fiscal_quarter_lt);
        self
    }

    /// Set the `fiscal_quarter_lte` argument.
    pub fn fiscal_quarter_lte(mut self, fiscal_quarter_lte: f64) -> Self {
        self.fiscal_quarter_lte = Some(fiscal_quarter_lte);
        self
    }

    /// Set the `timeframe` argument.
    pub fn timeframe(mut self, timeframe: impl Into<String>) -> Self {
        self.timeframe = Some(timeframe.into());
        self
    }

    /// Set the `timeframe_any_of` argument.
    pub fn timeframe_any_of(mut self, timeframe_any_of: impl Into<String>) -> Self {
        self.timeframe_any_of = Some(timeframe_any_of.into());
        self
    }

    /// Set the `timeframe_gt` argument.
    pub fn timeframe_gt(mut self, timeframe_gt: impl Into<String>) -> Self {
        self.timeframe_gt = Some(timeframe_gt.into());
        self
    }

    /// Set the `timeframe_gte` argument.
    pub fn timeframe_gte(mut self, timeframe_gte: impl Into<String>) -> Self {
        self.timeframe_gte = Some(timeframe_gte.into());
        self
    }

    /// Set the `timeframe_lt` argument.
    pub fn timeframe_lt(mut self, timeframe_lt: impl Into<String>) -> Self {
        self.timeframe_lt = Some(timeframe_lt.into());
        self
    }

    /// Set the `timeframe_lte` argument.
    pub fn timeframe_lte(mut self, timeframe_lte: impl Into<String>) -> Self {
        self.timeframe_lte = Some(timeframe_lte.into());
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

/// Optional arguments for [`FinancialsApi::list_financials_ratios`].
#[derive(Debug, Default, Clone)]
pub struct ListFinancialsRatiosParams {
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
    /// The `price` argument.
    pub price: Option<f64>,
    /// The `price_gt` argument.
    pub price_gt: Option<f64>,
    /// The `price_gte` argument.
    pub price_gte: Option<f64>,
    /// The `price_lt` argument.
    pub price_lt: Option<f64>,
    /// The `price_lte` argument.
    pub price_lte: Option<f64>,
    /// The `average_volume` argument.
    pub average_volume: Option<f64>,
    /// The `average_volume_gt` argument.
    pub average_volume_gt: Option<f64>,
    /// The `average_volume_gte` argument.
    pub average_volume_gte: Option<f64>,
    /// The `average_volume_lt` argument.
    pub average_volume_lt: Option<f64>,
    /// The `average_volume_lte` argument.
    pub average_volume_lte: Option<f64>,
    /// The `market_cap` argument.
    pub market_cap: Option<f64>,
    /// The `market_cap_gt` argument.
    pub market_cap_gt: Option<f64>,
    /// The `market_cap_gte` argument.
    pub market_cap_gte: Option<f64>,
    /// The `market_cap_lt` argument.
    pub market_cap_lt: Option<f64>,
    /// The `market_cap_lte` argument.
    pub market_cap_lte: Option<f64>,
    /// The `earnings_per_share` argument.
    pub earnings_per_share: Option<f64>,
    /// The `earnings_per_share_gt` argument.
    pub earnings_per_share_gt: Option<f64>,
    /// The `earnings_per_share_gte` argument.
    pub earnings_per_share_gte: Option<f64>,
    /// The `earnings_per_share_lt` argument.
    pub earnings_per_share_lt: Option<f64>,
    /// The `earnings_per_share_lte` argument.
    pub earnings_per_share_lte: Option<f64>,
    /// The `price_to_earnings` argument.
    pub price_to_earnings: Option<f64>,
    /// The `price_to_earnings_gt` argument.
    pub price_to_earnings_gt: Option<f64>,
    /// The `price_to_earnings_gte` argument.
    pub price_to_earnings_gte: Option<f64>,
    /// The `price_to_earnings_lt` argument.
    pub price_to_earnings_lt: Option<f64>,
    /// The `price_to_earnings_lte` argument.
    pub price_to_earnings_lte: Option<f64>,
    /// The `price_to_book` argument.
    pub price_to_book: Option<f64>,
    /// The `price_to_book_gt` argument.
    pub price_to_book_gt: Option<f64>,
    /// The `price_to_book_gte` argument.
    pub price_to_book_gte: Option<f64>,
    /// The `price_to_book_lt` argument.
    pub price_to_book_lt: Option<f64>,
    /// The `price_to_book_lte` argument.
    pub price_to_book_lte: Option<f64>,
    /// The `price_to_sales` argument.
    pub price_to_sales: Option<f64>,
    /// The `price_to_sales_gt` argument.
    pub price_to_sales_gt: Option<f64>,
    /// The `price_to_sales_gte` argument.
    pub price_to_sales_gte: Option<f64>,
    /// The `price_to_sales_lt` argument.
    pub price_to_sales_lt: Option<f64>,
    /// The `price_to_sales_lte` argument.
    pub price_to_sales_lte: Option<f64>,
    /// The `price_to_cash_flow` argument.
    pub price_to_cash_flow: Option<f64>,
    /// The `price_to_cash_flow_gt` argument.
    pub price_to_cash_flow_gt: Option<f64>,
    /// The `price_to_cash_flow_gte` argument.
    pub price_to_cash_flow_gte: Option<f64>,
    /// The `price_to_cash_flow_lt` argument.
    pub price_to_cash_flow_lt: Option<f64>,
    /// The `price_to_cash_flow_lte` argument.
    pub price_to_cash_flow_lte: Option<f64>,
    /// The `price_to_free_cash_flow` argument.
    pub price_to_free_cash_flow: Option<f64>,
    /// The `price_to_free_cash_flow_gt` argument.
    pub price_to_free_cash_flow_gt: Option<f64>,
    /// The `price_to_free_cash_flow_gte` argument.
    pub price_to_free_cash_flow_gte: Option<f64>,
    /// The `price_to_free_cash_flow_lt` argument.
    pub price_to_free_cash_flow_lt: Option<f64>,
    /// The `price_to_free_cash_flow_lte` argument.
    pub price_to_free_cash_flow_lte: Option<f64>,
    /// The `dividend_yield` argument.
    pub dividend_yield: Option<f64>,
    /// The `dividend_yield_gt` argument.
    pub dividend_yield_gt: Option<f64>,
    /// The `dividend_yield_gte` argument.
    pub dividend_yield_gte: Option<f64>,
    /// The `dividend_yield_lt` argument.
    pub dividend_yield_lt: Option<f64>,
    /// The `dividend_yield_lte` argument.
    pub dividend_yield_lte: Option<f64>,
    /// The `return_on_assets` argument.
    pub return_on_assets: Option<f64>,
    /// The `return_on_assets_gt` argument.
    pub return_on_assets_gt: Option<f64>,
    /// The `return_on_assets_gte` argument.
    pub return_on_assets_gte: Option<f64>,
    /// The `return_on_assets_lt` argument.
    pub return_on_assets_lt: Option<f64>,
    /// The `return_on_assets_lte` argument.
    pub return_on_assets_lte: Option<f64>,
    /// The `return_on_equity` argument.
    pub return_on_equity: Option<f64>,
    /// The `return_on_equity_gt` argument.
    pub return_on_equity_gt: Option<f64>,
    /// The `return_on_equity_gte` argument.
    pub return_on_equity_gte: Option<f64>,
    /// The `return_on_equity_lt` argument.
    pub return_on_equity_lt: Option<f64>,
    /// The `return_on_equity_lte` argument.
    pub return_on_equity_lte: Option<f64>,
    /// The `debt_to_equity` argument.
    pub debt_to_equity: Option<f64>,
    /// The `debt_to_equity_gt` argument.
    pub debt_to_equity_gt: Option<f64>,
    /// The `debt_to_equity_gte` argument.
    pub debt_to_equity_gte: Option<f64>,
    /// The `debt_to_equity_lt` argument.
    pub debt_to_equity_lt: Option<f64>,
    /// The `debt_to_equity_lte` argument.
    pub debt_to_equity_lte: Option<f64>,
    /// The `current` argument.
    pub current: Option<f64>,
    /// The `current_gt` argument.
    pub current_gt: Option<f64>,
    /// The `current_gte` argument.
    pub current_gte: Option<f64>,
    /// The `current_lt` argument.
    pub current_lt: Option<f64>,
    /// The `current_lte` argument.
    pub current_lte: Option<f64>,
    /// The `quick` argument.
    pub quick: Option<f64>,
    /// The `quick_gt` argument.
    pub quick_gt: Option<f64>,
    /// The `quick_gte` argument.
    pub quick_gte: Option<f64>,
    /// The `quick_lt` argument.
    pub quick_lt: Option<f64>,
    /// The `quick_lte` argument.
    pub quick_lte: Option<f64>,
    /// The `cash` argument.
    pub cash: Option<f64>,
    /// The `cash_gt` argument.
    pub cash_gt: Option<f64>,
    /// The `cash_gte` argument.
    pub cash_gte: Option<f64>,
    /// The `cash_lt` argument.
    pub cash_lt: Option<f64>,
    /// The `cash_lte` argument.
    pub cash_lte: Option<f64>,
    /// The `ev_to_sales` argument.
    pub ev_to_sales: Option<f64>,
    /// The `ev_to_sales_gt` argument.
    pub ev_to_sales_gt: Option<f64>,
    /// The `ev_to_sales_gte` argument.
    pub ev_to_sales_gte: Option<f64>,
    /// The `ev_to_sales_lt` argument.
    pub ev_to_sales_lt: Option<f64>,
    /// The `ev_to_sales_lte` argument.
    pub ev_to_sales_lte: Option<f64>,
    /// The `ev_to_ebitda` argument.
    pub ev_to_ebitda: Option<f64>,
    /// The `ev_to_ebitda_gt` argument.
    pub ev_to_ebitda_gt: Option<f64>,
    /// The `ev_to_ebitda_gte` argument.
    pub ev_to_ebitda_gte: Option<f64>,
    /// The `ev_to_ebitda_lt` argument.
    pub ev_to_ebitda_lt: Option<f64>,
    /// The `ev_to_ebitda_lte` argument.
    pub ev_to_ebitda_lte: Option<f64>,
    /// The `enterprise_value` argument.
    pub enterprise_value: Option<f64>,
    /// The `enterprise_value_gt` argument.
    pub enterprise_value_gt: Option<f64>,
    /// The `enterprise_value_gte` argument.
    pub enterprise_value_gte: Option<f64>,
    /// The `enterprise_value_lt` argument.
    pub enterprise_value_lt: Option<f64>,
    /// The `enterprise_value_lte` argument.
    pub enterprise_value_lte: Option<f64>,
    /// The `free_cash_flow` argument.
    pub free_cash_flow: Option<f64>,
    /// The `free_cash_flow_gt` argument.
    pub free_cash_flow_gt: Option<f64>,
    /// The `free_cash_flow_gte` argument.
    pub free_cash_flow_gte: Option<f64>,
    /// The `free_cash_flow_lt` argument.
    pub free_cash_flow_lt: Option<f64>,
    /// The `free_cash_flow_lte` argument.
    pub free_cash_flow_lte: Option<f64>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListFinancialsRatiosParams {
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

    /// Set the `price` argument.
    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    /// Set the `price_gt` argument.
    pub fn price_gt(mut self, price_gt: f64) -> Self {
        self.price_gt = Some(price_gt);
        self
    }

    /// Set the `price_gte` argument.
    pub fn price_gte(mut self, price_gte: f64) -> Self {
        self.price_gte = Some(price_gte);
        self
    }

    /// Set the `price_lt` argument.
    pub fn price_lt(mut self, price_lt: f64) -> Self {
        self.price_lt = Some(price_lt);
        self
    }

    /// Set the `price_lte` argument.
    pub fn price_lte(mut self, price_lte: f64) -> Self {
        self.price_lte = Some(price_lte);
        self
    }

    /// Set the `average_volume` argument.
    pub fn average_volume(mut self, average_volume: f64) -> Self {
        self.average_volume = Some(average_volume);
        self
    }

    /// Set the `average_volume_gt` argument.
    pub fn average_volume_gt(mut self, average_volume_gt: f64) -> Self {
        self.average_volume_gt = Some(average_volume_gt);
        self
    }

    /// Set the `average_volume_gte` argument.
    pub fn average_volume_gte(mut self, average_volume_gte: f64) -> Self {
        self.average_volume_gte = Some(average_volume_gte);
        self
    }

    /// Set the `average_volume_lt` argument.
    pub fn average_volume_lt(mut self, average_volume_lt: f64) -> Self {
        self.average_volume_lt = Some(average_volume_lt);
        self
    }

    /// Set the `average_volume_lte` argument.
    pub fn average_volume_lte(mut self, average_volume_lte: f64) -> Self {
        self.average_volume_lte = Some(average_volume_lte);
        self
    }

    /// Set the `market_cap` argument.
    pub fn market_cap(mut self, market_cap: f64) -> Self {
        self.market_cap = Some(market_cap);
        self
    }

    /// Set the `market_cap_gt` argument.
    pub fn market_cap_gt(mut self, market_cap_gt: f64) -> Self {
        self.market_cap_gt = Some(market_cap_gt);
        self
    }

    /// Set the `market_cap_gte` argument.
    pub fn market_cap_gte(mut self, market_cap_gte: f64) -> Self {
        self.market_cap_gte = Some(market_cap_gte);
        self
    }

    /// Set the `market_cap_lt` argument.
    pub fn market_cap_lt(mut self, market_cap_lt: f64) -> Self {
        self.market_cap_lt = Some(market_cap_lt);
        self
    }

    /// Set the `market_cap_lte` argument.
    pub fn market_cap_lte(mut self, market_cap_lte: f64) -> Self {
        self.market_cap_lte = Some(market_cap_lte);
        self
    }

    /// Set the `earnings_per_share` argument.
    pub fn earnings_per_share(mut self, earnings_per_share: f64) -> Self {
        self.earnings_per_share = Some(earnings_per_share);
        self
    }

    /// Set the `earnings_per_share_gt` argument.
    pub fn earnings_per_share_gt(mut self, earnings_per_share_gt: f64) -> Self {
        self.earnings_per_share_gt = Some(earnings_per_share_gt);
        self
    }

    /// Set the `earnings_per_share_gte` argument.
    pub fn earnings_per_share_gte(mut self, earnings_per_share_gte: f64) -> Self {
        self.earnings_per_share_gte = Some(earnings_per_share_gte);
        self
    }

    /// Set the `earnings_per_share_lt` argument.
    pub fn earnings_per_share_lt(mut self, earnings_per_share_lt: f64) -> Self {
        self.earnings_per_share_lt = Some(earnings_per_share_lt);
        self
    }

    /// Set the `earnings_per_share_lte` argument.
    pub fn earnings_per_share_lte(mut self, earnings_per_share_lte: f64) -> Self {
        self.earnings_per_share_lte = Some(earnings_per_share_lte);
        self
    }

    /// Set the `price_to_earnings` argument.
    pub fn price_to_earnings(mut self, price_to_earnings: f64) -> Self {
        self.price_to_earnings = Some(price_to_earnings);
        self
    }

    /// Set the `price_to_earnings_gt` argument.
    pub fn price_to_earnings_gt(mut self, price_to_earnings_gt: f64) -> Self {
        self.price_to_earnings_gt = Some(price_to_earnings_gt);
        self
    }

    /// Set the `price_to_earnings_gte` argument.
    pub fn price_to_earnings_gte(mut self, price_to_earnings_gte: f64) -> Self {
        self.price_to_earnings_gte = Some(price_to_earnings_gte);
        self
    }

    /// Set the `price_to_earnings_lt` argument.
    pub fn price_to_earnings_lt(mut self, price_to_earnings_lt: f64) -> Self {
        self.price_to_earnings_lt = Some(price_to_earnings_lt);
        self
    }

    /// Set the `price_to_earnings_lte` argument.
    pub fn price_to_earnings_lte(mut self, price_to_earnings_lte: f64) -> Self {
        self.price_to_earnings_lte = Some(price_to_earnings_lte);
        self
    }

    /// Set the `price_to_book` argument.
    pub fn price_to_book(mut self, price_to_book: f64) -> Self {
        self.price_to_book = Some(price_to_book);
        self
    }

    /// Set the `price_to_book_gt` argument.
    pub fn price_to_book_gt(mut self, price_to_book_gt: f64) -> Self {
        self.price_to_book_gt = Some(price_to_book_gt);
        self
    }

    /// Set the `price_to_book_gte` argument.
    pub fn price_to_book_gte(mut self, price_to_book_gte: f64) -> Self {
        self.price_to_book_gte = Some(price_to_book_gte);
        self
    }

    /// Set the `price_to_book_lt` argument.
    pub fn price_to_book_lt(mut self, price_to_book_lt: f64) -> Self {
        self.price_to_book_lt = Some(price_to_book_lt);
        self
    }

    /// Set the `price_to_book_lte` argument.
    pub fn price_to_book_lte(mut self, price_to_book_lte: f64) -> Self {
        self.price_to_book_lte = Some(price_to_book_lte);
        self
    }

    /// Set the `price_to_sales` argument.
    pub fn price_to_sales(mut self, price_to_sales: f64) -> Self {
        self.price_to_sales = Some(price_to_sales);
        self
    }

    /// Set the `price_to_sales_gt` argument.
    pub fn price_to_sales_gt(mut self, price_to_sales_gt: f64) -> Self {
        self.price_to_sales_gt = Some(price_to_sales_gt);
        self
    }

    /// Set the `price_to_sales_gte` argument.
    pub fn price_to_sales_gte(mut self, price_to_sales_gte: f64) -> Self {
        self.price_to_sales_gte = Some(price_to_sales_gte);
        self
    }

    /// Set the `price_to_sales_lt` argument.
    pub fn price_to_sales_lt(mut self, price_to_sales_lt: f64) -> Self {
        self.price_to_sales_lt = Some(price_to_sales_lt);
        self
    }

    /// Set the `price_to_sales_lte` argument.
    pub fn price_to_sales_lte(mut self, price_to_sales_lte: f64) -> Self {
        self.price_to_sales_lte = Some(price_to_sales_lte);
        self
    }

    /// Set the `price_to_cash_flow` argument.
    pub fn price_to_cash_flow(mut self, price_to_cash_flow: f64) -> Self {
        self.price_to_cash_flow = Some(price_to_cash_flow);
        self
    }

    /// Set the `price_to_cash_flow_gt` argument.
    pub fn price_to_cash_flow_gt(mut self, price_to_cash_flow_gt: f64) -> Self {
        self.price_to_cash_flow_gt = Some(price_to_cash_flow_gt);
        self
    }

    /// Set the `price_to_cash_flow_gte` argument.
    pub fn price_to_cash_flow_gte(mut self, price_to_cash_flow_gte: f64) -> Self {
        self.price_to_cash_flow_gte = Some(price_to_cash_flow_gte);
        self
    }

    /// Set the `price_to_cash_flow_lt` argument.
    pub fn price_to_cash_flow_lt(mut self, price_to_cash_flow_lt: f64) -> Self {
        self.price_to_cash_flow_lt = Some(price_to_cash_flow_lt);
        self
    }

    /// Set the `price_to_cash_flow_lte` argument.
    pub fn price_to_cash_flow_lte(mut self, price_to_cash_flow_lte: f64) -> Self {
        self.price_to_cash_flow_lte = Some(price_to_cash_flow_lte);
        self
    }

    /// Set the `price_to_free_cash_flow` argument.
    pub fn price_to_free_cash_flow(mut self, price_to_free_cash_flow: f64) -> Self {
        self.price_to_free_cash_flow = Some(price_to_free_cash_flow);
        self
    }

    /// Set the `price_to_free_cash_flow_gt` argument.
    pub fn price_to_free_cash_flow_gt(mut self, price_to_free_cash_flow_gt: f64) -> Self {
        self.price_to_free_cash_flow_gt = Some(price_to_free_cash_flow_gt);
        self
    }

    /// Set the `price_to_free_cash_flow_gte` argument.
    pub fn price_to_free_cash_flow_gte(mut self, price_to_free_cash_flow_gte: f64) -> Self {
        self.price_to_free_cash_flow_gte = Some(price_to_free_cash_flow_gte);
        self
    }

    /// Set the `price_to_free_cash_flow_lt` argument.
    pub fn price_to_free_cash_flow_lt(mut self, price_to_free_cash_flow_lt: f64) -> Self {
        self.price_to_free_cash_flow_lt = Some(price_to_free_cash_flow_lt);
        self
    }

    /// Set the `price_to_free_cash_flow_lte` argument.
    pub fn price_to_free_cash_flow_lte(mut self, price_to_free_cash_flow_lte: f64) -> Self {
        self.price_to_free_cash_flow_lte = Some(price_to_free_cash_flow_lte);
        self
    }

    /// Set the `dividend_yield` argument.
    pub fn dividend_yield(mut self, dividend_yield: f64) -> Self {
        self.dividend_yield = Some(dividend_yield);
        self
    }

    /// Set the `dividend_yield_gt` argument.
    pub fn dividend_yield_gt(mut self, dividend_yield_gt: f64) -> Self {
        self.dividend_yield_gt = Some(dividend_yield_gt);
        self
    }

    /// Set the `dividend_yield_gte` argument.
    pub fn dividend_yield_gte(mut self, dividend_yield_gte: f64) -> Self {
        self.dividend_yield_gte = Some(dividend_yield_gte);
        self
    }

    /// Set the `dividend_yield_lt` argument.
    pub fn dividend_yield_lt(mut self, dividend_yield_lt: f64) -> Self {
        self.dividend_yield_lt = Some(dividend_yield_lt);
        self
    }

    /// Set the `dividend_yield_lte` argument.
    pub fn dividend_yield_lte(mut self, dividend_yield_lte: f64) -> Self {
        self.dividend_yield_lte = Some(dividend_yield_lte);
        self
    }

    /// Set the `return_on_assets` argument.
    pub fn return_on_assets(mut self, return_on_assets: f64) -> Self {
        self.return_on_assets = Some(return_on_assets);
        self
    }

    /// Set the `return_on_assets_gt` argument.
    pub fn return_on_assets_gt(mut self, return_on_assets_gt: f64) -> Self {
        self.return_on_assets_gt = Some(return_on_assets_gt);
        self
    }

    /// Set the `return_on_assets_gte` argument.
    pub fn return_on_assets_gte(mut self, return_on_assets_gte: f64) -> Self {
        self.return_on_assets_gte = Some(return_on_assets_gte);
        self
    }

    /// Set the `return_on_assets_lt` argument.
    pub fn return_on_assets_lt(mut self, return_on_assets_lt: f64) -> Self {
        self.return_on_assets_lt = Some(return_on_assets_lt);
        self
    }

    /// Set the `return_on_assets_lte` argument.
    pub fn return_on_assets_lte(mut self, return_on_assets_lte: f64) -> Self {
        self.return_on_assets_lte = Some(return_on_assets_lte);
        self
    }

    /// Set the `return_on_equity` argument.
    pub fn return_on_equity(mut self, return_on_equity: f64) -> Self {
        self.return_on_equity = Some(return_on_equity);
        self
    }

    /// Set the `return_on_equity_gt` argument.
    pub fn return_on_equity_gt(mut self, return_on_equity_gt: f64) -> Self {
        self.return_on_equity_gt = Some(return_on_equity_gt);
        self
    }

    /// Set the `return_on_equity_gte` argument.
    pub fn return_on_equity_gte(mut self, return_on_equity_gte: f64) -> Self {
        self.return_on_equity_gte = Some(return_on_equity_gte);
        self
    }

    /// Set the `return_on_equity_lt` argument.
    pub fn return_on_equity_lt(mut self, return_on_equity_lt: f64) -> Self {
        self.return_on_equity_lt = Some(return_on_equity_lt);
        self
    }

    /// Set the `return_on_equity_lte` argument.
    pub fn return_on_equity_lte(mut self, return_on_equity_lte: f64) -> Self {
        self.return_on_equity_lte = Some(return_on_equity_lte);
        self
    }

    /// Set the `debt_to_equity` argument.
    pub fn debt_to_equity(mut self, debt_to_equity: f64) -> Self {
        self.debt_to_equity = Some(debt_to_equity);
        self
    }

    /// Set the `debt_to_equity_gt` argument.
    pub fn debt_to_equity_gt(mut self, debt_to_equity_gt: f64) -> Self {
        self.debt_to_equity_gt = Some(debt_to_equity_gt);
        self
    }

    /// Set the `debt_to_equity_gte` argument.
    pub fn debt_to_equity_gte(mut self, debt_to_equity_gte: f64) -> Self {
        self.debt_to_equity_gte = Some(debt_to_equity_gte);
        self
    }

    /// Set the `debt_to_equity_lt` argument.
    pub fn debt_to_equity_lt(mut self, debt_to_equity_lt: f64) -> Self {
        self.debt_to_equity_lt = Some(debt_to_equity_lt);
        self
    }

    /// Set the `debt_to_equity_lte` argument.
    pub fn debt_to_equity_lte(mut self, debt_to_equity_lte: f64) -> Self {
        self.debt_to_equity_lte = Some(debt_to_equity_lte);
        self
    }

    /// Set the `current` argument.
    pub fn current(mut self, current: f64) -> Self {
        self.current = Some(current);
        self
    }

    /// Set the `current_gt` argument.
    pub fn current_gt(mut self, current_gt: f64) -> Self {
        self.current_gt = Some(current_gt);
        self
    }

    /// Set the `current_gte` argument.
    pub fn current_gte(mut self, current_gte: f64) -> Self {
        self.current_gte = Some(current_gte);
        self
    }

    /// Set the `current_lt` argument.
    pub fn current_lt(mut self, current_lt: f64) -> Self {
        self.current_lt = Some(current_lt);
        self
    }

    /// Set the `current_lte` argument.
    pub fn current_lte(mut self, current_lte: f64) -> Self {
        self.current_lte = Some(current_lte);
        self
    }

    /// Set the `quick` argument.
    pub fn quick(mut self, quick: f64) -> Self {
        self.quick = Some(quick);
        self
    }

    /// Set the `quick_gt` argument.
    pub fn quick_gt(mut self, quick_gt: f64) -> Self {
        self.quick_gt = Some(quick_gt);
        self
    }

    /// Set the `quick_gte` argument.
    pub fn quick_gte(mut self, quick_gte: f64) -> Self {
        self.quick_gte = Some(quick_gte);
        self
    }

    /// Set the `quick_lt` argument.
    pub fn quick_lt(mut self, quick_lt: f64) -> Self {
        self.quick_lt = Some(quick_lt);
        self
    }

    /// Set the `quick_lte` argument.
    pub fn quick_lte(mut self, quick_lte: f64) -> Self {
        self.quick_lte = Some(quick_lte);
        self
    }

    /// Set the `cash` argument.
    pub fn cash(mut self, cash: f64) -> Self {
        self.cash = Some(cash);
        self
    }

    /// Set the `cash_gt` argument.
    pub fn cash_gt(mut self, cash_gt: f64) -> Self {
        self.cash_gt = Some(cash_gt);
        self
    }

    /// Set the `cash_gte` argument.
    pub fn cash_gte(mut self, cash_gte: f64) -> Self {
        self.cash_gte = Some(cash_gte);
        self
    }

    /// Set the `cash_lt` argument.
    pub fn cash_lt(mut self, cash_lt: f64) -> Self {
        self.cash_lt = Some(cash_lt);
        self
    }

    /// Set the `cash_lte` argument.
    pub fn cash_lte(mut self, cash_lte: f64) -> Self {
        self.cash_lte = Some(cash_lte);
        self
    }

    /// Set the `ev_to_sales` argument.
    pub fn ev_to_sales(mut self, ev_to_sales: f64) -> Self {
        self.ev_to_sales = Some(ev_to_sales);
        self
    }

    /// Set the `ev_to_sales_gt` argument.
    pub fn ev_to_sales_gt(mut self, ev_to_sales_gt: f64) -> Self {
        self.ev_to_sales_gt = Some(ev_to_sales_gt);
        self
    }

    /// Set the `ev_to_sales_gte` argument.
    pub fn ev_to_sales_gte(mut self, ev_to_sales_gte: f64) -> Self {
        self.ev_to_sales_gte = Some(ev_to_sales_gte);
        self
    }

    /// Set the `ev_to_sales_lt` argument.
    pub fn ev_to_sales_lt(mut self, ev_to_sales_lt: f64) -> Self {
        self.ev_to_sales_lt = Some(ev_to_sales_lt);
        self
    }

    /// Set the `ev_to_sales_lte` argument.
    pub fn ev_to_sales_lte(mut self, ev_to_sales_lte: f64) -> Self {
        self.ev_to_sales_lte = Some(ev_to_sales_lte);
        self
    }

    /// Set the `ev_to_ebitda` argument.
    pub fn ev_to_ebitda(mut self, ev_to_ebitda: f64) -> Self {
        self.ev_to_ebitda = Some(ev_to_ebitda);
        self
    }

    /// Set the `ev_to_ebitda_gt` argument.
    pub fn ev_to_ebitda_gt(mut self, ev_to_ebitda_gt: f64) -> Self {
        self.ev_to_ebitda_gt = Some(ev_to_ebitda_gt);
        self
    }

    /// Set the `ev_to_ebitda_gte` argument.
    pub fn ev_to_ebitda_gte(mut self, ev_to_ebitda_gte: f64) -> Self {
        self.ev_to_ebitda_gte = Some(ev_to_ebitda_gte);
        self
    }

    /// Set the `ev_to_ebitda_lt` argument.
    pub fn ev_to_ebitda_lt(mut self, ev_to_ebitda_lt: f64) -> Self {
        self.ev_to_ebitda_lt = Some(ev_to_ebitda_lt);
        self
    }

    /// Set the `ev_to_ebitda_lte` argument.
    pub fn ev_to_ebitda_lte(mut self, ev_to_ebitda_lte: f64) -> Self {
        self.ev_to_ebitda_lte = Some(ev_to_ebitda_lte);
        self
    }

    /// Set the `enterprise_value` argument.
    pub fn enterprise_value(mut self, enterprise_value: f64) -> Self {
        self.enterprise_value = Some(enterprise_value);
        self
    }

    /// Set the `enterprise_value_gt` argument.
    pub fn enterprise_value_gt(mut self, enterprise_value_gt: f64) -> Self {
        self.enterprise_value_gt = Some(enterprise_value_gt);
        self
    }

    /// Set the `enterprise_value_gte` argument.
    pub fn enterprise_value_gte(mut self, enterprise_value_gte: f64) -> Self {
        self.enterprise_value_gte = Some(enterprise_value_gte);
        self
    }

    /// Set the `enterprise_value_lt` argument.
    pub fn enterprise_value_lt(mut self, enterprise_value_lt: f64) -> Self {
        self.enterprise_value_lt = Some(enterprise_value_lt);
        self
    }

    /// Set the `enterprise_value_lte` argument.
    pub fn enterprise_value_lte(mut self, enterprise_value_lte: f64) -> Self {
        self.enterprise_value_lte = Some(enterprise_value_lte);
        self
    }

    /// Set the `free_cash_flow` argument.
    pub fn free_cash_flow(mut self, free_cash_flow: f64) -> Self {
        self.free_cash_flow = Some(free_cash_flow);
        self
    }

    /// Set the `free_cash_flow_gt` argument.
    pub fn free_cash_flow_gt(mut self, free_cash_flow_gt: f64) -> Self {
        self.free_cash_flow_gt = Some(free_cash_flow_gt);
        self
    }

    /// Set the `free_cash_flow_gte` argument.
    pub fn free_cash_flow_gte(mut self, free_cash_flow_gte: f64) -> Self {
        self.free_cash_flow_gte = Some(free_cash_flow_gte);
        self
    }

    /// Set the `free_cash_flow_lt` argument.
    pub fn free_cash_flow_lt(mut self, free_cash_flow_lt: f64) -> Self {
        self.free_cash_flow_lt = Some(free_cash_flow_lt);
        self
    }

    /// Set the `free_cash_flow_lte` argument.
    pub fn free_cash_flow_lte(mut self, free_cash_flow_lte: f64) -> Self {
        self.free_cash_flow_lte = Some(free_cash_flow_lte);
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

/// Optional arguments for [`FinancialsApi::list_stocks_floats`].
#[derive(Debug, Default, Clone)]
pub struct ListStocksFloatsParams {
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
    /// The `free_float_percent` argument.
    pub free_float_percent: Option<f64>,
    /// The `free_float_percent_gt` argument.
    pub free_float_percent_gt: Option<f64>,
    /// The `free_float_percent_gte` argument.
    pub free_float_percent_gte: Option<f64>,
    /// The `free_float_percent_lt` argument.
    pub free_float_percent_lt: Option<f64>,
    /// The `free_float_percent_lte` argument.
    pub free_float_percent_lte: Option<f64>,
    /// The `limit` argument.
    pub limit: Option<i64>,
    /// The `sort` argument.
    pub sort: Option<String>,
    /// The `options` argument.
    pub options: Option<RequestOptions>,
}

impl ListStocksFloatsParams {
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

    /// Set the `free_float_percent` argument.
    pub fn free_float_percent(mut self, free_float_percent: f64) -> Self {
        self.free_float_percent = Some(free_float_percent);
        self
    }

    /// Set the `free_float_percent_gt` argument.
    pub fn free_float_percent_gt(mut self, free_float_percent_gt: f64) -> Self {
        self.free_float_percent_gt = Some(free_float_percent_gt);
        self
    }

    /// Set the `free_float_percent_gte` argument.
    pub fn free_float_percent_gte(mut self, free_float_percent_gte: f64) -> Self {
        self.free_float_percent_gte = Some(free_float_percent_gte);
        self
    }

    /// Set the `free_float_percent_lt` argument.
    pub fn free_float_percent_lt(mut self, free_float_percent_lt: f64) -> Self {
        self.free_float_percent_lt = Some(free_float_percent_lt);
        self
    }

    /// Set the `free_float_percent_lte` argument.
    pub fn free_float_percent_lte(mut self, free_float_percent_lte: f64) -> Self {
        self.free_float_percent_lte = Some(free_float_percent_lte);
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
