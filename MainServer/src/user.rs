use crate::custom_errors::CustomError;
use Commands::Command;
#[derive(Debug)]
pub (crate)struct User {
    addr: String,
    port: u16,
}

impl User {
    fn new() -> Self {
        User {
            addr: "0.0.0.0".to_string(),
            port: 0,
        }
    }

    fn new_from(_addr: String, _port: u16 ) -> Self {
        User {
            addr: _addr,
            port: _port,
        }
    }

    fn from_command(cmd: Command) -> Result<Self, CustomError> {
        match cmd {
            Command::Connect {addr, port, password: _}=> {
                Ok(User {
                    addr,
                    port,
                })
            },
            _ => {
                Err(CustomError::InvalidCommand)
            }
        }
    }
}