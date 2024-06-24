use sea_orm_migration::prelude::*;

use database::entities::task as TaskEntity;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TaskEntity::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TaskEntity::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TaskEntity::Column::Name)
                            .char_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TaskEntity::Column::Description)
                            .char_len(512)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TaskEntity::Column::IsActive)
                            .boolean()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TaskEntity::Entity).to_owned())
            .await
    }
}
