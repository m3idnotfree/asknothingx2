use super::api_response::APIError;

#[derive(Debug, thiserror::Error)]
pub enum JsonError {
    #[error("Failed to deserialize response: {0}")]
    DeserializationError(#[from] serde_json::Error),
    #[error("API returned error - status {}: {}", .0.status(), .0.raw_message())]
    ResponseError(APIError),
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("HTTP client already configured. Call this only once at the start of your program.")]
    AlreadyConfigured,

    #[error("Invalid proxy URL '{url}': {reason}")]
    InvalidProxyUrl {
        url: String,
        reason: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("Failed to build HTTP client: {reason}")]
    ClientBuildFailed {
        reason: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("HTTP client not initialized. Call setup_for_web_apps(), setup_for_production(), or setup_automatically() first.")]
    NotInitialized,

    #[error("Could not acquire write lock on configuration")]
    ConfigurationLocked,

    #[error("Invalid configuration: {details}")]
    InvalidConfig { details: String },
}
