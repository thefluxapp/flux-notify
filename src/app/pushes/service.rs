use chrono::Utc;
use log::debug;
use sea_orm::DbConn;
use uuid::Uuid;

use crate::app::error::AppError;

use super::repo;
use super::state::PushesState;
use super::vapid::Vapid;

pub fn get_vapid(pushes_state: &PushesState) -> Result<get_vapid::Response, AppError> {
    let public_key = pushes_state.vapid.public_key.to_sec1_bytes().into_vec();

    Ok(get_vapid::Response { public_key })
}

pub mod get_vapid {
    pub struct Response {
        pub public_key: Vec<u8>,
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
        pub public_key: Vec<u8>,
        pub authentication_secret: Vec<u8>,
        pub device_id: String,
        pub user_id: Uuid,
    }
}

pub async fn get_web_pushes(
    db: &DbConn,
    req: get_web_pushes::Request,
) -> Result<get_web_pushes::Response, AppError> {
    let web_pushes = repo::find_web_pushes_by_user_ids(db, vec![req.user_id]).await?;

    Ok(web_pushes.into())
}

pub mod get_web_pushes {
    use uuid::Uuid;

    use crate::app::pushes::repo;

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

pub async fn send_web_push(
    db: &DbConn,
    vapid: &Vapid,
    req: send_web_push::Request,
) -> Result<(), AppError> {
    let web_pushes = repo::find_web_pushes_by_user_ids(
        db,
        req.user_ids
            .clone()
            .into_iter()
            .filter(|v| *v != req.user_id)
            .collect(),
    )
    .await?;

    // TODO: Make it async
    for web_push in web_pushes {
        debug!("SEND WEB PUSH TO {}", web_push.user_id);

        vapid
            .send(
                req.text.clone().into(),
                web_push.endpoint,
                web_push.authentication_secret,
                web_push.public_key,
            )
            .await?;
    }

    Ok(())
}

pub mod send_web_push {
    use uuid::Uuid;

    #[derive(Debug)]
    pub struct Request {
        pub text: String,
        pub user_id: Uuid,
        pub user_ids: Vec<Uuid>,
    }
}
