use std::fmt::{write, Formatter};

#[derive(Debug)]
pub enum CustomError {
    InvalidCommand,
    FailedConverting,
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
            _ => {
                write!(f, "WTF??")
            }
        }
    }
}