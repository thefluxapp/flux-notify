use crate::app::error::AppError;

use super::settings::VapidSettings;

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

pub async fn create_web_push(req: create_web_push::Request) -> Result<(), AppError> {
    dbg!(&req);

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
