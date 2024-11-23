use flux_notify_api::{notify_service_server::NotifyService, ExampleRequest, ExampleResponse};
use tonic::{Request, Response, Status};

use crate::app::state::AppState;

pub struct GrpcNotifyService {
    pub state: AppState,
}

impl GrpcNotifyService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl NotifyService for GrpcNotifyService {
    async fn example(
        &self,
        _req: Request<ExampleRequest>,
    ) -> Result<Response<ExampleResponse>, Status> {
        Ok(Response::new(ExampleResponse {
            data: Some("QQQ".into()),
        }))
    }
    // async fn get_users(
    //     &self,
    //     request: Request<GetUsersRequest>,
    // ) -> Result<Response<GetUsersResponse>, Status> {
    //     let response = get_users(&self.state, request.into_inner()).await?;

    //     Ok(Response::new(response))
    // }
}
