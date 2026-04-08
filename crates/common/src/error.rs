use std::fmt::Display;

pub mod message;
pub mod user;

#[derive(Debug)]
pub enum Error {
    User(user::Error),
    Message(message::Error),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::User(error_value) => error_value.fmt(f),
            Error::Message(error_value) => error_value.fmt(f),
        }
    }
}

impl From<user::Error> for Error {
    fn from(value: user::Error) -> Self {
        Error::User(value)
    }
}

impl From<message::Error> for Error {
    fn from(value: message::Error) -> Self {
        Error::Message(value)
    }
}
