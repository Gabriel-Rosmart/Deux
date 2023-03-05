mod api;
mod server;

use axum::Router;

use server::server::Server;
use api::auth::config::configure as auth;

#[tokio::main]
async fn main() {

    let routes = Router::new().nest("/auth", auth());

    let app = Router::new().nest("/api", routes);

    let addr = Server::init();
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
