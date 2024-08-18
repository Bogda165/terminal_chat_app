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
    //let listener = Arc::new(UdpSocket::bind("10.10.56.75:8089").await.unwrap());
    let listener = Arc::new(TcpListener::bind("127.0.0.1:8080").await.unwrap());

    loop {
        let mut buffer = [0; 1024];
        let listener = Arc::clone(&listener);

        //match listener.recv(&mut buffer).await {
        let (mut stream, _) = listener.accept().await.unwrap();
        match stream.read(&mut buffer).await{
            Err(e) => {
                println!("Accepted");
                println!("Error");
                break;
            },
            //Ok(size_buffer) => {
            Ok(size_buffer) => {
                println!("Accepted");
                tokio::spawn(async move {
                    func(&buffer[0..size_buffer]).await;
                })
            },
        };

    }

}
