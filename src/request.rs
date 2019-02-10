//! Various structs for storing request information.

use rvk::objects::message::Message;
use rvk::objects::Integer;
use serde_derive::Deserialize;

/// A request received from Callback API.
#[derive(Debug, Deserialize)]
pub struct CallbackAPIRequest {
    secret: String,
    group_id: i32,
    #[serde(rename = "type")]
    r#type: String,
    object: Object,
}

impl CallbackAPIRequest {
    /// Returns the secret associated with this [`CallbackAPIRequest`].
    pub fn secret(&self) -> &str {
        &self.secret
    }

    /// Returns the group ID associated with this [`CallbackAPIRequest`].
    pub fn group_id(&self) -> i32 {
        self.group_id
    }

    /// Returns the type associated with this [`CallbackAPIRequest`].
    pub fn r#type(&self) -> &str {
        &self.r#type
    }

    /// Returns the [`Object`] associated with this [`CallbackAPIRequest`].
    pub fn object(&self) -> &Object {
        &self.object
    }
}

/// An object of a [`CallbackAPIRequest`].
#[derive(Debug, Deserialize)]
pub struct Object {
    #[serde(flatten)]
    message: Option<Message>,
    user_id: Option<Integer>,
    key: Option<String>,
}

impl Object {
    /// Returns the message associated with this [`Object`].
    pub fn message(&self) -> &Option<Message> {
        &self.message
    }

    /// Returns the user ID associated with this [`Object`].
    pub fn user_id(&self) -> &Option<Integer> {
        &self.user_id
    }

    /// Returns the key associated with this [`Object`].
    pub fn key(&self) -> &Option<String> {
        &self.key
    }
}