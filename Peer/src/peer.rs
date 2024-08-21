use std::sync::Arc;
use tokio::sync::Mutex;
use bincode;
use tokio::net::UdpSocket;
use Commands::Command;

pub trait Peer {
    //TODO add a external function with custom commands to operate with
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
                                        println!("{:?}", Connect);
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
