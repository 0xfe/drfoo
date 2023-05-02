use serde::{Deserialize, Serialize};

use crate::{base_ext, Base, CompletionModel, Response};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Completion {
    /// ID of the model to use
    #[serde(default)]
    pub model: CompletionModel,

    #[serde(flatten, default)]
    pub base: Base,

    /// The prompt(s) to generate completions for.
    pub prompt: Vec<String>,

    /// The suffix to add to the completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

base_ext!(Completion);

impl<T: Into<String>> From<T> for Completion {
    fn from(prompt: T) -> Self {
        Self {
            prompt: vec![prompt.into()],
            ..Default::default()
        }
    }
}

impl Completion {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt.push(prompt.into());
        self
    }

    pub fn with_prompts(mut self, prompts: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.prompt.extend(prompts.into_iter().map(Into::into));
        self
    }

    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct CompletionResponse {
    /// The ID of the response
    #[serde(flatten)]
    pub meta: Response,

    pub model: CompletionModel,

    pub choices: Vec<ChatChoice>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ChatChoice {
    pub text: String,

    pub finish_reason: String,

    pub index: i64,
}
