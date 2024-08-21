use custom_errors::CustomError;
use Commands::Command;
#[derive(Debug)]
pub struct User {
    addr: String,
    port: u16,
}

impl User {
    pub fn new() -> Self {
        User {
            addr: "0.0.0.0".to_string(),
            port: 0,
        }
    }

    pub fn new_from(_addr: String, _port: u16 ) -> Self {
        User {
            addr: _addr,
            port: _port,
        }
    }

    pub fn from_command(cmd: Command) -> Result<Self, CustomError> {
        match cmd {
            Command::Connect {addr, port, password: _,}=> {
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

    pub fn to_command(&self, cmd: Command) -> Result<Command, CustomError> {
        match cmd {
            Command::Connect { .. } => {
                Ok(Command::Connect {
                    addr: self.addr.clone(),
                    port: self.port,
                    password: false,
                })
            },
            _ => {Err(CustomError::FailedConverting)}
        }
    }
}