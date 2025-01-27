use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct PushSettings {
    pub vapid: VapidSettings,
}

#[derive(Deserialize, Clone)]
pub struct VapidSettings {
    // pub private_key: String,
    pub public_key: String,
}
