use std::env;

use axum::BoxError;
use serde::{Deserialize, Serialize};
use web_push::{IsahcWebPushClient, WebPushClient};

pub struct PushAPI {
    client: IsahcWebPushClient,
    vapid_private_key: String,
    vapid_sub: String,
}

impl PushAPI {
    pub fn new() -> Self {
        Self {
            client: IsahcWebPushClient::new().unwrap(),
            vapid_private_key: env::var("VAPID_PRIVATE_KEY").unwrap(),
            vapid_sub: env::var("VAPID_SUB").unwrap(),
        }
    }

    pub async fn send(&self, push_notification: PushNotification) -> Result<(), BoxError> {
        let payload = Payload {
            title: push_notification.title,
            body: push_notification.body,
        };
        let payload = serde_json::to_string(&payload).unwrap();

        let subscription_info = web_push::SubscriptionInfo::new(
            push_notification.endpoint.clone(),
            push_notification.p256dh_key.clone(),
            push_notification.auth_key.clone(),
        );

        let mut message_builder = web_push::WebPushMessageBuilder::new(&subscription_info);

        let mut signature = web_push::VapidSignatureBuilder::from_base64(
            &self.vapid_private_key,
            web_push::URL_SAFE_NO_PAD,
            &subscription_info,
        )
        .unwrap();

        signature.add_claim("sub", self.vapid_sub.clone());

        message_builder.set_payload(web_push::ContentEncoding::Aes128Gcm, payload.as_bytes());
        message_builder.set_vapid_signature(signature.build().unwrap());

        self.client
            .send(message_builder.build().unwrap())
            .await
            .unwrap();

        Ok(())
    }
}

#[derive(Deserialize)]
pub struct PushNotification {
    title: String,
    body: String,
    endpoint: String,
    p256dh_key: String,
    auth_key: String,
}

#[derive(Serialize)]
struct Payload {
    title: String,
    body: String,
}
