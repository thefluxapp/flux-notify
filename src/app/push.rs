use flux_notify_api::push_service_server::PushServiceServer;
use grpc::GrpcPushService;

use super::state::AppState;

mod grpc;
mod service;
pub(super) mod settings;

pub fn push_service(state: AppState) -> PushServiceServer<GrpcPushService> {
    PushServiceServer::new(GrpcPushService::new(state))
}
