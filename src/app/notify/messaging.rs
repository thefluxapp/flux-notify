use anyhow::{Context, Error};
use async_nats::jetstream::consumer::PullConsumer;
use flux_core_api::NotifyMessage;
use prost::Message;
use serde::Serialize;
use tokio_stream::StreamExt as _;

use crate::app::state::AppState;

async fn create_message(state: &AppState, consumer: &PullConsumer) -> Result<(), Error> {
    let messages = consumer.messages().await?;
    tokio::pin!(messages);

    while let Some(message) = messages.try_next().await? {
        // TODO: Add error handler
        let notify_message = NotifyMessage::decode(message.payload.clone())?
            .message
            .context("NO_MESSAGE")?;

        let event = Event::Message(notify_message.into());

        match state.notify.tx.send(event.try_into()?) {
            Ok(_) => message.ack().await.map_err(Error::msg)?,
            Err(e) => {
                println!("ErroRR: {}", e)
            }
        };
    }

    Ok(())
}

pub mod create_message {
    use anyhow::Error;
    use async_nats::jetstream::consumer::PullConsumer;
    use serde::Serialize;

    use super::create_message;
    use crate::app::state::AppState;

    pub async fn consume(state: AppState, consumer: PullConsumer) -> Result<(), Error> {
        loop {
            if let Err(e) = create_message(&state, &consumer).await {
                println!("Error: {}", e)
            }
        }
    }

    #[derive(Serialize, Clone)]
    pub struct Event {
        pub message_id: String,
        pub text: String,
    }

    impl From<flux_core_api::notify_message::Message> for Event {
        fn from(message: flux_core_api::notify_message::Message) -> Self {
            Self {
                message_id: message.message_id().into(),
                text: message.text().into(),
            }
        }
    }

    impl TryFrom<super::Event> for axum::response::sse::Event {
        type Error = Error;

        fn try_from(event: super::Event) -> Result<Self, Self::Error> {
            Ok(Self::default().json_data(event)?)
        }
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "snake_case")]
enum Event {
    Message(create_message::Event),
}
