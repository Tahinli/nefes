use std::fmt::Display;

use common::error_from;

#[derive(Debug)]
pub enum Error {
    Inner(surrealdb::Error),
    Create,
    Read,
    Update,
    Delete,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Inner(error) => write!(f, "{}", error),
            Error::Create => write!(f, "Creating a record failed"),
            Error::Read => write!(f, "Reading a record failed"),
            Error::Update => write!(f, "Updating a record failed"),
            Error::Delete => write!(f, "Deleting a record failed"),
        }
    }
}

error_from! { Error {
    surrealdb::Error => Error::Inner,
} }
