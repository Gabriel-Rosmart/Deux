use std::sync::Arc;

use super::routes::index;
use crate::middleware::auth::Auth;
use axum::{routing::get, Router};
use tokio::sync::Mutex;
use crate::shared::state::AppState;

pub fn configure() -> Router<Arc<Mutex<AppState>>> {
    Router::new().route("/", get(index)).layer(Auth)
}
