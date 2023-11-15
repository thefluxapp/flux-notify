// use bytes::Bytes;
use futures::StreamExt;
use std::env;

use crate::push_api::{PushAPI, PushNotification};

pub async fn run() -> Result<(), async_nats::Error> {
    let client = async_nats::connect(env::var("FLUX_NATS_ADDR").unwrap()).await?;
    let push_api = PushAPI::new();

    let mut subscriber = client.subscribe("push-notifications").await?;

    while let Some(message) = subscriber.next().await {
        let push_notification: PushNotification = serde_json::from_slice(&message.payload).unwrap();

        push_api.send(push_notification).await?;
    }

    Ok(())
}
