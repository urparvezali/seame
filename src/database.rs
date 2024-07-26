use dotenvy::var;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn db_conn() -> Result<DatabaseConnection, DbErr> {
    let db_url = var("DATABASE_URL").expect("Env var must be set..!");
    Database::connect(db_url).await
}
