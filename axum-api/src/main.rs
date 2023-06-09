mod user;

use std::net::SocketAddr;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route(
            "/users",
            get(user::handler::list_users).post(user::handler::create_user),
        )
        .route("/users/:user_id", get(user::handler::show_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
