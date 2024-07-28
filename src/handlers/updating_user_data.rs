use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};

use crate::{
    entity::users::{self, Column},
    payloads::{UserUpdateBirthdayPayload, UserUpdateEmailPayload, UserUpdatePasswordPayload},
};

pub async fn user_update_password(
    Extension(db): Extension<DatabaseConnection>,
    Path(path_username): Path<String>,
    Json(payload): Json<UserUpdatePasswordPayload>,
) -> StatusCode {
    let mdl_res = users::Entity::find()
        .filter(
            Condition::all()
                .add(Column::Username.eq(path_username.clone()))
                .add(Column::Password.eq(payload.password.clone())),
        )
        .one(&db)
        .await;

    match mdl_res {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Ok(None) => StatusCode::UNAUTHORIZED,
        Ok(Some(mdl)) => {
            let mut mdl = mdl.into_active_model();
            mdl.password = Set(payload.new_password);
            if let Err(_) = mdl.update(&db).await {
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
            StatusCode::OK
        }
    }
}

pub async fn user_update_birthday(
    Extension(db): Extension<DatabaseConnection>,
    Path(path_username): Path<String>,
    Json(payload): Json<UserUpdateBirthdayPayload>,
) -> StatusCode {
    let mdl_res = users::Entity::find()
        .filter(Column::Username.eq(path_username.clone()))
        .one(&db)
        .await;

    match mdl_res {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Ok(None) => StatusCode::NOT_FOUND,
        Ok(Some(mdl)) => {
            let mut mdl = mdl.into_active_model();
            mdl.birthday = Set(payload.birthday);
            if let Err(_) = mdl.update(&db).await {
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
            StatusCode::OK
        }
    }
}
pub async fn user_update_email(
    Extension(db): Extension<DatabaseConnection>,
    Path(path_username): Path<String>,
    Json(payload): Json<UserUpdateEmailPayload>,
) -> StatusCode {
    let mdl_res = users::Entity::find()
        .filter(Column::Username.eq(path_username.clone()))
        .one(&db)
        .await;

    match mdl_res {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Ok(None) => StatusCode::NOT_FOUND,
        Ok(Some(mdl)) => {
            let mut mdl = mdl.into_active_model();
            mdl.email = Set(payload.email);
            if let Err(_) = mdl.update(&db).await {
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
            StatusCode::OK
        }
    }
}
