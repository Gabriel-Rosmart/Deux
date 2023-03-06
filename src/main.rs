mod api;
mod server;
mod db;
mod extractors;
mod crypto;
mod models;

use axum::{Router, Extension};

use server::config::ServerConfig;
use db::mongo::Mongo;
use api::auth::config::configure as auth;

#[tokio::main]
async fn main() {


    let db = Mongo::init().await.unwrap();

    let routes = Router::new().nest("/auth", auth());

    let app = Router::new().nest("/api", routes).layer(Extension(db));

    let addr = ServerConfig::init();
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}