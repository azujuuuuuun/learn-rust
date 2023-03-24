use axum::{http::StatusCode, Json};
use serde::Deserialize;

use crate::user::model::User;

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

pub async fn find_all() -> (StatusCode, Json<Vec<User>>) {
    let users = vec![
        User {
            id: 1337,
            username: "user1".to_string(),
        },
        User {
            id: 1338,
            username: "user2".to_string(),
        },
    ];

    (StatusCode::OK, Json(users))
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}
