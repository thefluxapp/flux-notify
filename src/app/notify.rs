use flux_notify_api::notify_service_server::NotifyServiceServer;
use grpc::GrpcNotifyService;

use super::state::AppState;

mod grpc;
// mod service;

pub fn notify_service(state: AppState) -> NotifyServiceServer<GrpcNotifyService> {
    NotifyServiceServer::new(GrpcNotifyService::new(state))
}
