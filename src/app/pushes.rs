use flux_notify_api::push_service_server::PushServiceServer;
use grpc::GrpcPushService;

use super::state::AppState;

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

pub fn messaging(state: &AppState) {
    tokio::spawn(messaging::message(state.clone()));
}
