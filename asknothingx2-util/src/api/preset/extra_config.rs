use reqwest::{redirect::Policy, tls};

/// **Default Security Profile (strict_1_2):**
/// - Cookies: Not saved, Referer: Not sent
/// - TLS: 1.2+ minimum required
/// - Redirects: Up to 3 allowed
#[derive(Debug)]
pub struct SecurityProfile {
    pub save_cookies: bool,
    pub send_referer: bool,
    pub min_tls_version: Option<tls::Version>,
    pub redirect: Policy,
}

impl SecurityProfile {
    /// - Cookies: Not saved, Referer: Not sent
    /// - TLS: 1.3+ minimum
    /// - Redirects: Up to 3 allowed
    pub fn strict_1_3() -> Self {
        Self {
            save_cookies: false,
            send_referer: false,
            min_tls_version: Some(tls::Version::TLS_1_3),
            redirect: Policy::limited(3),
        }
    }

    /// - Cookies: Not saved, Referer: Not sent
    /// - TLS: 1.2+ minimum
    /// - Redirects: Up to 3 allowed
    pub fn strict_1_2() -> Self {
        Self {
            save_cookies: false,
            send_referer: false,
            min_tls_version: Some(tls::Version::TLS_1_2),
            redirect: Policy::limited(3),
        }
    }

    /// - Cookies: Saved, Referer: Sent
    /// - TLS: 1.2+ minimum
    /// - Redirects: Up to 10 allowed
    pub fn permissive() -> Self {
        Self {
            save_cookies: true,
            send_referer: true,
            min_tls_version: Some(tls::Version::TLS_1_2),
            redirect: Policy::limited(10),
        }
    }

    /// - Cookies: Not saved, Referer: Not sent
    /// - TLS: No minimum version
    /// - Redirects: Up to 3 allowed
    pub fn test() -> Self {
        Self {
            save_cookies: false,
            send_referer: false,
            min_tls_version: None,
            redirect: Policy::limited(3),
        }
    }

    /// - Cookies: Saved, Referer: Sent
    /// - TLS: No minimum version
    /// - Redirects: Up to 15 allowed
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

impl Default for SecurityProfile {
    fn default() -> Self {
        Self::strict_1_2()
    }
}

/// HTTP/2 protocol tuning parameters for performance optimization.
///
/// Controls flow control windows, frame sizes, and adaptive algorithms to optimize
/// HTTP/2 performance for different use cases from low-latency real-time applications
/// to high-throughput batch processing.
///
/// **Default Configuration:**
/// - Stream window: 64KB (good for most use cases)
/// - Connection window: 1MB (allows multiple concurrent streams)
/// - Frame size: 16KB (HTTP/2 default)
/// - Adaptive window: Enabled (automatically tunes flow control)
///
/// # Performance Profiles
///
/// - **Low latency**: Small windows, disabled adaptive algorithms
/// - **High throughput**: Large windows, enabled adaptive algorithms  
/// - **Real-time**: Minimal buffering, predictable flow control
/// - **Batch processing**: Maximum windows for bulk transfers
///
#[derive(Debug)]
pub struct Http2Settings {
    pub initial_stream_window_size: u32,
    pub initial_connection_window_size: u32,
    pub max_frame_size: u32,
    pub adaptive_window: bool,
}

impl Default for Http2Settings {
    fn default() -> Self {
        Self {
            initial_stream_window_size: 65_536,
            initial_connection_window_size: 1_048_576,
            max_frame_size: 16_384,
            adaptive_window: true,
        }
    }
}

impl Http2Settings {
    /// Create custom HTTP/2 tuning parameters.
    ///
    /// # Parameters
    ///
    /// - `initial_stream_window_size`: Per-stream buffer size (bytes)
    /// - `initial_connection_window_size`: Total connection buffer size (bytes)
    /// - `max_frame_size`: Maximum frame size (bytes, 16KB to 16MB)
    /// - `adaptive_window`: Enable automatic window size adjustment
    ///
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
