use thiserror::Error;
use tonic::Status;

impl From<AppError> for Status {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Uuid(err) => Self::invalid_argument(err.to_string()),
            err => Self::internal(err.to_string()),
        }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Status(#[from] tonic::Status),
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
    #[error(transparent)]
    Decode(#[from] prost::DecodeError),
    #[error(transparent)]
    Encode(#[from] prost::EncodeError),
    #[error(transparent)]
    Publish(#[from] async_nats::jetstream::context::PublishError),
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),
    #[error("EMPTY")]
    Empty,
}
