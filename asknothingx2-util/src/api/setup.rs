use std::{
    sync::{Arc, OnceLock, RwLock},
    time::Duration,
};

use reqwest::Client;
use tracing::{debug, info, warn};

use super::{error::ConfigError, AppType, Config};

static GLOBAL_CLIENT: OnceLock<Client> = OnceLock::new();
static GLOBAL_CONFIG: OnceLock<Arc<RwLock<Config>>> = OnceLock::new();
static GLOBAL_APP_TYPE: OnceLock<AppType> = OnceLock::new();

pub fn get_global_client_or_default() -> &'static Client {
    GLOBAL_CLIENT.get_or_init(|| {
        warn!(
            client_type = "default_web_app",
            suggestion = "setup::web_apps()",
            "HTTP client auto-initialized with default configuration"
        );

        Config::for_web_apps()
            .build_client()
            .expect("Failed to create default HTTP client")
    })
}

fn initialize_global_client(config: Config) -> Result<(), ConfigError> {
    let client = config.clone().build_client()?;
    let app_type = config.app_type;

    GLOBAL_CLIENT
        .set(client)
        .map_err(|_| ConfigError::AlreadyConfigured {
            message: "HTTP client has already been configured".to_string(),
        })?;

    GLOBAL_APP_TYPE
        .set(config.app_type)
        .map_err(|_| ConfigError::AlreadyConfigured {
            message: "App type has already been set".to_string(),
        })?;

    let config_arc = Arc::new(RwLock::new(config));
    GLOBAL_CONFIG
        .set(config_arc)
        .map_err(|_| ConfigError::AlreadyConfigured {
            message: "Configuration has already been set".to_string(),
        })?;

    info!("HTTP client initialized for app type: {:?}", app_type);
    Ok(())
}

fn detect_app_type_from_env() -> AppType {
    match std::env::var("APP_TYPE")
        .unwrap_or_else(|_| "web".to_string())
        .to_lowercase()
        .as_str()
    {
        "cli" | "command" | "script" => AppType::Cli,
        "web" | "api" | "service" => AppType::Web,
        "production" | "prod" | "enterprise" => AppType::Production,
        "development" | "dev" | "test" | "testing" => AppType::Development,
        "gateway" | "proxy" | "loadbalancer" => AppType::Gateway,
        "scraping" | "crawler" | "scraper" => AppType::Scraping,
        _ => AppType::Web,
    }
}

fn apply_env_overrides(config: &mut Config) {
    if let Ok(timeout) = std::env::var("HTTP_TIMEOUT_SECONDS") {
        if let Ok(seconds) = timeout.parse::<u64>() {
            config.request_timeout = Duration::from_secs(seconds);
        }
    }

    if let Ok(proxy) = std::env::var("HTTP_PROXY") {
        debug!("Applied HTTP proxy override: {}", proxy);
        config.proxy_url = Some(proxy);
    }

    if let Ok(user_agent) = std::env::var("HTTP_USER_AGENT") {
        debug!("Applied user agent override: {}", user_agent);
        config.user_agent = user_agent;
    }
}

pub fn is_configured() -> bool {
    GLOBAL_CLIENT.get().is_some()
}

pub fn current_app_type() -> Option<AppType> {
    GLOBAL_APP_TYPE.get().copied()
}

pub fn current_configuration() -> Option<Config> {
    Some(GLOBAL_CONFIG.get()?.read().ok()?.clone())
}

pub fn update_global_config<F>(updater: F) -> Result<(), ConfigError>
where
    F: FnOnce(&mut Config),
{
    let config_arc = GLOBAL_CONFIG.get().ok_or(ConfigError::NotInitialized)?;
    let mut config = config_arc
        .write()
        .map_err(|_| ConfigError::ConfigurationLocked)?;
    updater(&mut config);
    debug!("Configuration updated successfully");
    Ok(())
}

pub fn app_type(app_type: AppType) -> Result<(), ConfigError> {
    match app_type {
        AppType::Cli => cli_tools(),
        AppType::Web => web_apps(),
        AppType::Production => production(),
        AppType::Development => development(),
        AppType::Gateway => api_gateway(),
        AppType::Scraping => web_scraping(),
    }
}

pub fn cli_tools() -> Result<(), ConfigError> {
    let config = Config::for_cli_tools();
    initialize_global_client(config)
}

pub fn web_apps() -> Result<(), ConfigError> {
    let config = Config::for_web_apps();
    initialize_global_client(config)
}

pub fn production() -> Result<(), ConfigError> {
    let config = Config::for_production();
    initialize_global_client(config)
}

pub fn development() -> Result<(), ConfigError> {
    let config = Config::for_development();
    initialize_global_client(config)
}

pub fn api_gateway() -> Result<(), ConfigError> {
    let config = Config::for_api_gateway();
    initialize_global_client(config)
}

pub fn web_scraping() -> Result<(), ConfigError> {
    let config = Config::for_web_scraping();
    initialize_global_client(config)
}

pub fn automatically() -> Result<(), ConfigError> {
    let app_type = detect_app_type_from_env();

    let mut config = match app_type {
        AppType::Cli => Config::for_cli_tools(),
        AppType::Web => Config::for_web_apps(),
        AppType::Production => Config::for_production(),
        AppType::Development => Config::for_development(),
        AppType::Gateway => Config::for_api_gateway(),
        AppType::Scraping => Config::for_web_scraping(),
    };

    apply_env_overrides(&mut config);
    initialize_global_client(config)
}
