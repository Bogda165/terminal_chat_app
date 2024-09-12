mod server;

use std::str::FromStr;
use std::sync::Arc;
use CustomServer_lib::{CustomServer, DefaultRecvHandler};
use tokio::io;
use tokio::io::AsyncBufReadExt;
use tokio::net::*;
use Commands;
use Commands::Command;
use Commands::Command::Disconnect;
use crate::server::{MainServer, MyRecvHandler, MyTimeoutHandler};

#[tokio::main]
//use udp safe protocols will be written later!!!
async fn main() {
    let timeout_handler = MyTimeoutHandler::new();
    let receive_handler = MyRecvHandler::new();

    let mut server = Arc::new(MainServer::new("127.0.0.1".parse().unwrap(), 8090, 8091, timeout_handler, receive_handler).await);

    server.server.timeout_handler.lock().await.set_socket(server.server.get_ss());

    let _server = server.clone();
    let recv = _server.server.start();

    let __server = server.clone();
    let send = tokio::spawn(async move {
        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin).lines();

        while let Ok(Some(line)) = reader.next_line().await {
            println!("{}", line);
            let cmd;
            match line.chars().next().unwrap() {
                '0' => {
                    let command_line = &line[1..];
                    let parts: Vec<_> = command_line.split(",").collect();

                    cmd = Command::Connect {
                        addr_recv: (parts[0].to_string(), u16::from_str(parts[1]).unwrap()),
                        addr_send: (parts[2].to_string(), u16::from_str(parts[3]).unwrap()),
                        password: false,
                    };
                },
                '1' => {
                    cmd = Disconnect { addr: "".to_string(), port: 0 }
                },
                _ => {
                    panic!("Error command is not recognized");
                }
            }
            __server.server.send("127.0.0.1".to_string(), 8081, cmd.to_vec()).await;
        }
    });

    tokio::join!(recv, send);
}
