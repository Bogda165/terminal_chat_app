use std::fmt::Formatter;

#[derive(Debug)]
pub(crate) enum CustomError {
    InvalidCommand,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::InvalidCommand => {
                write!(f, "Invalid Command, kys")
            }
        }
    }
}