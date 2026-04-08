use crate::constant::{
    MAXIMUM_MESSAGE_LENGTH, MAXIMUM_USERNAME_LENGTH, MINIMUM_MESSAGE_LENGTH,
    MINIMUM_USERNAME_LENGTH,
};

type UserError = crate::error::user::Error;
type MessageError = crate::error::message::Error;

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
