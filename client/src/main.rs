mod server;

use std::str::FromStr;
use std::sync::Arc;
use CustomServer_lib::DefaultRecvHandler;
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::*;
use Commands;
use Commands::Command;
use Commands::Command::Disconnect;
use crate::server::{Client, MyRecvHandler, MyTimeoutHandler};

//use udp safe protocols will be written later!!!

async fn read_ports() -> (u16, u16) {
    let mut line = String::new();
    io::stdout().flush().await.unwrap();
    let mut reader = BufReader::new(io::stdin());
    reader.read_line(&mut line).await.unwrap();

    let ports: Vec<&str> = line.split_whitespace().collect();

    (u16::from_str(ports[0]).unwrap(), u16::from_str(ports[1]).unwrap())
}

#[tokio::main]
async fn main() {
    let (recv, send) = read_ports().await;

    println!("{}, {}", recv, send);

    let timeout_handler = MyTimeoutHandler::new();
    let receive_handler = MyRecvHandler::new();

    let mut server = Arc::new(Client::new("127.0.0.1".parse().unwrap(), recv, send, timeout_handler, receive_handler).await);

    server.server.timeout_handler.lock().await.set_socket(server.server.get_ss());

    {
        let recv_handler = server.server.receive_handler.clone();
        let mut recv_handler_g = recv_handler.write().await;
        *recv_handler_g = MyRecvHandler::get_from_server(server.clone());
    }

    let _server = server.clone();
    let recv = _server.server.start();

    let __server = server.clone();
    let send = tokio::spawn(async move {
        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin).lines();

        while let Ok(Some(line)) = reader.next_line().await {
            println!("{}", line);
            let mut cmd = Disconnect { addr: "".to_string(), port: 0 };
            match line.chars().next().unwrap() {
                '0' => {
                    let command_line = &line[1..];
                    let parts: Vec<_> = command_line.split(",").collect();

                    cmd = Command::Connect {
                        addr_recv: (parts[0].to_string(), u16::from_str(parts[1]).unwrap()),
                        addr_send: (parts[2].to_string(), u16::from_str(parts[3]).unwrap()),
                        password: false,
                        add_info: "".to_string(),
                    };
                },
                '1' => {
                    cmd = Disconnect { addr: "".to_string(), port: 0 }
                },
                '2' => {
                    let command_line = &line[1..];
                    let parts: Vec<_> = command_line.split(",").collect();

                    cmd = Command::Message {
                        id: i32::from_str(parts[0]).unwrap(),
                        data: Commands::MessageD::Text{message: parts[1].to_string()},
                    };

                }
                '3' => {
                    let header = server.header.clone();
                    let users = server.users.clone();
                    tokio::spawn(async move {
                        let header = header.lock().await;
                        let users = users.read().await;

                        println!("Header: {:?}", header.id);
                        for user in &*users {
                            println!("User: {:?}", user);
                        }
                    });
                    continue;
                }
                //connect itself to a server
                '4' => {
                    let _header = __server.header.clone();
                    let _header_g = _header.lock().await;
                    let recv = _header_g.recv.clone();
                    let send = _header_g.send.clone();
                    println!("Header: {:?}", _header_g);
                    cmd = Command::Connect {
                        addr_recv: recv,
                        addr_send: send,
                        password: false,
                        add_info: "".to_string(),
                    };
                }
                _ => {
                    panic!("Error command is not recognized");
                }
            }
            __server.server.send("127.0.0.1".to_string(), 8090, cmd.to_vec()).await;
        }
    });

    tokio::join!(recv, send);
}
