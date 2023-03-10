use super::routes::index;
use crate::middleware::auth::Auth;
use axum::{routing::get, Router};

pub fn configure() -> Router {
    Router::new().route("/", get(index)).layer(Auth)
}
