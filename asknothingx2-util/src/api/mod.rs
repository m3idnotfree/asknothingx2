pub mod content_type;
pub mod request;
pub mod setup;

mod app_type;
mod auth_scheme;
mod config;
mod error;
mod header_mut;

pub use app_type::{AppType, AppTypeMarker};
pub use auth_scheme::{AuthError, AuthScheme, DigestBuilder, SCRAMVariant};
pub use config::Config;
pub use error::ConfigError;
pub use header_mut::HeaderMut;

// Re-export
pub use http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
pub use reqwest::{Client, Error as ReqwestError, RequestBuilder, Response};

use setup::get_global_client_or_default;

pub fn get(url: &str) -> RequestBuilder {
    let client = get_global_client_or_default();
    client.get(url)
}

pub fn post(url: &str) -> RequestBuilder {
    let client = get_global_client_or_default();
    client.post(url)
}
