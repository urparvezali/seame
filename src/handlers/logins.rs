use axum::{http::StatusCode, Extension, Json};
use dotenvy::var;
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::{
    entity::users::{self, Column},
    middlewares::Claim,
    payloads::{LoginPayload, UserPayload},
};

pub async fn login(
    Extension(db): Extension<DatabaseConnection>,
    Json(payload): Json<LoginPayload>,
) -> Result<String, StatusCode> {
    let find_mdl_res = users::Entity::find()
        .filter(
            Condition::all()
                .add(Column::Username.eq(payload.username))
                .add(Column::Password.eq(payload.password)),
        )
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();
    if find_mdl_res.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let mdl = find_mdl_res.unwrap();
    let secret = var("SECRET").unwrap();
    let token = encode(
        &Header::default(),
        &Claim {
            username: mdl.username,
            exp: 28349283497983292,
        },
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();

    Ok(token)
}

pub async fn signup(
    Extension(db): Extension<DatabaseConnection>,
    Json(payload): Json<UserPayload>,
) -> StatusCode {
    let active_payload = users::ActiveModel {
        uuid: Set(Uuid::new_v4()),
        username: Set(payload.username),
        password: Set(payload.password),
        email: Set(payload.email),
        birthday: Set(payload.birthday),
        ..Default::default()
    };
    match active_payload.insert(&db).await {
        Ok(mdl) => {
            dbg!(mdl); // will be removed later
            StatusCode::ACCEPTED
        }
        Err(_e) => StatusCode::CONFLICT,
    }
}
