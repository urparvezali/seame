use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::handlers::{
    creating_post::add_post,
    getting_post::{get_all, get_by_id, get_by_user},
    getting_user::{get_all as get_all_users, get_by_username},
    logins::{login, signup},
    updating_user_data::{user_update_birthday, user_update_email, user_update_password},
};

pub async fn login_router() -> Router {
    Router::new()
        .route("/logins/signup", post(signup))
        .route("/logins/login", post(login))
}

pub async fn post_router() -> Router {
    Router::new()
        .route("/post/create/:path_username", post(add_post))
        .route("/post/get/get_by_id/:id", get(get_by_id))
        .route("/post/get/get_by_user/:username", get(get_by_user))
        .route("/post/get/get_all", get(get_all))
}

pub async fn user_router() -> Router {
    Router::new()
        .route("/user/get/get_all", get(get_all_users))
        .route("/user/get/:username", get(get_by_username))
        .route(
            "/user/update/password/:username",
            patch(user_update_password),
        )
        .route(
            "/user/update/birthday/:username",
            patch(user_update_birthday),
        )
        .route("/user/update/email/:username", patch(user_update_email))
}
