use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entity::users::{self, Column, Model};

pub async fn get_all(
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<Vec<Model>>, StatusCode> {
    match users::Entity::find().all(&db).await {
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        Ok(usrs) => Ok(Json(usrs)),
    }
}
pub async fn get_by_username(
    Extension(db): Extension<DatabaseConnection>,
    Path(path_username): Path<String>,
) -> Result<Json<Model>, StatusCode> {
    match users::Entity::find()
        .filter(Column::Username.eq(path_username))
        .one(&db)
        .await
    {
        Ok(Some(mdl)) => Ok(Json(mdl)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
