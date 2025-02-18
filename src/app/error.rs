use async_nats::jetstream::stream::ConsumerErrorKind;
use thiserror::Error;
use tonic::Status;

impl From<async_nats::error::Error<ConsumerErrorKind>> for AppError {
    fn from(_: async_nats::error::Error<ConsumerErrorKind>) -> Self {
        Self::DUMMY
    }
}

impl From<AppError> for Status {
    fn from(error: AppError) -> Self {
        match error {
            AppError::Other(error) => Self::internal(error.to_string()),
            AppError::DUMMY => todo!(),
            AppError::UUID(_) => todo!(),
        }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("entity not found")]
    DUMMY,
    #[error(transparent)]
    UUID(#[from] uuid::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
