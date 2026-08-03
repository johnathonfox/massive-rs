use crate::client::{Client, RequestOptions};
use crate::error::Result;
use crate::models::{
    IndicesSnapshot, OptionContractSnapshot, SnapshotTickerFullBook, TickerSnapshot,
    UniversalSnapshot,
};
use futures::Stream;

/// Snapshot API.
pub trait SnapshotApi {
    /// Get snapshots for assets of all types (paginated stream).
    fn list_universal_snapshots(
        &self,
        r#type: Option<&str>,
        ticker_any_of: Option<&str>,
        order: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<UniversalSnapshot>>;

    /// Get the most up-to-date market data for all traded symbols in a market.
    async fn get_snapshot_all(
        &self,
        market_type: &str,
        tickers: Option<&str>,
        include_otc: Option<bool>,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<TickerSnapshot>>;

    /// Get the current top 20 gainers or losers of the day in a market.
    async fn get_snapshot_direction(
        &self,
        market_type: &str,
        direction: &str,
        include_otc: Option<bool>,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<TickerSnapshot>>;

    /// Get the most up-to-date market data for a single ticker.
    async fn get_snapshot_ticker(
        &self,
        market_type: &str,
        ticker: &str,
        options: Option<&RequestOptions>,
    ) -> Result<TickerSnapshot>;

    /// Get the snapshot of an option contract for an underlying asset.
    async fn get_snapshot_option(
        &self,
        underlying_asset: &str,
        option_contract: &str,
        options: Option<&RequestOptions>,
    ) -> Result<OptionContractSnapshot>;

    /// Get the snapshot of all options contracts for an underlying ticker (paginated stream).
    fn list_snapshot_options_chain(
        &self,
        underlying_asset: &str,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<OptionContractSnapshot>>;

    /// Get the current level 2 book of a single crypto ticker (all exchanges combined).
    async fn get_snapshot_crypto_book(
        &self,
        ticker: &str,
        options: Option<&RequestOptions>,
    ) -> Result<SnapshotTickerFullBook>;

    /// Get snapshots for indices.
    async fn get_snapshot_indices(
        &self,
        ticker_any_of: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<IndicesSnapshot>>;
}

fn get_locale(market_type: &str) -> &'static str {
    if market_type == "stocks" {
        "us"
    } else {
        "global"
    }
}

impl SnapshotApi for Client {
    fn list_universal_snapshots(
        &self,
        r#type: Option<&str>,
        ticker_any_of: Option<&str>,
        order: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<UniversalSnapshot>> {
        let path = "/v3/snapshot".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(t) = r#type {
            params.push(("type", t.to_string()));
        }
        if let Some(t) = ticker_any_of {
            params.push(("ticker_any_of", t.to_string()));
        }
        if let Some(o) = order {
            params.push(("order", o.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        if let Some(s) = sort {
            params.push(("sort", s.to_string()));
        }
        if let Some(t) = ticker_lt {
            params.push(("ticker.lt", t.to_string()));
        }
        if let Some(t) = ticker_lte {
            params.push(("ticker.lte", t.to_string()));
        }
        if let Some(t) = ticker_gt {
            params.push(("ticker.gt", t.to_string()));
        }
        if let Some(t) = ticker_gte {
            params.push(("ticker.gte", t.to_string()));
        }
        if self.pagination {
            self.paginate::<UniversalSnapshot>(&path, Some(&params), options)
        } else {
            self.single_page::<UniversalSnapshot>(&path, Some(&params), options)
        }
    }

    async fn get_snapshot_all(
        &self,
        market_type: &str,
        tickers: Option<&str>,
        include_otc: Option<bool>,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<TickerSnapshot>> {
        let locale = get_locale(market_type);
        let path = format!(
            "/v2/snapshot/locale/{}/markets/{}/tickers",
            locale, market_type
        );
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(t) = tickers {
            params.push(("tickers", t.to_string()));
        }
        if let Some(i) = include_otc {
            params.push(("include_otc", i.to_string()));
        }
        #[derive(serde::Deserialize)]
        struct Resp {
            tickers: Option<Vec<TickerSnapshot>>,
        }
        let resp: Resp = self.get(&path, Some(&params), options).await?;
        Ok(resp.tickers.unwrap_or_default())
    }

    async fn get_snapshot_direction(
        &self,
        market_type: &str,
        direction: &str,
        include_otc: Option<bool>,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<TickerSnapshot>> {
        let locale = get_locale(market_type);
        let path = format!(
            "/v2/snapshot/locale/{}/markets/{}/{}",
            locale, market_type, direction
        );
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(i) = include_otc {
            params.push(("include_otc", i.to_string()));
        }
        #[derive(serde::Deserialize)]
        struct Resp {
            tickers: Option<Vec<TickerSnapshot>>,
        }
        let resp: Resp = self.get(&path, Some(&params), options).await?;
        Ok(resp.tickers.unwrap_or_default())
    }

    async fn get_snapshot_ticker(
        &self,
        market_type: &str,
        ticker: &str,
        options: Option<&RequestOptions>,
    ) -> Result<TickerSnapshot> {
        let locale = get_locale(market_type);
        let path = format!(
            "/v2/snapshot/locale/{}/markets/{}/tickers/{}",
            locale, market_type, ticker
        );
        #[derive(serde::Deserialize)]
        struct Resp {
            ticker: TickerSnapshot,
        }
        let resp: Resp = self.get(&path, None, options).await?;
        Ok(resp.ticker)
    }

    async fn get_snapshot_option(
        &self,
        underlying_asset: &str,
        option_contract: &str,
        options: Option<&RequestOptions>,
    ) -> Result<OptionContractSnapshot> {
        let path = format!(
            "/v3/snapshot/options/{}/{}",
            underlying_asset, option_contract
        );
        #[derive(serde::Deserialize)]
        struct Resp {
            results: OptionContractSnapshot,
        }
        let resp: Resp = self.get(&path, None, options).await?;
        Ok(resp.results)
    }

    fn list_snapshot_options_chain(
        &self,
        underlying_asset: &str,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<OptionContractSnapshot>> {
        let path = format!("/v3/snapshot/options/{}", underlying_asset);
        if self.pagination {
            self.paginate::<OptionContractSnapshot>(&path, None, options)
        } else {
            self.single_page::<OptionContractSnapshot>(&path, None, options)
        }
    }

    async fn get_snapshot_crypto_book(
        &self,
        ticker: &str,
        options: Option<&RequestOptions>,
    ) -> Result<SnapshotTickerFullBook> {
        let path = format!(
            "/v2/snapshot/locale/global/markets/crypto/tickers/{}/book",
            ticker
        );
        #[derive(serde::Deserialize)]
        struct Resp {
            data: SnapshotTickerFullBook,
        }
        let resp: Resp = self.get(&path, None, options).await?;
        Ok(resp.data)
    }

    async fn get_snapshot_indices(
        &self,
        ticker_any_of: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<IndicesSnapshot>> {
        let path = "/v3/snapshot/indices".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(t) = ticker_any_of {
            params.push(("ticker_any_of", t.to_string()));
        }
        #[derive(serde::Deserialize)]
        struct Resp {
            results: Option<Vec<IndicesSnapshot>>,
        }
        let resp: Resp = self.get(&path, Some(&params), options).await?;
        Ok(resp.results.unwrap_or_default())
    }
}
