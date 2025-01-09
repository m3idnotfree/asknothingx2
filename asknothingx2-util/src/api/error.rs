#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP client error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Invalid Header name: {0}")]
    InvalidHeaderName(#[from] http::header::InvalidHeaderName),
    #[error("Invalid Header value: {0}")]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
}
