use CustomServer_lib::RecvHandler;
use std::collections::HashMap;
use std::future::Future;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use user::User;
use lazy_static::lazy_static;
use Commands::{Command, MessageD};
use CustomServer_lib::{CustomServer, DefaultRecvHandler};
use CustomSocket_lib::CustomSocket;
use CustomSocket_lib::timeout_handler::TimeoutHandler;
use custom_errors::CustomError;

mod server_header;
mod recv_handler;
mod timeout_handler;

use server_header::ServerHeader;
pub use crate::server::recv_handler::MyRecvHandler;
pub use crate::server::SendObj::SERVER;
pub use crate::server::timeout_handler::MyTimeoutHandler;

lazy_static!(
    static ref COUNTER: Mutex<i32> = Mutex::new(0);
);

pub enum SendObj {
    SERVER,
    CLIENT(i32)
}

pub struct Client {
    pub header: Arc<Mutex<ServerHeader>>,
    // with id -1 always must be a server, it can be empty but it is reserved to a server
    pub users: Arc<RwLock<HashMap<i32, User>>>,
    pub server: CustomServer<MyTimeoutHandler, MyRecvHandler>,
}

impl Client
{
    pub async fn new(addr: String, recv_port: u16, send_port: u16, timeout_handler: MyTimeoutHandler, recv_handler: MyRecvHandler) -> Self {
        let users: Arc<RwLock<HashMap<i32, User>>> = Arc::new(RwLock::new(HashMap::new()));
        let mut _server = CustomServer::new(addr.clone(), recv_port, addr.clone(), send_port, timeout_handler, recv_handler).await;
        let mut header = ServerHeader::new();
        header.send = (addr.clone(), send_port);
        header.recv = (addr, recv_port);

        Client {
            header: Arc::new(Mutex::new(header)),
            users,
            server: _server,
        }
    }

    pub async fn connect_main_server (&self, ms: User) -> Result<(), CustomError> {
        println!("Connection to main server");
        {
            let mut users_g = self.users.write().await;
            users_g.insert(-1, ms);
        }
        let _header = self.header.clone();
        let _header_g = _header.lock().await;
        let recv = _header_g.recv.clone();
        let send = _header_g.send.clone();


        let command = Command::Connect {
            addr_recv: recv,
            addr_send: send,
            password: false,
            add_info: "".to_string(),
        };

        self.send(SERVER, command).await
    }

    async fn connect_user(&self, user: User) {
        let user_id = {
            let mut _counter = COUNTER.lock().await;
            let result = _counter.clone();
            *_counter += 1;
            result
        };

        let mut users = self.users.write().await;
        users.insert(user_id, user);
    }

    async fn disconnect_user(&self, user_id: i32) {
        let mut users = self.users.write().await;
        users.remove(&user_id);
    }

    pub async fn send(&self, send_obj: SendObj, cmd: Command) -> Result<(), CustomError> {
        // send to a user from a hasp map
        // get a send obj
        let user_id =  match send_obj {
            SendObj::SERVER => {-1 }
            SendObj::CLIENT(id) => {id}
        };

        //get a user info
        let s_user_recv;
        {
            let users_g = self.users.read().await;
            s_user_recv = match users_g.get(&user_id) {
                None => {
                    return Err(CustomError::NoUserWithThisId)
                }
                Some(user) => {
                    user.get_recv_addr().clone()
                }
            };

            drop(users_g)
        }
        //send
        println!("{}, {}, {:?}", s_user_recv.0, s_user_recv.1, cmd);
        self.server.send(s_user_recv.0, s_user_recv.1, cmd.to_vec()).await;

        Ok(())
    }
}