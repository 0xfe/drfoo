use derive_more::Display;
use serde::{Deserialize, Serialize};

/// `OpenAIChatModel` is an enum of the OpenAI models available for chat.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Deserialize, Serialize, Display)]
pub enum OpenAIChatModel {
    #[serde(rename = "gpt-4")]
    Gpt_4,
    #[serde(rename = "gpt-4-32k")]
    Gpt_4_32k,
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt_3dot5_turbo,
}

/// `ChatModel` is an enum of the models available for chat.
#[derive(Debug, Clone, Deserialize, Serialize, Display)]
#[serde(untagged)]
pub enum ChatModel {
    OpenAI(OpenAIChatModel),

    /// If a model is not available in the enum, you can use a custom model.
    Custom(String),
}

impl Default for ChatModel {
    fn default() -> Self {
        Self::OpenAI(OpenAIChatModel::Gpt_4)
    }
}

impl<T: Into<String>> From<T> for ChatModel {
    fn from(model: T) -> Self {
        Self::Custom(model.into())
    }
}

impl From<OpenAIChatModel> for ChatModel {
    fn from(model: OpenAIChatModel) -> Self {
        Self::OpenAI(model)
    }
}

/// `OpenAICompletionModel` is an enum of the OpenAI models available for completion.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Deserialize, Serialize, Display)]
pub enum OpenAICompletionModel {
    #[serde(rename = "text-davinci-003")]
    Text_Davinci_003,
    #[serde(rename = "text-davinci-002")]
    Text_Davinci_002,
    #[serde(rename = "text-davinci-001")]
    Text_Davinci_001,
    #[serde(rename = "text-curie-001")]
    Text_Curie_001,
    #[serde(rename = "text-babbage-001")]
    Text_Babbage_001,
    #[serde(rename = "text-ada-001")]
    Text_Ada_001,
}

#[derive(Debug, Clone, Deserialize, Serialize, Display)]
#[serde(untagged)]
pub enum CompletionModel {
    OpenAI(OpenAICompletionModel),

    /// If a model is not available in the enum, you can use a custom model.
    Custom(String),
}

impl Default for CompletionModel {
    fn default() -> Self {
        Self::OpenAI(OpenAICompletionModel::Text_Davinci_003)
    }
}

impl<T: Into<String>> From<T> for CompletionModel {
    fn from(model: T) -> Self {
        Self::Custom(model.into())
    }
}

impl From<OpenAICompletionModel> for CompletionModel {
    fn from(model: OpenAICompletionModel) -> Self {
        Self::OpenAI(model)
    }
}
