use std::sync::Arc;

use crate::{bastion::Bastion, error::Error, user::User, validate::validate_message};

#[derive(Debug)]
struct MessageInner {
    id: String,
    user: User,
    bastion: Bastion,
    message: String,
}

impl MessageInner {
    pub fn new(user: User, bastion: Bastion, message: impl ToString) -> Result<Self, Error> {
        let message = message.to_string();
        validate_message(&message)?;

        Ok(Self {
            id: "replace".to_string(),
            user,
            bastion,
            message,
        })
    }

    pub fn get_user(&self) -> &User {
        &self.user
    }

    pub fn get_message(&self) -> &String {
        &self.message
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    message: Arc<MessageInner>,
}

impl Message {
    pub fn new(user: User, bastion: Bastion, message: impl ToString) -> Result<Self, Error> {
        Ok(Self {
            message: MessageInner::new(user, bastion, message)?.into(),
        })
    }

    pub fn get_user(&self) -> &User {
        self.message.get_user()
    }

    pub fn get_message(&self) -> &String {
        self.message.get_message()
    }
}
