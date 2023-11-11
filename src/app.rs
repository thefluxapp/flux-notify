use std::env;

use axum::{routing::get, Router};
use tracing::info;

pub async fn run() {
    let app = Router::new().route("/healthz", get(|| async {}));

    let addr = &env::var("APP_ADDR").unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!("App started on {}", &addr);
    axum::serve(listener, app).await.unwrap();
}
