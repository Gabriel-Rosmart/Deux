use std::sync::Arc;

use super::routes::{login, register};
use crate::middleware::redirect::Redirect;
use axum::{routing::post, Router};
use crate::shared::state::AppState;
use tokio::sync::Mutex;

pub fn configure() -> Router<Arc<Mutex<AppState>>> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .layer(Redirect)
}
