use derive_more::{Display, From, FromStr, Into};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, From, Into, FromStr, Display)]
pub struct ChatModel(String);

impl Default for ChatModel {
    fn default() -> Self {
        Self(String::from("gpt-4"))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, From, Into, FromStr, Display)]
pub struct CompletionModel(String);

impl Default for CompletionModel {
    fn default() -> Self {
        Self(String::from("davinci"))
    }
}
