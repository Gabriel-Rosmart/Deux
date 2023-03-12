use std::sync::Arc;

use super::routes::{
    delete as delete_user,
    update
};
use axum::{
    routing::{
        delete,
        put
    },
    Router
};
use mongodb::Database;

use crate::middleware::auth::Auth;

pub fn configure() -> Router<Arc<Database>> {
    Router::new()
        .route("/delete", delete(delete_user))
        .route("/update", put(update))
        .layer(Auth)
}
