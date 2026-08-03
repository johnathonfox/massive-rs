use crate::error::{Error, Result};
use futures::Stream;
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// A paginated response from the Massive API.
#[derive(Debug, serde::Deserialize)]
pub struct PaginatedResponse<T> {
    pub results: Option<Vec<T>>,
    #[serde(rename = "next_url")]
    pub next_url: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "request_id")]
    pub request_id: Option<String>,
    pub count: Option<i64>,
}

type PageFuture<T> = Pin<Box<dyn Future<Output = Result<PaginatedResponse<T>>> + Send>>;

/// A stream that automatically follows `next_url` pagination.
pub struct PaginatedStream<T> {
    client: reqwest::Client,
    headers: HeaderMap,
    next_url: Option<String>,
    follow_pages: bool,
    buffer: std::collections::VecDeque<T>,
    pending: Option<PageFuture<T>>,
}

// Safe: we never pin-project into the buffered items.
impl<T> Unpin for PaginatedStream<T> {}

impl<T: DeserializeOwned + Send + 'static> PaginatedStream<T> {
    pub(crate) fn new(client: reqwest::Client, headers: HeaderMap, initial_url: String) -> Self {
        Self {
            client,
            headers,
            next_url: Some(initial_url),
            follow_pages: true,
            buffer: std::collections::VecDeque::new(),
            pending: None,
        }
    }

    pub(crate) fn single_page(client: reqwest::Client, headers: HeaderMap, url: String) -> Self {
        Self {
            client,
            headers,
            next_url: Some(url),
            follow_pages: false,
            buffer: std::collections::VecDeque::new(),
            pending: None,
        }
    }

    fn fetch_page(client: reqwest::Client, headers: HeaderMap, url: String) -> PageFuture<T> {
        Box::pin(async move {
            let resp = client.get(&url).headers(headers).send().await?;
            let status = resp.status();
            if !status.is_success() {
                let body = resp.text().await.unwrap_or_default();
                return Err(Error::Http { status, body });
            }
            Ok(resp.json::<PaginatedResponse<T>>().await?)
        })
    }
}

impl<T: DeserializeOwned + Send + 'static> Stream for PaginatedStream<T> {
    type Item = Result<T>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // All fields are Unpin, so mutable projection is safe.
        let this = self.get_mut();
        loop {
            // Return buffered items first.
            if let Some(item) = this.buffer.pop_front() {
                return Poll::Ready(Some(Ok(item)));
            }

            // No more pages to fetch.
            if this.next_url.is_none() && this.pending.is_none() {
                return Poll::Ready(None);
            }

            // Start a new request if we have a URL and no pending request.
            if let Some(url) = this.next_url.take() {
                this.pending = Some(Self::fetch_page(
                    this.client.clone(),
                    this.headers.clone(),
                    url,
                ));
            }

            // Poll the pending request.
            if let Some(ref mut fut) = this.pending {
                match fut.as_mut().poll(cx) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(Err(e)) => {
                        this.pending = None;
                        return Poll::Ready(Some(Err(e)));
                    }
                    Poll::Ready(Ok(page)) => {
                        this.pending = None;
                        this.next_url = if this.follow_pages { page.next_url } else { None };
                        if let Some(results) = page.results {
                            this.buffer = results.into();
                        }
                        // If no results and no next_url, stream ends next loop.
                    }
                }
            }
        }
    }
}
