use axum::{ routing::get, Router };
use super::routes::index;
use crate::middleware::auth::Auth;

pub fn configure() -> Router {
    Router::new()
        .route("/", get(index))
        .layer(Auth)
}