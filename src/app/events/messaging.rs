use flux_lib::error::Error;
use log::error;
use tokio_stream::StreamExt as _;

use crate::app::state::AppState;

pub async fn message(state: AppState) -> Result<(), Error> {
    let AppState { js, settings, .. } = state.clone();

    let consumer = message::consumer(&js, &settings).await?;
    let mut messages = consumer.messages().await?;

    while let Some(message) = messages.next().await {
        if let Err(err) = message::handler(state.clone(), message?).await {
            error!("{}", err);
        }
    }

    Ok(())
}

mod message {
    use async_nats::jetstream::{
        consumer::{pull::Config, Consumer},
        Message,
    };
    use flux_lib::error::Error;
    use prost::Message as _;

    use crate::app::{
        error::AppError,
        events::service::{
            self,
            message::{Request, Stream},
        },
        settings::AppSettings,
        state::AppState,
        AppJS,
    };

    pub async fn consumer(js: &AppJS, settings: &AppSettings) -> Result<Consumer<Config>, Error> {
        Ok(js
            .create_consumer_on_stream(
                Config {
                    durable_name: Some(settings.events.messaging.message.consumer.clone()),
                    filter_subjects: settings.events.messaging.message.subjects.clone(),
                    ..Default::default()
                },
                settings.nats.stream.clone(),
            )
            .await?)
    }

    pub async fn handler(state: AppState, message: Message) -> Result<(), Error> {
        service::message(state, message.clone().try_into()?).await?;

        message.ack().await.map_err(Error::msg)?;
        Ok(())
    }

    impl TryFrom<Message> for Request {
        type Error = AppError;

        fn try_from(message: Message) -> Result<Self, Self::Error> {
            let message = flux_messages_api::Message::decode(message.payload.as_ref())?;

            Ok(Self {
                message_id: message.message_id().into(),
                user_id: message.user_id().into(),
                text: message.text().into(),
                code: message.code().into(),
                order: message.order(),
                stream: match message.stream {
                    Some(stream) => Some(Stream {
                        stream_id: stream.stream_id().into(),
                        message_id: stream.message_id().into(),
                    }),
                    None => None,
                },
                created_at: message.created_at.ok_or(AppError::Empty)?,
                updated_at: message.updated_at.ok_or(AppError::Empty)?,
            })
        }
    }
}
