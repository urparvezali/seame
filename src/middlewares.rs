use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use dotenvy::var;
use jsonwebtoken::{decode, DecodingKey, Validation};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::entity::users::{self, Column};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claim {
    pub username: String,
    pub exp: usize,
}

pub async fn guard_auth(
    request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let db = request.extensions().get::<DatabaseConnection>().unwrap();
    if request.headers().get(header::AUTHORIZATION).is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = get_token(&request);
    let secret = var("SECRET").unwrap();

    let decode_res = decode(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    );
    if decode_res.is_err() {
        return Err(StatusCode::EXPECTATION_FAILED);
    }
    let claim: Claim = decode_res.unwrap().claims;

    let res = users::Entity::find()
        .filter(Column::Username.eq(claim.username))
        .one(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();
    if res.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(next.run(request).await)
}

fn get_token(request: &Request<Body>) -> String {
    request
        .headers()
        .get(header::AUTHORIZATION)
        .unwrap()
        .to_str()
        .unwrap()
        .split_once(" ")
        .map(|(_, s)| s)
        .unwrap()
        .to_string()
}
