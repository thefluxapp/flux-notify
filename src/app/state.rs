use std::sync::Arc;

use async_nats::jetstream;
use flux_lib::error::Error;
use sea_orm::{ConnectOptions, Database, DbConn};

use super::{pushes::state::PushesState, settings::AppSettings, AppJS};

#[derive(Clone)]
pub struct AppState {
    pub settings: AppSettings,
    pub js: Arc<AppJS>,
    pub db: Arc<DbConn>,
    pub pushes_state: PushesState,
}

impl AppState {
    pub async fn new(settings: AppSettings) -> Result<Self, Error> {
        let nats = async_nats::connect(&settings.nats.endpoint).await?;
        let js = Arc::new(jetstream::new(nats));

        let opt = ConnectOptions::new(&settings.db.endpoint);
        let db = Arc::new(Database::connect(opt).await?);

        let pushes_state = PushesState::new(&settings.pushes).await?;

        Ok(Self {
            settings,
            db,
            js,
            pushes_state,
        })
    }
}
