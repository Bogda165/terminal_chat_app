mod recv_handler;
mod server_header;
mod timeout_handler;

use CustomServer_lib::RecvHandler;
use std::collections::{HashMap};
use std::future::Future;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use user::User;
use CustomServer_lib::{CustomServer};
use CustomSocket_lib::timeout_handler::{TimeoutHandler};
use serde::Deserialize;
pub use crate::server::recv_handler::{MyRecvHandler, COUNTER};
pub use crate::server::server_header::ServerHeader;
pub use crate::server::timeout_handler::MyTimeoutHandler;

pub struct MainServer {
    pub header: Arc<Mutex<ServerHeader>>,
    users: Arc<RwLock<HashMap<i32, User>>>,
    pub server: CustomServer<MyTimeoutHandler, MyRecvHandler>,
}

impl MainServer
{

    pub async fn new(addr: String, recv_port: u16, send_port: u16, timeout_handler: MyTimeoutHandler, recv_handler: MyRecvHandler) -> Self {
        let users: Arc<RwLock<HashMap<i32, User>>> = Arc::new(RwLock::new(HashMap::new()));
        let mut _server = CustomServer::new(addr.clone(), recv_port, addr.clone(), send_port, timeout_handler, recv_handler).await;
        //let mut send_queue = Arc::new(Mutex::new(VecDeque::new()));
        let mut header = ServerHeader::new();
        header.send = (addr.clone(), send_port);
        header.recv = (addr, recv_port);

        MainServer {
            header: Arc::new(Mutex::new(header)),
            users,
            server: _server,
        }
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
}