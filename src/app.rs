use anyhow::Error;
use axum::Router;
use settings::AppSettings;
use state::AppState;
use tonic::service::Routes;

mod error;
mod push;
mod settings;
mod state;

pub async fn run() -> Result<(), Error> {
    let settings = AppSettings::new()?;
    let state = AppState::new(settings).await?;

    messaging(&state).await?;
    http_and_grpc(&state).await?;

    Ok(())
}

async fn http_and_grpc(state: &AppState) -> Result<(), Error> {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(tonic_health::pb::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(flux_notify_api::PUSH_FILE_DESCRIPTOR_SET)
        .build_v1alpha()?;

    let (_, health_service) = tonic_health::server::health_reporter();

    let router = Router::new().with_state(());
    let routes = Routes::from(router);
    let router = routes
        .add_service(reflection_service)
        .add_service(health_service)
        .add_service(push::push_service(state.clone()))
        .into_axum_router();

    let listener = tokio::net::TcpListener::bind(&state.settings.http.endpoint).await?;

    axum::serve(listener, router).await?;

    Ok(())
}

async fn messaging(_: &AppState) -> Result<(), Error> {
    Ok(())
}

// pub type AppJS = jetstream::Context;
