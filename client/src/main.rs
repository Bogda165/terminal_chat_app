mod server;

use std::iter::Cycle;
use tokio::net::*;
use Commands;
use ::Peer::Server;
use Peer::peer::Peer;
use crate::server::Client;

#[tokio::main]
//use udp safe protocols will be written later!!!
async fn main() {
    let mut client =  Client::new("127.0.0.1".parse().unwrap(), 8079, 8078).await;
    client.try_connect(("127.0.0.1".to_string(), 8081, false)).await;
    println!("Request has been sent");
}
