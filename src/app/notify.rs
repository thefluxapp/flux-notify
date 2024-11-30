use async_nats::jetstream::consumer::pull::Config;
use axum::{
    extract::State,
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
    routing::get,
    Router,
};
use messaging::create_message;
use tokio_stream::{
    wrappers::{errors::BroadcastStreamRecvError, BroadcastStream},
    Stream,
};

use super::{error::AppError, state::AppState};

mod messaging;
pub(super) mod settings;
pub(super) mod state;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(notify))
}

async fn notify(
    State(AppState { notify, .. }): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, BroadcastStreamRecvError>>> {
    // TODO: How to handle BroadcastStreamRecvError?

    let rx = notify.tx.subscribe();

    Sse::new(BroadcastStream::new(rx)).keep_alive(KeepAlive::default())
}

pub mod notity {
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    pub struct Res {
        pub foo: String,
    }
}

pub async fn messaging(state: &AppState) -> Result<(), AppError> {
    let AppState { js, settings, .. } = state;

    let consumer = js
        .create_consumer_on_stream(
            Config {
                durable_name: Some(settings.notify.messaging.message.consumer.clone()),
                filter_subjects: settings.notify.messaging.message.subjects.clone(),
                ..Default::default()
            },
            settings.nats.stream.clone(),
        )
        .await?;

    tokio::spawn(create_message::consume(state.clone(), consumer));

    Ok(())
}
