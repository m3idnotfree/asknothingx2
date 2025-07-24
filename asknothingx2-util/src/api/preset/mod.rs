mod extra_config;

pub use extra_config::{Http2Settings, SecurityProfile};

use std::time::Duration;

use http::HeaderMap;
use reqwest::{
    redirect::{self, Policy},
    tls, Client, Proxy,
};

use super::{
    error::{self, Error},
    HeaderMut,
};

mod user_agents {
    pub const DEFAULT: &str = "asknothingx2/1.0";
}

/// HTTP client configuration preset with sensible defaults for various use cases.
///
/// **Default Configuration:**
/// - Request timeout: 30s, Connect timeout: 10s
/// - Connections: 20 max per host, 90s idle timeout
/// - TLS: 1.2+ minimum, strict validation, HTTPS-only
/// - Redirects: Up to 5 allowed
/// - Cookies: Not saved, Referer: Sent
/// - Compression: gzip enabled, brotli disabled
/// - User-Agent: "asknothingx2/1.0"
/// - HTTP/2: Auto-negotiated (supports both HTTP/1.1 and HTTP/2)
/// - TCP keep-alive: Disabled
///
/// # Predefined Presets
///
/// Choose from these optimized configurations for common use cases:
///
/// - [`rest_api()`] - Balanced performance for REST API consumption (auto-negotiated HTTP/1.1 or HTTP/2)
/// - [`authentication()`] - Secure settings for auth flows (HTTP/2 required)
/// - [`low_latency()`] - Ultra-fast for time-sensitive operations (HTTP/2 required)
/// - [`testing()`] - Permissive settings for test environments
/// - [`debugging()`] - Maximum compatibility for development
///
/// # Examples
///
/// ```rust
/// use crate::preset::{Preset, rest_api};
///
/// // Using a predefined preset
/// let client = rest_api("MyApp/1.0").build_client()?;
///
/// // Customizing with builder methods
/// let client = Preset::new()
///     .user_agent("MyApp/1.0")
///     .timeouts(Duration::from_secs(60), Duration::from_secs(5))
///     .http2(true, None)
///     .build_client()?;
/// ```
///
/// # Security Notes
///
/// See [`SecurityConfig`] for security-related configuration options and
/// [`debug_mode()`](Self::debug_mode) for important warnings about insecure settings.
#[derive(Debug)]
pub struct Preset {
    request_timeout: Duration,
    connect_timeout: Duration,

    pool_max_idle_per_host: usize,
    pool_idle_timeout: Duration,
    tcp_keepalive: Option<Duration>,
    tcp_nodelay: bool,

    minimum_tls_version: Option<tls::Version>,

    allow_invalid_certificates: bool,
    allow_wrong_hostnames: bool,
    tls_sni: bool,

    http2_prior_knowledge: bool,
    http2_config: Option<Http2Settings>,

    https_only: bool,

    redirect: redirect::Policy,
    save_cookies: bool,
    send_referer: bool,

    gzip: bool,
    brotli: bool,

    default_headers: HeaderMap,
    user_agent: String,
    proxy: Option<Proxy>,
}

impl Default for Preset {
    fn default() -> Self {
        Self {
            request_timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            pool_max_idle_per_host: 20,
            pool_idle_timeout: Duration::from_secs(90),
            tcp_keepalive: None,
            tcp_nodelay: true,
            minimum_tls_version: Some(tls::Version::TLS_1_2),

            allow_invalid_certificates: false,
            allow_wrong_hostnames: false,
            tls_sni: true,

            http2_prior_knowledge: false,
            http2_config: None,

            https_only: true,

            redirect: Policy::limited(5),
            save_cookies: false,
            send_referer: true,

            gzip: true,
            brotli: false,

            default_headers: HeaderMap::new(),
            user_agent: user_agents::DEFAULT.to_string(),
            proxy: None,
        }
    }
}

impl Preset {
    pub fn new() -> Self {
        Preset::default()
    }
    /// Configure request and connection timeout durations.
    ///
    /// - `timeout`: Maximum time to wait for complete request (including response)
    /// - `connect_timeout`: Maximum time to wait for initial connection establishment
    ///
    /// Use shorter timeouts for low-latency services, longer for slow/unreliable endpoints.
    ///
    pub fn timeouts(mut self, timeout: Duration, connect_timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self.connect_timeout = connect_timeout;
        self
    }
    /// Configure connection pool settings for HTTP keep-alive optimization.
    ///
    /// - `max`: Maximum idle connections to keep per host (higher = more reuse, more memory)
    /// - `pool_idle_timeout`: How long to keep idle connections before closing
    ///
    /// Increase `max` for high-throughput scenarios, decrease for memory-constrained environments.
    ///
    pub fn connections(mut self, max: usize, pool_idle_timeout: Duration) -> Self {
        self.pool_max_idle_per_host = max;
        self.pool_idle_timeout = pool_idle_timeout;
        self
    }
    /// Configure TCP keep-alive to detect dead connections.
    ///
    /// - `Some(duration)`: Send keep-alive probes every `duration` to detect dead connections
    /// - `None`: Disable keep-alive (connections may hang on network issues)
    ///
    /// Enable for long-lived connections, disable for short-lived or high-churn scenarios.
    ///
    pub fn keepalive(mut self, val: Option<Duration>) -> Self {
        self.tcp_keepalive = val;
        self
    }

    /// Enable TCP Nagle's algorithm (default: disabled for lower latency).
    ///
    /// Nagle's algorithm batches small packets to improve network efficiency but increases latency.
    /// Call this method only if you prioritize bandwidth over response time.
    ///
    pub fn tcp_delay(mut self) -> Self {
        self.tcp_nodelay = false;
        self
    }

    /// Set minimum required TLS version for secure connections.
    pub fn min_tls(mut self, version: tls::Version) -> Self {
        self.minimum_tls_version = Some(version);
        self
    }

    /// Enable insecure TLS settings for testing/debugging (NEVER use in production).
    ///
    /// - `invalid_certificates`: Accept self-signed or expired certificates
    /// - `wrong_hostnames`: Accept certificates for different hostnames
    ///
    pub fn debug_mode(mut self, invalid_certificates: bool, wrong_hostnames: bool) -> Self {
        self.allow_invalid_certificates = invalid_certificates;
        self.allow_wrong_hostnames = wrong_hostnames;
        self
    }

    /// Configure HTTP/2 protocol settings for improved performance.
    ///
    /// - `prior`: Skip protocol negotiation and use HTTP/2 only (faster but HTTP/2-only)
    /// - [`config`](Http2Settings): Custom HTTP/2 tuning parameters (None = use defaults)
    ///
    /// **Important:** When `prior = true`, HTTP/1.1 is NOT supported - connection fails if
    /// server doesn't support HTTP/2. When `prior = false` (default), both HTTP/1.1 and
    /// HTTP/2 are supported via protocol negotiation.
    ///
    pub fn http2(mut self, prior: bool, config: Option<Http2Settings>) -> Self {
        self.http2_prior_knowledge = prior;
        self.http2_config = config;
        self
    }

    /// Allow HTTP connections in addition to HTTPS (reduces security).
    ///
    /// By default, only HTTPS is allowed. Call this method to also allow unencrypted HTTP.
    /// Only use for testing, debugging, or when forced by legacy systems.
    ///
    pub fn disable_https_only(mut self) -> Self {
        self.https_only = false;
        self
    }

    /// Configure automatic redirect following behavior.
    pub fn redirect(mut self, policy: Policy) -> Self {
        self.redirect = policy;
        self
    }

    /// Apply a security configuration preset that sets multiple security-related options.
    pub fn security(mut self, config: SecurityProfile) -> Self {
        self.save_cookies = config.save_cookies;
        self.send_referer = config.send_referer;

        self.minimum_tls_version = config.min_tls_version;

        self.redirect = config.redirect;
        self
    }

    /// Set the User-Agent header sent with all requests.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Configure default headers sent with every request.
    pub fn default_headers<F>(self, f: F) -> Result<Self, Error>
    where
        F: FnOnce(&mut HeaderMut<'_>) -> Result<(), Error>,
    {
        let mut headers = self.default_headers.clone();
        let mut builder = HeaderMut::new(&mut headers);

        f(&mut builder)?;

        Ok(self)
    }
    /// Configure HTTP proxy for all requests through this client.
    pub fn proxy(mut self, proxy: Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
    /// Configure automatic response compression handling.
    ///
    /// - `gzip`
    /// - `brotli`
    ///
    pub fn compressions(mut self, gzip: bool, brotli: bool) -> Self {
        self.gzip = gzip;
        self.brotli = brotli;
        self
    }
}

impl Preset {
    /// Build the configured [`reqwest::Client`] from this preset.
    pub fn build_client(self) -> Result<Client, Error> {
        let mut builder = Client::builder()
            .timeout(self.request_timeout)
            .connect_timeout(self.connect_timeout)
            .pool_max_idle_per_host(self.pool_max_idle_per_host)
            .pool_idle_timeout(self.pool_idle_timeout)
            .tcp_keepalive(self.tcp_keepalive)
            .tcp_nodelay(self.tcp_nodelay)
            .danger_accept_invalid_certs(self.allow_invalid_certificates)
            .danger_accept_invalid_hostnames(self.allow_wrong_hostnames)
            .tls_sni(self.tls_sni)
            .redirect(self.redirect)
            .cookie_store(self.save_cookies)
            .referer(self.send_referer)
            .gzip(self.gzip)
            .brotli(self.brotli)
            .user_agent(&self.user_agent)
            .default_headers(self.default_headers)
            .https_only(self.https_only)
            .use_rustls_tls();

        if let Some(version) = self.minimum_tls_version {
            builder = builder.min_tls_version(version)
        }

        if self.http2_prior_knowledge {
            builder = builder.http2_prior_knowledge();
        }

        if let Some(config) = self.http2_config {
            builder = builder
                .http2_initial_stream_window_size(config.initial_stream_window_size)
                .http2_initial_connection_window_size(config.initial_connection_window_size)
                .http2_max_frame_size(config.max_frame_size)
                .http2_adaptive_window(config.adaptive_window);
        }

        if let Some(proxy) = self.proxy {
            builder = builder.proxy(proxy);
        }

        builder.build().map_err(error::request::build)
    }
}

/// Creates a basic HTTP client with standard defaults suitable for general-purpose requests.
///
/// **Configuration:**
/// - Request timeout: 30s, Connect timeout: 10s
/// - Connections: 20 max per host, 90s idle timeout
/// - TLS: 1.2+ minimum, strict validation
/// - HTTPS: Enforced (HTTP blocked)
/// - HTTP/2: Auto-negotiated (supports both HTTP/1.1 and HTTP/2)
/// - Redirects: Up to 5 allowed
/// - Cookies: Not saved, Referer: Sent
/// - Compression: gzip enabled, brotli disabled
pub fn default(user_agent: &str) -> Preset {
    Preset::default().user_agent(user_agent)
}

/// Creates an HTTP client optimized for REST API consumption with balanced performance
/// and reliability.
///
/// **Configuration:**
/// - Request timeout: 30s, Connect timeout: 10s
/// - Connections: 20 max per host, 90s idle timeout
/// - TLS: 1.2+ minimum, strict validation
/// - HTTPS: Enforced (HTTP blocked)
/// - HTTP/2: Auto-negotiated (supports both HTTP/1.1 and HTTP/2)
/// - Redirects: Up to 5 allowed
/// - Cookies: Not saved, Referer: Not sent
/// - Compression: gzip + brotli enabled
/// - Headers: Accept JSON, standard encoding
pub fn rest_api(user_agent: &str) -> Preset {
    Preset::default()
        .compressions(true, true)
        .user_agent(user_agent)
        .default_headers(|h| {
            h.accept_json().accept_encoding_standard();
            Ok(())
        })
        .unwrap()
}

/// Creates an HTTP client configured for authentication flows and secure operations.
///
/// **Configuration:**
/// - Request timeout: 60s, Connect timeout: 10s
/// - Connections: 30 max per host, 90s idle timeout
/// - TLS: 1.2+ minimum, strict validation
/// - HTTPS: Enforced (HTTP blocked)
/// - HTTP/2: Required (no HTTP/1.1 fallback)
/// - Redirects: Up to 5 allowed
/// - Cookies: Not saved, Referer: Not sent (strict security)
/// - Headers: Accept JSON, no-cache control
pub fn authentication(user_agent: &str) -> Preset {
    Preset::default()
        .timeouts(Duration::from_secs(60), Duration::from_secs(10))
        .connections(30, Duration::from_secs(90))
        .http2(true, Some(Http2Settings::default()))
        .user_agent(user_agent)
        .default_headers(|h| {
            h.accept_json().cache_control_no_cache();
            Ok(())
        })
        .unwrap()
}

/// Creates an HTTP client optimized for ultra-low latency and time-sensitive operations.
///
/// **Configuration:**
/// - Request timeout: 3s, Connect timeout: 500ms
/// - Connections: 100 max per host, 180s idle timeout
/// - TLS: 1.2+ minimum, strict validation
/// - HTTPS: Enforced (HTTP blocked)
/// - HTTP/2: Required with custom tuning (no HTTP/1.1 fallback)
/// - Redirects: Disabled
/// - Cookies: Not saved, Referer: Not sent (strict security)
/// - Compression: Disabled for minimal overhead
/// - Headers: Accept JSON, no-cache control
pub fn low_latency(user_agent: &str) -> Preset {
    Preset::default()
        .timeouts(Duration::from_secs(3), Duration::from_millis(500))
        .connections(100, Duration::from_secs(180))
        .security(SecurityProfile::strict_1_2().redirect(Policy::none()))
        .http2(
            true,
            Some(Http2Settings::new(65_536, 1_048_576, 16_384, false)),
        )
        .compressions(false, false)
        .user_agent(user_agent)
        .default_headers(|h| {
            h.accept_json().cache_control_no_cache();
            Ok(())
        })
        .unwrap()
}

/// Creates an HTTP client with permissive settings for testing environments.
///
/// **Configuration:**
/// - Request timeout: 10s, Connect timeout: 3s
/// - Connections: 1 max per host, 5s idle timeout
/// - TLS: Test security config, accepts invalid certificates
/// - HTTPS: Not enforced (HTTP allowed)
/// - Cookies: Saved, Referer: Sent (permissive for testing)
/// - Debug mode: Accepts invalid certs and wrong hostnames
/// - Headers: Accept JSON
pub fn testing(user_agent: &str) -> Preset {
    Preset::default()
        .timeouts(Duration::from_secs(10), Duration::from_secs(3))
        .connections(1, Duration::from_secs(5))
        .security(SecurityProfile::test())
        .http2(false, None)
        .user_agent(user_agent)
        .default_headers(|h| {
            h.accept_json();

            Ok(())
        })
        .unwrap()
        .disable_https_only()
        .debug_mode(true, true)
}

/// Creates an HTTP client with maximum compatibility for debugging and development.
///
/// **Configuration:**
/// - Request timeout: 300s (5min), Connect timeout: 30s
/// - Connections: 1 max per host, 60s idle timeout
/// - TLS: Debug security config, all validation disabled
/// - HTTPS: Not enforced (HTTP allowed)
/// - Cookies: Saved, Referer: Sent (maximum compatibility)
/// - Debug mode: Accepts invalid certs and wrong hostnames
/// - Headers: Accept any content type, no-cache control
pub fn debugging(user_agent: &str) -> Preset {
    Preset::default()
        .timeouts(Duration::from_secs(300), Duration::from_secs(30))
        .connections(1, Duration::from_secs(60))
        .security(SecurityProfile::debug())
        .http2(false, None)
        .user_agent(user_agent)
        .default_headers(|h| {
            h.accept_any().cache_control_no_cache();
            Ok(())
        })
        .unwrap()
        .disable_https_only()
        .debug_mode(true, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn build() {
        rest_api("rest-api/1.0").build_client().unwrap();
        authentication("auth/1.0").build_client().unwrap();
        low_latency("real-time/1.0").build_client().unwrap();
        testing("test/1.0").build_client().unwrap();
        debugging("debug/1.0").build_client().unwrap();
    }
}
