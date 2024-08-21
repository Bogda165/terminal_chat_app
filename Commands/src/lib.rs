use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Connect {addr: String, port: u16, password: bool},
    Disconnect {addr: String, port: u16},
}