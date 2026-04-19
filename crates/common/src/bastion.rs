use std::sync::Arc;

use crate::{error::Error, user::User, validate::validate_bastion_name};

#[derive(Debug)]
struct BastionInner {
    id: String,
    name: String,
}

impl BastionInner {
    pub fn new(name: impl ToString) -> Result<Self, Error> {
        let id = "replace".to_string();
        let name = name.to_string();

        validate_bastion_name(&name)?;

        Ok(Self { id, name })
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, Clone)]
pub struct Bastion {
    bastion: Arc<BastionInner>,
}

impl Bastion {
    pub fn new(name: impl ToString) -> Result<Self, Error> {
        Ok(Self {
            bastion: BastionInner::new(name)?.into(),
        })
    }

    pub fn get_id(&self) -> &String {
        self.bastion.get_id()
    }

    pub fn get_name(&self) -> &String {
        self.bastion.get_name()
    }
}

#[derive(Debug)]
pub enum BastionEvent {
    JoinBastion(JoinBastion),
    LeftBastion(LeftBastion),
}

#[derive(Debug)]
pub struct JoinBastion {
    user: User,
    bastion: Bastion,
}

impl JoinBastion {
    pub fn new(user: User, bastion: Bastion) -> Self {
        Self { user, bastion }
    }

    pub fn gather(self) -> (User, Bastion) {
        (self.user, self.bastion)
    }
}

#[derive(Debug)]
pub struct LeftBastion {
    user: User,
    bastion: Bastion,
}

impl LeftBastion {
    pub fn new(user: User, bastion: Bastion) -> Self {
        Self { user, bastion }
    }

    pub fn gather(self) -> (User, Bastion) {
        (self.user, self.bastion)
    }
}
