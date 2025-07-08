pub mod content_type;
pub mod request;
pub mod setup;

mod api_response;
mod auth_scheme;
mod config;
mod error;
mod header_builder;

pub use api_response::{APIError, APIResponse, EmptyArrayBody, EmptyObjectBody, EmptyStringBody};
pub use auth_scheme::{AuthError, AuthScheme, DigestBuilder, SCRAMVariant};
pub use config::{AppType, Config};
pub use error::ConfigError;
pub use header_builder::HeaderBuilder;

// Re-export
pub use http::{HeaderMap, Method, StatusCode};
pub use reqwest::{Client, Error as ReqwestError, RequestBuilder};

use setup::get_global_client_or_default;

pub fn get(url: &str) -> RequestBuilder {
    let client = get_global_client_or_default();
    client.get(url)
}

pub fn post(url: &str) -> RequestBuilder {
    let client = get_global_client_or_default();
    client.post(url)
}
