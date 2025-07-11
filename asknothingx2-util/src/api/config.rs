use std::time::Duration;

use http::{
    header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION},
    HeaderMap, HeaderValue,
};
use reqwest::{tls, Certificate, Client, Proxy};

use super::{content_type::Application, error::ConfigError, AppType};

mod user_agents {
    pub const CLI: &str = "asknothingx2-cli/0.0.28";
    pub const WEB: &str = "asknothingx2/0.0.28";
    pub const PRODUCTION: &str = "asknothingx2-production/0.0.28";
    pub const DEVELOPMENT: &str = "asknothingx2-dev/0.0.28";
    pub const GATEWAY: &str = "asknothingx2-gateway/0.0.28";
    pub const SCRAPING: &str =
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:91.0) Gecko/20100101 Firefox/91.0";
}

mod custom_headers {
    pub const X_CLIENT: &str = "x-client";
    pub const X_DEVELOPMENT: &str = "x-development";
    pub const DNT: &str = "dnt";
    pub const UPGRADE_INSECURE_REQUESTS: &str = "upgrade-insecure-requests";
}

#[derive(Debug, Clone)]
pub struct Config {
    pub app_type: AppType,

    // Timeouts
    pub request_timeout: Duration,
    pub connection_timeout: Duration,

    // Performance
    pub max_connections: usize,
    pub keep_connections_for: Duration,
    pub detect_dead_connections: Option<Duration>,

    // Network settings
    pub proxy_url: Option<String>,
    pub custom_certificates: Vec<Certificate>,
    pub default_headers: HeaderMap,
    pub user_agent: String,

    // Behavior
    pub follow_redirects: u32,
    pub save_cookies: bool,
    pub send_referer: bool,
    pub compress_requests: bool,

    // Security
    pub allow_invalid_certificates: bool,
    pub allow_wrong_hostnames: bool,
    pub require_https: bool,
    pub minimum_tls_version: Option<tls::Version>,

    // Advanced (usually don't need to change)
    pub prefer_http2: bool,
    pub force_http2_only: bool,
    pub async_dns: bool,
}

impl Config {
    pub fn for_cli_tools() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));

        Self {
            app_type: AppType::Cli,
            request_timeout: Duration::from_secs(60),
            connection_timeout: Duration::from_secs(10),
            max_connections: 1,
            keep_connections_for: Duration::from_secs(5),
            detect_dead_connections: None,
            proxy_url: None,
            custom_certificates: Vec::new(),
            default_headers: headers,
            user_agent: user_agents::CLI.to_string(),
            follow_redirects: 5,
            save_cookies: false,
            send_referer: true,
            compress_requests: true,
            allow_invalid_certificates: false,
            allow_wrong_hostnames: false,
            require_https: false,
            minimum_tls_version: None,
            prefer_http2: false,
            force_http2_only: false,
            async_dns: false,
        }
    }

    pub fn for_web_apps() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/json, text/plain, */*"),
        );
        headers.insert(
            ACCEPT_ENCODING,
            HeaderValue::from_static("gzip, br, deflate"),
        );

        Self {
            app_type: AppType::Web,
            request_timeout: Duration::from_secs(30),
            connection_timeout: Duration::from_secs(5),
            max_connections: 10,
            keep_connections_for: Duration::from_secs(90),
            detect_dead_connections: Some(Duration::from_secs(60)),
            proxy_url: None,
            custom_certificates: Vec::new(),
            default_headers: headers,
            user_agent: user_agents::WEB.to_string(),
            follow_redirects: 10,
            save_cookies: false,
            send_referer: true,
            compress_requests: true,
            allow_invalid_certificates: false,
            allow_wrong_hostnames: false,
            require_https: false,
            minimum_tls_version: Some(tls::Version::TLS_1_2),
            prefer_http2: true,
            force_http2_only: false,
            async_dns: true,
        }
    }

    pub fn for_production() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, Application::Json.to_header_value());
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, br"));
        headers.insert(
            custom_headers::X_CLIENT,
            HeaderValue::from_static("asknothingx2-production"),
        );

        Self {
            app_type: AppType::Production,
            request_timeout: Duration::from_secs(10),
            connection_timeout: Duration::from_secs(3),
            max_connections: 50,
            keep_connections_for: Duration::from_secs(300),
            detect_dead_connections: Some(Duration::from_secs(30)),
            proxy_url: None,
            custom_certificates: Vec::new(),
            default_headers: headers,
            user_agent: user_agents::PRODUCTION.to_string(),
            follow_redirects: 3,
            save_cookies: false,
            send_referer: false,
            compress_requests: true,
            allow_invalid_certificates: false,
            allow_wrong_hostnames: false,
            require_https: true,
            minimum_tls_version: Some(tls::Version::TLS_1_3),
            prefer_http2: true,
            force_http2_only: true,
            async_dns: true,
        }
    }

    pub fn for_development() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        headers.insert(
            custom_headers::X_DEVELOPMENT,
            HeaderValue::from_static("true"),
        );

        Self {
            app_type: AppType::Development,
            request_timeout: Duration::from_secs(5),
            connection_timeout: Duration::from_secs(2),
            max_connections: 1,
            keep_connections_for: Duration::from_secs(1),
            detect_dead_connections: None,
            proxy_url: None,
            custom_certificates: Vec::new(),
            default_headers: headers,
            user_agent: user_agents::DEVELOPMENT.to_string(),
            follow_redirects: 0,
            save_cookies: false,
            send_referer: false,
            compress_requests: true,
            allow_invalid_certificates: true,
            allow_wrong_hostnames: true,
            require_https: false,
            minimum_tls_version: None,
            prefer_http2: false,
            force_http2_only: false,
            async_dns: false,
        }
    }

    pub fn for_api_gateway() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, Application::Json.to_header_value());

        Self {
            app_type: AppType::Gateway,
            request_timeout: Duration::from_secs(5),
            connection_timeout: Duration::from_secs(1),
            max_connections: 100,
            keep_connections_for: Duration::from_secs(600),
            detect_dead_connections: Some(Duration::from_secs(15)),
            proxy_url: None,
            custom_certificates: Vec::new(),
            default_headers: headers,
            user_agent: user_agents::GATEWAY.to_string(),
            follow_redirects: 0,
            save_cookies: false,
            send_referer: false,
            compress_requests: true,
            allow_invalid_certificates: false,
            allow_wrong_hostnames: false,
            require_https: false,
            minimum_tls_version: Some(tls::Version::TLS_1_3),
            prefer_http2: true,
            force_http2_only: true,
            async_dns: true,
        }
    }

    pub fn for_web_scraping() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static(
                "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
            ),
        );
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.5"));
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip,deflate,br"));
        headers.insert(custom_headers::DNT, HeaderValue::from_static("1"));
        headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
        headers.insert(
            custom_headers::UPGRADE_INSECURE_REQUESTS,
            HeaderValue::from_static("1"),
        );

        Self {
            app_type: AppType::Scraping,
            request_timeout: Duration::from_secs(30),
            connection_timeout: Duration::from_secs(10),
            max_connections: 5,
            keep_connections_for: Duration::from_secs(60),
            detect_dead_connections: Some(Duration::from_secs(60)),
            proxy_url: None,
            custom_certificates: Vec::new(),
            default_headers: headers,
            user_agent: user_agents::SCRAPING.to_string(),
            follow_redirects: 10,
            save_cookies: true,
            send_referer: true,
            compress_requests: true,
            allow_invalid_certificates: false,
            allow_wrong_hostnames: false,
            require_https: false,
            minimum_tls_version: Some(tls::Version::TLS_1_2),
            prefer_http2: true,
            force_http2_only: false,
            async_dns: true,
        }
    }

    pub fn build_client(self) -> Result<Client, ConfigError> {
        let mut builder = Client::builder()
            .timeout(self.request_timeout)
            .connect_timeout(self.connection_timeout)
            .pool_max_idle_per_host(self.max_connections)
            .pool_idle_timeout(self.keep_connections_for)
            .user_agent(&self.user_agent)
            .gzip(self.compress_requests)
            .brotli(self.compress_requests)
            .deflate(self.compress_requests)
            .referer(self.send_referer)
            .cookie_store(self.save_cookies);

        // TCP keepalive
        if let Some(keepalive) = self.detect_dead_connections {
            builder = builder.tcp_keepalive(Some(keepalive));
        }

        // Redirects
        builder = if self.follow_redirects == 0 {
            builder.redirect(reqwest::redirect::Policy::none())
        } else {
            builder.redirect(reqwest::redirect::Policy::limited(
                self.follow_redirects as usize,
            ))
        };

        if self.prefer_http2 && !self.force_http2_only {
            builder = builder.http2_adaptive_window(true);
        }

        // HTTP/2 settings
        if self.force_http2_only {
            builder = builder.http2_prior_knowledge();
        }

        // TLS settings
        if let Some(tls_ver) = &self.minimum_tls_version {
            builder = builder.min_tls_version(*tls_ver);
        }

        // TLS settings
        if self.allow_invalid_certificates {
            builder = builder.danger_accept_invalid_certs(true);
        }
        if self.allow_wrong_hostnames {
            builder = builder.danger_accept_invalid_hostnames(true);
        }

        if self.async_dns {
            builder = builder.hickory_dns(true);
        }

        // Proxy
        if let Some(proxy_url) = &self.proxy_url {
            let proxy = Proxy::all(proxy_url).map_err(|e| ConfigError::InvalidProxyUrl {
                url: proxy_url.to_string(),
                reason: e.to_string(),
                source: e,
            })?;
            builder = builder.proxy(proxy);
        }

        // Custom certificates
        for cert in &self.custom_certificates {
            builder = builder.add_root_certificate(cert.clone());
        }

        // Default headers
        if !self.default_headers.is_empty() {
            builder = builder.default_headers(self.default_headers.clone());
        }

        builder.build().map_err(|e| ConfigError::ClientBuildFailed {
            reason: e.to_string(),
            source: e,
        })
    }
}
