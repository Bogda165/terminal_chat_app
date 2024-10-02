mod server;

use std::num::ParseIntError;
use std::str::FromStr;
use std::sync::Arc;
use CustomServer_lib::DefaultRecvHandler;
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::*;
use Commands;
use Commands::Command;
use Commands::Command::Disconnect;
use user::User;
use server::{Client, MyRecvHandler, MyTimeoutHandler, SendObj};

//use udp safe protocols will be written later!!!

async fn read_ports() -> (u16, u16) {
    let mut line = String::new();
    io::stdout().flush().await.unwrap();
    let mut reader = BufReader::new(io::stdin());
    reader.read_line(&mut line).await.unwrap();

    let ports: Vec<&str> = line.split_whitespace().collect();

    (u16::from_str(ports[0]).unwrap(), u16::from_str(ports[1]).unwrap())
}


async fn test_config(server: Arc<Client>) {
    let server = server.clone();

    server.server.timeout_handler.lock().await.set_socket(server.server.get_ss());

    {
        let recv_handler = server.server.receive_handler.clone();
        let mut recv_handler_g = recv_handler.write().await;
        *recv_handler_g = MyRecvHandler::get_from_server(server.clone());
    }

    let ms = User::new_from(("10.10.15.128".to_string(), 8090), ("10.10.15.128".to_string(), 8091),);
    server.connect_main_server(ms).await.unwrap();
}

#[tokio::main]
async fn main() {
    let (recv, send) = read_ports().await;

    println!("{}, {}", recv, send);
    let mut server = Arc::new(Client::new("10.10.15.128".parse().unwrap(), recv, send, MyTimeoutHandler::new(), MyRecvHandler::new()).await);

    test_config(server.clone()).await;

    let _server = server.clone();
    let recv = _server.server.start();

    let __server = server.clone();
    let send = tokio::spawn(async move {
        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin).lines();
        let mut current_id = -1;

        while let Ok(Some(line)) = reader.next_line().await {
            let server = __server.clone();
            println!("{}", line);
            let mut cmd = Disconnect { addr: "".to_string(), port: 0 };
            match line.as_str() {
                "dis" => {
                    cmd = Disconnect { addr: "".to_string(), port: 0 }
                },
                "message" => {
                    let line = reader.next_line().await.unwrap().unwrap();
                    let mut id;
                    {
                        let header_g = server.header.lock().await;
                        id = match header_g.id{
                            None => {
                                println!("Client is not connected to any servers");
                                continue;
                            }
                            Some(id) => {id}
                        };
                    }


                    cmd = Command::Message {
                        id: id as i32,
                        data: Commands::MessageD::Text{message: line},
                    };

                }
                "ht" => {
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

                "dos" => {
                    let mut id;
                    {
                        let header_g = server.header.lock().await;
                        id = match header_g.id{
                            None => {
                                println!("Client is not connected to any servers");
                                continue;
                            }
                            Some(id) => {id}
                        };
                    }

                    for _ in 0..1000 {
                        let cmd = Command::Message {
                            id: id as i32,
                            data: Commands::MessageD::Text { message: "Hello how are you? I am fine btw".to_string() },
                        };

                        server.send(SendObj::CLIENT(current_id), cmd).await.unwrap()
                    }

                    let cmd = Command::Message {
                        id: id as i32,
                        data: Commands::MessageD::Text {message: "it's the end".to_string() },
                    };

                    server.send(SendObj::CLIENT(current_id), cmd).await.unwrap();

                    println!("I mean it is the end");
                }

                _ => {
                    //change receiver
                    current_id = match i32::from_str(&*line){
                        Ok(id) => {id}
                        Err(_) => {panic!("Error command is not recognized");}
                    };
                    continue;
                }
            }
            //#[cfg(debug_assertions)]
            println!("Send to: {}->{:?}", current_id, cmd);
            server.send(SendObj::CLIENT(current_id), cmd).await.unwrap()
        }
    });

    tokio::join!(recv, send);
}
