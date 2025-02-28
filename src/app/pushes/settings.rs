use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct PushesSettings {
    pub vapid: VapidSettings,
    pub messaging: MessagingSettings,
}

#[derive(Deserialize, Clone)]
pub struct VapidSettings {
    pub sub: String,
    pub ttl: i64,
    pub private_key_file: String,
    pub public_key_file: String,
}

#[derive(Deserialize, Clone)]
pub struct MessagingSettings {
    pub message: MessagingMessageSettings,
}

#[derive(Deserialize, Clone)]
pub struct MessagingMessageSettings {
    pub subjects: Vec<String>,
    pub consumer: String,
}
