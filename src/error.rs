use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("API authentication failed: {0}")]
    Auth(String),

    #[error("HTTP error {status}: {body}")]
    Http {
        status: StatusCode,
        body: String,
    },

    #[error("JSON deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Pagination stream ended with error: {0}")]
    Pagination(String),

    #[error("Missing API key. Set MASSIVE_API_KEY env var or pass api_key to the client.")]
    MissingApiKey,
}

pub type Result<T> = std::result::Result<T, Error>;
