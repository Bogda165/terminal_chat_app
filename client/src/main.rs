mod server;

use std::iter::Cycle;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::*;
use Commands;
use ::Peer::Server;
use Commands::Command;
use Peer::peer::Peer;
use crate::server::Client;

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
    let mut client = Client::new("127.0.0.1".parse().unwrap(), 8079, 8078).await;
    client.set_main_server(("127.0.0.1".to_string(), 8081, false));
    let mut client = Arc::new(client);
    let mut _client = Arc::clone(&client);

    tokio::spawn(async move {
        client.server.run(handler).await;
    });

    _client.try_connect().await;
}
