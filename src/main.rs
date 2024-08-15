use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let addr = "46.211.1.28:60125.";
    let msg = b"Hello, UDP!";

    socket.send_to(msg, addr).await?;
    println!("Message sent to {}", addr);

    Ok(())
}