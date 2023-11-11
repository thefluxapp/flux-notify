use dotenv::dotenv;

mod app;
mod notifier;
mod tracing;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing::run().await;
    notifier::run().await.unwrap();

    app::run().await;
}
