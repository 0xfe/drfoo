use derive_more::{From, Into};
use serde::{Deserialize, Serialize};

use crate::{base_ext, Base, ChatModel, Response};

#[derive(Debug, Clone, Deserialize, Serialize, From, Into)]
pub struct Role(String);

impl Default for Role {
    fn default() -> Self {
        Self(String::from("user"))
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    /// The role author of this message
    #[serde(default)]
    pub role: Role,

    /// The message content
    pub content: String,

    /// The name of the author of the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Message {
    pub fn new(role: impl Into<Role>, content: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: content.into(),
            ..Default::default()
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_role(mut self, role: impl Into<Role>) -> Self {
        self.role = role.into();
        self
    }
}

impl<T: Into<String>> From<T> for Message {
    fn from(content: T) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Chat {
    /// ID of the model to use
    #[serde(default)]
    pub model: ChatModel,

    #[serde(flatten, default)]
    pub base: Base,

    /// The list of messages to feed the model
    pub messages: Vec<Message>,
}

base_ext!(Chat);

impl Chat {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_message(mut self, message: impl Into<Message>) -> Self {
        self.messages.push(message.into());
        self
    }

    pub fn with_messages<T: Into<Message>>(
        mut self,
        messages: impl IntoIterator<Item = T>,
    ) -> Self {
        self.messages.extend(messages.into_iter().map(Into::into));
        self
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ChatResponse {
    /// The ID of the response
    #[serde(flatten)]
    pub meta: Response,

    pub model: ChatModel,

    pub choices: Vec<ChatChoice>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ChatChoice {
    pub message: Message,

    pub finish_reason: String,

    pub index: i64,
}
