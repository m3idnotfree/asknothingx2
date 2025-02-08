#[cfg(feature = "twitch-webhook")]
#[derive(Debug, thiserror::Error)]
pub enum HeaderVerificationError {
    #[error("Missing required header: {0}")]
    MissingHeader(&'static str),
    #[error("Invalid header value: {name}={value}")]
    InvalidHeaderValue { name: &'static str, value: String },
    #[error("Signature verification failed")]
    SignatureVerification(#[from] hmac::digest::MacError),
    #[error("Invalid hex signature")]
    InvalidHexSignature(#[from] hex::FromHexError),
}

#[cfg(feature = "twitch-websocket")]
#[derive(Debug, thiserror::Error)]
pub enum WebSocketError {
    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("Missing required field: {0}")]
    MissingField(&'static str),
    #[error("Invalid message type: {0}")]
    InvalidMessageType(String),
    #[error("Deserialization error: {0}")]
    DeserializeError(String),
}
