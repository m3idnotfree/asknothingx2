use std::sync::{Arc, OnceLock};

use reqwest::Client;
use tokio::sync::RwLock;
use tracing::{debug, warn};

use crate::api::{
    app_type::{AppType, AppTypeMap, InvalidAppType},
    error, Config, Error,
};

#[derive(Debug)]
pub struct ClientRegistry {
    inner: RwLock<RegistryInner>,
    default_client: OnceLock<Arc<Client>>,
}

#[derive(Debug)]
struct RegistryInner {
    clients: AppTypeMap<Arc<Client>>,
    configs: AppTypeMap<Config>,
    default_name: Option<AppType>,
}

impl ClientRegistry {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(RegistryInner {
                clients: AppTypeMap::new(),
                configs: AppTypeMap::new(),
                default_name: None,
            }),
            default_client: OnceLock::new(),
        }
    }

    pub async fn register(
        &self,
        name: AppType,
        client: Client,
        config: Config,
    ) -> Result<(), Error> {
        let mut inner = self.inner.write().await;

        if inner.clients.contains_key(&name) {
            return Err(Error::from(InvalidAppType::reserved(name.as_str())));
        }

        debug!("Registering client: {}", name);

        if inner.default_name.is_none() {
            inner.default_name = Some(name.clone());
        }

        inner.clients.insert(name.clone(), Arc::new(client));
        inner.configs.insert(name, config);

        Ok(())
    }

    pub async fn set_default(&self, name: AppType) -> Result<(), Error> {
        let mut inner = self.inner.write().await;

        if !inner.clients.contains_key(&name) {
            return Err(error::registry::client_not_found(name.as_str()));
        }

        inner.default_name = Some(name);

        Ok(())
    }

    pub async fn register_default(
        &self,
        name: AppType,
        client: Client,
        config: Config,
    ) -> Result<(), Error> {
        self.register(name.clone(), client, config).await?;
        self.set_default(name).await?;
        Ok(())
    }

    pub async fn get(&self, name: &AppType) -> Option<Arc<Client>> {
        let inner = self.inner.read().await;
        inner.clients.get(name).cloned()
    }

    pub async fn get_or_default(&self, name: &AppType) -> Arc<Client> {
        if let Some(client) = self.get(name).await {
            return client;
        }

        debug!("Client '{}' not found, falling back to default", name);
        self.get_default().await
    }

    pub async fn get_default(&self) -> Arc<Client> {
        if let Some(client) = self.default_client.get() {
            return client.clone();
        }

        let client = {
            let inner = self.inner.read().await;

            if let Some(default_name) = inner.default_name.clone() {
                inner.clients.get(&default_name).cloned()
            } else {
                None
            }
        };

        match client {
            Some(client) => {
                let _ = self.default_client.set(client.clone());
                client
            }
            None => {
                warn!("No default client configured, creating fallback web client");

                let fallback = Config::for_web_apps()
                    .build_client()
                    .expect("Failed to create fallback client");
                let fallback = Arc::new(fallback);

                let _ = self.default_client.set(fallback.clone());
                fallback
            }
        }
    }

    pub async fn get_default_name(&self) -> Option<AppType> {
        let inner = self.inner.read().await;
        inner.default_name.clone()
    }

    pub async fn exists(&self, name: AppType) -> bool {
        let inner = self.inner.read().await;
        inner.clients.contains_key(&name)
    }

    pub async fn remove(&self, name: AppType) -> Result<(), Error> {
        let mut inner = self.inner.write().await;

        if !inner.clients.contains_key(&name) {
            return Err(error::registry::client_not_found(name.as_str()));
        }

        inner.clients.remove(&name);
        inner.configs.remove(&name);

        if inner.default_name == Some(name) {
            inner.default_name = None;
        }

        Ok(())
    }

    pub async fn count(&self) -> usize {
        let inner = self.inner.read().await;
        inner.clients.len()
    }

    pub async fn list_names(&self) -> Vec<AppType> {
        let inner = self.inner.read().await;
        inner.clients.keys().cloned().collect()
    }

    pub async fn get_config(&self, name: AppType) -> Result<Config, Error> {
        let inner = self.inner.read().await;

        inner
            .configs
            .get(&name)
            .cloned()
            .ok_or_else(|| error::registry::client_not_found(name.as_str()))
    }

    pub async fn update_config(&self, name: AppType, config: Config) -> Result<(), Error> {
        let mut inner = self.inner.write().await;

        if !inner.clients.contains_key(&name) {
            return Err(error::registry::client_not_found(name.as_str()));
        }

        let new_client = config.clone().build_client()?;

        inner.clients.insert(name.clone(), Arc::new(new_client));
        inner.configs.insert(name, config);

        Ok(())
    }

    pub async fn clear(&self) {
        let mut inner = self.inner.write().await;
        inner.clients.clear();
        inner.configs.clear();
        inner.default_name = None;
    }
}

impl Default for ClientRegistry {
    fn default() -> Self {
        Self::new()
    }
}
