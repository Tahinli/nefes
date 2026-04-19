use std::fmt::Display;

use crate::constant::{MAXIMUM_BASTION_NAME_LENGTH, MINIMUM_BASTION_NAME_LENGTH};

#[derive(Debug)]
pub enum Error {
    MaximumLength(usize),
    MinimumLength(usize),
    ASCII,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MaximumLength(length) => write!(
                f,
                "More than maximum length({}) is provided({})",
                MAXIMUM_BASTION_NAME_LENGTH, length
            ),
            Error::MinimumLength(length) => write!(
                f,
                "Less than minimum length({}) is provided({})",
                MINIMUM_BASTION_NAME_LENGTH, length
            ),
            Error::ASCII => write!(f, "Given input is not ASCII compatible"),
        }
    }
}
