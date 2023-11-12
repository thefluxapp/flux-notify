// use bytes::Bytes;
use futures::StreamExt;
use std::env;
use tracing::info;

pub async fn run() -> Result<(), async_nats::Error> {
    let client = async_nats::connect(env::var("NATS_ADDR").unwrap()).await?;

    let mut subscriber = client.subscribe("messages").await?;

    while let Some(message) = subscriber.next().await {
        info!("Received message {:?}", message);
        // println!("Received message {:?}", message);
    }

    Ok(())
}
