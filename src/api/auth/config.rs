use axum::{routing::post, Router};
use crate::middleware::redirect::Redirect;
use super::routes::{login, register};

pub fn configure() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .layer(Redirect)
}
