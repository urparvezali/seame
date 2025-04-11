use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use uuid::Uuid;

use crate::{entity::posts, payloads::PostPayload};

pub async fn add_post(
    Extension(db): Extension<DatabaseConnection>,
    Path(path_username): Path<String>,
    Json(payload): Json<PostPayload>,
) -> Result<Json<posts::Model>, StatusCode> {
    let active_payload = posts::ActiveModel {
        uuid: Set(Uuid::new_v4()),
        title: Set(payload.title),
        body: Set(payload.body),
        username: Set(path_username),
        ..Default::default()
    };
    match active_payload.insert(&db).await {
        Ok(mdl) => Ok(Json(mdl)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
