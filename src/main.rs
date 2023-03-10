mod api;
mod crypto;
mod db;
mod errors;
mod extractors;
mod middleware;
mod models;
mod server;

use std::sync::Arc;

use axum::{Extension, Router};

use api::auth::config::configure as auth;
use api::user::config::configure as user;
use db::mongo::Mongo;
use server::config::ServerConfig;

#[tokio::main]
async fn main() {
    let db = Mongo::init().await.unwrap();

    let state = Arc::new(db);

    let routes = Router::new().nest("/auth", auth()).nest("/user", user());

    let app = Router::new().nest("/api", routes)
        .layer(Extension(state));

    let addr = ServerConfig::init();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}