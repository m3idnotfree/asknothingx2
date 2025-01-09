use super::api_response::APIError;

#[derive(Debug, thiserror::Error)]
pub enum JsonError {
    #[error("Failed to deserialize response: {0}")]
    DeserializationError(#[from] serde_json::Error),
    #[error("API returned error - status {}: {}", .0.status(), .0.raw_message())]
    ResponseError(APIError),
}
