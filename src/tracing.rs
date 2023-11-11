use tracing_subscriber::EnvFilter;

pub async fn run() {
    tracing_subscriber::fmt()
        .with_thread_names(true)
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();
}
