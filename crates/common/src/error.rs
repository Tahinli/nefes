use bitcode::{Decode, Encode};

use crate::{error_display, error_from};

pub mod community;
pub mod decode;
pub mod file_operation;
pub mod macro_;
pub mod message;
pub mod network;
pub mod user;

#[derive(Debug, Clone, Encode, Decode)]
pub enum Error {
    Authenticate,
    User(user::Error),
    Community(community::Error),
    Message(message::Error),
    FileOperation(file_operation::Error),
    Network(network::Error),
    Decode(decode::Error),
    Server,
}

impl std::error::Error for Error {}

error_display! { Error,
   delegate = [Error::User, Error::Message, Error::Community, Error::FileOperation, Error::Network, Error::Decode],
   unit = [Error::Authenticate => "Authenticate", Error::Server => "Server"],
}

error_from! { Error {
    user::Error => Error::User,
    community::Error => Error::Community,
    message::Error => Error::Message,
    file_operation::Error => Error::FileOperation,
    network::Error => Error::Network,
    decode::Error => Error::Decode,

    std::net::AddrParseError => Error::Network as network::Error,
    std::io::Error => Error::Network as network::Error,
    quinn::ConnectionError => Error::Network as network::Error,
    quinn::ConnectError => Error::Network as network::Error,
    quinn::ReadError => Error::Network as network::Error,
    quinn::ReadToEndError => Error::Network as network::Error,
    quinn::ReadExactError => Error::Network as network::Error,
    quinn::WriteError => Error::Network as network::Error,
    quinn::ClosedStream => Error::Network as network::Error,
    bitcode::Error => Error::Decode as decode::Error,
} }
