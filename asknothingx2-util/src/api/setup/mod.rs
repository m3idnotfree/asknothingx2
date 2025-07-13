mod registry;

use std::sync::OnceLock;

use registry::ClientRegistry;
use tracing::info;

use crate::api::app_type::AppType;

use super::{Config, Error};

static GLOBAL_REGISTRY: OnceLock<ClientRegistry> = OnceLock::new();

#[inline]
pub fn registry() -> &'static ClientRegistry {
    GLOBAL_REGISTRY.get_or_init(ClientRegistry::new)
}

pub fn clients() -> MultiClientBuilder {
    MultiClientBuilder
}

pub async fn web() -> Result<(), Error> {
    clients().add_web_app().await?;
    Ok(())
}

pub async fn production() -> Result<(), Error> {
    clients().add_production().await?;
    Ok(())
}

pub async fn development() -> Result<(), Error> {
    clients().add_development().await?;
    Ok(())
}

pub async fn cli() -> Result<(), Error> {
    clients().add_cli().await?;
    Ok(())
}

pub async fn scraping() -> Result<(), Error> {
    clients().add_scraping().await?;
    Ok(())
}

pub struct MultiClientBuilder;

impl MultiClientBuilder {
    pub async fn set_default(self, name: AppType) -> Result<Self, Error> {
        registry().set_default(name).await?;
        Ok(self)
    }

    pub async fn add_web_app(self) -> Result<Self, Error> {
        let web = AppType::WEB;
        let config = Config::for_web_apps();

        let client = config
            .clone()
            .build_client()
            .map_err(|e| e.with_input("web app configuration"))?;

        registry().register(web, client, config).await?;
        info!("Added web application client");

        Ok(self)
    }

    pub async fn add_production(self) -> Result<Self, Error> {
        let prod = AppType::PRODUCTION;
        let config = Config::for_production();

        let client = config
            .clone()
            .build_client()
            .map_err(|e| e.with_input("production configuration"))?;

        registry().register(prod, client, config).await?;

        info!("Added production API client");
        Ok(self)
    }

    pub async fn add_development(self) -> Result<Self, Error> {
        let dev = AppType::DEVELOPMENT;
        let config = Config::for_development();

        let client = config
            .clone()
            .build_client()
            .map_err(|e| e.with_input("development configuration"))?;

        registry().register(dev, client, config).await?;

        info!("Added development client");
        Ok(self)
    }

    pub async fn add_cli(self) -> Result<Self, Error> {
        let cli = AppType::CLI;
        let config = Config::for_cli_tools();

        let client = config
            .clone()
            .build_client()
            .map_err(|e| e.with_input("CLI configuration"))?;

        registry().register(cli, client, config).await?;

        info!("Added CLI tools client");
        Ok(self)
    }

    pub async fn add_gateway(self) -> Result<Self, Error> {
        let gateway = AppType::GATEWAY;
        let config = Config::for_api_gateway();

        let client = config
            .clone()
            .build_client()
            .map_err(|e| e.with_input("gateway configuration"))?;

        registry().register(gateway, client, config).await?;

        info!("Added API gateway client");
        Ok(self)
    }

    pub async fn add_scraping(self) -> Result<Self, Error> {
        let scraping = AppType::SCRAPING;
        let config = Config::for_web_scraping();

        let client = config
            .clone()
            .build_client()
            .map_err(|e| e.with_input("scraping configuration"))?;

        registry().register(scraping, client, config).await?;

        info!("Added web scraping client");
        Ok(self)
    }

    pub async fn add_custom(self, name: AppType, config: Config) -> Result<Self, Error> {
        let client = config
            .clone()
            .build_client()
            .map_err(|e| e.with_input(format!("custom '{name}' configuration")))?;

        registry().register(name, client, config).await?;

        Ok(self)
    }
}
