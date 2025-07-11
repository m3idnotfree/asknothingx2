use std::{fmt, hash::Hash};

use crate::api::Config;

pub trait AppTypeMarker:
    fmt::Debug + fmt::Display + Clone + PartialEq + Eq + Hash + Send + Sync + 'static
{
    fn name(&self) -> &'static str;
    fn config(&self) -> Config;

    fn static_name() -> &'static str
    where
        Self: Sized;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppType {
    Cli,
    Web,
    Production,
    Development,
    Gateway,
    Scraping,
}

impl AppType {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Web => "web",
            Self::Production => "production",
            Self::Gateway => "gateway",
            Self::Scraping => "scraping",
            Self::Cli => "cli",
            Self::Development => "development",
        }
    }
}

impl fmt::Display for AppType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cli => write!(f, "Cli"),
            Self::Web => write!(f, "Web"),
            Self::Production => write!(f, "Production"),
            Self::Development => write!(f, "Development"),
            Self::Gateway => write!(f, "Gateway"),
            Self::Scraping => write!(f, "Scraping"),
        }
    }
}

impl AppTypeMarker for AppType {
    fn name(&self) -> &'static str {
        self.name()
    }
    fn config(&self) -> Config {
        match self {
            Self::Cli => Config::for_web_apps(),
            Self::Web => Config::for_web_apps(),
            Self::Production => Config::for_web_apps(),
            Self::Development => Config::for_web_apps(),
            Self::Gateway => Config::for_web_apps(),
            Self::Scraping => Config::for_web_apps(),
        }
    }

    fn static_name() -> &'static str {
        "web"
    }
}

impl From<AppType> for &'static str {
    fn from(app_type: AppType) -> Self {
        app_type.name()
    }
}
