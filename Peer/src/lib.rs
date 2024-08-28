pub mod peer;

use std::ops::Deref;
use std::sync::{Arc, RwLock};
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use Commands::Command;
use custom_errors::CustomError;
pub use crate::peer::Peer;

//TODO change this struct!!!!
#[derive(Clone)]
pub struct Server
{
    recv_socket: Arc<Mutex<UdpSocket>>,
    send_socket: Arc<Mutex<UdpSocket>>,
    addr: String,
    recv_port: u16,
    send_port: u16,
}

fn handler(cmd: Command) {
    match cmd {
        Connect => {
            println!("{:?}", Connect);
        },
        Disconnect => {
            println!("Disconnect");
        },
    }
}
impl Server
{
    pub async fn new(addr: String, recv_port: u16, send_port: u16) -> Self{
        let _recv_socket = Arc::new(Mutex::new(UdpSocket::bind(format!("{}:{}", addr, recv_port)).await.unwrap()));
        let _send_socket = Arc::new(Mutex::new(UdpSocket::bind(format!("{}:{}", addr, send_port)).await.unwrap()));

        Server {
            send_socket: _send_socket,
            recv_socket: _recv_socket,
            addr,
            recv_port,
            send_port,
        }
    }
}

impl Peer for Server
{
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

impl Drop for Server
{
    fn drop(&mut self) {
        //TODO send to all users that server is shutodown
        //Drop the socket
    }
}