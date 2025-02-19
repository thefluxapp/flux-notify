use std::sync::Arc;

use anyhow::Error;
use sea_orm::{ConnectOptions, Database, DbConn};

use super::settings::AppSettings;

#[derive(Clone)]
pub struct AppState {
    pub settings: AppSettings,
    // pub js: Arc<AppJS>,
    pub db: Arc<DbConn>,
}

impl AppState {
    pub async fn new(settings: AppSettings) -> Result<Self, Error> {
        // let nats = async_nats::connect(&settings.nats.endpoint).await.unwrap();
        // let js = Arc::new(jetstream::new(nats));

        let opt = ConnectOptions::new(&settings.db.endpoint);
        let db = Arc::new(Database::connect(opt).await?);

        Ok(Self { settings, db })
    }
}
