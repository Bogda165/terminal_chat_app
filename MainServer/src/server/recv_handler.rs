use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::sync::Arc;
use CustomServer_lib::RecvHandler;
use lazy_static::lazy_static;
use tokio::sync::{Mutex, RwLock};
use Commands::Command;
use user::User;
use crate::server::MainServer;

lazy_static!(
    pub static ref COUNTER: Mutex<i32> = Mutex::new(0);
);

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
