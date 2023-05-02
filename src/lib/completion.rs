use serde::{Deserialize, Serialize};

use crate::{base_ext, Base, CompletionModel};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Completion {
    /// ID of the model to use
    #[serde(default)]
    pub model: CompletionModel,

    #[serde(flatten)]
    base: Base,

    /// The prompt(s) to generate completions for.
    pub prompt: Vec<String>,

    /// The suffix to add to the completion.
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
    pub fn with_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt.push(prompt.into());
        self
    }

    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }
}
