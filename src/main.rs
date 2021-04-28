use log::info;
use rust_server::listen_to_stream;
use std::env;
use std::net::TcpListener;

fn main() {
    let address = format!(
        "{}:{}",
        env::var("SERVER_HOST").unwrap_or("127.0.0.1".to_string()),
        env::var("SERVER_PORT").unwrap_or("8080".to_string()),
    );
    info!("Started server at this address: {}", &address);
    let tcp_listener = TcpListener::bind(address).expect("Can't bind to the server.");
    listen_to_stream(tcp_listener)
}
