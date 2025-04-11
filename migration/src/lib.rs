pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20240722_185356_posts;
mod m20240806_161430_comments;
mod m20240806_163137_likes;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20240722_185356_posts::Migration),
            Box::new(m20240806_161430_comments::Migration),
            Box::new(m20240806_163137_likes::Migration),
        ]
    }
}
