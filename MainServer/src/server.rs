use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use tokio::sync::Mutex;
use user::User;
use lazy_static::lazy_static;
use Commands::Command;
use Peer::Server;

lazy_static!(
    static ref COUNTER: Mutex<i32> = Mutex::new(0);
);
//TODO wrap sockets in the struct!!! with a custom function, to handle the result
pub struct MainServer
{
    users: Arc<RwLock<HashMap<i32, User>>>,
    pub server: Server,
}

impl MainServer
{
    pub async fn new(addr: String, recv_port: u16, send_port: u16) -> Self {
        let users: Arc<RwLock<HashMap<i32, User>>> = Arc::new(RwLock::new(HashMap::new()));
        let mut _server = Server::new(addr, recv_port, send_port).await;

        MainServer {
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

        let mut users = self.users.write().unwrap();
        users.insert(user_id, user);
    }

    async fn disconnect_user(&self, user_id: i32) {
        let mut users = self.users.write().unwrap();
        users.remove(&user_id);
    }
}