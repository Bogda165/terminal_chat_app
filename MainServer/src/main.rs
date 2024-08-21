mod server;
use tokio::net::*;
use Commands;
use ::Peer::Server;
use Peer::peer::Peer;
use crate::server::MainServer;

#[tokio::main]
//use udp safe protocols will be written later!!!
async fn main() {
    let server = MainServer::new("127.0.0.1".parse().unwrap(), 8081, 8082).await;
    server.server.run().await;
}
