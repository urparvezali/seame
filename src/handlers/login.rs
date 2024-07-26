use axum::{http::StatusCode, Extension, Json};
use bcrypt::hash;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use uuid::Uuid;

use crate::{entity::users, payloads::UserPayload};

pub async fn signup(
    Extension(db): Extension<DatabaseConnection>,
    Json(mut payload): Json<UserPayload>,
) -> StatusCode {
    payload.password = hash(payload.password, 10).unwrap();

    let active_payload = users::ActiveModel {
        uuid: Set(Uuid::new_v4()),
        username: Set(payload.username),
        password: Set(payload.password),
        email: Set(payload.email),
        birthday: Set(payload.birthday),
        ..Default::default()
    };
    active_payload
        .insert(&db)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();
    StatusCode::OK
}
