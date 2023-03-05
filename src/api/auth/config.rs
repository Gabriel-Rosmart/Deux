use axum::{
    Router,
    routing::get
};

use super::routes::index;

pub fn configure() -> Router {
    Router::new().route("/", get(index))
}