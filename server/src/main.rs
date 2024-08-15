use std::os::fd::AsFd;
use std::sync::Arc;
use tokio::net::{TcpListener, UdpSocket};
use std::thread;
use tokio::io::AsyncReadExt;


async fn func(data: &[u8]) {
    println!("{:?}", std::str::from_utf8(data).unwrap());
}

#[tokio::main]
async fn main() {
    let listener = Arc::new(UdpSocket::bind("172.20.10.2:8009").await.unwrap());


    loop {
        let mut buffer = [0; 1024];
        let listener = Arc::clone(&listener);
        let (_, mut socket) = listener.recv_from(&mut buffer).await.unwrap();
        println!("Accepted");

        match listener.recv_from(&mut buffer).await {
            Ok((size_buffer, _)) if size_buffer == 0 => {
                println!("buffer size if zero");
                break;
            },
            Err(e) => {
                println!("Error");
                break;
            },
            Ok((size_buffer, _)) => {
                tokio::spawn(async move {
                    func(&buffer[0..size_buffer]).await;
                })
            },
        };

    }

}
