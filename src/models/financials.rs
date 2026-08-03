use serde::{Deserialize, Serialize};

/// A single numeric or textual data point in the financials.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DataPoint {
    pub label: Option<String>,
    pub order: Option<i64>,
    pub unit: Option<String>,
    pub value: Option<f64>,
    #[serde(rename = "derived_from")]
    pub derived_from: Option<Vec<String>>,
    pub formula: Option<String>,
    pub source: Option<std::collections::HashMap<String, String>>,
    pub xpath: Option<String>,
}

/// Balance sheet statement with per-line-item data points.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BalanceSheet {
    pub assets: Option<DataPoint>,
    #[serde(rename = "current_assets")]
    pub current_assets: Option<DataPoint>,
    pub cash: Option<DataPoint>,
    #[serde(rename = "accounts_receivable")]
    pub accounts_receivable: Option<DataPoint>,
    pub inventory: Option<DataPoint>,
    #[serde(rename = "prepaid_expenses")]
    pub prepaid_expenses: Option<DataPoint>,
    #[serde(rename = "other_current_assets")]
    pub other_current_assets: Option<DataPoint>,
    #[serde(rename = "noncurrent_assets")]
    pub noncurrent_assets: Option<DataPoint>,
    #[serde(rename = "long_term_investments")]
    pub long_term_investments: Option<DataPoint>,
    #[serde(rename = "fixed_assets")]
    pub fixed_assets: Option<DataPoint>,
    #[serde(rename = "intangible_assets")]
    pub intangible_assets: Option<DataPoint>,
    #[serde(rename = "noncurrent_prepaid_expense")]
    pub noncurrent_prepaid_expense: Option<DataPoint>,
    #[serde(rename = "other_noncurrent_assets")]
    pub other_noncurrent_assets: Option<DataPoint>,
    pub liabilities: Option<DataPoint>,
    #[serde(rename = "current_liabilities")]
    pub current_liabilities: Option<DataPoint>,
    #[serde(rename = "accounts_payable")]
    pub accounts_payable: Option<DataPoint>,
    #[serde(rename = "interest_payable")]
    pub interest_payable: Option<DataPoint>,
    pub wages: Option<DataPoint>,
    #[serde(rename = "other_current_liabilities")]
    pub other_current_liabilities: Option<DataPoint>,
    #[serde(rename = "noncurrent_liabilities")]
    pub noncurrent_liabilities: Option<DataPoint>,
    #[serde(rename = "long_term_debt")]
    pub long_term_debt: Option<DataPoint>,
    #[serde(rename = "other_noncurrent_liabilities")]
    pub other_noncurrent_liabilities: Option<DataPoint>,
    #[serde(rename = "commitments_and_contingencies")]
    pub commitments_and_contingencies: Option<DataPoint>,
    #[serde(rename = "redeemable_noncontrolling_interest")]
    pub redeemable_noncontrolling_interest: Option<DataPoint>,
    #[serde(rename = "redeemable_noncontrolling_interest_common")]
    pub redeemable_noncontrolling_interest_common: Option<DataPoint>,
    #[serde(rename = "redeemable_noncontrolling_interest_other")]
    pub redeemable_noncontrolling_interest_other: Option<DataPoint>,
    #[serde(rename = "redeemable_noncontrolling_interest_preferred")]
    pub redeemable_noncontrolling_interest_preferred: Option<DataPoint>,
    pub equity: Option<DataPoint>,
    #[serde(rename = "equity_attributable_to_noncontrolling_interest")]
    pub equity_attributable_to_noncontrolling_interest: Option<DataPoint>,
    #[serde(rename = "equity_attributable_to_parent")]
    pub equity_attributable_to_parent: Option<DataPoint>,
    #[serde(rename = "temporary_equity")]
    pub temporary_equity: Option<DataPoint>,
    #[serde(rename = "temporary_equity_attributable_to_parent")]
    pub temporary_equity_attributable_to_parent: Option<DataPoint>,
    #[serde(rename = "liabilities_and_equity")]
    pub liabilities_and_equity: Option<DataPoint>,
}

/// Cash flow statement with per-line-item data points.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CashFlowStatement {
    #[serde(rename = "net_cash_flow_from_operating_activities")]
    pub net_cash_flow_from_operating_activities: Option<DataPoint>,
    #[serde(rename = "net_cash_flow_from_operating_activities_continuing")]
    pub net_cash_flow_from_operating_activities_continuing: Option<DataPoint>,
    #[serde(rename = "net_cash_flow_from_operating_activities_discontinued")]
    pub net_cash_flow_from_operating_activities_discontinued: Option<DataPoint>,
    #[serde(rename = "net_cash_flow_from_investing_activities")]
    pub net_cash_flow_from_investing_activities: Option<DataPoint>,
    #[serde(rename = "net_cash_flow_from_investing_activities_continuing")]
    pub net_cash_flow_from_investing_activities_continuing: Option<DataPoint>,
    #[serde(rename = "net_cash_flow_from_investing_activities_discontinued")]
    pub net_cash_flow_from_investing_activities_discontinued: Option<DataPoint>,
    #[serde(rename = "net_cash_flow_from_financing_activities")]
    pub net_cash_flow_from_financing_activities: Option<DataPoint>,
    #[serde(rename = "net_cash_flow_from_financing_activities_continuing")]
    pub net_cash_flow_from_financing_activities_continuing: Option<DataPoint>,
    #[serde(rename = "net_cash_flow_from_financing_activities_discontinued")]
    pub net_cash_flow_from_financing_activities_discontinued: Option<DataPoint>,
    #[serde(rename = "exchange_gains_losses")]
    pub exchange_gains_losses: Option<DataPoint>,
    #[serde(rename = "net_cash_flow")]
    pub net_cash_flow: Option<DataPoint>,
    #[serde(rename = "net_cash_flow_continuing")]
    pub net_cash_flow_continuing: Option<DataPoint>,
    #[serde(rename = "net_cash_flow_discontinued")]
    pub net_cash_flow_discontinued: Option<DataPoint>,
}

/// Comprehensive income statement with per-line-item data points.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ComprehensiveIncome {
    #[serde(rename = "comprehensive_income_loss")]
    pub comprehensive_income_loss: Option<DataPoint>,
    #[serde(rename = "comprehensive_income_loss_attributable_to_noncontrolling_interest")]
    pub comprehensive_income_loss_attributable_to_noncontrolling_interest: Option<DataPoint>,
    #[serde(rename = "comprehensive_income_loss_attributable_to_parent")]
    pub comprehensive_income_loss_attributable_to_parent: Option<DataPoint>,
    #[serde(rename = "other_comprehensive_income_loss")]
    pub other_comprehensive_income_loss: Option<DataPoint>,
    #[serde(rename = "other_comprehensive_income_loss_attributable_to_noncontrolling_interest")]
    pub other_comprehensive_income_loss_attributable_to_noncontrolling_interest: Option<DataPoint>,
    #[serde(rename = "other_comprehensive_income_loss_attributable_to_parent")]
    pub other_comprehensive_income_loss_attributable_to_parent: Option<DataPoint>,
}

/// Income statement with per-line-item data points.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct IncomeStatement {
    pub revenues: Option<DataPoint>,
    #[serde(rename = "benefits_costs_expenses")]
    pub benefits_costs_expenses: Option<DataPoint>,
    #[serde(rename = "cost_of_revenue")]
    pub cost_of_revenue: Option<DataPoint>,
    #[serde(rename = "cost_of_revenue_goods")]
    pub cost_of_revenue_goods: Option<DataPoint>,
    #[serde(rename = "cost_of_revenue_services")]
    pub cost_of_revenue_services: Option<DataPoint>,
    #[serde(rename = "costs_and_expenses")]
    pub costs_and_expenses: Option<DataPoint>,
    #[serde(rename = "gross_profit")]
    pub gross_profit: Option<DataPoint>,
    #[serde(rename = "gain_loss_on_sale_properties_net_tax")]
    pub gain_loss_on_sale_properties_net_tax: Option<DataPoint>,
    #[serde(rename = "nonoperating_income_loss")]
    pub nonoperating_income_loss: Option<DataPoint>,
    #[serde(rename = "operating_expenses")]
    pub operating_expenses: Option<DataPoint>,
    #[serde(rename = "selling_general_and_administrative_expenses")]
    pub selling_general_and_administrative_expenses: Option<DataPoint>,
    #[serde(rename = "depreciation_and_amortization")]
    pub depreciation_and_amortization: Option<DataPoint>,
    #[serde(rename = "research_and_development")]
    pub research_and_development: Option<DataPoint>,
    #[serde(rename = "other_operating_expenses")]
    pub other_operating_expenses: Option<DataPoint>,
    #[serde(rename = "operating_income_loss")]
    pub operating_income_loss: Option<DataPoint>,
    #[serde(rename = "other_operating_income_expenses")]
    pub other_operating_income_expenses: Option<DataPoint>,
    #[serde(rename = "income_loss_before_equity_method_investments")]
    pub income_loss_before_equity_method_investments: Option<DataPoint>,
    #[serde(rename = "income_loss_from_continuing_operations_after_tax")]
    pub income_loss_from_continuing_operations_after_tax: Option<DataPoint>,
    #[serde(rename = "income_loss_from_continuing_operations_before_tax")]
    pub income_loss_from_continuing_operations_before_tax: Option<DataPoint>,
    #[serde(rename = "income_loss_from_discontinued_operations_net_of_tax")]
    pub income_loss_from_discontinued_operations_net_of_tax: Option<DataPoint>,
    #[serde(rename = "income_loss_from_discontinued_operations_net_of_tax_adjustment_to_prior_year_gain_loss_on_disposal")]
    pub income_loss_from_discontinued_operations_net_of_tax_adjustment_to_prior_year_gain_loss_on_disposal: Option<DataPoint>,
    #[serde(rename = "income_loss_from_discontinued_operations_net_of_tax_during_phase_out")]
    pub income_loss_from_discontinued_operations_net_of_tax_during_phase_out: Option<DataPoint>,
    #[serde(rename = "income_loss_from_discontinued_operations_net_of_tax_gain_loss_on_disposal")]
    pub income_loss_from_discontinued_operations_net_of_tax_gain_loss_on_disposal: Option<DataPoint>,
    #[serde(rename = "income_loss_from_discontinued_operations_net_of_tax_provision_for_gain_loss_on_disposal")]
    pub income_loss_from_discontinued_operations_net_of_tax_provision_for_gain_loss_on_disposal: Option<DataPoint>,
    #[serde(rename = "income_loss_from_equity_method_investments")]
    pub income_loss_from_equity_method_investments: Option<DataPoint>,
    #[serde(rename = "income_tax_expense_benefit")]
    pub income_tax_expense_benefit: Option<DataPoint>,
    #[serde(rename = "income_tax_expense_benefit_current")]
    pub income_tax_expense_benefit_current: Option<DataPoint>,
    #[serde(rename = "income_tax_expense_benefit_deferred")]
    pub income_tax_expense_benefit_deferred: Option<DataPoint>,
    #[serde(rename = "interest_and_debt_expense")]
    pub interest_and_debt_expense: Option<DataPoint>,
    #[serde(rename = "interest_and_dividend_income_operating")]
    pub interest_and_dividend_income_operating: Option<DataPoint>,
    #[serde(rename = "interest_expense_operating")]
    pub interest_expense_operating: Option<DataPoint>,
    #[serde(rename = "interest_income_expense_after_provision_for_losses")]
    pub interest_income_expense_after_provision_for_losses: Option<DataPoint>,
    #[serde(rename = "interest_income_expense_operating_net")]
    pub interest_income_expense_operating_net: Option<DataPoint>,
    #[serde(rename = "noninterest_expense")]
    pub noninterest_expense: Option<DataPoint>,
    #[serde(rename = "noninterest_income")]
    pub noninterest_income: Option<DataPoint>,
    #[serde(rename = "provision_for_loan_lease_and_other_losses")]
    pub provision_for_loan_lease_and_other_losses: Option<DataPoint>,
    #[serde(rename = "net_income_loss")]
    pub net_income_loss: Option<DataPoint>,
    #[serde(rename = "net_income_loss_attributable_to_noncontrolling_interest")]
    pub net_income_loss_attributable_to_noncontrolling_interest: Option<DataPoint>,
    #[serde(rename = "net_income_loss_attributable_to_nonredeemable_noncontrolling_interest")]
    pub net_income_loss_attributable_to_nonredeemable_noncontrolling_interest: Option<DataPoint>,
    #[serde(rename = "net_income_loss_attributable_to_parent")]
    pub net_income_loss_attributable_to_parent: Option<DataPoint>,
    #[serde(rename = "net_income_loss_attributable_to_redeemable_noncontrolling_interest")]
    pub net_income_loss_attributable_to_redeemable_noncontrolling_interest: Option<DataPoint>,
    #[serde(rename = "net_income_loss_available_to_common_stockholders_basic")]
    pub net_income_loss_available_to_common_stockholders_basic: Option<DataPoint>,
    #[serde(rename = "participating_securities_distributed_and_undistributed_earnings_loss_basic")]
    pub participating_securities_distributed_and_undistributed_earnings_loss_basic: Option<DataPoint>,
    #[serde(rename = "undistributed_earnings_loss_allocated_to_participating_securities_basic")]
    pub undistributed_earnings_loss_allocated_to_participating_securities_basic: Option<DataPoint>,
    #[serde(rename = "preferred_stock_dividends_and_other_adjustments")]
    pub preferred_stock_dividends_and_other_adjustments: Option<DataPoint>,
    #[serde(rename = "basic_earnings_per_share")]
    pub basic_earnings_per_share: Option<DataPoint>,
    #[serde(rename = "diluted_earnings_per_share")]
    pub diluted_earnings_per_share: Option<DataPoint>,
    #[serde(rename = "basic_average_shares")]
    pub basic_average_shares: Option<DataPoint>,
    #[serde(rename = "diluted_average_shares")]
    pub diluted_average_shares: Option<DataPoint>,
    #[serde(rename = "common_stock_dividends")]
    pub common_stock_dividends: Option<DataPoint>,
}

/// Financial statements: balance sheet, cash flow, comprehensive income, income statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Financials {
    #[serde(rename = "balance_sheet")]
    pub balance_sheet: Option<BalanceSheet>,
    #[serde(rename = "cash_flow_statement")]
    pub cash_flow_statement: Option<CashFlowStatement>,
    #[serde(rename = "comprehensive_income")]
    pub comprehensive_income: Option<ComprehensiveIncome>,
    #[serde(rename = "income_statement")]
    pub income_statement: Option<IncomeStatement>,
}

/// Historical financial data for a stock ticker.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct StockFinancial {
    pub cik: Option<String>,
    #[serde(rename = "company_name")]
    pub company_name: Option<String>,
    #[serde(rename = "end_date")]
    pub end_date: Option<String>,
    #[serde(rename = "filing_date")]
    pub filing_date: Option<String>,
    pub financials: Option<Financials>,
    #[serde(rename = "fiscal_period")]
    pub fiscal_period: Option<String>,
    #[serde(rename = "fiscal_year")]
    pub fiscal_year: Option<String>,
    #[serde(rename = "source_filing_file_url")]
    pub source_filing_file_url: Option<String>,
    #[serde(rename = "source_filing_url")]
    pub source_filing_url: Option<String>,
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FinancialBalanceSheet {
    #[serde(rename = "accounts_payable")]
    pub accounts_payable: Option<f64>,
    #[serde(rename = "accrued_and_other_current_liabilities")]
    pub accrued_and_other_current_liabilities: Option<f64>,
    #[serde(rename = "accumulated_other_comprehensive_income")]
    pub accumulated_other_comprehensive_income: Option<f64>,
    #[serde(rename = "additional_paid_in_capital")]
    pub additional_paid_in_capital: Option<f64>,
    #[serde(rename = "cash_and_equivalents")]
    pub cash_and_equivalents: Option<f64>,
    pub cik: Option<String>,
    #[serde(rename = "commitments_and_contingencies")]
    pub commitments_and_contingencies: Option<f64>,
    #[serde(rename = "common_stock")]
    pub common_stock: Option<f64>,
    #[serde(rename = "debt_current")]
    pub debt_current: Option<f64>,
    #[serde(rename = "deferred_revenue_current")]
    pub deferred_revenue_current: Option<f64>,
    #[serde(rename = "filing_date")]
    pub filing_date: Option<String>,
    #[serde(rename = "fiscal_quarter")]
    pub fiscal_quarter: Option<f64>,
    #[serde(rename = "fiscal_year")]
    pub fiscal_year: Option<f64>,
    pub goodwill: Option<f64>,
    #[serde(rename = "intangible_assets_net")]
    pub intangible_assets_net: Option<f64>,
    pub inventories: Option<f64>,
    #[serde(rename = "long_term_debt_and_capital_lease_obligations")]
    pub long_term_debt_and_capital_lease_obligations: Option<f64>,
    #[serde(rename = "noncontrolling_interest")]
    pub noncontrolling_interest: Option<f64>,
    #[serde(rename = "other_assets")]
    pub other_assets: Option<f64>,
    #[serde(rename = "other_current_assets")]
    pub other_current_assets: Option<f64>,
    #[serde(rename = "other_equity")]
    pub other_equity: Option<f64>,
    #[serde(rename = "other_noncurrent_liabilities")]
    pub other_noncurrent_liabilities: Option<f64>,
    #[serde(rename = "period_end")]
    pub period_end: Option<String>,
    #[serde(rename = "preferred_stock")]
    pub preferred_stock: Option<f64>,
    #[serde(rename = "property_plant_equipment_net")]
    pub property_plant_equipment_net: Option<f64>,
    pub receivables: Option<f64>,
    #[serde(rename = "retained_earnings_deficit")]
    pub retained_earnings_deficit: Option<f64>,
    #[serde(rename = "short_term_investments")]
    pub short_term_investments: Option<f64>,
    pub tickers: Option<Vec<String>>,
    pub timeframe: Option<String>,
    #[serde(rename = "total_assets")]
    pub total_assets: Option<f64>,
    #[serde(rename = "total_current_assets")]
    pub total_current_assets: Option<f64>,
    #[serde(rename = "total_current_liabilities")]
    pub total_current_liabilities: Option<f64>,
    #[serde(rename = "total_equity")]
    pub total_equity: Option<f64>,
    #[serde(rename = "total_equity_attributable_to_parent")]
    pub total_equity_attributable_to_parent: Option<f64>,
    #[serde(rename = "total_liabilities")]
    pub total_liabilities: Option<f64>,
    #[serde(rename = "total_liabilities_and_equity")]
    pub total_liabilities_and_equity: Option<f64>,
    #[serde(rename = "treasury_stock")]
    pub treasury_stock: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FinancialCashFlowStatement {
    #[serde(rename = "cash_from_operating_activities_continuing_operations")]
    pub cash_from_operating_activities_continuing_operations: Option<f64>,
    #[serde(rename = "change_in_cash_and_equivalents")]
    pub change_in_cash_and_equivalents: Option<f64>,
    #[serde(rename = "change_in_other_operating_assets_and_liabilities_net")]
    pub change_in_other_operating_assets_and_liabilities_net: Option<f64>,
    pub cik: Option<String>,
    #[serde(rename = "depreciation_depletion_and_amortization")]
    pub depreciation_depletion_and_amortization: Option<f64>,
    pub dividends: Option<f64>,
    #[serde(rename = "effect_of_currency_exchange_rate")]
    pub effect_of_currency_exchange_rate: Option<f64>,
    #[serde(rename = "filing_date")]
    pub filing_date: Option<String>,
    #[serde(rename = "fiscal_quarter")]
    pub fiscal_quarter: Option<f64>,
    #[serde(rename = "fiscal_year")]
    pub fiscal_year: Option<f64>,
    #[serde(rename = "income_loss_from_discontinued_operations")]
    pub income_loss_from_discontinued_operations: Option<f64>,
    #[serde(rename = "long_term_debt_issuances_repayments")]
    pub long_term_debt_issuances_repayments: Option<f64>,
    #[serde(rename = "net_cash_from_financing_activities")]
    pub net_cash_from_financing_activities: Option<f64>,
    #[serde(rename = "net_cash_from_financing_activities_continuing_operations")]
    pub net_cash_from_financing_activities_continuing_operations: Option<f64>,
    #[serde(rename = "net_cash_from_financing_activities_discontinued_operations")]
    pub net_cash_from_financing_activities_discontinued_operations: Option<f64>,
    #[serde(rename = "net_cash_from_investing_activities")]
    pub net_cash_from_investing_activities: Option<f64>,
    #[serde(rename = "net_cash_from_investing_activities_continuing_operations")]
    pub net_cash_from_investing_activities_continuing_operations: Option<f64>,
    #[serde(rename = "net_cash_from_investing_activities_discontinued_operations")]
    pub net_cash_from_investing_activities_discontinued_operations: Option<f64>,
    #[serde(rename = "net_cash_from_operating_activities")]
    pub net_cash_from_operating_activities: Option<f64>,
    #[serde(rename = "net_cash_from_operating_activities_discontinued_operations")]
    pub net_cash_from_operating_activities_discontinued_operations: Option<f64>,
    #[serde(rename = "net_income")]
    pub net_income: Option<f64>,
    #[serde(rename = "noncontrolling_interests")]
    pub noncontrolling_interests: Option<f64>,
    #[serde(rename = "other_cash_adjustments")]
    pub other_cash_adjustments: Option<f64>,
    #[serde(rename = "other_financing_activities")]
    pub other_financing_activities: Option<f64>,
    #[serde(rename = "other_investing_activities")]
    pub other_investing_activities: Option<f64>,
    #[serde(rename = "other_operating_activities")]
    pub other_operating_activities: Option<f64>,
    #[serde(rename = "period_end")]
    pub period_end: Option<String>,
    #[serde(rename = "purchase_of_property_plant_and_equipment")]
    pub purchase_of_property_plant_and_equipment: Option<f64>,
    #[serde(rename = "sale_of_property_plant_and_equipment")]
    pub sale_of_property_plant_and_equipment: Option<f64>,
    #[serde(rename = "short_term_debt_issuances_repayments")]
    pub short_term_debt_issuances_repayments: Option<f64>,
    pub tickers: Option<Vec<String>>,
    pub timeframe: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FinancialIncomeStatement {
    #[serde(rename = "basic_earnings_per_share")]
    pub basic_earnings_per_share: Option<f64>,
    #[serde(rename = "basic_shares_outstanding")]
    pub basic_shares_outstanding: Option<f64>,
    pub cik: Option<String>,
    #[serde(rename = "consolidated_net_income_loss")]
    pub consolidated_net_income_loss: Option<f64>,
    #[serde(rename = "cost_of_revenue")]
    pub cost_of_revenue: Option<f64>,
    #[serde(rename = "depreciation_depletion_amortization")]
    pub depreciation_depletion_amortization: Option<f64>,
    #[serde(rename = "diluted_earnings_per_share")]
    pub diluted_earnings_per_share: Option<f64>,
    #[serde(rename = "diluted_shares_outstanding")]
    pub diluted_shares_outstanding: Option<f64>,
    #[serde(rename = "discontinued_operations")]
    pub discontinued_operations: Option<f64>,
    pub ebitda: Option<f64>,
    #[serde(rename = "equity_in_affiliates")]
    pub equity_in_affiliates: Option<f64>,
    #[serde(rename = "extraordinary_items")]
    pub extraordinary_items: Option<f64>,
    #[serde(rename = "filing_date")]
    pub filing_date: Option<String>,
    #[serde(rename = "fiscal_quarter")]
    pub fiscal_quarter: Option<f64>,
    #[serde(rename = "fiscal_year")]
    pub fiscal_year: Option<f64>,
    #[serde(rename = "gross_profit")]
    pub gross_profit: Option<f64>,
    #[serde(rename = "income_before_income_taxes")]
    pub income_before_income_taxes: Option<f64>,
    #[serde(rename = "income_taxes")]
    pub income_taxes: Option<f64>,
    #[serde(rename = "interest_expense")]
    pub interest_expense: Option<f64>,
    #[serde(rename = "interest_income")]
    pub interest_income: Option<f64>,
    #[serde(rename = "net_income_loss_attributable_common_shareholders")]
    pub net_income_loss_attributable_common_shareholders: Option<f64>,
    #[serde(rename = "noncontrolling_interest")]
    pub noncontrolling_interest: Option<f64>,
    #[serde(rename = "operating_income")]
    pub operating_income: Option<f64>,
    #[serde(rename = "other_income_expense")]
    pub other_income_expense: Option<f64>,
    #[serde(rename = "other_operating_expenses")]
    pub other_operating_expenses: Option<f64>,
    #[serde(rename = "period_end")]
    pub period_end: Option<String>,
    #[serde(rename = "preferred_stock_dividends_declared")]
    pub preferred_stock_dividends_declared: Option<f64>,
    #[serde(rename = "research_development")]
    pub research_development: Option<f64>,
    pub revenue: Option<f64>,
    #[serde(rename = "selling_general_administrative")]
    pub selling_general_administrative: Option<f64>,
    pub tickers: Option<Vec<String>>,
    pub timeframe: Option<String>,
    #[serde(rename = "total_operating_expenses")]
    pub total_operating_expenses: Option<f64>,
    #[serde(rename = "total_other_income_expense")]
    pub total_other_income_expense: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FinancialRatio {
    #[serde(rename = "average_volume")]
    pub average_volume: Option<f64>,
    pub cash: Option<f64>,
    pub cik: Option<String>,
    pub current: Option<f64>,
    pub date: Option<String>,
    #[serde(rename = "debt_to_equity")]
    pub debt_to_equity: Option<f64>,
    #[serde(rename = "dividend_yield")]
    pub dividend_yield: Option<f64>,
    #[serde(rename = "earnings_per_share")]
    pub earnings_per_share: Option<f64>,
    #[serde(rename = "enterprise_value")]
    pub enterprise_value: Option<f64>,
    #[serde(rename = "ev_to_ebitda")]
    pub ev_to_ebitda: Option<f64>,
    #[serde(rename = "ev_to_sales")]
    pub ev_to_sales: Option<f64>,
    #[serde(rename = "free_cash_flow")]
    pub free_cash_flow: Option<f64>,
    #[serde(rename = "market_cap")]
    pub market_cap: Option<f64>,
    pub price: Option<f64>,
    #[serde(rename = "price_to_book")]
    pub price_to_book: Option<f64>,
    #[serde(rename = "price_to_cash_flow")]
    pub price_to_cash_flow: Option<f64>,
    #[serde(rename = "price_to_earnings")]
    pub price_to_earnings: Option<f64>,
    #[serde(rename = "price_to_free_cash_flow")]
    pub price_to_free_cash_flow: Option<f64>,
    #[serde(rename = "price_to_sales")]
    pub price_to_sales: Option<f64>,
    pub quick: Option<f64>,
    #[serde(rename = "return_on_assets")]
    pub return_on_assets: Option<f64>,
    #[serde(rename = "return_on_equity")]
    pub return_on_equity: Option<f64>,
    pub ticker: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FinancialFloat {
    #[serde(rename = "effective_date")]
    pub effective_date: Option<String>,
    #[serde(rename = "free_float")]
    pub free_float: Option<i64>,
    #[serde(rename = "free_float_percent")]
    pub free_float_percent: Option<f64>,
    pub ticker: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RiskFactor {
    pub cik: Option<String>,
    #[serde(rename = "filing_date")]
    pub filing_date: Option<String>,
    #[serde(rename = "primary_category")]
    pub primary_category: Option<String>,
    #[serde(rename = "secondary_category")]
    pub secondary_category: Option<String>,
    #[serde(rename = "supporting_text")]
    pub supporting_text: Option<String>,
    #[serde(rename = "tertiary_category")]
    pub tertiary_category: Option<String>,
    pub ticker: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RiskFactorTaxonomy {
    pub description: Option<String>,
    #[serde(rename = "primary_category")]
    pub primary_category: Option<String>,
    #[serde(rename = "secondary_category")]
    pub secondary_category: Option<String>,
    pub taxonomy: Option<f64>,
    #[serde(rename = "tertiary_category")]
    pub tertiary_category: Option<String>,
}

/// A single tagged disclosure within an SEC 8-K filing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Disclosure {
    #[serde(rename = "accession_number")]
    pub accession_number: Option<String>,
    pub cik: Option<String>,
    #[serde(rename = "filing_date")]
    pub filing_date: Option<String>,
    #[serde(rename = "filing_url")]
    pub filing_url: Option<String>,
    #[serde(rename = "primary_category")]
    pub primary_category: Option<String>,
    #[serde(rename = "secondary_category")]
    pub secondary_category: Option<String>,
    #[serde(rename = "supporting_text")]
    pub supporting_text: Option<String>,
    #[serde(rename = "tertiary_category")]
    pub tertiary_category: Option<String>,
    pub tickers: Option<Vec<String>>,
}

/// A single 8-K disclosure classification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DisclosureTaxonomy {
    pub description: Option<String>,
    #[serde(rename = "primary_category")]
    pub primary_category: Option<String>,
    #[serde(rename = "secondary_category")]
    pub secondary_category: Option<String>,
    pub taxonomy: Option<String>,
    #[serde(rename = "tertiary_category")]
    pub tertiary_category: Option<String>,
}

/// SEC Form 13F filings data showing institutional investment manager holdings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Filing13F {
    #[serde(rename = "accession_number")]
    pub accession_number: Option<String>,
    pub cusip: Option<String>,
    #[serde(rename = "file_number")]
    pub file_number: Option<String>,
    #[serde(rename = "filer_cik")]
    pub filer_cik: Option<String>,
    #[serde(rename = "filing_date")]
    pub filing_date: Option<String>,
    #[serde(rename = "filing_url")]
    pub filing_url: Option<String>,
    #[serde(rename = "film_number")]
    pub film_number: Option<String>,
    #[serde(rename = "form_type")]
    pub form_type: Option<String>,
    #[serde(rename = "investment_discretion")]
    pub investment_discretion: Option<String>,
    #[serde(rename = "issuer_name")]
    pub issuer_name: Option<String>,
    #[serde(rename = "market_value")]
    pub market_value: Option<i64>,
    #[serde(rename = "other_managers")]
    pub other_managers: Option<Vec<String>>,
    pub period: Option<String>,
    #[serde(rename = "put_call")]
    pub put_call: Option<String>,
    #[serde(rename = "shares_or_principal_amount")]
    pub shares_or_principal_amount: Option<i64>,
    #[serde(rename = "shares_or_principal_type")]
    pub shares_or_principal_type: Option<String>,
    #[serde(rename = "title_of_class")]
    pub title_of_class: Option<String>,
    #[serde(rename = "voting_authority_none")]
    pub voting_authority_none: Option<i64>,
    #[serde(rename = "voting_authority_shared")]
    pub voting_authority_shared: Option<i64>,
    #[serde(rename = "voting_authority_sole")]
    pub voting_authority_sole: Option<i64>,
}

/// SEC document text section from a 10-K/10-Q (raw text content).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FilingSection {
    pub cik: Option<String>,
    #[serde(rename = "filing_date")]
    pub filing_date: Option<String>,
    #[serde(rename = "filing_url")]
    pub filing_url: Option<String>,
    #[serde(rename = "period_end")]
    pub period_end: Option<String>,
    pub section: Option<String>,
    pub text: Option<String>,
    pub ticker: Option<String>,
}

/// Footnote from SEC Form 3/4 filings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FilingFootnote {
    pub id: Option<String>,
    pub description: Option<String>,
}

/// SEC Form 3 filings reporting initial statements of beneficial ownership of securities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FilingForm3 {
    #[serde(rename = "accession_number")]
    pub accession_number: Option<String>,
    #[serde(rename = "aff_10b5_one")]
    pub aff_10b5_one: Option<bool>,
    #[serde(rename = "date_of_original_submission")]
    pub date_of_original_submission: Option<String>,
    #[serde(rename = "direct_or_indirect")]
    pub direct_or_indirect: Option<String>,
    #[serde(rename = "exercise_date")]
    pub exercise_date: Option<String>,
    #[serde(rename = "exercise_price")]
    pub exercise_price: Option<f64>,
    #[serde(rename = "filing_date")]
    pub filing_date: Option<String>,
    #[serde(rename = "filing_url")]
    pub filing_url: Option<String>,
    pub footnotes: Option<Vec<FilingFootnote>>,
    #[serde(rename = "form_type")]
    pub form_type: Option<String>,
    #[serde(rename = "is_director")]
    pub is_director: Option<bool>,
    #[serde(rename = "is_officer")]
    pub is_officer: Option<bool>,
    #[serde(rename = "is_other")]
    pub is_other: Option<bool>,
    #[serde(rename = "is_ten_percent_owner")]
    pub is_ten_percent_owner: Option<bool>,
    #[serde(rename = "issuer_cik")]
    pub issuer_cik: Option<String>,
    #[serde(rename = "issuer_name")]
    pub issuer_name: Option<String>,
    #[serde(rename = "nature_of_ownership")]
    pub nature_of_ownership: Option<String>,
    #[serde(rename = "not_subject_to_section_16")]
    pub not_subject_to_section_16: Option<bool>,
    #[serde(rename = "officer_title")]
    pub officer_title: Option<String>,
    #[serde(rename = "owner_cik")]
    pub owner_cik: Option<String>,
    #[serde(rename = "owner_name")]
    pub owner_name: Option<String>,
    #[serde(rename = "period_of_report")]
    pub period_of_report: Option<String>,
    pub remarks: Option<String>,
    #[serde(rename = "security_title")]
    pub security_title: Option<String>,
    #[serde(rename = "security_type")]
    pub security_type: Option<String>,
    #[serde(rename = "shares_owned")]
    pub shares_owned: Option<f64>,
    pub tickers: Option<Vec<String>>,
    #[serde(rename = "underlying_security_shares")]
    pub underlying_security_shares: Option<f64>,
    #[serde(rename = "underlying_security_title")]
    pub underlying_security_title: Option<String>,
}

/// SEC Form 4 filings reporting changes in beneficial ownership of securities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FilingForm4 {
    #[serde(rename = "accession_number")]
    pub accession_number: Option<String>,
    #[serde(rename = "aff_10b5_one")]
    pub aff_10b5_one: Option<bool>,
    #[serde(rename = "date_of_original_submission")]
    pub date_of_original_submission: Option<String>,
    #[serde(rename = "deemed_execution_date")]
    pub deemed_execution_date: Option<String>,
    #[serde(rename = "direct_or_indirect")]
    pub direct_or_indirect: Option<String>,
    #[serde(rename = "equity_swap_involved")]
    pub equity_swap_involved: Option<bool>,
    #[serde(rename = "exercise_date")]
    pub exercise_date: Option<String>,
    #[serde(rename = "exercise_price")]
    pub exercise_price: Option<f64>,
    #[serde(rename = "expiration_date")]
    pub expiration_date: Option<String>,
    #[serde(rename = "filing_date")]
    pub filing_date: Option<String>,
    #[serde(rename = "filing_url")]
    pub filing_url: Option<String>,
    pub footnotes: Option<Vec<FilingFootnote>>,
    #[serde(rename = "form_type")]
    pub form_type: Option<String>,
    #[serde(rename = "is_director")]
    pub is_director: Option<bool>,
    #[serde(rename = "is_officer")]
    pub is_officer: Option<bool>,
    #[serde(rename = "is_other")]
    pub is_other: Option<bool>,
    #[serde(rename = "is_ten_percent_owner")]
    pub is_ten_percent_owner: Option<bool>,
    #[serde(rename = "issuer_cik")]
    pub issuer_cik: Option<String>,
    #[serde(rename = "issuer_name")]
    pub issuer_name: Option<String>,
    #[serde(rename = "nature_of_ownership")]
    pub nature_of_ownership: Option<String>,
    #[serde(rename = "not_subject_to_section_16")]
    pub not_subject_to_section_16: Option<bool>,
    #[serde(rename = "officer_title")]
    pub officer_title: Option<String>,
    #[serde(rename = "owner_cik")]
    pub owner_cik: Option<String>,
    #[serde(rename = "owner_name")]
    pub owner_name: Option<String>,
    #[serde(rename = "period_of_report")]
    pub period_of_report: Option<String>,
    #[serde(rename = "record_type")]
    pub record_type: Option<String>,
    pub remarks: Option<String>,
    #[serde(rename = "security_title")]
    pub security_title: Option<String>,
    #[serde(rename = "security_type")]
    pub security_type: Option<String>,
    #[serde(rename = "shares_owned_following_transaction")]
    pub shares_owned_following_transaction: Option<f64>,
    pub tickers: Option<Vec<String>>,
    #[serde(rename = "transaction_acquired_disposed")]
    pub transaction_acquired_disposed: Option<String>,
    #[serde(rename = "transaction_code")]
    pub transaction_code: Option<String>,
    #[serde(rename = "transaction_date")]
    pub transaction_date: Option<String>,
    #[serde(rename = "transaction_price_per_share")]
    pub transaction_price_per_share: Option<f64>,
    #[serde(rename = "transaction_shares")]
    pub transaction_shares: Option<f64>,
    #[serde(rename = "transaction_timeliness")]
    pub transaction_timeliness: Option<String>,
    #[serde(rename = "transaction_value")]
    pub transaction_value: Option<f64>,
    #[serde(rename = "underlying_security_shares")]
    pub underlying_security_shares: Option<f64>,
    #[serde(rename = "underlying_security_title")]
    pub underlying_security_title: Option<String>,
}

/// Parsed 8-K filing with item-level text content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Filing8K {
    #[serde(rename = "accession_number")]
    pub accession_number: Option<String>,
    pub cik: Option<String>,
    #[serde(rename = "filing_date")]
    pub filing_date: Option<String>,
    #[serde(rename = "filing_url")]
    pub filing_url: Option<String>,
    #[serde(rename = "form_type")]
    pub form_type: Option<String>,
    #[serde(rename = "items_text")]
    pub items_text: Option<String>,
    pub ticker: Option<String>,
}

/// Master index entry for any SEC filing (10-K, 8-K, 10-Q, etc.).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FilingIndex {
    #[serde(rename = "accession_number")]
    pub accession_number: Option<String>,
    pub cik: Option<String>,
    #[serde(rename = "filing_date")]
    pub filing_date: Option<String>,
    #[serde(rename = "filing_url")]
    pub filing_url: Option<String>,
    #[serde(rename = "form_type")]
    pub form_type: Option<String>,
    #[serde(rename = "issuer_name")]
    pub issuer_name: Option<String>,
    pub ticker: Option<String>,
}
