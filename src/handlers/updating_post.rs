use axum::Extension;
use sea_orm::DatabaseConnection;

pub async fn do_like(Extension(db): Extension<DatabaseConnection>,) {
	
}