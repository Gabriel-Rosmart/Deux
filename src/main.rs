mod api;
mod crypto;
mod db;
mod extractors;
mod models;
mod server;
mod middleware;

use axum::{Extension, Router};

use api::auth::config::configure as auth;
use db::mongo::Mongo;
use server::config::ServerConfig;

#[tokio::main]
async fn main() {
    let db = Mongo::init().await.unwrap();

    let routes = Router::new().nest("/auth", auth());

    let app = Router::new().nest("/api", routes).layer(Extension(db)).layer(middleware::auth::Auth);

    let addr = ServerConfig::init();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
