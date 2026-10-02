use crate::error::{Error, Result};
use crate::paginate::PaginatedStream;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, AUTHORIZATION, USER_AGENT};
use std::time::Duration;
use tracing::info;

const DEFAULT_BASE: &str = "https://api.massive.com";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

/// Options for customizing requests, e.g. Launchpad edge headers.
#[derive(Debug, Default, Clone)]
pub struct RequestOptions {
    pub headers: HeaderMap,
}

impl RequestOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_edge_headers(edge_id: &str, edge_ip: &str, edge_user: Option<&str>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("X-Massive-Edge-ID", HeaderValue::from_str(edge_id).unwrap());
        headers.insert(
            "X-Massive-Edge-IP-Address",
            HeaderValue::from_str(edge_ip).unwrap(),
        );
        if let Some(u) = edge_user {
            headers.insert(
                "X-Massive-Edge-User-Agent",
                HeaderValue::from_str(u).unwrap(),
            );
        }
        Self { headers }
    }
}

/// Core HTTP client for the Massive API.
#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) api_key: String,
    pub(crate) base: String,
    pub(crate) http: reqwest::Client,
    pub(crate) pagination: bool,
    pub(crate) trace: bool,
    pub(crate) max_retries: u32,
}

impl Client {
    /// Create a new client with the given API key.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        let api_key = api_key.into();
        if api_key.is_empty() {
            return Err(Error::MissingApiKey);
        }
        let http = reqwest::Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .gzip(true)
            .build()?;
        Ok(Self {
            api_key,
            base: DEFAULT_BASE.to_string(),
            http,
            pagination: true,
            trace: false,
            max_retries: 0,
        })
    }

    /// Create a client from the `MASSIVE_API_KEY` environment variable.
    pub fn from_env() -> Result<Self> {
        let key = std::env::var("MASSIVE_API_KEY").map_err(|_| Error::MissingApiKey)?;
        Self::new(key)
    }

    /// Set a custom base URL (e.g. for testing or `api.polygon.io`).
    pub fn with_base(mut self, base: impl Into<String>) -> Self {
        self.base = base.into();
        self
    }

    /// Enable or disable automatic pagination (default: true).
    pub fn with_pagination(mut self, pagination: bool) -> Self {
        self.pagination = pagination;
        self
    }

    /// Enable request/response tracing.
    pub fn with_trace(mut self, trace: bool) -> Self {
        self.trace = trace;
        self
    }

    /// Retry failed requests up to `max_retries` times on HTTP 429 and 5xx
    /// responses, with exponential backoff (default: 0, matching the Python client).
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Build default headers including auth.
    fn default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let auth = format!("Bearer {}", self.api_key);
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth).unwrap());
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip"));
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(concat!("massive-rs/", env!("CARGO_PKG_VERSION"))),
        );
        headers
    }

    /// Build the merged header set for a request (default + per-request options).
    fn request_headers(&self, options: Option<&RequestOptions>) -> HeaderMap {
        let mut headers = self.default_headers();
        if let Some(opts) = options {
            for (k, v) in &opts.headers {
                headers.insert(k, v.clone());
            }
        }
        headers
    }

    /// Append a pre-encoded query string (from `rest::encode_query`) to a path.
    fn build_url(&self, path: &str, query: &str) -> String {
        let mut url = format!("{}{}", self.base, path);
        if !query.is_empty() {
            url.push('?');
            url.push_str(query);
        }
        url
    }

    /// Internal GET request taking a pre-encoded query string.
    pub(crate) async fn get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: &str,
        options: Option<&RequestOptions>,
    ) -> Result<T> {
        let url = self.build_url(path, query);
        let headers = self.request_headers(options);

        if self.trace {
            info!("Request URL: {}", url);
        }

        let resp =
            crate::paginate::send_with_retry(&self.http, &url, headers, self.max_retries).await?;
        let status = resp.status();

        if self.trace {
            info!("Response Status: {}", status);
        }

        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::Http { status, body });
        }

        let data = resp.json().await?;
        Ok(data)
    }

    /// Start a stream of results, following `next_url` pages when pagination
    /// is enabled (the default) or returning a single page when it is not.
    ///
    /// This is the only transport entry point for `list_*` methods: the
    /// pagination branch lives here, not at the call sites. Takes a
    /// pre-encoded query string from `rest::encode_query`.
    pub(crate) fn list<T: serde::de::DeserializeOwned + Send + 'static>(
        &self,
        path: &str,
        query: &str,
        options: Option<&RequestOptions>,
    ) -> PaginatedStream<T> {
        let url = self.build_url(path, query);
        let headers = self.request_headers(options);
        if self.pagination {
            PaginatedStream::new(self.http.clone(), headers, url, self.max_retries)
        } else {
            PaginatedStream::single_page(self.http.clone(), headers, url, self.max_retries)
        }
    }
}
