pub mod mime_type;
pub mod preset;
pub mod request;

mod auth_scheme;
mod error;
mod header_mut;

pub use auth_scheme::{AuthScheme, DigestBuilder, SCRAMVariant};
pub use error::{Error, Kind};
pub use header_mut::HeaderMut;

// Re-export
pub use http::{header, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};

#[cfg(feature = "reqwest")]
pub use reqwest;
