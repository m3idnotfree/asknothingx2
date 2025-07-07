#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("{message}")]
    AlreadyConfigured { message: String },

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
