use anyhow::Error;
use async_nats::jetstream::consumer::pull::Config;
use log::error;
use prost::Message as _;
use tokio_stream::StreamExt as _;
use uuid::Uuid;

use crate::app::{pushes::service, state::AppState};

pub async fn message(state: AppState) -> Result<(), Error> {
    let AppState {
        db,
        js,
        settings,
        pushes_state,
        ..
    } = state;

    let consumer = js
        .create_consumer_on_stream(
            Config {
                durable_name: Some(settings.pushes.messaging.message.consumer.clone()),
                filter_subjects: settings.pushes.messaging.message.subjects.clone(),
                ..Default::default()
            },
            settings.nats.stream.clone(),
        )
        .await?;

    let msgs = consumer.messages().await?;
    tokio::pin!(msgs);

    while let Some(msg) = msgs.next().await {
        if let Err(err) = async {
            let msg = msg.map_err(Error::msg)?;

            let flux_messages_api::Message { message, stream } =
                flux_messages_api::Message::decode(msg.payload.clone())?;

            if let (Some(message), Some(stream)) = (message, stream) {
                service::send_web_push(
                    &db,
                    &pushes_state.vapid,
                    service::send_web_push::Request {
                        text: message.text().into(),
                        user_id: Uuid::parse_str(message.user_id())?,
                        user_ids: stream
                            .user_ids
                            .into_iter()
                            .map(|v| -> Result<Uuid, Error> {
                                Uuid::parse_str(&v).map_err(Error::msg)
                            })
                            .collect::<Result<Vec<Uuid>, Error>>()?,
                    },
                )
                .await?;

                // dbg!(&message);
                // dbg!(&stream);
            }

            msg.ack().await.map_err(Error::msg)?;

            Ok::<(), Error>(())
        }
        .await
        {
            error!("{}", err);
        }
    }

    Ok(())
}
