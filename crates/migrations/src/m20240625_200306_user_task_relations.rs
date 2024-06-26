use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let fk_user = TableForeignKey::new()
            .name("fk_user")
            .from_tbl(Task::Table)
            .from_col(Task::UserId)
            .to_tbl(User::Table)
            .to_col(User::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table(Task::Table)
                    .add_foreign_key(&fk_user)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Task::Table)
                    .drop_foreign_key(Alias::new("fk_user"))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    #[sea_orm(iden = "users")]
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Task {
    #[sea_orm(iden = "tasks")]
    Table,
    #[sea_orm(iden = "user_id")]
    UserId,
}
