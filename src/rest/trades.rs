use crate::client::{Client, RequestOptions};
use crate::error::Result;
use crate::models::{CryptoTrade, LastTrade, Trade};
use futures::Stream;

/// Trades API.
pub trait TradesApi {
    /// List trades for a ticker (paginated stream).
    fn list_trades(
        &self,
        ticker: &str,
        timestamp: Option<&str>,
        timestamp_lt: Option<&str>,
        timestamp_lte: Option<&str>,
        timestamp_gt: Option<&str>,
        timestamp_gte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Trade>>;

    /// Get the last trade for a ticker.
    async fn get_last_trade(
        &self,
        ticker: &str,
        options: Option<&RequestOptions>,
    ) -> Result<LastTrade>;

    /// Get the last trade tick for a cryptocurrency pair.
    async fn get_last_crypto_trade(
        &self,
        from: &str,
        to: &str,
        options: Option<&RequestOptions>,
    ) -> Result<CryptoTrade>;
}

impl TradesApi for Client {
    fn list_trades(
        &self,
        ticker: &str,
        timestamp: Option<&str>,
        timestamp_lt: Option<&str>,
        timestamp_lte: Option<&str>,
        timestamp_gt: Option<&str>,
        timestamp_gte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        order: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<Trade>> {
        let path = format!("/v3/trades/{}", ticker);
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(t) = timestamp {
            params.push(("timestamp", t.to_string()));
        }
        if let Some(t) = timestamp_lt {
            params.push(("timestamp.lt", t.to_string()));
        }
        if let Some(t) = timestamp_lte {
            params.push(("timestamp.lte", t.to_string()));
        }
        if let Some(t) = timestamp_gt {
            params.push(("timestamp.gt", t.to_string()));
        }
        if let Some(t) = timestamp_gte {
            params.push(("timestamp.gte", t.to_string()));
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
            self.paginate::<Trade>(&path, Some(&params), options)
        } else {
            self.single_page::<Trade>(&path, Some(&params), options)
        }
    }

    async fn get_last_trade(
        &self,
        ticker: &str,
        options: Option<&RequestOptions>,
    ) -> Result<LastTrade> {
        let path = format!("/v2/last/trade/{}", ticker);
        #[derive(serde::Deserialize)]
        struct Resp {
            results: LastTrade,
        }
        let resp: Resp = self.get(&path, None, options).await?;
        Ok(resp.results)
    }

    async fn get_last_crypto_trade(
        &self,
        from: &str,
        to: &str,
        options: Option<&RequestOptions>,
    ) -> Result<CryptoTrade> {
        let path = format!("/v1/last/crypto/{}/{}", from, to);
        #[derive(serde::Deserialize)]
        struct Resp {
            last: CryptoTrade,
        }
        let resp: Resp = self.get(&path, None, options).await?;
        Ok(resp.last)
    }
}
