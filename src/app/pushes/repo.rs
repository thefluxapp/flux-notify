use sea_orm::{
    sea_query::OnConflict, ColumnTrait as _, ConnectionTrait, EntityTrait as _,
    IntoActiveModel as _, Order, QueryFilter as _, QueryOrder as _,
};
use uuid::Uuid;

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
                    web_push::Column::UpdatedAt,
                ])
                .to_owned(),
        )
        .do_nothing()
        .exec(db)
        .await?;
    Ok(())
}

pub async fn find_web_pushes_by_user_ids<T: ConnectionTrait>(
    db: &T,
    user_ids: Vec<Uuid>,
) -> Result<Vec<web_push::Model>, AppError> {
    let web_pushes = web_push::Entity::find()
        .filter(web_push::Column::UserId.is_in(user_ids))
        .order_by(web_push::Column::Id, Order::Asc)
        .all(db)
        .await?;

    Ok(web_pushes)
}
