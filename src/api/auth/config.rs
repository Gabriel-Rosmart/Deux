use std::sync::Arc;

use super::routes::{login, register};
use crate::middleware::redirect::Redirect;
use axum::{routing::post, Router};
use mongodb::Database;

pub fn configure() -> Router<Arc<Database>> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .layer(Redirect)
}
