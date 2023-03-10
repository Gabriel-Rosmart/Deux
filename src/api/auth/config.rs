use super::routes::{login, register};
use crate::middleware::redirect::Redirect;
use axum::{routing::post, Router};

pub fn configure() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .layer(Redirect)
}
