pub mod app_type;
pub mod extra_config;
pub mod mime_type;
pub mod request;
pub mod setup;

mod auth_scheme;
mod config;
mod error;
mod header_mut;

pub use auth_scheme::{AuthScheme, DigestBuilder, SCRAMVariant};
pub use config::Config;
pub use error::{Error, Kind};
pub use header_mut::HeaderMut;

// Re-export
pub use http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
pub use reqwest::{Client, Error as ReqwestError, RequestBuilder, Response};

use app_type::AppType;
use url::Url;

pub mod client {
    use std::sync::Arc;

    use reqwest::Client;

    use super::{app_type::AppType, setup::registry};

    #[inline]
    pub async fn get_or_default(name: &AppType) -> Arc<Client> {
        registry().get_or_default(name).await
    }

    #[inline]
    pub async fn default() -> Arc<Client> {
        registry().get_default().await
    }

    #[inline]
    pub async fn web() -> Arc<Client> {
        get_or_default(&AppType::WEB).await
    }

    #[inline]
    pub async fn production() -> Arc<Client> {
        get_or_default(&AppType::PRODUCTION).await
    }

    #[inline]
    pub async fn gateway() -> Arc<Client> {
        get_or_default(&AppType::GATEWAY).await
    }

    #[inline]
    pub async fn scraping() -> Arc<Client> {
        get_or_default(&AppType::SCRAPING).await
    }

    #[inline]
    pub async fn development() -> Arc<Client> {
        get_or_default(&AppType::DEVELOPMENT).await
    }
}

#[inline]
pub async fn get(url: Url) -> RequestBuilder {
    client::default().await.get(url)
}

#[inline]
pub async fn get_for(name: &AppType, url: Url) -> RequestBuilder {
    client::get_or_default(name).await.get(url)
}

#[inline]
pub async fn post(url: Url) -> RequestBuilder {
    client::default().await.post(url)
}

#[inline]
pub async fn post_for(name: &AppType, url: Url) -> RequestBuilder {
    client::get_or_default(name).await.post(url)
}

#[inline]
pub async fn put(url: Url) -> RequestBuilder {
    client::default().await.put(url)
}

#[inline]
pub async fn put_for(name: &AppType, url: Url) -> RequestBuilder {
    client::get_or_default(name).await.put(url)
}

#[inline]
pub async fn delete(url: Url) -> RequestBuilder {
    client::default().await.delete(url)
}

#[inline]
pub async fn delete_for(name: &AppType, url: Url) -> RequestBuilder {
    client::get_or_default(name).await.delete(url)
}

pub mod production {
    use reqwest::RequestBuilder;
    use url::Url;

    use super::client;

    #[inline]
    pub async fn get(url: Url) -> RequestBuilder {
        client::production().await.get(url)
    }

    #[inline]
    pub async fn post(url: Url) -> RequestBuilder {
        client::production().await.post(url)
    }

    #[inline]
    pub async fn put(url: Url) -> RequestBuilder {
        client::production().await.put(url)
    }

    #[inline]
    pub async fn delete(url: Url) -> RequestBuilder {
        client::production().await.delete(url)
    }
}

pub mod gateway {
    use reqwest::RequestBuilder;
    use url::Url;

    use super::client;

    #[inline]
    pub async fn get(url: Url) -> RequestBuilder {
        client::gateway().await.get(url)
    }

    #[inline]
    pub async fn post(url: Url) -> RequestBuilder {
        client::gateway().await.post(url)
    }
}

pub mod scraping {
    use reqwest::RequestBuilder;
    use url::Url;

    use super::client;

    #[inline]
    pub async fn get(url: Url) -> RequestBuilder {
        client::scraping().await.get(url)
    }
}
