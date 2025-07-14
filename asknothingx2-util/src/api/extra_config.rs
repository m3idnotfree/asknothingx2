pub mod minimal {
    use std::time::Duration;

    use crate::api::{app_type::AppType, Config};

    pub const TYPE: AppType = AppType::from_static("minimal");
    pub fn config() -> Config {
        let mut config = Config::for_cli_tools();

        config.max_connections = 1;
        config.keep_connections_for = Duration::from_secs(10);
        config.request_timeout = Duration::from_secs(30);
        config.connection_timeout = Duration::from_secs(5);
        config.prefer_http2 = false;
        config.force_http2_only = false;
        config.async_dns = false;
        config.save_cookies = false;

        config
    }
}

pub mod minimal_web {
    use std::time::Duration;

    use crate::api::{app_type::AppType, Config};

    pub const TYPE: AppType = AppType::from_static("minimal-web");
    pub fn config() -> Config {
        let mut config = Config::for_web_apps();

        config.max_connections = 2;
        config.keep_connections_for = Duration::from_secs(30);
        config.request_timeout = Duration::from_secs(15);
        config.connection_timeout = Duration::from_secs(3);
        config.detect_dead_connections = Some(Duration::from_secs(30));
        config.prefer_http2 = false;
        config.async_dns = false;

        config
    }
}

pub mod minimal_production {
    use std::time::Duration;

    use crate::api::{app_type::AppType, Config};

    pub const TYPE: AppType = AppType::from_static("minimal-production");
    pub fn config() -> Config {
        let mut config = Config::for_web_apps();

        config.max_connections = 2;
        config.keep_connections_for = Duration::from_secs(30);
        config.request_timeout = Duration::from_secs(15);
        config.connection_timeout = Duration::from_secs(3);
        config.detect_dead_connections = Some(Duration::from_secs(30));
        config.prefer_http2 = false;
        config.async_dns = false;

        config
    }
}

#[cfg(test)]
mod tests {
    use super::{minimal, minimal_production, minimal_web};

    #[test]
    fn build() {
        minimal::config().build_client().unwrap();
        minimal_web::config().build_client().unwrap();
        minimal_production::config().build_client().unwrap();
    }
}
