use std::fmt;

use ::http::header::{InvalidHeaderName, InvalidHeaderValue};

pub struct Error {
    inner: Box<Inner>,
}

#[derive(Debug)]
struct Inner {
    kind: Kind,
    message: Option<String>,
    input: Option<String>,
    source: Option<BoxError>,
}

#[derive(Debug)]
pub enum Kind {
    // Request building errors
    RequestBuild,
    RequestInvalid,
    RequestTimeout,

    // Network and connection errors
    NetworkConnect,
    NetworkTimeout,
    NetworkDns,
    NetworkProxy,

    // HTTP protocol errors
    HttpInvalidMethod,
    HttpInvalidUrl,
    HttpInvalidHeader,
    HttpInvalidBody,
    HttpRedirectLoop,

    // Authentication errors
    AuthInvalidScheme,
    AuthInvalidCredentials,
    AuthMissing,

    // Content type and encoding errors
    ContentTypeInvalid,
    ContentTypeUnsupported,
    ContentEncodingInvalid,

    // File and I/O errors
    FileNotFound,
    FilePermission,
    FileCorrupted,
    IoOperation,

    // Process and stream errors
    ProcessSpawn,
    ProcessTimeout,
    StreamOperation,
    StreamClosed,

    // TLS and security errors
    TlsHandshake,
    TlsCertificateInvalid,
    TlsVersionUnsupported,

    // Serialization errors
    JsonParse,
    JsonGenerate,
    UrlEncode,
    UrlDecode,

    // Resource limits
    LimitExceeded,

    // Internal errors
    Internal,
    Unknown,
}

impl Kind {
    pub fn category(self) -> ErrorCategory {
        match self {
            Kind::RequestBuild | Kind::RequestInvalid | Kind::RequestTimeout => {
                ErrorCategory::Request
            }
            Kind::NetworkConnect | Kind::NetworkTimeout | Kind::NetworkDns | Kind::NetworkProxy => {
                ErrorCategory::Network
            }
            Kind::HttpInvalidMethod
            | Kind::HttpInvalidUrl
            | Kind::HttpInvalidHeader
            | Kind::HttpInvalidBody
            | Kind::HttpRedirectLoop => ErrorCategory::Http,
            Kind::AuthInvalidScheme | Kind::AuthInvalidCredentials | Kind::AuthMissing => {
                ErrorCategory::Authentication
            }
            Kind::ContentTypeInvalid
            | Kind::ContentTypeUnsupported
            | Kind::ContentEncodingInvalid => ErrorCategory::ContentType,
            Kind::FileNotFound | Kind::FilePermission | Kind::FileCorrupted | Kind::IoOperation => {
                ErrorCategory::Io
            }
            Kind::ProcessSpawn | Kind::ProcessTimeout => ErrorCategory::Process,
            Kind::StreamOperation | Kind::StreamClosed => ErrorCategory::Stream,
            Kind::TlsHandshake | Kind::TlsCertificateInvalid | Kind::TlsVersionUnsupported => {
                ErrorCategory::Tls
            }
            Kind::JsonParse | Kind::JsonGenerate | Kind::UrlEncode | Kind::UrlDecode => {
                ErrorCategory::Serialization
            }
            Kind::LimitExceeded => ErrorCategory::Resource,
            Kind::Internal | Kind::Unknown => ErrorCategory::Internal,
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::RequestBuild => f.write_str("failed to build request"),
            Kind::RequestInvalid => f.write_str("invalid request"),
            Kind::RequestTimeout => f.write_str("request timeout"),

            Kind::NetworkConnect => f.write_str("connection failed"),
            Kind::NetworkTimeout => f.write_str("network timeout"),
            Kind::NetworkDns => f.write_str("DNS resolution failed"),
            Kind::NetworkProxy => f.write_str("proxy connection failed"),

            Kind::HttpInvalidMethod => f.write_str("invalid HTTP method"),
            Kind::HttpInvalidUrl => f.write_str("invalid URL"),
            Kind::HttpInvalidHeader => f.write_str("invalid HTTP header"),
            Kind::HttpInvalidBody => f.write_str("invalid request body"),
            Kind::HttpRedirectLoop => f.write_str("redirect loop detected"),

            Kind::AuthInvalidScheme => f.write_str("invalid authentication scheme"),
            Kind::AuthInvalidCredentials => f.write_str("invalid credentials"),
            Kind::AuthMissing => f.write_str("authentication required"),

            Kind::ContentTypeInvalid => f.write_str("invalid content type"),
            Kind::ContentTypeUnsupported => f.write_str("unsupported content type"),
            Kind::ContentEncodingInvalid => f.write_str("invalid content encoding"),

            Kind::FileNotFound => f.write_str("file not found"),
            Kind::FilePermission => f.write_str("file permission denied"),
            Kind::FileCorrupted => f.write_str("file is corrupted"),
            Kind::IoOperation => f.write_str("I/O operation failed"),

            Kind::ProcessSpawn => f.write_str("failed to spawn process"),
            Kind::ProcessTimeout => f.write_str("process timeout"),
            Kind::StreamOperation => f.write_str("stream operation failed"),
            Kind::StreamClosed => f.write_str("stream closed unexpectedly"),

            Kind::TlsHandshake => f.write_str("TLS handshake failed"),
            Kind::TlsCertificateInvalid => f.write_str("invalid TLS certificate"),
            Kind::TlsVersionUnsupported => f.write_str("unsupported TLS version"),

            Kind::JsonParse => f.write_str("JSON parsing failed"),
            Kind::JsonGenerate => f.write_str("JSON generation failed"),
            Kind::UrlEncode => f.write_str("URL encoding failed"),
            Kind::UrlDecode => f.write_str("URL decoding failed"),

            Kind::LimitExceeded => f.write_str("resource limit exceeded"),

            Kind::Internal => f.write_str("internal error"),
            Kind::Unknown => f.write_str("unknown error"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    Request,
    Network,
    Http,
    Authentication,
    ContentType,
    Io,
    Process,
    Stream,
    Tls,
    Serialization,
    Resource,
    Internal,
}

type BoxError = Box<dyn std::error::Error + Send + Sync>;

impl Error {
    pub fn new(kind: Kind) -> Self {
        Self {
            inner: Box::new(Inner {
                kind,
                message: None,
                input: None,
                source: None,
            }),
        }
    }

    pub fn with_message(kind: Kind, message: impl Into<String>) -> Self {
        Self {
            inner: Box::new(Inner {
                kind,
                message: Some(message.into()),
                input: None,
                source: None,
            }),
        }
    }

    pub fn with_source(kind: Kind, source: impl Into<BoxError>) -> Self {
        Self {
            inner: Box::new(Inner {
                kind,
                message: None,
                input: None,
                source: Some(source.into()),
            }),
        }
    }

    pub fn with_message_and_source(
        kind: Kind,
        message: impl Into<String>,
        source: impl Into<BoxError>,
    ) -> Self {
        Self {
            inner: Box::new(Inner {
                kind,
                message: Some(message.into()),
                input: None,
                source: Some(source.into()),
            }),
        }
    }

    pub fn with_input(mut self, input: impl Into<String>) -> Self {
        self.inner.input = Some(input.into());
        self
    }

    pub fn message(&self) -> Option<&str> {
        self.inner.message.as_deref()
    }

    pub fn input(&self) -> Option<&str> {
        self.inner.input.as_deref()
    }

    pub fn is_network(&self) -> bool {
        matches!(
            self.inner.kind,
            Kind::NetworkConnect | Kind::NetworkTimeout | Kind::NetworkDns | Kind::NetworkProxy
        )
    }

    pub fn is_request(&self) -> bool {
        matches!(
            self.inner.kind,
            Kind::RequestBuild | Kind::RequestInvalid | Kind::RequestTimeout
        )
    }

    pub fn is_retryable(&self) -> bool {
        matches!(
            self.inner.kind,
            Kind::NetworkTimeout
                | Kind::NetworkConnect
                | Kind::RequestTimeout
                | Kind::ProcessTimeout
                | Kind::TlsHandshake
        )
    }

    pub fn is_recoverable(&self) -> bool {
        matches!(
            self.inner.kind,
            Kind::NetworkTimeout
                | Kind::NetworkConnect
                | Kind::RequestTimeout
                | Kind::ProcessTimeout
                | Kind::StreamOperation
        )
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("asknothingx2-util::api::Error");

        builder.field("kind", &self.inner.kind);

        if let Some(ref message) = self.inner.message {
            builder.field("message", message);
        }

        if let Some(ref input) = self.inner.input {
            builder.field("input", input);
        }

        if let Some(ref source) = self.inner.source {
            builder.field("source", source);
        }

        builder.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref message) = self.inner.message {
            write!(f, "{message}")?;
        } else {
            write!(f, "{}", self.inner.kind)?;
        }

        if let Some(ref input) = self.inner.input {
            let truncated = truncate_input(input);
            if !truncated.is_empty() {
                write!(
                    f,
                    " [input: {}{}]",
                    truncated,
                    if input.len() > truncated.len() {
                        "..."
                    } else {
                        ""
                    }
                )?;
            }
        }

        if let Some(ref source) = self.inner.source {
            write!(f, " -> {source})")?;
        }

        Ok(())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source.as_ref().map(|e| &**e as _)
    }
}

pub mod request {
    use super::{BoxError, Error, Kind};

    pub fn build<E: Into<BoxError>>(source: E) -> Error {
        Error::with_source(Kind::RequestBuild, source)
    }

    pub fn invalid<M: Into<String>>(message: M) -> Error {
        Error::with_message(Kind::RequestInvalid, message)
    }
}

pub mod network {
    use super::{Error, Kind};

    pub fn connect<E: Into<super::BoxError>>(source: E) -> Error {
        Error::with_source(Kind::NetworkConnect, source)
    }

    pub fn timeout() -> Error {
        Error::new(Kind::NetworkTimeout)
    }
}

pub mod http {
    use super::{BoxError, Error, Kind};

    pub fn invalid_method<M: Into<String>>(method: M) -> Error {
        Error::with_message(
            Kind::HttpInvalidMethod,
            format!("invalid HTTP method '{}'", method.into()),
        )
    }

    pub fn invalid_url<E: Into<BoxError>>(source: E) -> Error {
        Error::with_source(Kind::HttpInvalidUrl, source)
    }

    pub fn invalid_header<E: Into<BoxError>>(source: E) -> Error {
        Error::with_source(Kind::HttpInvalidHeader, source)
    }

    pub fn invalid_body<M: Into<String>>(message: M) -> Error {
        Error::with_message(Kind::HttpInvalidBody, message)
    }
}

pub mod auth {
    use super::{Error, Kind};

    pub fn invalid_scheme<S: Into<String>>(scheme: S) -> Error {
        Error::with_message(
            Kind::AuthInvalidScheme,
            format!("invalid authentication scheme '{}'", scheme.into()),
        )
    }

    pub fn invalid_credentials() -> Error {
        Error::new(Kind::AuthInvalidCredentials)
    }
}

pub mod content {
    use super::{Error, Kind};

    pub fn invalid_type<T: Into<String>>(content_type: T) -> Error {
        Error::with_message(
            Kind::ContentTypeInvalid,
            format!("invalid content type '{}'", content_type.into()),
        )
    }

    pub fn unsupported<T: Into<String>>(content_type: T) -> Error {
        Error::with_message(
            Kind::ContentTypeUnsupported,
            format!("unsupported content type '{}'", content_type.into()),
        )
    }

    pub fn invalid_encoding<E: Into<String>>(encoding: E) -> Error {
        Error::with_message(
            Kind::ContentEncodingInvalid,
            format!("invalid content encoding '{}'", encoding.into()),
        )
    }
}

pub mod io {
    use std::path::Path;

    use super::{BoxError, Error, Kind};

    pub fn file_not_found<P: AsRef<Path>>(path: P) -> Error {
        Error::with_message(
            Kind::FileNotFound,
            format!("file not found: {}", path.as_ref().display()),
        )
    }

    pub fn permission_denied<P: AsRef<Path>>(path: P) -> Error {
        Error::with_message(
            Kind::FilePermission,
            format!("permission denied: {}", path.as_ref().display()),
        )
    }

    pub fn corrupted<P: AsRef<Path>>(path: P) -> Error {
        Error::with_message(
            Kind::FileCorrupted,
            format!("file corrupted: {}", path.as_ref().display()),
        )
    }

    pub fn operation<E: Into<BoxError>>(source: E) -> Error {
        Error::with_source(Kind::IoOperation, source)
    }
}

pub mod process {
    use super::{BoxError, Error, Kind};

    pub fn spawn<E: Into<BoxError>>(source: E) -> Error {
        Error::with_source(Kind::ProcessSpawn, source)
    }

    pub fn timeout() -> Error {
        Error::new(Kind::ProcessTimeout)
    }
}

pub mod stream {
    use super::{BoxError, Error, Kind};

    pub fn operation<E: Into<BoxError>>(source: E) -> Error {
        Error::with_source(Kind::StreamOperation, source)
    }

    pub fn closed() -> Error {
        Error::new(Kind::StreamClosed)
    }
}

pub mod serialization {
    use super::{BoxError, Error, Kind};

    pub fn json_parse<E: Into<BoxError>>(source: E) -> Error {
        Error::with_source(Kind::JsonParse, source)
    }

    pub fn json_generate<E: Into<BoxError>>(source: E) -> Error {
        Error::with_source(Kind::JsonGenerate, source)
    }

    pub fn url_encode<E: Into<BoxError>>(source: E) -> Error {
        Error::with_source(Kind::UrlEncode, source)
    }

    pub fn url_decode<E: Into<BoxError>>(source: E) -> Error {
        Error::with_source(Kind::UrlDecode, source)
    }
}

pub mod resource {
    use super::{Error, Kind};

    pub fn limit_exceeded<L: Into<String>>(limit: L) -> Error {
        Error::with_message(
            Kind::LimitExceeded,
            format!("limit exceeded: {}", limit.into()),
        )
    }
}

fn truncate_input(input: &str) -> &str {
    const MAX_LEN: usize = 80;
    if input.len() <= MAX_LEN {
        input
    } else {
        &input[..MAX_LEN]
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        use std::io::ErrorKind;

        let kind = match err.kind() {
            ErrorKind::NotFound => Kind::FileNotFound,
            ErrorKind::PermissionDenied => Kind::FilePermission,
            ErrorKind::ConnectionRefused | ErrorKind::ConnectionReset => Kind::NetworkConnect,
            ErrorKind::TimedOut => Kind::NetworkTimeout,
            ErrorKind::InvalidData => Kind::FileCorrupted,
            _ => Kind::IoOperation,
        };

        Error::with_source(kind, err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Error::with_source(Kind::NetworkTimeout, err)
        } else if err.is_connect() {
            Error::with_source(Kind::NetworkConnect, err)
        } else if err.is_request() {
            Error::with_source(Kind::RequestInvalid, err)
        } else if err.is_redirect() {
            Error::with_source(Kind::HttpRedirectLoop, err)
        } else {
            Error::with_source(Kind::Unknown, err)
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::with_source(Kind::HttpInvalidUrl, err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        if err.is_syntax() || err.is_data() {
            Error::with_source(Kind::JsonParse, err)
        } else {
            Error::with_source(Kind::JsonGenerate, err)
        }
    }
}

impl From<InvalidHeaderName> for Error {
    fn from(err: InvalidHeaderName) -> Self {
        Error::with_source(Kind::HttpInvalidHeader, err)
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(err: InvalidHeaderValue) -> Self {
        Error::with_source(Kind::HttpInvalidHeader, err)
    }
}
