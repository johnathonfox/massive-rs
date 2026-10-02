use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// A single numeric or textual data point in the financials.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DataPoint {
    pub label: Option<String>,
    pub order: Option<i64>,
    pub unit: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub value: Option<Decimal>,
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
    #[serde(
        rename = "income_loss_from_discontinued_operations_net_of_tax_adjustment_to_prior_year_gain_loss_on_disposal"
    )]
    pub income_loss_from_discontinued_operations_net_of_tax_adjustment_to_prior_year_gain_loss_on_disposal:
        Option<DataPoint>,
    #[serde(rename = "income_loss_from_discontinued_operations_net_of_tax_during_phase_out")]
    pub income_loss_from_discontinued_operations_net_of_tax_during_phase_out: Option<DataPoint>,
    #[serde(rename = "income_loss_from_discontinued_operations_net_of_tax_gain_loss_on_disposal")]
    pub income_loss_from_discontinued_operations_net_of_tax_gain_loss_on_disposal:
        Option<DataPoint>,
    #[serde(
        rename = "income_loss_from_discontinued_operations_net_of_tax_provision_for_gain_loss_on_disposal"
    )]
    pub income_loss_from_discontinued_operations_net_of_tax_provision_for_gain_loss_on_disposal:
        Option<DataPoint>,
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
    pub participating_securities_distributed_and_undistributed_earnings_loss_basic:
        Option<DataPoint>,
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
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub accounts_payable: Option<Decimal>,
    #[serde(rename = "accrued_and_other_current_liabilities")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub accrued_and_other_current_liabilities: Option<Decimal>,
    #[serde(rename = "accumulated_other_comprehensive_income")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub accumulated_other_comprehensive_income: Option<Decimal>,
    #[serde(rename = "additional_paid_in_capital")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub additional_paid_in_capital: Option<Decimal>,
    #[serde(rename = "cash_and_equivalents")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub cash_and_equivalents: Option<Decimal>,
    pub cik: Option<String>,
    #[serde(rename = "commitments_and_contingencies")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub commitments_and_contingencies: Option<Decimal>,
    #[serde(rename = "common_stock")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub common_stock: Option<Decimal>,
    #[serde(rename = "debt_current")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub debt_current: Option<Decimal>,
    #[serde(rename = "deferred_revenue_current")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub deferred_revenue_current: Option<Decimal>,
    #[serde(rename = "filing_date")]
    pub filing_date: Option<String>,
    #[serde(rename = "fiscal_quarter")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub fiscal_quarter: Option<Decimal>,
    #[serde(rename = "fiscal_year")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub fiscal_year: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub goodwill: Option<Decimal>,
    #[serde(rename = "intangible_assets_net")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub intangible_assets_net: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub inventories: Option<Decimal>,
    #[serde(rename = "long_term_debt_and_capital_lease_obligations")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub long_term_debt_and_capital_lease_obligations: Option<Decimal>,
    #[serde(rename = "noncontrolling_interest")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub noncontrolling_interest: Option<Decimal>,
    #[serde(rename = "other_assets")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub other_assets: Option<Decimal>,
    #[serde(rename = "other_current_assets")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub other_current_assets: Option<Decimal>,
    #[serde(rename = "other_equity")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub other_equity: Option<Decimal>,
    #[serde(rename = "other_noncurrent_liabilities")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub other_noncurrent_liabilities: Option<Decimal>,
    #[serde(rename = "period_end")]
    pub period_end: Option<String>,
    #[serde(rename = "preferred_stock")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub preferred_stock: Option<Decimal>,
    #[serde(rename = "property_plant_equipment_net")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub property_plant_equipment_net: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub receivables: Option<Decimal>,
    #[serde(rename = "retained_earnings_deficit")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub retained_earnings_deficit: Option<Decimal>,
    #[serde(rename = "short_term_investments")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub short_term_investments: Option<Decimal>,
    pub tickers: Option<Vec<String>>,
    pub timeframe: Option<String>,
    #[serde(rename = "total_assets")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub total_assets: Option<Decimal>,
    #[serde(rename = "total_current_assets")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub total_current_assets: Option<Decimal>,
    #[serde(rename = "total_current_liabilities")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub total_current_liabilities: Option<Decimal>,
    #[serde(rename = "total_equity")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub total_equity: Option<Decimal>,
    #[serde(rename = "total_equity_attributable_to_parent")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub total_equity_attributable_to_parent: Option<Decimal>,
    #[serde(rename = "total_liabilities")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub total_liabilities: Option<Decimal>,
    #[serde(rename = "total_liabilities_and_equity")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub total_liabilities_and_equity: Option<Decimal>,
    #[serde(rename = "treasury_stock")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub treasury_stock: Option<Decimal>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FinancialCashFlowStatement {
    #[serde(rename = "cash_from_operating_activities_continuing_operations")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub cash_from_operating_activities_continuing_operations: Option<Decimal>,
    #[serde(rename = "change_in_cash_and_equivalents")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub change_in_cash_and_equivalents: Option<Decimal>,
    #[serde(rename = "change_in_other_operating_assets_and_liabilities_net")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub change_in_other_operating_assets_and_liabilities_net: Option<Decimal>,
    pub cik: Option<String>,
    #[serde(rename = "depreciation_depletion_and_amortization")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub depreciation_depletion_and_amortization: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub dividends: Option<Decimal>,
    #[serde(rename = "effect_of_currency_exchange_rate")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub effect_of_currency_exchange_rate: Option<Decimal>,
    #[serde(rename = "filing_date")]
    pub filing_date: Option<String>,
    #[serde(rename = "fiscal_quarter")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub fiscal_quarter: Option<Decimal>,
    #[serde(rename = "fiscal_year")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub fiscal_year: Option<Decimal>,
    #[serde(rename = "income_loss_from_discontinued_operations")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub income_loss_from_discontinued_operations: Option<Decimal>,
    #[serde(rename = "long_term_debt_issuances_repayments")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub long_term_debt_issuances_repayments: Option<Decimal>,
    #[serde(rename = "net_cash_from_financing_activities")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub net_cash_from_financing_activities: Option<Decimal>,
    #[serde(rename = "net_cash_from_financing_activities_continuing_operations")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub net_cash_from_financing_activities_continuing_operations: Option<Decimal>,
    #[serde(rename = "net_cash_from_financing_activities_discontinued_operations")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub net_cash_from_financing_activities_discontinued_operations: Option<Decimal>,
    #[serde(rename = "net_cash_from_investing_activities")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub net_cash_from_investing_activities: Option<Decimal>,
    #[serde(rename = "net_cash_from_investing_activities_continuing_operations")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub net_cash_from_investing_activities_continuing_operations: Option<Decimal>,
    #[serde(rename = "net_cash_from_investing_activities_discontinued_operations")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub net_cash_from_investing_activities_discontinued_operations: Option<Decimal>,
    #[serde(rename = "net_cash_from_operating_activities")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub net_cash_from_operating_activities: Option<Decimal>,
    #[serde(rename = "net_cash_from_operating_activities_discontinued_operations")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub net_cash_from_operating_activities_discontinued_operations: Option<Decimal>,
    #[serde(rename = "net_income")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub net_income: Option<Decimal>,
    #[serde(rename = "noncontrolling_interests")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub noncontrolling_interests: Option<Decimal>,
    #[serde(rename = "other_cash_adjustments")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub other_cash_adjustments: Option<Decimal>,
    #[serde(rename = "other_financing_activities")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub other_financing_activities: Option<Decimal>,
    #[serde(rename = "other_investing_activities")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub other_investing_activities: Option<Decimal>,
    #[serde(rename = "other_operating_activities")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub other_operating_activities: Option<Decimal>,
    #[serde(rename = "period_end")]
    pub period_end: Option<String>,
    #[serde(rename = "purchase_of_property_plant_and_equipment")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub purchase_of_property_plant_and_equipment: Option<Decimal>,
    #[serde(rename = "sale_of_property_plant_and_equipment")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub sale_of_property_plant_and_equipment: Option<Decimal>,
    #[serde(rename = "short_term_debt_issuances_repayments")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub short_term_debt_issuances_repayments: Option<Decimal>,
    pub tickers: Option<Vec<String>>,
    pub timeframe: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FinancialIncomeStatement {
    #[serde(rename = "basic_earnings_per_share")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub basic_earnings_per_share: Option<Decimal>,
    #[serde(rename = "basic_shares_outstanding")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub basic_shares_outstanding: Option<Decimal>,
    pub cik: Option<String>,
    #[serde(rename = "consolidated_net_income_loss")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub consolidated_net_income_loss: Option<Decimal>,
    #[serde(rename = "cost_of_revenue")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub cost_of_revenue: Option<Decimal>,
    #[serde(rename = "depreciation_depletion_amortization")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub depreciation_depletion_amortization: Option<Decimal>,
    #[serde(rename = "diluted_earnings_per_share")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub diluted_earnings_per_share: Option<Decimal>,
    #[serde(rename = "diluted_shares_outstanding")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub diluted_shares_outstanding: Option<Decimal>,
    #[serde(rename = "discontinued_operations")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub discontinued_operations: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub ebitda: Option<Decimal>,
    #[serde(rename = "equity_in_affiliates")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub equity_in_affiliates: Option<Decimal>,
    #[serde(rename = "extraordinary_items")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub extraordinary_items: Option<Decimal>,
    #[serde(rename = "filing_date")]
    pub filing_date: Option<String>,
    #[serde(rename = "fiscal_quarter")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub fiscal_quarter: Option<Decimal>,
    #[serde(rename = "fiscal_year")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub fiscal_year: Option<Decimal>,
    #[serde(rename = "gross_profit")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub gross_profit: Option<Decimal>,
    #[serde(rename = "income_before_income_taxes")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub income_before_income_taxes: Option<Decimal>,
    #[serde(rename = "income_taxes")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub income_taxes: Option<Decimal>,
    #[serde(rename = "interest_expense")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub interest_expense: Option<Decimal>,
    #[serde(rename = "interest_income")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub interest_income: Option<Decimal>,
    #[serde(rename = "net_income_loss_attributable_common_shareholders")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub net_income_loss_attributable_common_shareholders: Option<Decimal>,
    #[serde(rename = "noncontrolling_interest")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub noncontrolling_interest: Option<Decimal>,
    #[serde(rename = "operating_income")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub operating_income: Option<Decimal>,
    #[serde(rename = "other_income_expense")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub other_income_expense: Option<Decimal>,
    #[serde(rename = "other_operating_expenses")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub other_operating_expenses: Option<Decimal>,
    #[serde(rename = "period_end")]
    pub period_end: Option<String>,
    #[serde(rename = "preferred_stock_dividends_declared")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub preferred_stock_dividends_declared: Option<Decimal>,
    #[serde(rename = "research_development")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub research_development: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub revenue: Option<Decimal>,
    #[serde(rename = "selling_general_administrative")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub selling_general_administrative: Option<Decimal>,
    pub tickers: Option<Vec<String>>,
    pub timeframe: Option<String>,
    #[serde(rename = "total_operating_expenses")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub total_operating_expenses: Option<Decimal>,
    #[serde(rename = "total_other_income_expense")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub total_other_income_expense: Option<Decimal>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FinancialRatio {
    #[serde(rename = "average_volume")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub average_volume: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub cash: Option<Decimal>,
    pub cik: Option<String>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub current: Option<Decimal>,
    pub date: Option<String>,
    #[serde(rename = "debt_to_equity")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub debt_to_equity: Option<Decimal>,
    #[serde(rename = "dividend_yield")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub dividend_yield: Option<Decimal>,
    #[serde(rename = "earnings_per_share")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub earnings_per_share: Option<Decimal>,
    #[serde(rename = "enterprise_value")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub enterprise_value: Option<Decimal>,
    #[serde(rename = "ev_to_ebitda")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub ev_to_ebitda: Option<Decimal>,
    #[serde(rename = "ev_to_sales")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub ev_to_sales: Option<Decimal>,
    #[serde(rename = "free_cash_flow")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub free_cash_flow: Option<Decimal>,
    #[serde(rename = "market_cap")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub market_cap: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub price: Option<Decimal>,
    #[serde(rename = "price_to_book")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub price_to_book: Option<Decimal>,
    #[serde(rename = "price_to_cash_flow")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub price_to_cash_flow: Option<Decimal>,
    #[serde(rename = "price_to_earnings")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub price_to_earnings: Option<Decimal>,
    #[serde(rename = "price_to_free_cash_flow")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub price_to_free_cash_flow: Option<Decimal>,
    #[serde(rename = "price_to_sales")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub price_to_sales: Option<Decimal>,
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub quick: Option<Decimal>,
    #[serde(rename = "return_on_assets")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub return_on_assets: Option<Decimal>,
    #[serde(rename = "return_on_equity")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub return_on_equity: Option<Decimal>,
    pub ticker: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FinancialFloat {
    #[serde(rename = "effective_date")]
    pub effective_date: Option<String>,
    #[serde(rename = "free_float")]
    pub free_float: Option<i64>,
    #[serde(rename = "free_float_percent")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub free_float_percent: Option<Decimal>,
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
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub taxonomy: Option<Decimal>,
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
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub exercise_price: Option<Decimal>,
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
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub shares_owned: Option<Decimal>,
    pub tickers: Option<Vec<String>>,
    #[serde(rename = "underlying_security_shares")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub underlying_security_shares: Option<Decimal>,
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
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub exercise_price: Option<Decimal>,
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
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub shares_owned_following_transaction: Option<Decimal>,
    pub tickers: Option<Vec<String>>,
    #[serde(rename = "transaction_acquired_disposed")]
    pub transaction_acquired_disposed: Option<String>,
    #[serde(rename = "transaction_code")]
    pub transaction_code: Option<String>,
    #[serde(rename = "transaction_date")]
    pub transaction_date: Option<String>,
    #[serde(rename = "transaction_price_per_share")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub transaction_price_per_share: Option<Decimal>,
    #[serde(rename = "transaction_shares")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub transaction_shares: Option<Decimal>,
    #[serde(rename = "transaction_timeliness")]
    pub transaction_timeliness: Option<String>,
    #[serde(rename = "transaction_value")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub transaction_value: Option<Decimal>,
    #[serde(rename = "underlying_security_shares")]
    #[serde(default, deserialize_with = "crate::de::decimal_opt")]
    pub underlying_security_shares: Option<Decimal>,
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
