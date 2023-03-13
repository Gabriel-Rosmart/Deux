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

use tokio::sync::Mutex;
use crate::shared::state::AppState;

use crate::middleware::auth::Auth;

pub fn configure() -> Router<Arc<Mutex<AppState>>> {
    Router::new()
        .route("/delete", delete(delete_user))
        .route("/update", put(update))
        .layer(Auth)
}
