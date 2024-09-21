use std::fmt::{write, Formatter};

#[derive(Debug)]
pub enum CustomError {
    InvalidCommand,
    FailedConverting,
    FailedToChangeHandler,
    NoUserWithThisId,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::InvalidCommand => {
                write!(f, "Invalid Command, kys")
            }
            CustomError::FailedConverting => {
                write!(f, "FailedConverting, loh")
            }
            CustomError::FailedToChangeHandler => {
                write!(f, "The server's handler function was already initialized")
            }
            CustomError::NoUserWithThisId => {
                write!(f, "There is no user with this id")
            }
            _ => {
                write!(f, "WTF??")
            }
        }
    }
}