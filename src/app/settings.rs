use std::env;

use config::{Config, ConfigError, Environment, File};
use flux_lib::settings::{DBSettings, HttpSettings, NATSSettings};
use serde::Deserialize;

use super::pushes::settings::PushesSettings;

#[derive(Deserialize, Clone)]
pub struct AppSettings {
    pub _name: String,
    pub http: HttpSettings,
    pub nats: NATSSettings,
    pub db: DBSettings,
    pub pushes: PushesSettings,
}

impl AppSettings {
    pub fn new() -> Result<Self, ConfigError> {
        let app_dir = env::var("APP_DIR").unwrap_or_else(|_| ".".into());

        let config = Config::builder()
            .add_source(File::with_name(&format!("{}/settings/default", app_dir)))
            .add_source(File::with_name(&format!("{}/settings/local", app_dir)).required(false))
            .add_source(Environment::with_prefix("app").separator("_"))
            .build()?;

        config.try_deserialize()
    }
}
