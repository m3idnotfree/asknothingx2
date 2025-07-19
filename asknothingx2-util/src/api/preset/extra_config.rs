use reqwest::{redirect::Policy, tls};

#[derive(Debug)]
pub struct SecurityConfig {
    pub save_cookies: bool,
    pub send_referer: bool,
    pub min_tls_version: Option<tls::Version>,
    pub redirect: Policy,
}

impl SecurityConfig {
    pub fn strict_1_3() -> Self {
        Self {
            save_cookies: false,
            send_referer: false,
            min_tls_version: Some(tls::Version::TLS_1_3),
            redirect: Policy::limited(3),
        }
    }

    pub fn strict_1_2() -> Self {
        Self {
            save_cookies: false,
            send_referer: false,
            min_tls_version: Some(tls::Version::TLS_1_2),
            redirect: Policy::limited(3),
        }
    }

    pub fn permissive() -> Self {
        Self {
            save_cookies: true,
            send_referer: true,
            min_tls_version: Some(tls::Version::TLS_1_2),
            redirect: Policy::limited(10),
        }
    }

    pub fn test() -> Self {
        Self {
            save_cookies: false,
            send_referer: false,
            min_tls_version: None,
            redirect: Policy::limited(3),
        }
    }

    pub fn debug() -> Self {
        Self {
            save_cookies: true,
            send_referer: true,
            min_tls_version: None,
            redirect: Policy::limited(15),
        }
    }

    pub fn redirect(mut self, policy: Policy) -> Self {
        self.redirect = policy;
        self
    }
}

#[derive(Debug)]
pub struct Http2Spec {
    pub initial_stream_window_size: u32,
    pub initial_connection_window_size: u32,
    pub max_frame_size: u32,
    pub adaptive_window: bool,
}

impl Default for Http2Spec {
    fn default() -> Self {
        Self {
            initial_stream_window_size: 65536,
            initial_connection_window_size: 1048576,
            max_frame_size: 16384,
            adaptive_window: true,
        }
    }
}

impl Http2Spec {
    pub fn new(
        initial_stream_window_size: u32,
        initial_connection_window_size: u32,
        max_frame_size: u32,
        adaptive_window: bool,
    ) -> Self {
        Self {
            initial_stream_window_size,
            initial_connection_window_size,
            max_frame_size,
            adaptive_window,
        }
    }
}
