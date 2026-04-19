use crate::constant::{
    MAXIMUM_BASTION_NAME_LENGTH, MAXIMUM_MESSAGE_LENGTH, MAXIMUM_USERNAME_LENGTH,
    MINIMUM_BASTION_NAME_LENGTH, MINIMUM_MESSAGE_LENGTH, MINIMUM_USERNAME_LENGTH,
};

type UserError = crate::error::user::Error;
type MessageError = crate::error::message::Error;
type BastionError = crate::error::bastion::Error;

pub fn validate_username(username: &str) -> Result<(), UserError> {
    if !username.is_ascii() {
        return Err(UserError::ASCII);
    }

    if username.len() < MINIMUM_USERNAME_LENGTH {
        return Err(UserError::MinimumLength(username.len()));
    }

    if username.len() > MAXIMUM_USERNAME_LENGTH {
        return Err(UserError::MaximumLength(username.len()));
    }

    Ok(())
}

pub fn validate_message(message: &str) -> Result<(), MessageError> {
    if !message.is_ascii() {
        return Err(MessageError::ASCII);
    }

    if message.len() < MINIMUM_MESSAGE_LENGTH {
        return Err(MessageError::MinimumLength(message.len()));
    }

    if message.len() > MAXIMUM_MESSAGE_LENGTH {
        return Err(MessageError::MaximumLength(message.len()));
    }

    Ok(())
}

pub fn validate_bastion_name(name: &str) -> Result<(), BastionError> {
    if !name.is_ascii() {
        return Err(BastionError::ASCII);
    }

    if name.len() < MINIMUM_BASTION_NAME_LENGTH {
        return Err(BastionError::MinimumLength(name.len()));
    }

    if name.len() > MAXIMUM_BASTION_NAME_LENGTH {
        return Err(BastionError::MaximumLength(name.len()));
    }

    Ok(())
}
