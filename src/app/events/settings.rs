use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct EventsSettings {
    pub messaging: MessagingSettings,
}

#[derive(Deserialize, Clone)]
pub struct MessagingSettings {
    pub message: MessagingMessageSettings,
    pub event: MessagingEventSettings,
}

#[derive(Deserialize, Clone)]
pub struct MessagingMessageSettings {
    pub subjects: Vec<String>,
    pub consumer: String,
}

#[derive(Deserialize, Clone)]
pub struct MessagingEventSettings {
    pub subject: String,
}
