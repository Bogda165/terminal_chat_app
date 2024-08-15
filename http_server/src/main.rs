use stunclient::StunClient;
use std::net::{ToSocketAddrs, UdpSocket};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // STUN server address (Google's public STUN server)
    let stun_server = "stun3.l.google.com:19302";

    // Resolve the STUN server address
    let addr = stun_server.to_socket_addrs()?.next().ok_or("Failed to resolve address")?;

    // Create a STUN client
    let client = StunClient::new(addr);

    // Create a UDP socket bound to a local address
    let local_addr = "172.20.10.2:8009"; // Use a wildcard address to bind to any available port
    let udp_socket = UdpSocket::bind(local_addr)?;

    // Perform the STUN request to get public IP and port
    match client.query_external_address(&udp_socket) {
        Ok(external_addr) => {
            println!("Public IP: {}", external_addr.ip());
            println!("Public Port: {}", external_addr.port());
        },
        Err(e) => {
            eprintln!("Failed to query external address: {}", e);
        }
    }

    Ok(())
}