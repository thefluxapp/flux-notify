use super::state::AppState;

mod messaging;
mod service;
pub(super) mod settings;

pub fn messaging(state: &AppState) {
    tokio::spawn(messaging::message(state.clone()));
}
