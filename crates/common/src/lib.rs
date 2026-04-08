use std::sync::Arc;

use crate::{
    error::Error,
    validate::{validate_message, validate_username},
};

pub mod constant;
pub mod error;
pub mod validate;

#[derive(Debug)]
struct UserInner {
    id: u8,
    username: String,
}

impl UserInner {
    pub fn new(username: impl ToString) -> Result<Self, Error> {
        let username = username.to_string();
        validate_username(&username)?;

        Ok(Self { id: 0, username })
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }
}

#[derive(Debug, Clone)]
pub struct User {
    user: Arc<UserInner>,
}

impl User {
    pub fn new(username: impl ToString) -> Result<Self, Error> {
        Ok(Self {
            user: UserInner::new(username)?.into(),
        })
    }

    pub fn get_id(&self) -> u8 {
        self.user.get_id()
    }

    pub fn get_username(&self) -> &String {
        &self.user.get_username()
    }
}

#[derive(Debug)]
pub struct Message {
    sender: User,
    message: String,
}

impl Message {
    pub fn new(sender: User, message: impl ToString) -> Result<Self, Error> {
        let message = message.to_string();
        validate_message(&message)?;

        Ok(Self { sender, message })
    }

    pub fn get_sender(&self) -> &User {
        &self.sender
    }

    pub fn get_message(&self) -> &String {
        &self.message
    }
}
