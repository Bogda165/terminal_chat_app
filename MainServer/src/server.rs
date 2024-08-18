use std::collections::HashMap;
use std::fmt::format;
use std::sync::{Arc, RwLock};
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use crate::user;
use crate::user::User;
use lazy_static::lazy_static;
use Commands::Command;

lazy_static!(
    static ref COUNTER: Mutex<i32> = Mutex::new(0);
);
//TODO wrap sockets in the struct!!! with a custom function, to handle the result
pub struct Server {
    users: Arc<RwLock<HashMap<i32, User>>>,
    recv_socket: Arc<Mutex<UdpSocket>>,
    send_socket: Arc<Mutex<UdpSocket>>,
    addr: String,
    recv_port: u16,
    send_port: u16,

}

impl Server {
    pub async fn new(addr: String, recv_port: u16, send_port: u16) -> Self{
        let _recv_socket = Arc::new(Mutex::new(UdpSocket::bind(format!("{}:{}", addr, recv_port)).await.unwrap()));
        let _send_socket = Arc::new(Mutex::new(UdpSocket::bind(format!("{}:{}", addr, send_port)).await.unwrap()));

        Server {
            send_socket: _send_socket,
            recv_socket: _recv_socket,
            addr,
            recv_port,
            send_port,
            users: Arc::new(RwLock::new(HashMap::<i32, User>::new())),
        }
    }

    pub async fn connect_user(&self, user: User) {
        let user_id = {
            let mut _counter = COUNTER.lock().await;
            let result = _counter.clone();
            *_counter += 1;
            result
        };

        let mut users = self.users.write().unwrap();
        users.insert(user_id, user);
    }

    pub async fn disconnect_user(&self, user_id: i32) {
        let mut users = self.users.write().unwrap();
        users.remove(&user_id);
    }
}

pub trait Peer {
    async fn run(&self) {
        loop {
            let mut buffer = [0; 1024];
            let socket = Arc::clone(self.get_recv_socket());
            let socket = socket.lock().await;

            match socket.recv(&mut buffer).await {
                Ok(buffer_size) => {
                    tokio::spawn(async move {
                        //TODO handler should be taken from self written crate
                        match bincode::deserialize::<Command>(&buffer[..buffer_size]) {
                            Ok(cmd) => {
                                match cmd {
                                    Connect => {
                                        println!("Connect");
                                    },
                                    Disconnect => {
                                        println!("Disconnect");
                                    },
                                }
                            },

                            Err(_) => {
                                println!("Error while serialization");
                            }
                        }
                    });
                }
                Err(_) => {
                    println!("Error while reading from the socket!!!");
                }
            }
        }
    }

    // buffer contains encoded information

    async fn send(&self, addr: String, port: u16, buffer: &[u8] ) {
        let _socket = self.get_send_socket().lock().await;
        //TODO change later to use a custom sender crate!!!! IDEA is to create a custom handler, that will deal with hube packets of data!!!

        _socket.send_to(buffer, format!("{}:{}", addr, port)).await.unwrap();
    }

    fn get_addr(&self) -> &String;
    fn get_send_port(&self) -> u16;
    fn get_recv_port(&self) -> u16;
    fn get_recv_socket(&self) -> &Arc<Mutex<UdpSocket>>;
    fn get_send_socket(&self) -> &Arc<Mutex<UdpSocket>>;
}

impl Peer for Server {
    fn get_addr(&self) -> &String {
        &self.addr
    }

    fn get_send_port(&self) -> u16 {
        self.send_port
    }

    fn get_recv_port(&self) -> u16 {
        self.recv_port
    }

    fn get_recv_socket(&self) -> &Arc<Mutex<UdpSocket>> {
        &self.recv_socket
    }

    fn get_send_socket(&self) -> &Arc<Mutex<UdpSocket>> {
        &self.send_socket
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        //TODO send to all users that server is shutodown
        //Drop the socket
    }
}