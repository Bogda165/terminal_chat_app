use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use CustomServer_lib::RecvHandler;
use tokio::sync::{Mutex, RwLock};
use Commands::{Command, MessageD};
use user::User;
use crate::server::Client;
use crate::server::server_header::ServerHeader;

pub struct MyRecvHandler {
    header: Arc<Mutex<ServerHeader>>,
    users: Arc<RwLock<HashMap<i32, User>>>,
}

impl MyRecvHandler {
    pub fn new() -> Self {
        MyRecvHandler {
            header: Arc::new(Mutex::new(ServerHeader::new())),
            users: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn get_from_server(server: Arc<Client>) -> Self {
        MyRecvHandler {
            header: server.header.clone(),
            users: server.users.clone(),
        }
    }

    pub async fn show_users(&self) {
        println!("Printing all users:");
        let users = self.users.clone();
        let users_g = users.write().await;
        for i in users_g.iter().clone() {
            println!("User: {:?}", i);
        }
    }
}



impl RecvHandler for MyRecvHandler {
    fn on_recv(&self, data: Vec<u8>) -> impl Future<Output=()> + Send + Sync {
        #[cfg(debug_assertions)]
        println!("recv");
        let mut cmd = match Command::from_vec(data) {
            Ok(cmd) => {
                cmd
            },
            Err(err) => {panic!("{}", err)},
        };
        async move {
            match &cmd {
                Command::Connect { .. } => {
                    #[cfg(debug_assertions)]
                    println!("Recv connect command");
                    let (user, id) = User::from_command(cmd).unwrap();

                    let id = match u16::from_str(id.as_str()){
                        Ok(id) => {id}
                        Err(_) => {
                            println!("Additional information does not contain id");
                            return;
                        }
                    };
                    //check
                    let mut header_g = self.header.lock().await;
                    if (header_g.id == None) && (user.get_recv_addr() == header_g.recv && user.get_send_addr() == header_g.send) {
                        header_g.id = Some(id);
                        #[cfg(debug_assertions)]
                        println!("Id changed to some)()()");
                        return;
                    }

                    //add to hasp map
                    drop(header_g);
                    let users = self.users.clone();
                    let mut users_g = users.write().await;

                    users_g.insert(id as i32, user);
                }
                Command::Disconnect { .. } => {
                    self.show_users().await;
                    return;
                }
                Command::Message {id, data} => {
                    match data {
                        MessageD::Text { message } => {
                            //get ip of the user
                            let (ip, port) =
                                {
                                    let users = self.users.read().await;
                                    let user = users.get(id).unwrap();
                                    //TODO no users exeption
                                    user.get_recv_addr()
                                };
                            println!("{}:{} -> {}", ip, port, message);
                        }
                        MessageD::File { .. } => {
                            println!("file received");
                            unreachable!()
                        }
                        _ => {
                            panic!("Unknown type of the command")
                        }
                    }
                }
                _ => {
                    panic!("Unknown command")
                }
            }
        }
    }
}