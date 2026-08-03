use crate::client::{Client, RequestOptions};
use crate::error::Result;
use crate::models::TmxCorporateEvent;
use futures::Stream;

/// Push a query param when the optional value is present, using the literal
/// (possibly dotted) wire key.
fn push_param<T: ToString>(
    params: &mut Vec<(&'static str, String)>,
    key: &'static str,
    value: Option<T>,
) {
    if let Some(v) = value {
        params.push((key, v.to_string()));
    }
}

/// TMX API.
pub trait TmxApi {
    /// List TMX corporate events (paginated stream). Endpoint: GET /tmx/v1/corporate-events.
    #[allow(clippy::too_many_arguments)]
    fn list_tmx_corporate_events(
        &self,
        date: Option<&str>,
        date_any_of: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        r#type: Option<&str>,
        type_any_of: Option<&str>,
        type_gt: Option<&str>,
        type_gte: Option<&str>,
        type_lt: Option<&str>,
        type_lte: Option<&str>,
        status: Option<&str>,
        status_any_of: Option<&str>,
        status_gt: Option<&str>,
        status_gte: Option<&str>,
        status_lt: Option<&str>,
        status_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        isin: Option<&str>,
        isin_any_of: Option<&str>,
        isin_gt: Option<&str>,
        isin_gte: Option<&str>,
        isin_lt: Option<&str>,
        isin_lte: Option<&str>,
        trading_venue: Option<&str>,
        trading_venue_any_of: Option<&str>,
        trading_venue_gt: Option<&str>,
        trading_venue_gte: Option<&str>,
        trading_venue_lt: Option<&str>,
        trading_venue_lte: Option<&str>,
        tmx_company_id: Option<i64>,
        tmx_company_id_any_of: Option<&str>,
        tmx_company_id_gt: Option<i64>,
        tmx_company_id_gte: Option<i64>,
        tmx_company_id_lt: Option<i64>,
        tmx_company_id_lte: Option<i64>,
        tmx_record_id: Option<&str>,
        tmx_record_id_any_of: Option<&str>,
        tmx_record_id_gt: Option<&str>,
        tmx_record_id_gte: Option<&str>,
        tmx_record_id_lt: Option<&str>,
        tmx_record_id_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<TmxCorporateEvent>>;
}

impl TmxApi for Client {
    #[allow(clippy::too_many_arguments)]
    fn list_tmx_corporate_events(
        &self,
        date: Option<&str>,
        date_any_of: Option<&str>,
        date_gt: Option<&str>,
        date_gte: Option<&str>,
        date_lt: Option<&str>,
        date_lte: Option<&str>,
        r#type: Option<&str>,
        type_any_of: Option<&str>,
        type_gt: Option<&str>,
        type_gte: Option<&str>,
        type_lt: Option<&str>,
        type_lte: Option<&str>,
        status: Option<&str>,
        status_any_of: Option<&str>,
        status_gt: Option<&str>,
        status_gte: Option<&str>,
        status_lt: Option<&str>,
        status_lte: Option<&str>,
        ticker: Option<&str>,
        ticker_any_of: Option<&str>,
        ticker_gt: Option<&str>,
        ticker_gte: Option<&str>,
        ticker_lt: Option<&str>,
        ticker_lte: Option<&str>,
        isin: Option<&str>,
        isin_any_of: Option<&str>,
        isin_gt: Option<&str>,
        isin_gte: Option<&str>,
        isin_lt: Option<&str>,
        isin_lte: Option<&str>,
        trading_venue: Option<&str>,
        trading_venue_any_of: Option<&str>,
        trading_venue_gt: Option<&str>,
        trading_venue_gte: Option<&str>,
        trading_venue_lt: Option<&str>,
        trading_venue_lte: Option<&str>,
        tmx_company_id: Option<i64>,
        tmx_company_id_any_of: Option<&str>,
        tmx_company_id_gt: Option<i64>,
        tmx_company_id_gte: Option<i64>,
        tmx_company_id_lt: Option<i64>,
        tmx_company_id_lte: Option<i64>,
        tmx_record_id: Option<&str>,
        tmx_record_id_any_of: Option<&str>,
        tmx_record_id_gt: Option<&str>,
        tmx_record_id_gte: Option<&str>,
        tmx_record_id_lt: Option<&str>,
        tmx_record_id_lte: Option<&str>,
        limit: Option<i64>,
        sort: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> impl Stream<Item = Result<TmxCorporateEvent>> {
        let path = "/tmx/v1/corporate-events".to_string();
        let mut params: Vec<(&str, String)> = Vec::new();
        push_param(&mut params, "date", date);
        push_param(&mut params, "date.any_of", date_any_of);
        push_param(&mut params, "date.gt", date_gt);
        push_param(&mut params, "date.gte", date_gte);
        push_param(&mut params, "date.lt", date_lt);
        push_param(&mut params, "date.lte", date_lte);
        push_param(&mut params, "type", r#type);
        push_param(&mut params, "type.any_of", type_any_of);
        push_param(&mut params, "type.gt", type_gt);
        push_param(&mut params, "type.gte", type_gte);
        push_param(&mut params, "type.lt", type_lt);
        push_param(&mut params, "type.lte", type_lte);
        push_param(&mut params, "status", status);
        push_param(&mut params, "status.any_of", status_any_of);
        push_param(&mut params, "status.gt", status_gt);
        push_param(&mut params, "status.gte", status_gte);
        push_param(&mut params, "status.lt", status_lt);
        push_param(&mut params, "status.lte", status_lte);
        push_param(&mut params, "ticker", ticker);
        push_param(&mut params, "ticker.any_of", ticker_any_of);
        push_param(&mut params, "ticker.gt", ticker_gt);
        push_param(&mut params, "ticker.gte", ticker_gte);
        push_param(&mut params, "ticker.lt", ticker_lt);
        push_param(&mut params, "ticker.lte", ticker_lte);
        push_param(&mut params, "isin", isin);
        push_param(&mut params, "isin.any_of", isin_any_of);
        push_param(&mut params, "isin.gt", isin_gt);
        push_param(&mut params, "isin.gte", isin_gte);
        push_param(&mut params, "isin.lt", isin_lt);
        push_param(&mut params, "isin.lte", isin_lte);
        push_param(&mut params, "trading_venue", trading_venue);
        push_param(&mut params, "trading_venue.any_of", trading_venue_any_of);
        push_param(&mut params, "trading_venue.gt", trading_venue_gt);
        push_param(&mut params, "trading_venue.gte", trading_venue_gte);
        push_param(&mut params, "trading_venue.lt", trading_venue_lt);
        push_param(&mut params, "trading_venue.lte", trading_venue_lte);
        push_param(&mut params, "tmx_company_id", tmx_company_id);
        push_param(&mut params, "tmx_company_id.any_of", tmx_company_id_any_of);
        push_param(&mut params, "tmx_company_id.gt", tmx_company_id_gt);
        push_param(&mut params, "tmx_company_id.gte", tmx_company_id_gte);
        push_param(&mut params, "tmx_company_id.lt", tmx_company_id_lt);
        push_param(&mut params, "tmx_company_id.lte", tmx_company_id_lte);
        push_param(&mut params, "tmx_record_id", tmx_record_id);
        push_param(&mut params, "tmx_record_id.any_of", tmx_record_id_any_of);
        push_param(&mut params, "tmx_record_id.gt", tmx_record_id_gt);
        push_param(&mut params, "tmx_record_id.gte", tmx_record_id_gte);
        push_param(&mut params, "tmx_record_id.lt", tmx_record_id_lt);
        push_param(&mut params, "tmx_record_id.lte", tmx_record_id_lte);
        push_param(&mut params, "limit", limit);
        push_param(&mut params, "sort", sort);
        if self.pagination {
            self.paginate::<TmxCorporateEvent>(&path, Some(&params), options)
        } else {
            self.single_page::<TmxCorporateEvent>(&path, Some(&params), options)
        }
    }
}
