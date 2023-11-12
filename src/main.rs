use dotenv::dotenv;

mod app;
mod notifier;
mod tracing;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing::run();

    tokio::spawn(async { notifier::run().await.unwrap() });

    app::run().await
}
