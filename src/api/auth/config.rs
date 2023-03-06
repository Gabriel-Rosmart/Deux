use axum::{routing::post, Router};

use super::routes::{login, register};

pub fn configure() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}
