mod api;
mod server;
mod db;
mod extractors;

use axum::{Router, Extension};

use server::server::Server;
use db::mongo::Mongo;
use api::auth::config::configure as auth;

#[tokio::main]
async fn main() {


    let db = Mongo::init().await.unwrap();

    let routes = Router::new().nest("/auth", auth());

    let app = Router::new().nest("/api", routes).layer(Extension(db));

    let addr = Server::init();
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}