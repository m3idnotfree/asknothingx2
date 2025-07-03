#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ContentTypeError {
    #[error("Content-Type header is required but was empty")]
    Empty,
    #[error("Content-Type header contains invalid UTF-8")]
    InvalidUtf8,
    #[error("Invalid MIME type `{0}` (expected format: type/subtype)")]
    InvalidMimeType(String),
    #[error("Unsupported content type `{0}`")]
    Unsupported(String),
    #[error("Malformed Content-Type header: {0}")]
    Malformed(String),
}
