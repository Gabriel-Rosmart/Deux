mod api;
mod crypto;
mod db;
mod errors;
mod extractors;
mod middleware;
mod models;
mod server;
mod constants;
mod cache;
mod shared;

use std::sync::Arc;

use api::auth::config::configure as auth;
use api::profile::config::configure as profile;
use api::user::config::configure as user;
use axum::Router;
use db::mongo::Mongo;
use server::config::ServerConfig;
use tokio::sync::Mutex;
use shared::state::AppState;

#[tokio::main]
async fn main() {
    let db = Mongo::init().await.unwrap();

    //let state = Arc::new(db);

    let state = Arc::new(Mutex::new(AppState::new(db)));

    let routes = Router::new()
        .nest("/auth", auth())
        .nest("/user", user())
        .nest("/profile", profile());

    let app: Router<()> = Router::new().nest("/api", routes).with_state(state);

    let addr = ServerConfig::init();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
