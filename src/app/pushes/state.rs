use std::sync::Arc;

use flux_lib::error::Error;
use p256::pkcs8::{DecodePrivateKey as _, DecodePublicKey as _};
use tokio::fs;

use super::{settings::PushesSettings, vapid::Vapid};

#[derive(Clone)]
pub struct PushesState {
    pub vapid: Arc<Vapid>,
}

impl PushesState {
    pub async fn new(settings: &PushesSettings) -> Result<Self, Error> {
        let private_key = p256::SecretKey::from_pkcs8_pem(
            &fs::read_to_string(&settings.vapid.private_key_file).await?,
        )?;

        let public_key = p256::PublicKey::from_public_key_pem(
            &fs::read_to_string(&settings.vapid.public_key_file).await?,
        )?;

        let vapid = Arc::new(Vapid::new(
            private_key,
            public_key,
            settings.vapid.sub.clone(),
            settings.vapid.ttl,
        ));

        Ok(Self { vapid })
    }
}
