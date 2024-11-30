use async_nats::jetstream::stream::ConsumerErrorKind;
use thiserror::Error;

impl From<async_nats::error::Error<ConsumerErrorKind>> for AppError {
    fn from(_: async_nats::error::Error<ConsumerErrorKind>) -> Self {
        Self::DUMMY
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("entity not found")]
    DUMMY,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
