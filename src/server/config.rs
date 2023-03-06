use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

pub struct ServerConfig;

impl ServerConfig {
    pub fn init() -> SocketAddr {
        dotenv().ok();

        let addr = env::var("SERVER_ADDR").expect("No SERVER_ADDR env variable");
        let port = env::var("SERVER_PORT").expect("No SERVER_PORT env variable");

        format!("{addr}:{port}")
            .parse::<SocketAddr>()
            .expect("Failed to parse server config")
    }
}
