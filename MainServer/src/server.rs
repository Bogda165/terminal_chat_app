use CustomServer_lib::RecvHandler;
use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use user::User;
use lazy_static::lazy_static;
use Commands::Command;
use CustomServer_lib::{CustomServer, DefaultRecvHandler};
use CustomSocket_lib::CustomSocket;
use CustomSocket_lib::timeout_handler::{DefaultTimeoutHandler, TimeoutHandler};
use serde::Deserialize;
use custom_errors::CustomError;
pub struct MyTimeoutHandler {
    socket_send: Option<Arc<CustomSocket>>,
}

impl MyTimeoutHandler {
    pub(crate) fn set_socket(&mut self, socket: Arc<CustomSocket>) {
        self.socket_send = Some(socket);
    }

    pub(crate) fn new() -> Self {
        MyTimeoutHandler {
            socket_send : None,
        }
    }
}

impl TimeoutHandler for MyTimeoutHandler {
    fn timeouts_handler(&mut self, timeouts: Vec<String>) -> impl Future<Output=()> + Send + Sync {
        async {
            for timeout in timeouts {
                println!("timeout {}", timeout);
                let a_m: Vec<&str> = timeout.split("|").collect();
                let i_p: Vec<&str> = a_m[0].clone().split(":").collect();
                let (ip, port) = (i_p[0].clone().to_string(), i_p[1].clone().parse::<u16>().unwrap());
                println!("Try to send on {}:{}", ip, port);
                match &self.socket_send {
                    None => {
                        panic!("There is no socket in timeouthandler")
                    }
                    Some(socket) => {
                        socket.send(ip.to_string(), port, "Timeout".as_bytes().to_vec(), 100).await.unwrap()
                    }
                }
            }
        }
    }
}

pub struct MyRecvHandler {
    users: Arc<RwLock<HashMap<i32, User>>>,
    send_queue: Arc<Mutex<VecDeque<(String, u16, Vec<u8>)>>>,
}

impl MyRecvHandler {
    pub fn new() -> Self {
        MyRecvHandler {
            send_queue : Arc::new(Mutex::new(VecDeque::new())),
            users: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn get_from_server(server: Arc<MainServer>) -> Self {
        MyRecvHandler {
            send_queue: server.server.send_queue.clone(),
            users: server.users.clone()
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
//notify all users that user is connected
    pub async fn notify(&self, new_user_id: i32) {
        #[cfg(debug_assertions)]
        println!("notify");
        let ug = self.users.read().await;

        let new_user = match ug.get(&new_user_id) {
            None => { panic!("WTF USER has not been added to the map")}
            Some(_user) => {_user}
        };

        let mut cmd;

        for user in &*ug {
            cmd = new_user.to_command(Command::Connect{
                addr_recv: ("".to_string(), 0),
                addr_send: ("".to_string(), 0),
                password: false,
                add_info: "".to_string(),
            }, new_user_id.to_string()).unwrap();
            {
                let send_queue = self.send_queue.clone();
                let mut sqg = send_queue.lock().await;
                #[cfg(debug_assertions)]
                println!("Added to a queue");
                sqg.push_back((user.1.get_recv_addr().0, user.1.get_recv_addr().1, cmd.to_vec()));
            }
        }
    }

    pub async fn send_table(&self, new_user: &(User, String)) {
        #[cfg(debug_assertions)]
        println!("Send table of users to user~");

        let ug = self.users.read().await;
        let mut cmd;

        for user in &*ug {
            cmd = user.1.to_command(Command::Connect{
                addr_recv: ("".to_string(), 0),
                addr_send: ("".to_string(), 0),
                password: false,
                add_info: "".to_string(),
            }, user.0.to_string()).unwrap();
            {
                let send_queue = self.send_queue.clone();
                let mut sqg = send_queue.lock().await;
                //println!("");
                sqg.push_back((new_user.0.get_recv_addr().0, new_user.0.get_recv_addr().1, cmd.to_vec()));
            }
        }
    }
}

impl RecvHandler for MyRecvHandler {
    fn on_recv(&self, data: Vec<u8>) -> impl Future<Output=()> + Send + Sync {
        let mut cmd = match Command::from_vec(data) {
            Ok(cmd) => {
                cmd
            },
            Err(err) => {panic!("{}", err)},
        };
        async move {
            match cmd {
                Command::Connect { .. } => {
                    let user = User::from_command(cmd).unwrap();
                    println!("User: {:?}", user);
                    //send all users to a new user
                    self.send_table(&user).await;
                    let id: i32;
                    {
                        {
                            let mut id_g = COUNTER.lock().await;
                            id = *id_g;
                            *id_g += 1;
                        }
                        let users = self.users.clone();
                        let mut users_g = users.write().await;
                        users_g.insert(id, user.0);
                        #[cfg(debug_assertions)]
                        println!("Added a user");
                    }
                    //notify all users
                    // TODO do I need to spawn this trait?
                    self.notify(id).await;
                }
                Command::Disconnect { .. } => {
                    self.show_users().await;
                    return;
                }
                _ => {
                    panic!("Error in commands");
                }
            }
        }
    }
}

lazy_static!(
    static ref COUNTER: Mutex<i32> = Mutex::new(0);
);

pub struct MainServer {
    users: Arc<RwLock<HashMap<i32, User>>>,
    pub server: CustomServer<MyTimeoutHandler, MyRecvHandler>,
}

impl MainServer
{
    pub async fn new(addr: String, recv_port: u16, send_port: u16, timeout_handler: MyTimeoutHandler, recv_handler: MyRecvHandler) -> Self {
        let users: Arc<RwLock<HashMap<i32, User>>> = Arc::new(RwLock::new(HashMap::new()));
        let mut _server = CustomServer::new(addr.clone(), recv_port, addr, send_port, timeout_handler, recv_handler).await;
        //let mut send_queue = Arc::new(Mutex::new(VecDeque::new()));

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

        let mut users = self.users.write().await;
        users.insert(user_id, user);
    }

    async fn disconnect_user(&self, user_id: i32) {
        let mut users = self.users.write().await;
        users.remove(&user_id);
    }
}