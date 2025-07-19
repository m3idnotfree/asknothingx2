mod extra_config;

pub use extra_config::{Http2Spec, SecurityConfig};

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
    http2_config: Option<Http2Spec>,

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

    pub fn timeouts(mut self, timeout: Duration, connect_timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self.connect_timeout = connect_timeout;
        self
    }

    pub fn connections(mut self, max: usize, pool_idle_timeout: Duration) -> Self {
        self.pool_max_idle_per_host = max;
        self.pool_idle_timeout = pool_idle_timeout;
        self
    }

    pub fn keepalive(mut self, val: Option<Duration>) -> Self {
        self.tcp_keepalive = val;
        self
    }

    pub fn tcp_delay(mut self) -> Self {
        self.tcp_nodelay = false;
        self
    }

    pub fn min_tls(mut self, version: tls::Version) -> Self {
        self.minimum_tls_version = Some(version);
        self
    }

    pub fn debug_mode(mut self, invalid_certificates: bool, wrong_hostnames: bool) -> Self {
        self.allow_invalid_certificates = invalid_certificates;
        self.allow_wrong_hostnames = wrong_hostnames;
        self
    }

    pub fn http2(mut self, prior: bool, config: Option<Http2Spec>) -> Self {
        self.http2_prior_knowledge = prior;
        self.http2_config = config;
        self
    }

    pub fn disable_https_only(mut self) -> Self {
        self.https_only = false;
        self
    }

    pub fn redirect(mut self, policy: Policy) -> Self {
        self.redirect = policy;
        self
    }

    pub fn security(mut self, config: SecurityConfig) -> Self {
        self.save_cookies = config.save_cookies;
        self.send_referer = config.send_referer;

        self.minimum_tls_version = config.min_tls_version;

        self.redirect = config.redirect;
        self
    }

    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    pub fn default_headers<F>(self, f: F) -> Result<Self, Error>
    where
        F: FnOnce(&mut HeaderMut<'_>) -> Result<(), Error>,
    {
        let mut headers = self.default_headers.clone();
        let mut builder = HeaderMut::new(&mut headers);

        f(&mut builder)?;

        Ok(self)
    }

    pub fn proxy(mut self, proxy: Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }

    pub fn compressions(mut self, gzip: bool, brotli: bool) -> Self {
        self.gzip = gzip;
        self.brotli = brotli;
        self
    }
}

impl Preset {
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

pub fn for_rest_api(user_agent: &str) -> Preset {
    Preset::default()
        .timeouts(Duration::from_secs(30), Duration::from_secs(10))
        .connections(20, Duration::from_secs(90))
        .security(SecurityConfig::strict_1_2().redirect(Policy::limited(5)))
        .compressions(true, true)
        .user_agent(user_agent)
        .default_headers(|h| {
            h.accept_json().accept_encoding_standard();
            Ok(())
        })
        .unwrap()
}

pub fn for_auth(user_agent: &str) -> Preset {
    Preset::default()
        .timeouts(Duration::from_secs(60), Duration::from_secs(10))
        .connections(30, Duration::from_secs(90))
        .security(SecurityConfig::strict_1_2().redirect(Policy::limited(5)))
        .http2(true, Some(Http2Spec::default()))
        .user_agent(user_agent)
        .default_headers(|h| {
            h.accept_json().cache_control_no_cache();
            Ok(())
        })
        .unwrap()
}

pub fn for_real_time(user_agent: &str) -> Preset {
    Preset::default()
        .timeouts(Duration::from_secs(3), Duration::from_millis(500))
        .connections(100, Duration::from_secs(180))
        .security(SecurityConfig::strict_1_2().redirect(Policy::none()))
        .http2(true, Some(Http2Spec::new(65_536, 1_048_576, 16_384, false)))
        .compressions(false, false)
        .user_agent(user_agent)
        .default_headers(|h| {
            h.accept_json().cache_control_no_cache();
            Ok(())
        })
        .unwrap()
}

pub fn for_test(user_agent: &str) -> Preset {
    Preset::default()
        .timeouts(Duration::from_secs(10), Duration::from_secs(3))
        .connections(1, Duration::from_secs(5))
        .security(SecurityConfig::test())
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

pub fn for_debug(user_agent: &str) -> Preset {
    Preset::default()
        .timeouts(Duration::from_secs(300), Duration::from_secs(30))
        .connections(1, Duration::from_secs(60))
        .security(SecurityConfig::debug())
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
        for_rest_api("rest-api/1.0").build_client().unwrap();
        for_auth("auth/1.0").build_client().unwrap();
        for_real_time("real-time/1.0").build_client().unwrap();
        for_test("test/1.0").build_client().unwrap();
        for_debug("debug/1.0").build_client().unwrap();
    }
}
