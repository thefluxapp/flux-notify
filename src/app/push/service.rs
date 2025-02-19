use chrono::Utc;
use sea_orm::DbConn;
use uuid::Uuid;

use crate::app::error::AppError;

use super::{repo, settings::VapidSettings};

pub(super) fn get_vapid(settings: &VapidSettings) -> Result<get_vapid::Response, AppError> {
    Ok(get_vapid::Response {
        public_key: settings.public_key.clone(),
    })
}

pub mod get_vapid {
    pub struct Response {
        pub public_key: String,
    }
}

pub async fn create_web_push(db: &DbConn, req: create_web_push::Request) -> Result<(), AppError> {
    repo::create_web_push(db, {
        repo::web_push::Model {
            id: Uuid::now_v7(),
            user_id: req.user_id,
            device_id: req.device_id,
            endpoint: req.endpoint,
            public_key: req.public_key,
            authentication_secret: req.authentication_secret,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    })
    .await?;

    Ok(())
}

pub mod create_web_push {
    use uuid::Uuid;

    #[derive(Debug)]
    pub struct Request {
        pub endpoint: String,
        pub public_key: String,
        pub authentication_secret: String,
        pub device_id: String,
        pub user_id: Uuid,
    }
}
