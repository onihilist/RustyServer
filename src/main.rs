mod utils;
mod server;
mod database;
mod config;

#[tokio::main]
async fn main() {
    server::startServer().await.unwrap();
}