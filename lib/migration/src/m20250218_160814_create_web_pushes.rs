use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(WebPushes::Table)
                    .col(uuid(WebPushes::Id).primary_key())
                    .col(uuid(WebPushes::UserId))
                    .col(text(WebPushes::DeviceId))
                    .col(text(WebPushes::Endpoint))
                    .col(binary(WebPushes::PublicKey))
                    .col(binary(WebPushes::AuthenticationSecret))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("web_pushes_user_id_device_id_udx")
                    .unique()
                    .table(WebPushes::Table)
                    .col(WebPushes::UserId)
                    .col(WebPushes::DeviceId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("web_pushes_user_id_idx")
                    .table(WebPushes::Table)
                    .col(WebPushes::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WebPushes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum WebPushes {
    Table,
    Id,
    UserId,
    DeviceId,
    Endpoint,
    PublicKey,
    AuthenticationSecret,
}
