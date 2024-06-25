pub use sea_orm_migration::prelude::*;

mod m20240624_213509_task_create;
mod m20240625_184538_user_create;
mod m20240625_193356_user_task_relations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240624_213509_task_create::Migration),
            Box::new(m20240625_184538_user_create::Migration),
            Box::new(m20240625_193356_user_task_relations::Migration),
        ]
    }
}
