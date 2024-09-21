use tokio::io::AsyncWriteExt;
use tokio::net::{TcpSocket, UdpSocket};
use tokio::process::Command;
use CustomServer_lib::CustomServer;

async fn send(buffer: &[u8], socket: &mut UdpSocket, addr: &str) {
    socket.send_to(buffer, addr).await.unwrap();
}

//TODO create sender trait
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    let addr = "127.0.0.1:8081";

    let message = Commands::Command::Connect {
        addr: "127.0.0.2".to_string(),
        port: 8080,
        password: false,
        add_info: "".to_string(),
    };

    let message = bincode::serialize(&message).unwrap();

    send(&*message, &mut socket, addr).await;

    println!("Message sent to {}", addr);

    Ok(())
}
