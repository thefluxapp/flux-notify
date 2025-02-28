use flux_notify_api::push_service_server::PushServiceServer;
use grpc::GrpcPushService;

use super::{error::AppError, state::AppState};

mod grpc;
mod messaging;
mod repo;
mod service;
pub(super) mod settings;
pub(super) mod state;
mod vapid;

pub fn push_service(state: AppState) -> PushServiceServer<GrpcPushService> {
    PushServiceServer::new(GrpcPushService::new(state))
}

pub async fn messaging(state: &AppState) -> Result<(), AppError> {
    tokio::spawn(messaging::message(state.clone()));

    Ok(())
}
