mod utils;
mod server;
mod database;
mod config;

use futures::executor::block_on;

fn main() {
    block_on(server::startServer());
}