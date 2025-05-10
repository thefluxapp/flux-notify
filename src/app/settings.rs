use std::env;

use config::{Config, ConfigError, Environment, File};
use flux_lib::settings::{DBSettings, HttpSettings, NATSSettings};
use serde::Deserialize;

use super::{events::settings::EventsSettings, pushes::settings::PushesSettings};

#[derive(Deserialize, Clone)]
pub struct AppSettings {
    pub _name: String,
    pub http: HttpSettings,
    pub nats: NATSSettings,
    pub db: DBSettings,
    pub pushes: PushesSettings,
    pub events: EventsSettings,
    pub clients: ClientsSettings,
}

#[derive(Deserialize, Clone)]
pub struct ClientsSettings {
    pub flux_users: ClientSettings,
}

#[derive(Deserialize, Clone)]
pub struct ClientSettings {
    pub endpoint: String,
}

impl AppSettings {
    pub fn new() -> Result<Self, ConfigError> {
        let app_dir = env::var("APP__DIR").unwrap_or_else(|_| "./settings".into());

        let config = Config::builder()
            .add_source(File::with_name(&format!("{}/default", app_dir)))
            .add_source(File::with_name(&format!("{}/local", app_dir)).required(false))
            .add_source(Environment::with_prefix("app").separator("__"))
            .build()?;

        config.try_deserialize()
    }
}
