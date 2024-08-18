mod user;
mod custom_errors;
mod server;

use std::fmt::Formatter;
use std::io::Error;
use std::sync::Arc;
use tokio::net::*;
use Commands;
use bincode;
use Commands::Command;
use crate::server::{Peer, Server};

fn handler(buffer: &[u8]) {
    match bincode::deserialize::<Command>(buffer) {
        Ok(cmd) => {
            match cmd {
                Connect => {
                    println!("Connect");


                },
                Disconnect => {
                    println!("Disconnect");
                },
            }
        },

        Err(_) => {
            println!("Error while serialization");
        }
    }
}

#[tokio::main]
//use udp safe protocols will be written later!!!
async fn main() {

    let udp_socket = Arc::new(UdpSocket::bind("127.0.0.1:8080").await.unwrap());

    let server = Server::new("127.0.0.1".parse().unwrap(), 8081, 8082).await;
    server.run().await;

}
