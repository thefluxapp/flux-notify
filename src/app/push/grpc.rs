use flux_notify_api::{push_service_server::PushService, GetVapidRequest, GetVapidResponse};
use tonic::{Request, Response, Status};

use crate::app::{error::AppError, state::AppState};

use super::service;

pub struct GrpcPushService {
    pub state: AppState,
}

impl GrpcPushService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl PushService for GrpcPushService {
    async fn get_vapid(
        &self,
        _: Request<GetVapidRequest>,
    ) -> Result<Response<GetVapidResponse>, Status> {
        let response = get_vapid(&self.state)?;

        Ok(Response::new(response.into()))
    }
}

fn get_vapid(
    AppState { settings, .. }: &AppState,
) -> Result<service::get_vapid::Response, AppError> {
    Ok(service::get_vapid(&settings.push.vapid)?)
}

mod get_vapid {
    use flux_notify_api::GetVapidResponse;

    use crate::app::push::service::get_vapid::Response;

    impl From<Response> for GetVapidResponse {
        fn from(res: Response) -> Self {
            Self {
                public_key: Some(res.public_key),
            }
        }
    }
}
