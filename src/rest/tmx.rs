use super::{encode_query, BoxStream};
use crate::client::{Client, RequestOptions};
use crate::models::TmxCorporateEvent;

/// TMX API.
pub trait TmxApi {
    /// List TMX corporate events (paginated stream). Endpoint: GET /tmx/v1/corporate-events.
    #[allow(clippy::too_many_arguments)]
    fn list_tmx_corporate_events<'a>(
        &'a self,
        date: Option<&'a str>,
        date_any_of: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        r#type: Option<&'a str>,
        type_any_of: Option<&'a str>,
        type_gt: Option<&'a str>,
        type_gte: Option<&'a str>,
        type_lt: Option<&'a str>,
        type_lte: Option<&'a str>,
        status: Option<&'a str>,
        status_any_of: Option<&'a str>,
        status_gt: Option<&'a str>,
        status_gte: Option<&'a str>,
        status_lt: Option<&'a str>,
        status_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        isin: Option<&'a str>,
        isin_any_of: Option<&'a str>,
        isin_gt: Option<&'a str>,
        isin_gte: Option<&'a str>,
        isin_lt: Option<&'a str>,
        isin_lte: Option<&'a str>,
        trading_venue: Option<&'a str>,
        trading_venue_any_of: Option<&'a str>,
        trading_venue_gt: Option<&'a str>,
        trading_venue_gte: Option<&'a str>,
        trading_venue_lt: Option<&'a str>,
        trading_venue_lte: Option<&'a str>,
        tmx_company_id: Option<i64>,
        tmx_company_id_any_of: Option<&'a str>,
        tmx_company_id_gt: Option<i64>,
        tmx_company_id_gte: Option<i64>,
        tmx_company_id_lt: Option<i64>,
        tmx_company_id_lte: Option<i64>,
        tmx_record_id: Option<&'a str>,
        tmx_record_id_any_of: Option<&'a str>,
        tmx_record_id_gt: Option<&'a str>,
        tmx_record_id_gte: Option<&'a str>,
        tmx_record_id_lt: Option<&'a str>,
        tmx_record_id_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, TmxCorporateEvent>;

    /// Same as [`Self::list_tmx_corporate_events`], but takes the optional arguments as a
    /// chainable [`ListTmxCorporateEventsParams`] struct.
    fn list_tmx_corporate_events_with_params<'a>(
        &'a self,
        params: ListTmxCorporateEventsParams,
    ) -> BoxStream<'a, TmxCorporateEvent>;
}

impl TmxApi for Client {
    #[allow(clippy::too_many_arguments)]
    fn list_tmx_corporate_events<'a>(
        &'a self,
        date: Option<&'a str>,
        date_any_of: Option<&'a str>,
        date_gt: Option<&'a str>,
        date_gte: Option<&'a str>,
        date_lt: Option<&'a str>,
        date_lte: Option<&'a str>,
        r#type: Option<&'a str>,
        type_any_of: Option<&'a str>,
        type_gt: Option<&'a str>,
        type_gte: Option<&'a str>,
        type_lt: Option<&'a str>,
        type_lte: Option<&'a str>,
        status: Option<&'a str>,
        status_any_of: Option<&'a str>,
        status_gt: Option<&'a str>,
        status_gte: Option<&'a str>,
        status_lt: Option<&'a str>,
        status_lte: Option<&'a str>,
        ticker: Option<&'a str>,
        ticker_any_of: Option<&'a str>,
        ticker_gt: Option<&'a str>,
        ticker_gte: Option<&'a str>,
        ticker_lt: Option<&'a str>,
        ticker_lte: Option<&'a str>,
        isin: Option<&'a str>,
        isin_any_of: Option<&'a str>,
        isin_gt: Option<&'a str>,
        isin_gte: Option<&'a str>,
        isin_lt: Option<&'a str>,
        isin_lte: Option<&'a str>,
        trading_venue: Option<&'a str>,
        trading_venue_any_of: Option<&'a str>,
        trading_venue_gt: Option<&'a str>,
        trading_venue_gte: Option<&'a str>,
        trading_venue_lt: Option<&'a str>,
        trading_venue_lte: Option<&'a str>,
        tmx_company_id: Option<i64>,
        tmx_company_id_any_of: Option<&'a str>,
        tmx_company_id_gt: Option<i64>,
        tmx_company_id_gte: Option<i64>,
        tmx_company_id_lt: Option<i64>,
        tmx_company_id_lte: Option<i64>,
        tmx_record_id: Option<&'a str>,
        tmx_record_id_any_of: Option<&'a str>,
        tmx_record_id_gt: Option<&'a str>,
        tmx_record_id_gte: Option<&'a str>,
        tmx_record_id_lt: Option<&'a str>,
        tmx_record_id_lte: Option<&'a str>,
        limit: Option<i64>,
        sort: Option<&'a str>,
        options: Option<&'a RequestOptions>,
    ) -> BoxStream<'a, TmxCorporateEvent> {
        self.list_tmx_corporate_events_with_params(ListTmxCorporateEventsParams {
            date: date.map(String::from),
            date_any_of: date_any_of.map(String::from),
            date_gt: date_gt.map(String::from),
            date_gte: date_gte.map(String::from),
            date_lt: date_lt.map(String::from),
            date_lte: date_lte.map(String::from),
            r#type: r#type.map(String::from),
            type_any_of: type_any_of.map(String::from),
            type_gt: type_gt.map(String::from),
            type_gte: type_gte.map(String::from),
            type_lt: type_lt.map(String::from),
            type_lte: type_lte.map(String::from),
            status: status.map(String::from),
            status_any_of: status_any_of.map(String::from),
            status_gt: status_gt.map(String::from),
            status_gte: status_gte.map(String::from),
            status_lt: status_lt.map(String::from),
            status_lte: status_lte.map(String::from),
            ticker: ticker.map(String::from),
            ticker_any_of: ticker_any_of.map(String::from),
            ticker_gt: ticker_gt.map(String::from),
            ticker_gte: ticker_gte.map(String::from),
            ticker_lt: ticker_lt.map(String::from),
            ticker_lte: ticker_lte.map(String::from),
            isin: isin.map(String::from),
            isin_any_of: isin_any_of.map(String::from),
            isin_gt: isin_gt.map(String::from),
            isin_gte: isin_gte.map(String::from),
            isin_lt: isin_lt.map(String::from),
            isin_lte: isin_lte.map(String::from),
            trading_venue: trading_venue.map(String::from),
            trading_venue_any_of: trading_venue_any_of.map(String::from),
            trading_venue_gt: trading_venue_gt.map(String::from),
            trading_venue_gte: trading_venue_gte.map(String::from),
            trading_venue_lt: trading_venue_lt.map(String::from),
            trading_venue_lte: trading_venue_lte.map(String::from),
            tmx_company_id,
            tmx_company_id_any_of: tmx_company_id_any_of.map(String::from),
            tmx_company_id_gt,
            tmx_company_id_gte,
            tmx_company_id_lt,
            tmx_company_id_lte,
            tmx_record_id: tmx_record_id.map(String::from),
            tmx_record_id_any_of: tmx_record_id_any_of.map(String::from),
            tmx_record_id_gt: tmx_record_id_gt.map(String::from),
            tmx_record_id_gte: tmx_record_id_gte.map(String::from),
            tmx_record_id_lt: tmx_record_id_lt.map(String::from),
            tmx_record_id_lte: tmx_record_id_lte.map(String::from),
            limit,
            sort: sort.map(String::from),
            options: options.cloned(),
        })
    }

    fn list_tmx_corporate_events_with_params<'a>(
        &'a self,
        params: ListTmxCorporateEventsParams,
    ) -> BoxStream<'a, TmxCorporateEvent> {
        Box::pin({
            let path = "/tmx/v1/corporate-events".to_string();
            let query = encode_query(&params);
            self.list::<TmxCorporateEvent>(&path, &query, params.options.as_ref())
        })
    }
}

// --- Params structs (additive builder API) ---

/// Optional arguments for [`TmxApi::list_tmx_corporate_events`].
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ListTmxCorporateEventsParams {
    /// The `date` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// The `date_any_of` argument.
    #[serde(rename = "date.any_of", skip_serializing_if = "Option::is_none")]
    pub date_any_of: Option<String>,
    /// The `date_gt` argument.
    #[serde(rename = "date.gt", skip_serializing_if = "Option::is_none")]
    pub date_gt: Option<String>,
    /// The `date_gte` argument.
    #[serde(rename = "date.gte", skip_serializing_if = "Option::is_none")]
    pub date_gte: Option<String>,
    /// The `date_lt` argument.
    #[serde(rename = "date.lt", skip_serializing_if = "Option::is_none")]
    pub date_lt: Option<String>,
    /// The `date_lte` argument.
    #[serde(rename = "date.lte", skip_serializing_if = "Option::is_none")]
    pub date_lte: Option<String>,
    /// The `type` argument.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The `type_any_of` argument.
    #[serde(rename = "type.any_of", skip_serializing_if = "Option::is_none")]
    pub type_any_of: Option<String>,
    /// The `type_gt` argument.
    #[serde(rename = "type.gt", skip_serializing_if = "Option::is_none")]
    pub type_gt: Option<String>,
    /// The `type_gte` argument.
    #[serde(rename = "type.gte", skip_serializing_if = "Option::is_none")]
    pub type_gte: Option<String>,
    /// The `type_lt` argument.
    #[serde(rename = "type.lt", skip_serializing_if = "Option::is_none")]
    pub type_lt: Option<String>,
    /// The `type_lte` argument.
    #[serde(rename = "type.lte", skip_serializing_if = "Option::is_none")]
    pub type_lte: Option<String>,
    /// The `status` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The `status_any_of` argument.
    #[serde(rename = "status.any_of", skip_serializing_if = "Option::is_none")]
    pub status_any_of: Option<String>,
    /// The `status_gt` argument.
    #[serde(rename = "status.gt", skip_serializing_if = "Option::is_none")]
    pub status_gt: Option<String>,
    /// The `status_gte` argument.
    #[serde(rename = "status.gte", skip_serializing_if = "Option::is_none")]
    pub status_gte: Option<String>,
    /// The `status_lt` argument.
    #[serde(rename = "status.lt", skip_serializing_if = "Option::is_none")]
    pub status_lt: Option<String>,
    /// The `status_lte` argument.
    #[serde(rename = "status.lte", skip_serializing_if = "Option::is_none")]
    pub status_lte: Option<String>,
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
    /// The `isin` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isin: Option<String>,
    /// The `isin_any_of` argument.
    #[serde(rename = "isin.any_of", skip_serializing_if = "Option::is_none")]
    pub isin_any_of: Option<String>,
    /// The `isin_gt` argument.
    #[serde(rename = "isin.gt", skip_serializing_if = "Option::is_none")]
    pub isin_gt: Option<String>,
    /// The `isin_gte` argument.
    #[serde(rename = "isin.gte", skip_serializing_if = "Option::is_none")]
    pub isin_gte: Option<String>,
    /// The `isin_lt` argument.
    #[serde(rename = "isin.lt", skip_serializing_if = "Option::is_none")]
    pub isin_lt: Option<String>,
    /// The `isin_lte` argument.
    #[serde(rename = "isin.lte", skip_serializing_if = "Option::is_none")]
    pub isin_lte: Option<String>,
    /// The `trading_venue` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_venue: Option<String>,
    /// The `trading_venue_any_of` argument.
    #[serde(
        rename = "trading_venue.any_of",
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_venue_any_of: Option<String>,
    /// The `trading_venue_gt` argument.
    #[serde(rename = "trading_venue.gt", skip_serializing_if = "Option::is_none")]
    pub trading_venue_gt: Option<String>,
    /// The `trading_venue_gte` argument.
    #[serde(
        rename = "trading_venue.gte",
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_venue_gte: Option<String>,
    /// The `trading_venue_lt` argument.
    #[serde(rename = "trading_venue.lt", skip_serializing_if = "Option::is_none")]
    pub trading_venue_lt: Option<String>,
    /// The `trading_venue_lte` argument.
    #[serde(
        rename = "trading_venue.lte",
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_venue_lte: Option<String>,
    /// The `tmx_company_id` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmx_company_id: Option<i64>,
    /// The `tmx_company_id_any_of` argument.
    #[serde(
        rename = "tmx_company_id.any_of",
        skip_serializing_if = "Option::is_none"
    )]
    pub tmx_company_id_any_of: Option<String>,
    /// The `tmx_company_id_gt` argument.
    #[serde(
        rename = "tmx_company_id.gt",
        skip_serializing_if = "Option::is_none"
    )]
    pub tmx_company_id_gt: Option<i64>,
    /// The `tmx_company_id_gte` argument.
    #[serde(
        rename = "tmx_company_id.gte",
        skip_serializing_if = "Option::is_none"
    )]
    pub tmx_company_id_gte: Option<i64>,
    /// The `tmx_company_id_lt` argument.
    #[serde(
        rename = "tmx_company_id.lt",
        skip_serializing_if = "Option::is_none"
    )]
    pub tmx_company_id_lt: Option<i64>,
    /// The `tmx_company_id_lte` argument.
    #[serde(
        rename = "tmx_company_id.lte",
        skip_serializing_if = "Option::is_none"
    )]
    pub tmx_company_id_lte: Option<i64>,
    /// The `tmx_record_id` argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmx_record_id: Option<String>,
    /// The `tmx_record_id_any_of` argument.
    #[serde(
        rename = "tmx_record_id.any_of",
        skip_serializing_if = "Option::is_none"
    )]
    pub tmx_record_id_any_of: Option<String>,
    /// The `tmx_record_id_gt` argument.
    #[serde(rename = "tmx_record_id.gt", skip_serializing_if = "Option::is_none")]
    pub tmx_record_id_gt: Option<String>,
    /// The `tmx_record_id_gte` argument.
    #[serde(
        rename = "tmx_record_id.gte",
        skip_serializing_if = "Option::is_none"
    )]
    pub tmx_record_id_gte: Option<String>,
    /// The `tmx_record_id_lt` argument.
    #[serde(rename = "tmx_record_id.lt", skip_serializing_if = "Option::is_none")]
    pub tmx_record_id_lt: Option<String>,
    /// The `tmx_record_id_lte` argument.
    #[serde(
        rename = "tmx_record_id.lte",
        skip_serializing_if = "Option::is_none"
    )]
    pub tmx_record_id_lte: Option<String>,
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

impl ListTmxCorporateEventsParams {
    /// Create params with all filters unset (server defaults apply).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `date` argument.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }

    /// Set the `date_any_of` argument.
    pub fn date_any_of(mut self, date_any_of: impl Into<String>) -> Self {
        self.date_any_of = Some(date_any_of.into());
        self
    }

    /// Set the `date_gt` argument.
    pub fn date_gt(mut self, date_gt: impl Into<String>) -> Self {
        self.date_gt = Some(date_gt.into());
        self
    }

    /// Set the `date_gte` argument.
    pub fn date_gte(mut self, date_gte: impl Into<String>) -> Self {
        self.date_gte = Some(date_gte.into());
        self
    }

    /// Set the `date_lt` argument.
    pub fn date_lt(mut self, date_lt: impl Into<String>) -> Self {
        self.date_lt = Some(date_lt.into());
        self
    }

    /// Set the `date_lte` argument.
    pub fn date_lte(mut self, date_lte: impl Into<String>) -> Self {
        self.date_lte = Some(date_lte.into());
        self
    }

    /// Set the `type` argument.
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Set the `type_any_of` argument.
    pub fn type_any_of(mut self, type_any_of: impl Into<String>) -> Self {
        self.type_any_of = Some(type_any_of.into());
        self
    }

    /// Set the `type_gt` argument.
    pub fn type_gt(mut self, type_gt: impl Into<String>) -> Self {
        self.type_gt = Some(type_gt.into());
        self
    }

    /// Set the `type_gte` argument.
    pub fn type_gte(mut self, type_gte: impl Into<String>) -> Self {
        self.type_gte = Some(type_gte.into());
        self
    }

    /// Set the `type_lt` argument.
    pub fn type_lt(mut self, type_lt: impl Into<String>) -> Self {
        self.type_lt = Some(type_lt.into());
        self
    }

    /// Set the `type_lte` argument.
    pub fn type_lte(mut self, type_lte: impl Into<String>) -> Self {
        self.type_lte = Some(type_lte.into());
        self
    }

    /// Set the `status` argument.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Set the `status_any_of` argument.
    pub fn status_any_of(mut self, status_any_of: impl Into<String>) -> Self {
        self.status_any_of = Some(status_any_of.into());
        self
    }

    /// Set the `status_gt` argument.
    pub fn status_gt(mut self, status_gt: impl Into<String>) -> Self {
        self.status_gt = Some(status_gt.into());
        self
    }

    /// Set the `status_gte` argument.
    pub fn status_gte(mut self, status_gte: impl Into<String>) -> Self {
        self.status_gte = Some(status_gte.into());
        self
    }

    /// Set the `status_lt` argument.
    pub fn status_lt(mut self, status_lt: impl Into<String>) -> Self {
        self.status_lt = Some(status_lt.into());
        self
    }

    /// Set the `status_lte` argument.
    pub fn status_lte(mut self, status_lte: impl Into<String>) -> Self {
        self.status_lte = Some(status_lte.into());
        self
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

    /// Set the `isin` argument.
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }

    /// Set the `isin_any_of` argument.
    pub fn isin_any_of(mut self, isin_any_of: impl Into<String>) -> Self {
        self.isin_any_of = Some(isin_any_of.into());
        self
    }

    /// Set the `isin_gt` argument.
    pub fn isin_gt(mut self, isin_gt: impl Into<String>) -> Self {
        self.isin_gt = Some(isin_gt.into());
        self
    }

    /// Set the `isin_gte` argument.
    pub fn isin_gte(mut self, isin_gte: impl Into<String>) -> Self {
        self.isin_gte = Some(isin_gte.into());
        self
    }

    /// Set the `isin_lt` argument.
    pub fn isin_lt(mut self, isin_lt: impl Into<String>) -> Self {
        self.isin_lt = Some(isin_lt.into());
        self
    }

    /// Set the `isin_lte` argument.
    pub fn isin_lte(mut self, isin_lte: impl Into<String>) -> Self {
        self.isin_lte = Some(isin_lte.into());
        self
    }

    /// Set the `trading_venue` argument.
    pub fn trading_venue(mut self, trading_venue: impl Into<String>) -> Self {
        self.trading_venue = Some(trading_venue.into());
        self
    }

    /// Set the `trading_venue_any_of` argument.
    pub fn trading_venue_any_of(mut self, trading_venue_any_of: impl Into<String>) -> Self {
        self.trading_venue_any_of = Some(trading_venue_any_of.into());
        self
    }

    /// Set the `trading_venue_gt` argument.
    pub fn trading_venue_gt(mut self, trading_venue_gt: impl Into<String>) -> Self {
        self.trading_venue_gt = Some(trading_venue_gt.into());
        self
    }

    /// Set the `trading_venue_gte` argument.
    pub fn trading_venue_gte(mut self, trading_venue_gte: impl Into<String>) -> Self {
        self.trading_venue_gte = Some(trading_venue_gte.into());
        self
    }

    /// Set the `trading_venue_lt` argument.
    pub fn trading_venue_lt(mut self, trading_venue_lt: impl Into<String>) -> Self {
        self.trading_venue_lt = Some(trading_venue_lt.into());
        self
    }

    /// Set the `trading_venue_lte` argument.
    pub fn trading_venue_lte(mut self, trading_venue_lte: impl Into<String>) -> Self {
        self.trading_venue_lte = Some(trading_venue_lte.into());
        self
    }

    /// Set the `tmx_company_id` argument.
    pub fn tmx_company_id(mut self, tmx_company_id: i64) -> Self {
        self.tmx_company_id = Some(tmx_company_id);
        self
    }

    /// Set the `tmx_company_id_any_of` argument.
    pub fn tmx_company_id_any_of(mut self, tmx_company_id_any_of: impl Into<String>) -> Self {
        self.tmx_company_id_any_of = Some(tmx_company_id_any_of.into());
        self
    }

    /// Set the `tmx_company_id_gt` argument.
    pub fn tmx_company_id_gt(mut self, tmx_company_id_gt: i64) -> Self {
        self.tmx_company_id_gt = Some(tmx_company_id_gt);
        self
    }

    /// Set the `tmx_company_id_gte` argument.
    pub fn tmx_company_id_gte(mut self, tmx_company_id_gte: i64) -> Self {
        self.tmx_company_id_gte = Some(tmx_company_id_gte);
        self
    }

    /// Set the `tmx_company_id_lt` argument.
    pub fn tmx_company_id_lt(mut self, tmx_company_id_lt: i64) -> Self {
        self.tmx_company_id_lt = Some(tmx_company_id_lt);
        self
    }

    /// Set the `tmx_company_id_lte` argument.
    pub fn tmx_company_id_lte(mut self, tmx_company_id_lte: i64) -> Self {
        self.tmx_company_id_lte = Some(tmx_company_id_lte);
        self
    }

    /// Set the `tmx_record_id` argument.
    pub fn tmx_record_id(mut self, tmx_record_id: impl Into<String>) -> Self {
        self.tmx_record_id = Some(tmx_record_id.into());
        self
    }

    /// Set the `tmx_record_id_any_of` argument.
    pub fn tmx_record_id_any_of(mut self, tmx_record_id_any_of: impl Into<String>) -> Self {
        self.tmx_record_id_any_of = Some(tmx_record_id_any_of.into());
        self
    }

    /// Set the `tmx_record_id_gt` argument.
    pub fn tmx_record_id_gt(mut self, tmx_record_id_gt: impl Into<String>) -> Self {
        self.tmx_record_id_gt = Some(tmx_record_id_gt.into());
        self
    }

    /// Set the `tmx_record_id_gte` argument.
    pub fn tmx_record_id_gte(mut self, tmx_record_id_gte: impl Into<String>) -> Self {
        self.tmx_record_id_gte = Some(tmx_record_id_gte.into());
        self
    }

    /// Set the `tmx_record_id_lt` argument.
    pub fn tmx_record_id_lt(mut self, tmx_record_id_lt: impl Into<String>) -> Self {
        self.tmx_record_id_lt = Some(tmx_record_id_lt.into());
        self
    }

    /// Set the `tmx_record_id_lte` argument.
    pub fn tmx_record_id_lte(mut self, tmx_record_id_lte: impl Into<String>) -> Self {
        self.tmx_record_id_lte = Some(tmx_record_id_lte.into());
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
