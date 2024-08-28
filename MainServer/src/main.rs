mod server;
use tokio::net::*;
use Commands;
use ::Peer::Server;
use Commands::Command;
use Peer::peer::Peer;
use crate::server::MainServer;

fn handler(cmd: Command) {
    match cmd {
        Connect => {
            println!("{:?}", Connect);
        },
        Disconnect => {
            println!("Disconnect");
        },
    }
}

#[tokio::main]
//use udp safe protocols will be written later!!!
async fn main() {
    let mut server: MainServer = MainServer::new("127.0.0.1".parse().unwrap(), 8081, 8082).await;
    server.server.run(&handler).await;
}
