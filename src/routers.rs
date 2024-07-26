use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::{
    creating_post::add_post,
    geting_post::{get_all, get_by_id, get_by_user},
    login::signup,
};

pub async fn login_routers() -> Router {
    Router::new().route("/login/signup", post(signup))
}

pub async fn posts_router() -> Router {
    Router::new()
        .route("/post/create/:path_username", post(add_post))
        .route("/post/get/get_by_id/:id", get(get_by_id))
        .route("/post/get/get_by_user/:username", get(get_by_user))
        .route("/post/get/get_all", get(get_all))
}
