use serde::{Deserialize, Serialize};

use crate::{Base, ChatModel};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    /// The role author of this message
    pub role: String,

    /// The message content
    pub content: String,

    /// The name of the author of the message
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Chat {
    /// ID of the model to use
    #[serde(default)]
    pub model: ChatModel,

    #[serde(flatten)]
    base: Base,

    /// The list of messages to feed the model
    pub messages: Vec<Message>,
}
