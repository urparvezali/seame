use axum::{middleware, Extension, Router};
use database::db_conn;
use middlewares::guard_auth;
use routers::{login_router, post_router, user_router};
use tokio::net::TcpListener;

mod database;
mod entity;
mod handlers;
mod middlewares;
pub mod payloads;
mod routers;

#[tokio::main]
async fn main() {
    let tcp = TcpListener::bind("localhost:8000").await.unwrap();
    let db = db_conn().await.unwrap();
    let app = Router::new()
        .merge(post_router().await)
        .merge(user_router().await)
        .layer(middleware::from_fn(guard_auth))
        .merge(login_router().await)
        .layer(Extension(db));
    axum::serve(tcp, app.into_make_service()).await.unwrap();
}
