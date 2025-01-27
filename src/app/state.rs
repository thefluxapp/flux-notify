use std::sync::Arc;

use anyhow::Error;
use async_nats::jetstream;

use super::{settings::AppSettings, AppJS};

#[derive(Clone)]
pub struct AppState {
    pub settings: AppSettings,
    pub js: Arc<AppJS>,
    // pub notify: NotifyState,
}

impl AppState {
    pub async fn new(settings: AppSettings) -> Result<Self, Error> {
        let nats = async_nats::connect(&settings.nats.endpoint).await.unwrap();
        let js = Arc::new(jetstream::new(nats));

        // let notify = NotifyState::new(settings.notify.clone());

        Ok(Self {
            settings,
            js,
            // notify,
        })
    }
}
