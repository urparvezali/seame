use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Likes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Likes::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Likes::PostId).big_integer().not_null())
                    .col(ColumnDef::new(Likes::UserId).big_integer().not_null())
                    .col(
                        ColumnDef::new(Likes::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Likes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Likes {
    Table,
    Id,
    PostId,
    UserId,
    CreatedAt,
}
