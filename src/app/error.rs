use thiserror::Error;
use tonic::Status;

impl From<AppError> for Status {
    fn from(err: AppError) -> Self {
        match err {
            AppError::UUID(err) => Self::invalid_argument(err.to_string()),
            err => Self::internal(err.to_string()),
        }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    UUID(#[from] uuid::Error),
    #[error(transparent)]
    DB(#[from] sea_orm::DbErr),
}
