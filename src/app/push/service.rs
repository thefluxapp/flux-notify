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
