mod utils;
mod server;
mod database;
mod config;
mod tests;

use std::env;

#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "0");
    server::startServer().await.unwrap();
}