use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entity::posts::{self, Column};

pub async fn get_by_id(
    Extension(db): Extension<DatabaseConnection>,
    Path(path_id): Path<i64>,
) -> Result<Json<posts::Model>, StatusCode> {
    match posts::Entity::find_by_id(path_id).one(&db).await {
        Ok(Some(mdl)) => Ok(Json(mdl)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_err) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_by_user(
    Extension(db): Extension<DatabaseConnection>,
    Path(path_username): Path<String>,
) -> Result<Json<Vec<posts::Model>>, StatusCode> {
    match posts::Entity::find()
        .filter(Column::Username.eq(path_username))
        .all(&db)
        .await
    {
        Ok(vec) => Ok(Json(vec)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_all(
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<Vec<posts::Model>>, StatusCode> {
    match posts::Entity::find().all(&db).await {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
