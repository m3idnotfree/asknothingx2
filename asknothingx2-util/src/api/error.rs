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
    RequestBuild,
    HttpInvalidHeader,
    AuthInvalidScheme,
    ContentTypeInvalid,
    ContentTypeUnsupported,
}

impl Kind {
    pub fn category(self) -> ErrorCategory {
        match self {
            Kind::RequestBuild => ErrorCategory::Request,
            Kind::HttpInvalidHeader => ErrorCategory::Http,
            Kind::AuthInvalidScheme => ErrorCategory::Authentication,
            Kind::ContentTypeInvalid | Kind::ContentTypeUnsupported => ErrorCategory::ContentType,
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::RequestBuild => f.write_str("failed to build request"),
            Kind::HttpInvalidHeader => f.write_str("invalid HTTP header"),
            Kind::AuthInvalidScheme => f.write_str("invalid authentication scheme"),
            Kind::ContentTypeInvalid => f.write_str("invalid content type"),
            Kind::ContentTypeUnsupported => f.write_str("unsupported content type"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    Request,
    Http,
    Authentication,
    ContentType,
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

    pub fn is_request(&self) -> bool {
        matches!(self.inner.kind, Kind::RequestBuild)
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
}

pub mod http {
    use super::{BoxError, Error, Kind};
    pub fn invalid_header<E: Into<BoxError>>(source: E) -> Error {
        Error::with_source(Kind::HttpInvalidHeader, source)
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
}

fn truncate_input(input: &str) -> &str {
    const MAX_LEN: usize = 80;
    if input.len() <= MAX_LEN {
        input
    } else {
        &input[..MAX_LEN]
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
