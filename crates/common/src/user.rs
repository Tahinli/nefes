use std::sync::Arc;

use crate::{error::Error, validate::validate_username};

#[derive(Debug)]
struct UserInner {
    id: String,
    username: String,
}

impl UserInner {
    pub fn new(username: impl ToString) -> Result<Self, Error> {
        let username = username.to_string();
        validate_username(&username)?;

        Ok(Self {
            id: "Tahinli".into(),
            username,
        })
    }

    pub fn get_id(&self) -> &String {
        &self.id
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

    pub fn get_id(&self) -> &String {
        self.user.get_id()
    }

    pub fn get_username(&self) -> &String {
        self.user.get_username()
    }
}
