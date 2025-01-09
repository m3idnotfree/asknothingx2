use super::api_response::APIError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP client error: {0}")]
    RequestError(#[from] super::ReqwestError),
    #[error("Invalid Header name: {0}")]
    InvalidHeaderName(#[from] http::header::InvalidHeaderName),
    #[error("Invalid Header value: {0}")]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
    #[error("Failed to deserialize response: {0}")]
    DeserializationError(String),
    #[error("API returned error- status {}: {}",.0.status(), .0.raw_message())]
    ResponseError(APIError),
}
