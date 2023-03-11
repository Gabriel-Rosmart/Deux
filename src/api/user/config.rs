use std::sync::Arc;

use super::routes::index;
use crate::middleware::auth::Auth;
use axum::{routing::get, Router};
use mongodb::Database;

pub fn configure() -> Router<Arc<Database>> {
    Router::new().route("/", get(index)).layer(Auth)
}
