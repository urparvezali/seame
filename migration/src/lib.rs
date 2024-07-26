pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20240722_185356_posts;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20240722_185356_posts::Migration),
        ]
    }
}
