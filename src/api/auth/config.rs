use axum::{
    Router,
    routing::{ get, post }
};

use super::routes::{ login, register };

pub fn configure() -> Router {
    Router::new()
        .route("/login", get(login))
        .route("/register", post(register))
}