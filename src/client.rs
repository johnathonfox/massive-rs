use crate::error::{Error, Result};
use crate::paginate::PaginatedStream;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, ACCEPT_ENCODING, USER_AGENT};
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
        headers.insert("X-Massive-Edge-IP-Address", HeaderValue::from_str(edge_ip).unwrap());
        if let Some(u) = edge_user {
            headers.insert("X-Massive-Edge-User-Agent", HeaderValue::from_str(u).unwrap());
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

    /// Build default headers including auth.
    fn default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let auth = format!("Bearer {}", self.api_key);
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth).unwrap());
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip"));
        headers.insert(USER_AGENT, HeaderValue::from_static(concat!("massive-rs/", env!("CARGO_PKG_VERSION"))));
        headers
    }

    /// Internal GET request.
    pub(crate) async fn get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        params: Option<&[(&str, String)]>,
        options: Option<&RequestOptions>,
    ) -> Result<T> {
        let url = format!("{}{}", self.base, path);
        let mut req = self.http.get(&url);

        let mut headers = self.default_headers();
        if let Some(opts) = options {
            for (k, v) in &opts.headers {
                headers.insert(k, v.clone());
            }
        }
        req = req.headers(headers);

        if let Some(p) = params {
            req = req.query(p);
        }

        if self.trace {
            info!("Request URL: {}", url);
            let redacted = self.default_headers();
            info!("Request Headers: {:?}", redacted);
        }

        let resp = req.send().await?;
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

    fn build_url(&self, path: &str, params: Option<&[(&str, String)]>) -> String {
        let mut url = format!("{}{}", self.base, path);
        if let Some(p) = params {
            let query = serde_urlencoded::to_string(p).unwrap_or_default();
            if !query.is_empty() {
                url.push('?');
                url.push_str(&query);
            }
        }
        url
    }

    /// Start a paginated stream.
    pub(crate) fn paginate<T: serde::de::DeserializeOwned + Send + 'static>(
        &self,
        path: &str,
        params: Option<&[(&str, String)]>,
        options: Option<&RequestOptions>,
    ) -> PaginatedStream<T> {
        let url = self.build_url(path, params);
        PaginatedStream::new(self.http.clone(), self.request_headers(options), url)
    }

    /// Single page request (no pagination follow).
    pub(crate) fn single_page<T: serde::de::DeserializeOwned + Send + 'static>(
        &self,
        path: &str,
        params: Option<&[(&str, String)]>,
        options: Option<&RequestOptions>,
    ) -> PaginatedStream<T> {
        let url = self.build_url(path, params);
        PaginatedStream::single_page(self.http.clone(), self.request_headers(options), url)
    }
}
