use chrono::Utc;
use sea_orm::DbConn;
use uuid::Uuid;

use crate::app::error::AppError;

use super::{repo, settings::VapidSettings};

pub fn get_vapid(settings: &VapidSettings) -> Result<get_vapid::Response, AppError> {
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

pub async fn get_web_pushes(
    db: &DbConn,
    req: get_web_pushes::Request,
) -> Result<get_web_pushes::Response, AppError> {
    let web_pushes = repo::find_web_pushes_by_user_id(db, req.user_id).await?;

    Ok(web_pushes.into())
}

pub mod get_web_pushes {
    use uuid::Uuid;

    use crate::app::push::repo;

    pub struct Request {
        pub user_id: Uuid,
    }

    pub struct Response {
        pub device_ids: Vec<String>,
    }

    impl From<Vec<repo::web_push::Model>> for Response {
        fn from(web_pushes: Vec<repo::web_push::Model>) -> Self {
            Self {
                device_ids: web_pushes.into_iter().map(|wp| wp.device_id).collect(),
            }
        }
    }
}
