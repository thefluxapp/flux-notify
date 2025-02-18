use flux_notify_api::{
    push_service_server::PushService, CreateWebPushRequest, CreateWebPushResponse, GetVapidRequest,
    GetVapidResponse,
};
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

    async fn create_web_push(
        &self,
        req: Request<CreateWebPushRequest>,
    ) -> Result<Response<CreateWebPushResponse>, Status> {
        create_web_push(req.into_inner())?;

        Ok(Response::new(CreateWebPushResponse::default()))
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

fn create_web_push(req: CreateWebPushRequest) -> Result<(), AppError> {
    service::create_web_push(req.try_into()?);

    Ok(())
}

mod create_web_push {
    use crate::app::{error::AppError, push::service};
    use flux_notify_api::CreateWebPushRequest;
    use uuid::Uuid;

    impl TryFrom<CreateWebPushRequest> for service::create_web_push::Request {
        type Error = AppError;

        fn try_from(req: CreateWebPushRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                endpoint: req.endpoint().into(),
                public_key: req.public_key().into(),
                authentication_secret: req.authentication_secret().into(),
                user_id: Uuid::parse_str(req.user_id())?,
                device_id: req.device_id().into(),
            })
        }
    }
}
