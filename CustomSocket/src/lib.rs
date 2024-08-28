use std::net::UdpSocket;
use std::sync::RwLock;

enum SocketType {
    Recv,
    Send,
}

struct CustomSocket {
    socket_addr: String,
    port: u16,
    socket: RwLock<Option<UdpSocket>>,
    s_type: SocketType,
}

impl CustomSocket {
    fn new(socket_addr: String, port: u16, s_type: SocketType) -> Self {
        CustomSocket {
            socket_addr,
            port,
            s_type,
            socket: RwLock::new(None),
        }
    }

    fn connect(&mut self) -> Result<Ok, Err>{
        let new_socket = UdpSocket::bind(format!("{}:{}", self.socket_addr, self.port)).unwrap();
        let mut tmp = self.socket.write().unwrap();
        *tmp = Some(new_socket);

        Ok(())
    }

    async fn recv(&mut self, buffer: &mut [u8]) -> Result<i32, String> {
        match self.s_type {
            SocketType::Recv => {
                let socket = self.socket.read().unwrap();
                match *socket {
                    None => Err("Socket hasn't been created".to_string()),
                    Some(ref socket) => {
                        match socket.recv(buffer) {
                            Ok(bytes_received) => Ok(bytes_received as i32),
                            Err(e) => Err(e.to_string()),
                        }
                    }
                }
            }
            SocketType::Send => {
                Err("Uncorrect socket type".to_string())
            }
        }
    }

    async fn send(&self, addr: String, port: u16, buffer: &[u8]) {
        let _socket = self.get_send_socket().lock().await;
        //TODO change later to use a custom sender crate!!!! IDEA is to create a custom handler, that will deal with hube packets of data!!!

        _socket.send_to(buffer, format!("{}:{}", addr, port)).await.unwrap();
    }
}
