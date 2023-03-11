use std::sync::Arc;

use super::routes::delete as delete_user;
use axum::{routing::delete, Router};
use mongodb::Database;

use crate::middleware::auth::Auth;

pub fn configure() -> Router<Arc<Database>> {
    Router::new()
        .route("/delete", delete(delete_user))
        .layer(Auth)
}