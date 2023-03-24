use axum::{http::StatusCode, Json};
use serde::Deserialize;

use crate::user::model::User;

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}
