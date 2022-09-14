mod core;
mod server;
mod utils;

use crate::{core::config::Config, server::server::ProxyServer};

fn main() {
    let config = Config::load().unwrap();

    let server = ProxyServer::new(&config).unwrap();

    server.start().unwrap();
}
