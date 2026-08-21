use crate::error::Result;
use ::futures::Stream;
use std::future::Future;
use std::pin::Pin;

/// Boxed future returned by every object-safe `get_*` REST trait method.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;

/// Boxed stream returned by every object-safe `list_*` REST trait method.
pub type BoxStream<'a, T> = Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>>;

pub mod aggs;
pub mod benzinga;
pub mod economy;
pub mod etf_global;
pub mod financials;
pub mod futures;
pub mod indicators;
pub mod quotes;
pub mod reference;
pub mod snapshot;
pub mod summaries;
pub mod tmx;
pub mod trades;
pub mod vx;

pub use aggs::{
    AggsApi, GetAggsParams, GetDailyOpenCloseAggParams, GetGroupedDailyAggsParams,
    GetPreviousCloseAggParams, ListAggsParams,
};
pub use benzinga::{
    BenzingaApi, ListBenzingaAnalystInsightsParams, ListBenzingaAnalystsParams,
    ListBenzingaBullsBearsSayParams, ListBenzingaConsensusRatingsParams,
    ListBenzingaEarningsParams, ListBenzingaFirmsParams, ListBenzingaGuidanceParams,
    ListBenzingaNewsParams, ListBenzingaNewsV2Params, ListBenzingaRatingsParams,
};
pub use economy::{
    EconomyApi, ListEuMerchantAggregatesParams, ListEuMerchantHierarchyParams,
    ListInflationExpectationsParams, ListInflationParams, ListLaborMarketIndicatorsParams,
    ListTreasuryYieldsParams,
};
pub use etf_global::{
    EtfGlobalApi, GetEtfGlobalAnalyticsParams, GetEtfGlobalConstituentsParams,
    GetEtfGlobalFundFlowsParams, GetEtfGlobalProfilesParams, GetEtfGlobalTaxonomiesParams,
};
pub use financials::{
    FinancialsApi, ListFinancialsBalanceSheetsParams, ListFinancialsCashFlowStatementsParams,
    ListFinancialsIncomeStatementsParams, ListFinancialsRatiosParams, ListStocksFloatsParams,
};
pub use futures::{
    FuturesApi, GetFuturesSnapshotParams, ListFuturesAggregatesParams, ListFuturesContractsParams,
    ListFuturesExchangesParams, ListFuturesMarketStatusesParams, ListFuturesProductsParams,
    ListFuturesQuotesParams, ListFuturesSchedulesParams, ListFuturesTradesParams,
};
pub use indicators::{GetEmaParams, GetMacdParams, GetRsiParams, GetSmaParams, IndicatorsApi};
pub use quotes::{
    GetLastForexQuoteParams, GetLastQuoteParams, GetRealTimeCurrencyConversionParams,
    ListQuotesParams, QuotesApi,
};
pub use reference::{
    GetExchangesParams, GetMarketHolidaysParams, GetMarketStatusParams, GetOptionsContractParams,
    GetRelatedCompaniesParams, GetTickerDetailsParams, GetTickerEventsParams, GetTickerTypesParams,
    ListConditionsParams, ListDividendsParams, ListOptionsContractsParams, ListShortInterestParams,
    ListShortVolumeParams, ListSplitsParams, ListStocksDividendsParams,
    ListStocksFilings10kSectionsParams, ListStocksFilings13fParams,
    ListStocksFilings8kDisclosuresParams, ListStocksFilings8kTextParams,
    ListStocksFilingsForm3Params, ListStocksFilingsForm4Params, ListStocksFilingsIndexParams,
    ListStocksFilingsRiskFactorsParams, ListStocksSplitsParams,
    ListStocksTaxonomiesDisclosuresParams, ListStocksTaxonomiesRiskFactorsParams,
    ListTickerNewsParams, ListTickersParams, ReferenceApi,
};
pub use snapshot::{
    GetSnapshotAllParams, GetSnapshotCryptoBookParams, GetSnapshotDirectionParams,
    GetSnapshotIndicesParams, GetSnapshotOptionParams, GetSnapshotTickerParams,
    ListSnapshotOptionsChainParams, ListUniversalSnapshotsParams, SnapshotApi,
};
pub use summaries::{GetSummariesParams, SummariesApi};
pub use tmx::{ListTmxCorporateEventsParams, TmxApi};
pub use trades::{GetLastCryptoTradeParams, GetLastTradeParams, ListTradesParams, TradesApi};
pub use vx::{ListIposParams, ListStockFinancialsParams, VxApi};

#[cfg(test)]
mod tests {
    use super::*;

    // Compile-time proof that every REST trait is object-safe: they can all be
    // held behind `Box<dyn ...>` (e.g. for dependency injection in tests).
    #[test]
    fn traits_are_object_safe() {
        fn assert_object_safe<T: ?Sized>() {}
        assert_object_safe::<dyn AggsApi>();
        assert_object_safe::<dyn BenzingaApi>();
        assert_object_safe::<dyn EconomyApi>();
        assert_object_safe::<dyn EtfGlobalApi>();
        assert_object_safe::<dyn FinancialsApi>();
        assert_object_safe::<dyn FuturesApi>();
        assert_object_safe::<dyn IndicatorsApi>();
        assert_object_safe::<dyn QuotesApi>();
        assert_object_safe::<dyn ReferenceApi>();
        assert_object_safe::<dyn SnapshotApi>();
        assert_object_safe::<dyn SummariesApi>();
        assert_object_safe::<dyn TmxApi>();
        assert_object_safe::<dyn TradesApi>();
        assert_object_safe::<dyn VxApi>();
    }
}
