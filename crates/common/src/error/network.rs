use std::fmt::Display;

use bitcode::{Decode, Encode};

use crate::error_from_display;

#[derive(Debug, Clone, Encode, Decode)]
pub enum Error {
    AddressParse(String),
    InputOutput(String),
    Connect(String),
    Connection(String),
    Read(String),
    ReadToEnd(String),
    ReadExact(String),
    Write(String),
    ClosedStream(String),
    ConnectionError(String),
    ChannelClosed,
    ReadBoundExceed(usize, usize),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AddressParse(error_value) => error_value.fmt(f),
            Error::InputOutput(error_value) => error_value.fmt(f),
            Error::Connect(error_value) => error_value.fmt(f),
            Error::Connection(error_value) => error_value.fmt(f),
            Error::Read(error_value) => error_value.fmt(f),
            Error::ReadToEnd(error_value) => error_value.fmt(f),
            Error::ReadExact(error_value) => error_value.fmt(f),
            Error::Write(error_value) => error_value.fmt(f),
            Error::ClosedStream(error_value) => error_value.fmt(f),
            Error::ConnectionError(error_value) => error_value.fmt(f),
            Error::ChannelClosed => write!(f, "Channel Closed"),
            Error::ReadBoundExceed(actual, expected) => write!(
                f,
                "Read Bound Exceed | Actual = {} | Expected = {}",
                actual, expected
            ),
        }
    }
}

error_from_display! { Error {
    std::net::AddrParseError => Error::AddressParse,
    std::io::Error => Error::InputOutput,
    quinn::ConnectionError => Error::Connection,
    quinn::ConnectError => Error::Connect,
    quinn::ReadError => Error::Read,
    quinn::ReadToEndError => Error::ReadToEnd,
    quinn::ReadExactError => Error::ReadExact,
    quinn::WriteError => Error::Write,
    quinn::ClosedStream => Error::ClosedStream,
} }
