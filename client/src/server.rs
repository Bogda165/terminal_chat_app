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
mod server_header;
use server_header::ServerHeader;

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

lazy_static!(
    static ref COUNTER: Mutex<i32> = Mutex::new(0);
);

pub struct Client {
    pub header: Arc<Mutex<ServerHeader>>,
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