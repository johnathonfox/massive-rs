use super::{encode_query, BoxStream};
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
            let path = "/stocks/financials/v1/balance-sheets".to_string();
            let query = encode_query(&params);
            self.list::<FinancialBalanceSheet>(&path, &query, params.options.as_ref())
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
            let path = "/stocks/financials/v1/cash-flow-statements".to_string();
            let query = encode_query(&params);
            self.list::<FinancialCashFlowStatement>(&path, &query, params.options.as_ref())
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
            let path = "/stocks/financials/v1/income-statements".to_string();
            let query = encode_query(&params);
            self.list::<FinancialIncomeStatement>(&path, &query, params.options.as_ref())
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
            let path = "/stocks/financials/v1/ratios".to_string();
            let query = encode_query(&params);
            self.list::<FinancialRatio>(&path, &query, params.options.as_ref())
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
            let path = "/stocks/vX/float".to_string();
            let query = encode_query(&params);
            self.list::<FinancialFloat>(&path, &query, params.options.as_ref())
        })
    }
}

// --- Params structs (additive builder API) ---
//
// Query serialization is derived: field order is wire order, `rename` carries
// dotted filter operators, unset fields are omitted, and `options` is skipped.

/// Optional arguments for [`FinancialsApi::list_financials_balance_sheets`].
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ListFinancialsBalanceSheetsParams {
    /// The `cik` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cik: Option<String>,
    /// The `cik_any_of` argument.
    #[serde(rename = "cik.any_of", skip_serializing_if = "Option::is_none")]
    pub cik_any_of: Option<String>,
    /// The `cik_gt` argument.
    #[serde(rename = "cik.gt", skip_serializing_if = "Option::is_none")]
    pub cik_gt: Option<String>,
    /// The `cik_gte` argument.
    #[serde(rename = "cik.gte", skip_serializing_if = "Option::is_none")]
    pub cik_gte: Option<String>,
    /// The `cik_lt` argument.
    #[serde(rename = "cik.lt", skip_serializing_if = "Option::is_none")]
    pub cik_lt: Option<String>,
    /// The `cik_lte` argument.
    #[serde(rename = "cik.lte", skip_serializing_if = "Option::is_none")]
    pub cik_lte: Option<String>,
    /// The `tickers` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tickers: Option<String>,
    /// The `tickers_all_of` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tickers_all_of: Option<String>,
    /// The `tickers_any_of` argument.
    #[serde(rename = "tickers.any_of", skip_serializing_if = "Option::is_none")]
    pub tickers_any_of: Option<String>,
    /// The `period_end` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_end: Option<String>,
    /// The `period_end_gt` argument.
    #[serde(rename = "period_end.gt", skip_serializing_if = "Option::is_none")]
    pub period_end_gt: Option<String>,
    /// The `period_end_gte` argument.
    #[serde(rename = "period_end.gte", skip_serializing_if = "Option::is_none")]
    pub period_end_gte: Option<String>,
    /// The `period_end_lt` argument.
    #[serde(rename = "period_end.lt", skip_serializing_if = "Option::is_none")]
    pub period_end_lt: Option<String>,
    /// The `period_end_lte` argument.
    #[serde(rename = "period_end.lte", skip_serializing_if = "Option::is_none")]
    pub period_end_lte: Option<String>,
    /// The `filing_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filing_date: Option<String>,
    /// The `filing_date_gt` argument.
    #[serde(rename = "filing_date.gt", skip_serializing_if = "Option::is_none")]
    pub filing_date_gt: Option<String>,
    /// The `filing_date_gte` argument.
    #[serde(rename = "filing_date.gte", skip_serializing_if = "Option::is_none")]
    pub filing_date_gte: Option<String>,
    /// The `filing_date_lt` argument.
    #[serde(rename = "filing_date.lt", skip_serializing_if = "Option::is_none")]
    pub filing_date_lt: Option<String>,
    /// The `filing_date_lte` argument.
    #[serde(rename = "filing_date.lte", skip_serializing_if = "Option::is_none")]
    pub filing_date_lte: Option<String>,
    /// The `fiscal_year` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_year: Option<f64>,
    /// The `fiscal_year_gt` argument.
    #[serde(
        rename = "fiscal_year.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_year_gt: Option<f64>,
    /// The `fiscal_year_gte` argument.
    #[serde(
        rename = "fiscal_year.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_year_gte: Option<f64>,
    /// The `fiscal_year_lt` argument.
    #[serde(
        rename = "fiscal_year.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_year_lt: Option<f64>,
    /// The `fiscal_year_lte` argument.
    #[serde(
        rename = "fiscal_year.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_year_lte: Option<f64>,
    /// The `fiscal_quarter` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_quarter: Option<f64>,
    /// The `fiscal_quarter_gt` argument.
    #[serde(
        rename = "fiscal_quarter.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_quarter_gt: Option<f64>,
    /// The `fiscal_quarter_gte` argument.
    #[serde(
        rename = "fiscal_quarter.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_quarter_gte: Option<f64>,
    /// The `fiscal_quarter_lt` argument.
    #[serde(
        rename = "fiscal_quarter.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_quarter_lt: Option<f64>,
    /// The `fiscal_quarter_lte` argument.
    #[serde(
        rename = "fiscal_quarter.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_quarter_lte: Option<f64>,
    /// The `timeframe` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<String>,
    /// The `timeframe_any_of` argument.
    #[serde(rename = "timeframe.any_of", skip_serializing_if = "Option::is_none")]
    pub timeframe_any_of: Option<String>,
    /// The `timeframe_gt` argument.
    #[serde(rename = "timeframe.gt", skip_serializing_if = "Option::is_none")]
    pub timeframe_gt: Option<String>,
    /// The `timeframe_gte` argument.
    #[serde(rename = "timeframe.gte", skip_serializing_if = "Option::is_none")]
    pub timeframe_gte: Option<String>,
    /// The `timeframe_lt` argument.
    #[serde(rename = "timeframe.lt", skip_serializing_if = "Option::is_none")]
    pub timeframe_lt: Option<String>,
    /// The `timeframe_lte` argument.
    #[serde(rename = "timeframe.lte", skip_serializing_if = "Option::is_none")]
    pub timeframe_lte: Option<String>,
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
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ListFinancialsCashFlowStatementsParams {
    /// The `cik` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cik: Option<String>,
    /// The `cik_any_of` argument.
    #[serde(rename = "cik.any_of", skip_serializing_if = "Option::is_none")]
    pub cik_any_of: Option<String>,
    /// The `cik_gt` argument.
    #[serde(rename = "cik.gt", skip_serializing_if = "Option::is_none")]
    pub cik_gt: Option<String>,
    /// The `cik_gte` argument.
    #[serde(rename = "cik.gte", skip_serializing_if = "Option::is_none")]
    pub cik_gte: Option<String>,
    /// The `cik_lt` argument.
    #[serde(rename = "cik.lt", skip_serializing_if = "Option::is_none")]
    pub cik_lt: Option<String>,
    /// The `cik_lte` argument.
    #[serde(rename = "cik.lte", skip_serializing_if = "Option::is_none")]
    pub cik_lte: Option<String>,
    /// The `period_end` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_end: Option<String>,
    /// The `period_end_gt` argument.
    #[serde(rename = "period_end.gt", skip_serializing_if = "Option::is_none")]
    pub period_end_gt: Option<String>,
    /// The `period_end_gte` argument.
    #[serde(rename = "period_end.gte", skip_serializing_if = "Option::is_none")]
    pub period_end_gte: Option<String>,
    /// The `period_end_lt` argument.
    #[serde(rename = "period_end.lt", skip_serializing_if = "Option::is_none")]
    pub period_end_lt: Option<String>,
    /// The `period_end_lte` argument.
    #[serde(rename = "period_end.lte", skip_serializing_if = "Option::is_none")]
    pub period_end_lte: Option<String>,
    /// The `filing_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filing_date: Option<String>,
    /// The `filing_date_gt` argument.
    #[serde(rename = "filing_date.gt", skip_serializing_if = "Option::is_none")]
    pub filing_date_gt: Option<String>,
    /// The `filing_date_gte` argument.
    #[serde(rename = "filing_date.gte", skip_serializing_if = "Option::is_none")]
    pub filing_date_gte: Option<String>,
    /// The `filing_date_lt` argument.
    #[serde(rename = "filing_date.lt", skip_serializing_if = "Option::is_none")]
    pub filing_date_lt: Option<String>,
    /// The `filing_date_lte` argument.
    #[serde(rename = "filing_date.lte", skip_serializing_if = "Option::is_none")]
    pub filing_date_lte: Option<String>,
    /// The `tickers` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tickers: Option<String>,
    /// The `tickers_all_of` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tickers_all_of: Option<String>,
    /// The `tickers_any_of` argument.
    #[serde(rename = "tickers.any_of", skip_serializing_if = "Option::is_none")]
    pub tickers_any_of: Option<String>,
    /// The `fiscal_year` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_year: Option<f64>,
    /// The `fiscal_year_gt` argument.
    #[serde(
        rename = "fiscal_year.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_year_gt: Option<f64>,
    /// The `fiscal_year_gte` argument.
    #[serde(
        rename = "fiscal_year.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_year_gte: Option<f64>,
    /// The `fiscal_year_lt` argument.
    #[serde(
        rename = "fiscal_year.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_year_lt: Option<f64>,
    /// The `fiscal_year_lte` argument.
    #[serde(
        rename = "fiscal_year.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_year_lte: Option<f64>,
    /// The `fiscal_quarter` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_quarter: Option<f64>,
    /// The `fiscal_quarter_gt` argument.
    #[serde(
        rename = "fiscal_quarter.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_quarter_gt: Option<f64>,
    /// The `fiscal_quarter_gte` argument.
    #[serde(
        rename = "fiscal_quarter.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_quarter_gte: Option<f64>,
    /// The `fiscal_quarter_lt` argument.
    #[serde(
        rename = "fiscal_quarter.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_quarter_lt: Option<f64>,
    /// The `fiscal_quarter_lte` argument.
    #[serde(
        rename = "fiscal_quarter.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_quarter_lte: Option<f64>,
    /// The `timeframe` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<String>,
    /// The `timeframe_any_of` argument.
    #[serde(rename = "timeframe.any_of", skip_serializing_if = "Option::is_none")]
    pub timeframe_any_of: Option<String>,
    /// The `timeframe_gt` argument.
    #[serde(rename = "timeframe.gt", skip_serializing_if = "Option::is_none")]
    pub timeframe_gt: Option<String>,
    /// The `timeframe_gte` argument.
    #[serde(rename = "timeframe.gte", skip_serializing_if = "Option::is_none")]
    pub timeframe_gte: Option<String>,
    /// The `timeframe_lt` argument.
    #[serde(rename = "timeframe.lt", skip_serializing_if = "Option::is_none")]
    pub timeframe_lt: Option<String>,
    /// The `timeframe_lte` argument.
    #[serde(rename = "timeframe.lte", skip_serializing_if = "Option::is_none")]
    pub timeframe_lte: Option<String>,
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
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ListFinancialsIncomeStatementsParams {
    /// The `cik` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cik: Option<String>,
    /// The `cik_any_of` argument.
    #[serde(rename = "cik.any_of", skip_serializing_if = "Option::is_none")]
    pub cik_any_of: Option<String>,
    /// The `cik_gt` argument.
    #[serde(rename = "cik.gt", skip_serializing_if = "Option::is_none")]
    pub cik_gt: Option<String>,
    /// The `cik_gte` argument.
    #[serde(rename = "cik.gte", skip_serializing_if = "Option::is_none")]
    pub cik_gte: Option<String>,
    /// The `cik_lt` argument.
    #[serde(rename = "cik.lt", skip_serializing_if = "Option::is_none")]
    pub cik_lt: Option<String>,
    /// The `cik_lte` argument.
    #[serde(rename = "cik.lte", skip_serializing_if = "Option::is_none")]
    pub cik_lte: Option<String>,
    /// The `tickers` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tickers: Option<String>,
    /// The `tickers_all_of` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tickers_all_of: Option<String>,
    /// The `tickers_any_of` argument.
    #[serde(rename = "tickers.any_of", skip_serializing_if = "Option::is_none")]
    pub tickers_any_of: Option<String>,
    /// The `period_end` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_end: Option<String>,
    /// The `period_end_gt` argument.
    #[serde(rename = "period_end.gt", skip_serializing_if = "Option::is_none")]
    pub period_end_gt: Option<String>,
    /// The `period_end_gte` argument.
    #[serde(rename = "period_end.gte", skip_serializing_if = "Option::is_none")]
    pub period_end_gte: Option<String>,
    /// The `period_end_lt` argument.
    #[serde(rename = "period_end.lt", skip_serializing_if = "Option::is_none")]
    pub period_end_lt: Option<String>,
    /// The `period_end_lte` argument.
    #[serde(rename = "period_end.lte", skip_serializing_if = "Option::is_none")]
    pub period_end_lte: Option<String>,
    /// The `filing_date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filing_date: Option<String>,
    /// The `filing_date_gt` argument.
    #[serde(rename = "filing_date.gt", skip_serializing_if = "Option::is_none")]
    pub filing_date_gt: Option<String>,
    /// The `filing_date_gte` argument.
    #[serde(rename = "filing_date.gte", skip_serializing_if = "Option::is_none")]
    pub filing_date_gte: Option<String>,
    /// The `filing_date_lt` argument.
    #[serde(rename = "filing_date.lt", skip_serializing_if = "Option::is_none")]
    pub filing_date_lt: Option<String>,
    /// The `filing_date_lte` argument.
    #[serde(rename = "filing_date.lte", skip_serializing_if = "Option::is_none")]
    pub filing_date_lte: Option<String>,
    /// The `fiscal_year` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_year: Option<f64>,
    /// The `fiscal_year_gt` argument.
    #[serde(
        rename = "fiscal_year.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_year_gt: Option<f64>,
    /// The `fiscal_year_gte` argument.
    #[serde(
        rename = "fiscal_year.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_year_gte: Option<f64>,
    /// The `fiscal_year_lt` argument.
    #[serde(
        rename = "fiscal_year.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_year_lt: Option<f64>,
    /// The `fiscal_year_lte` argument.
    #[serde(
        rename = "fiscal_year.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_year_lte: Option<f64>,
    /// The `fiscal_quarter` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_quarter: Option<f64>,
    /// The `fiscal_quarter_gt` argument.
    #[serde(
        rename = "fiscal_quarter.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_quarter_gt: Option<f64>,
    /// The `fiscal_quarter_gte` argument.
    #[serde(
        rename = "fiscal_quarter.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_quarter_gte: Option<f64>,
    /// The `fiscal_quarter_lt` argument.
    #[serde(
        rename = "fiscal_quarter.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_quarter_lt: Option<f64>,
    /// The `fiscal_quarter_lte` argument.
    #[serde(
        rename = "fiscal_quarter.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub fiscal_quarter_lte: Option<f64>,
    /// The `timeframe` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<String>,
    /// The `timeframe_any_of` argument.
    #[serde(rename = "timeframe.any_of", skip_serializing_if = "Option::is_none")]
    pub timeframe_any_of: Option<String>,
    /// The `timeframe_gt` argument.
    #[serde(rename = "timeframe.gt", skip_serializing_if = "Option::is_none")]
    pub timeframe_gt: Option<String>,
    /// The `timeframe_gte` argument.
    #[serde(rename = "timeframe.gte", skip_serializing_if = "Option::is_none")]
    pub timeframe_gte: Option<String>,
    /// The `timeframe_lt` argument.
    #[serde(rename = "timeframe.lt", skip_serializing_if = "Option::is_none")]
    pub timeframe_lt: Option<String>,
    /// The `timeframe_lte` argument.
    #[serde(rename = "timeframe.lte", skip_serializing_if = "Option::is_none")]
    pub timeframe_lte: Option<String>,
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
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ListFinancialsRatiosParams {
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
    /// The `cik` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cik: Option<String>,
    /// The `cik_any_of` argument.
    #[serde(rename = "cik.any_of", skip_serializing_if = "Option::is_none")]
    pub cik_any_of: Option<String>,
    /// The `cik_gt` argument.
    #[serde(rename = "cik.gt", skip_serializing_if = "Option::is_none")]
    pub cik_gt: Option<String>,
    /// The `cik_gte` argument.
    #[serde(rename = "cik.gte", skip_serializing_if = "Option::is_none")]
    pub cik_gte: Option<String>,
    /// The `cik_lt` argument.
    #[serde(rename = "cik.lt", skip_serializing_if = "Option::is_none")]
    pub cik_lt: Option<String>,
    /// The `cik_lte` argument.
    #[serde(rename = "cik.lte", skip_serializing_if = "Option::is_none")]
    pub cik_lte: Option<String>,
    /// The `price` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price: Option<f64>,
    /// The `price_gt` argument.
    #[serde(
        rename = "price.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_gt: Option<f64>,
    /// The `price_gte` argument.
    #[serde(
        rename = "price.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_gte: Option<f64>,
    /// The `price_lt` argument.
    #[serde(
        rename = "price.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_lt: Option<f64>,
    /// The `price_lte` argument.
    #[serde(
        rename = "price.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_lte: Option<f64>,
    /// The `average_volume` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub average_volume: Option<f64>,
    /// The `average_volume_gt` argument.
    #[serde(
        rename = "average_volume.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub average_volume_gt: Option<f64>,
    /// The `average_volume_gte` argument.
    #[serde(
        rename = "average_volume.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub average_volume_gte: Option<f64>,
    /// The `average_volume_lt` argument.
    #[serde(
        rename = "average_volume.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub average_volume_lt: Option<f64>,
    /// The `average_volume_lte` argument.
    #[serde(
        rename = "average_volume.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub average_volume_lte: Option<f64>,
    /// The `market_cap` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub market_cap: Option<f64>,
    /// The `market_cap_gt` argument.
    #[serde(
        rename = "market_cap.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub market_cap_gt: Option<f64>,
    /// The `market_cap_gte` argument.
    #[serde(
        rename = "market_cap.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub market_cap_gte: Option<f64>,
    /// The `market_cap_lt` argument.
    #[serde(
        rename = "market_cap.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub market_cap_lt: Option<f64>,
    /// The `market_cap_lte` argument.
    #[serde(
        rename = "market_cap.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub market_cap_lte: Option<f64>,
    /// The `earnings_per_share` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub earnings_per_share: Option<f64>,
    /// The `earnings_per_share_gt` argument.
    #[serde(
        rename = "earnings_per_share.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub earnings_per_share_gt: Option<f64>,
    /// The `earnings_per_share_gte` argument.
    #[serde(
        rename = "earnings_per_share.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub earnings_per_share_gte: Option<f64>,
    /// The `earnings_per_share_lt` argument.
    #[serde(
        rename = "earnings_per_share.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub earnings_per_share_lt: Option<f64>,
    /// The `earnings_per_share_lte` argument.
    #[serde(
        rename = "earnings_per_share.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub earnings_per_share_lte: Option<f64>,
    /// The `price_to_earnings` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_earnings: Option<f64>,
    /// The `price_to_earnings_gt` argument.
    #[serde(
        rename = "price_to_earnings.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_earnings_gt: Option<f64>,
    /// The `price_to_earnings_gte` argument.
    #[serde(
        rename = "price_to_earnings.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_earnings_gte: Option<f64>,
    /// The `price_to_earnings_lt` argument.
    #[serde(
        rename = "price_to_earnings.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_earnings_lt: Option<f64>,
    /// The `price_to_earnings_lte` argument.
    #[serde(
        rename = "price_to_earnings.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_earnings_lte: Option<f64>,
    /// The `price_to_book` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_book: Option<f64>,
    /// The `price_to_book_gt` argument.
    #[serde(
        rename = "price_to_book.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_book_gt: Option<f64>,
    /// The `price_to_book_gte` argument.
    #[serde(
        rename = "price_to_book.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_book_gte: Option<f64>,
    /// The `price_to_book_lt` argument.
    #[serde(
        rename = "price_to_book.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_book_lt: Option<f64>,
    /// The `price_to_book_lte` argument.
    #[serde(
        rename = "price_to_book.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_book_lte: Option<f64>,
    /// The `price_to_sales` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_sales: Option<f64>,
    /// The `price_to_sales_gt` argument.
    #[serde(
        rename = "price_to_sales.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_sales_gt: Option<f64>,
    /// The `price_to_sales_gte` argument.
    #[serde(
        rename = "price_to_sales.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_sales_gte: Option<f64>,
    /// The `price_to_sales_lt` argument.
    #[serde(
        rename = "price_to_sales.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_sales_lt: Option<f64>,
    /// The `price_to_sales_lte` argument.
    #[serde(
        rename = "price_to_sales.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_sales_lte: Option<f64>,
    /// The `price_to_cash_flow` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_cash_flow: Option<f64>,
    /// The `price_to_cash_flow_gt` argument.
    #[serde(
        rename = "price_to_cash_flow.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_cash_flow_gt: Option<f64>,
    /// The `price_to_cash_flow_gte` argument.
    #[serde(
        rename = "price_to_cash_flow.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_cash_flow_gte: Option<f64>,
    /// The `price_to_cash_flow_lt` argument.
    #[serde(
        rename = "price_to_cash_flow.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_cash_flow_lt: Option<f64>,
    /// The `price_to_cash_flow_lte` argument.
    #[serde(
        rename = "price_to_cash_flow.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_cash_flow_lte: Option<f64>,
    /// The `price_to_free_cash_flow` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_free_cash_flow: Option<f64>,
    /// The `price_to_free_cash_flow_gt` argument.
    #[serde(
        rename = "price_to_free_cash_flow.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_free_cash_flow_gt: Option<f64>,
    /// The `price_to_free_cash_flow_gte` argument.
    #[serde(
        rename = "price_to_free_cash_flow.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_free_cash_flow_gte: Option<f64>,
    /// The `price_to_free_cash_flow_lt` argument.
    #[serde(
        rename = "price_to_free_cash_flow.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_free_cash_flow_lt: Option<f64>,
    /// The `price_to_free_cash_flow_lte` argument.
    #[serde(
        rename = "price_to_free_cash_flow.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub price_to_free_cash_flow_lte: Option<f64>,
    /// The `dividend_yield` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub dividend_yield: Option<f64>,
    /// The `dividend_yield_gt` argument.
    #[serde(
        rename = "dividend_yield.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub dividend_yield_gt: Option<f64>,
    /// The `dividend_yield_gte` argument.
    #[serde(
        rename = "dividend_yield.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub dividend_yield_gte: Option<f64>,
    /// The `dividend_yield_lt` argument.
    #[serde(
        rename = "dividend_yield.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub dividend_yield_lt: Option<f64>,
    /// The `dividend_yield_lte` argument.
    #[serde(
        rename = "dividend_yield.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub dividend_yield_lte: Option<f64>,
    /// The `return_on_assets` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub return_on_assets: Option<f64>,
    /// The `return_on_assets_gt` argument.
    #[serde(
        rename = "return_on_assets.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub return_on_assets_gt: Option<f64>,
    /// The `return_on_assets_gte` argument.
    #[serde(
        rename = "return_on_assets.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub return_on_assets_gte: Option<f64>,
    /// The `return_on_assets_lt` argument.
    #[serde(
        rename = "return_on_assets.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub return_on_assets_lt: Option<f64>,
    /// The `return_on_assets_lte` argument.
    #[serde(
        rename = "return_on_assets.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub return_on_assets_lte: Option<f64>,
    /// The `return_on_equity` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub return_on_equity: Option<f64>,
    /// The `return_on_equity_gt` argument.
    #[serde(
        rename = "return_on_equity.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub return_on_equity_gt: Option<f64>,
    /// The `return_on_equity_gte` argument.
    #[serde(
        rename = "return_on_equity.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub return_on_equity_gte: Option<f64>,
    /// The `return_on_equity_lt` argument.
    #[serde(
        rename = "return_on_equity.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub return_on_equity_lt: Option<f64>,
    /// The `return_on_equity_lte` argument.
    #[serde(
        rename = "return_on_equity.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub return_on_equity_lte: Option<f64>,
    /// The `debt_to_equity` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub debt_to_equity: Option<f64>,
    /// The `debt_to_equity_gt` argument.
    #[serde(
        rename = "debt_to_equity.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub debt_to_equity_gt: Option<f64>,
    /// The `debt_to_equity_gte` argument.
    #[serde(
        rename = "debt_to_equity.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub debt_to_equity_gte: Option<f64>,
    /// The `debt_to_equity_lt` argument.
    #[serde(
        rename = "debt_to_equity.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub debt_to_equity_lt: Option<f64>,
    /// The `debt_to_equity_lte` argument.
    #[serde(
        rename = "debt_to_equity.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub debt_to_equity_lte: Option<f64>,
    /// The `current` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub current: Option<f64>,
    /// The `current_gt` argument.
    #[serde(
        rename = "current.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub current_gt: Option<f64>,
    /// The `current_gte` argument.
    #[serde(
        rename = "current.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub current_gte: Option<f64>,
    /// The `current_lt` argument.
    #[serde(
        rename = "current.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub current_lt: Option<f64>,
    /// The `current_lte` argument.
    #[serde(
        rename = "current.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub current_lte: Option<f64>,
    /// The `quick` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quick: Option<f64>,
    /// The `quick_gt` argument.
    #[serde(
        rename = "quick.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quick_gt: Option<f64>,
    /// The `quick_gte` argument.
    #[serde(
        rename = "quick.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quick_gte: Option<f64>,
    /// The `quick_lt` argument.
    #[serde(
        rename = "quick.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quick_lt: Option<f64>,
    /// The `quick_lte` argument.
    #[serde(
        rename = "quick.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub quick_lte: Option<f64>,
    /// The `cash` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub cash: Option<f64>,
    /// The `cash_gt` argument.
    #[serde(
        rename = "cash.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub cash_gt: Option<f64>,
    /// The `cash_gte` argument.
    #[serde(
        rename = "cash.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub cash_gte: Option<f64>,
    /// The `cash_lt` argument.
    #[serde(
        rename = "cash.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub cash_lt: Option<f64>,
    /// The `cash_lte` argument.
    #[serde(
        rename = "cash.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub cash_lte: Option<f64>,
    /// The `ev_to_sales` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub ev_to_sales: Option<f64>,
    /// The `ev_to_sales_gt` argument.
    #[serde(
        rename = "ev_to_sales.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub ev_to_sales_gt: Option<f64>,
    /// The `ev_to_sales_gte` argument.
    #[serde(
        rename = "ev_to_sales.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub ev_to_sales_gte: Option<f64>,
    /// The `ev_to_sales_lt` argument.
    #[serde(
        rename = "ev_to_sales.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub ev_to_sales_lt: Option<f64>,
    /// The `ev_to_sales_lte` argument.
    #[serde(
        rename = "ev_to_sales.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub ev_to_sales_lte: Option<f64>,
    /// The `ev_to_ebitda` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub ev_to_ebitda: Option<f64>,
    /// The `ev_to_ebitda_gt` argument.
    #[serde(
        rename = "ev_to_ebitda.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub ev_to_ebitda_gt: Option<f64>,
    /// The `ev_to_ebitda_gte` argument.
    #[serde(
        rename = "ev_to_ebitda.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub ev_to_ebitda_gte: Option<f64>,
    /// The `ev_to_ebitda_lt` argument.
    #[serde(
        rename = "ev_to_ebitda.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub ev_to_ebitda_lt: Option<f64>,
    /// The `ev_to_ebitda_lte` argument.
    #[serde(
        rename = "ev_to_ebitda.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub ev_to_ebitda_lte: Option<f64>,
    /// The `enterprise_value` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub enterprise_value: Option<f64>,
    /// The `enterprise_value_gt` argument.
    #[serde(
        rename = "enterprise_value.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub enterprise_value_gt: Option<f64>,
    /// The `enterprise_value_gte` argument.
    #[serde(
        rename = "enterprise_value.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub enterprise_value_gte: Option<f64>,
    /// The `enterprise_value_lt` argument.
    #[serde(
        rename = "enterprise_value.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub enterprise_value_lt: Option<f64>,
    /// The `enterprise_value_lte` argument.
    #[serde(
        rename = "enterprise_value.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub enterprise_value_lte: Option<f64>,
    /// The `free_cash_flow` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub free_cash_flow: Option<f64>,
    /// The `free_cash_flow_gt` argument.
    #[serde(
        rename = "free_cash_flow.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub free_cash_flow_gt: Option<f64>,
    /// The `free_cash_flow_gte` argument.
    #[serde(
        rename = "free_cash_flow.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub free_cash_flow_gte: Option<f64>,
    /// The `free_cash_flow_lt` argument.
    #[serde(
        rename = "free_cash_flow.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub free_cash_flow_lt: Option<f64>,
    /// The `free_cash_flow_lte` argument.
    #[serde(
        rename = "free_cash_flow.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub free_cash_flow_lte: Option<f64>,
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
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ListStocksFloatsParams {
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
    /// The `free_float_percent` argument.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub free_float_percent: Option<f64>,
    /// The `free_float_percent_gt` argument.
    #[serde(
        rename = "free_float_percent.gt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub free_float_percent_gt: Option<f64>,
    /// The `free_float_percent_gte` argument.
    #[serde(
        rename = "free_float_percent.gte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub free_float_percent_gte: Option<f64>,
    /// The `free_float_percent_lt` argument.
    #[serde(
        rename = "free_float_percent.lt",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub free_float_percent_lt: Option<f64>,
    /// The `free_float_percent_lte` argument.
    #[serde(
        rename = "free_float_percent.lte",
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::ser_opt_f64"
    )]
    pub free_float_percent_lte: Option<f64>,
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
