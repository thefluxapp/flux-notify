use sea_orm::{sea_query::OnConflict, ConnectionTrait, EntityTrait as _, IntoActiveModel as _};

use crate::app::error::AppError;

pub mod web_push;

pub async fn create_web_push<T: ConnectionTrait>(
    db: &T,
    model: web_push::Model,
) -> Result<(), AppError> {
    web_push::Entity::insert(model.into_active_model())
        .on_conflict(
            OnConflict::columns([web_push::Column::UserId, web_push::Column::DeviceId])
                .update_columns([
                    web_push::Column::Endpoint,
                    web_push::Column::AuthenticationSecret,
                    web_push::Column::PublicKey,
                ])
                .to_owned(),
        )
        .do_nothing()
        .exec(db)
        .await?;
    Ok(())
}
