use std::fmt::Display;

use common::error_from;

pub mod database;

#[derive(Debug)]
pub enum Error {
    Common(common::error::Error),
    Database(database::Error),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Common(error_value) => error_value.fmt(f),
            Error::Database(error_value) => error_value.fmt(f),
        }
    }
}

impl From<Error> for common::error::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Common(error_value) => error_value,
            Error::Database(error_value) => {
                eprintln!("Error: Database | {}", error_value);

                common::error::Error::Server
            }
        }
    }
}

error_from! { Error {
    common::error::Error => Error::Common,
    database::Error => Error::Database,

    common::error::user::Error => Error::Common as common::error::Error,
    common::error::community::Error => Error::Common as common::error::Error,
    common::error::message::Error => Error::Common as common::error::Error,

    surrealdb::Error => Error::Database as database::Error,

    std::net::AddrParseError => Error::Common as common::error::network::Error,
    std::io::Error => Error::Common as common::error::network::Error,
    quinn::ConnectionError => Error::Common as common::error::network::Error,
    quinn::ConnectError => Error::Common as common::error::network::Error,
    quinn::ReadError => Error::Common as common::error::network::Error,
    quinn::ReadToEndError => Error::Common as common::error::network::Error,
    quinn::ReadExactError => Error::Common as common::error::network::Error,
    quinn::WriteError => Error::Common as common::error::network::Error,
    quinn::ClosedStream => Error::Common as common::error::network::Error,
    bitcode::Error => Error::Common as common::error::decode::Error,
} }
