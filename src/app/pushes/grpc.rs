use flux_notify_api::{
    push_service_server::PushService, CreateWebPushRequest, CreateWebPushResponse, GetVapidRequest,
    GetVapidResponse, GetWebPushesRequest, GetWebPushesResponse,
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
        let res = get_vapid(&self.state)?;

        Ok(Response::new(res.into()))
    }

    async fn create_web_push(
        &self,
        req: Request<CreateWebPushRequest>,
    ) -> Result<Response<CreateWebPushResponse>, Status> {
        create_web_push(&self.state, req.into_inner()).await?;

        Ok(Response::new(CreateWebPushResponse::default()))
    }

    async fn get_web_pushes(
        &self,
        req: Request<GetWebPushesRequest>,
    ) -> Result<Response<GetWebPushesResponse>, Status> {
        let res = get_web_pushes(&self.state, req.into_inner()).await?;

        Ok(Response::new(res.into()))
    }
}

fn get_vapid(
    AppState { settings, .. }: &AppState,
) -> Result<service::get_vapid::Response, AppError> {
    Ok(service::get_vapid(&settings.push.vapid)?)
}

mod get_vapid {
    use flux_notify_api::GetVapidResponse;

    use crate::app::pushes::service::get_vapid::Response;

    impl From<Response> for GetVapidResponse {
        fn from(res: Response) -> Self {
            Self {
                public_key: Some(res.public_key),
            }
        }
    }
}

async fn create_web_push(
    AppState { db, .. }: &AppState,
    req: CreateWebPushRequest,
) -> Result<(), AppError> {
    service::create_web_push(db, req.try_into()?).await?;

    Ok(())
}

mod create_web_push {
    use crate::app::{error::AppError, pushes::service};
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

async fn get_web_pushes(
    AppState { db, .. }: &AppState,
    req: GetWebPushesRequest,
) -> Result<service::get_web_pushes::Response, AppError> {
    Ok(service::get_web_pushes(db, req.try_into()?).await?)
}

mod get_web_pushes {
    use flux_notify_api::{GetWebPushesRequest, GetWebPushesResponse};
    use uuid::Uuid;

    use crate::app::{error::AppError, pushes::service};

    impl TryFrom<GetWebPushesRequest> for service::get_web_pushes::Request {
        type Error = AppError;

        fn try_from(req: GetWebPushesRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                user_id: Uuid::parse_str(req.user_id())?,
            })
        }
    }

    impl From<service::get_web_pushes::Response> for GetWebPushesResponse {
        fn from(res: service::get_web_pushes::Response) -> Self {
            Self {
                device_ids: res.device_ids,
            }
        }
    }
}
