use serde::{Deserialize, Serialize};

/// OpenAI Chat Completion API
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Base {
    /// What sampling temperature to use.
    pub temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability mass.
    pub top_p: Option<f64>,

    /// If set, the server will stream back partial progress.
    pub stream: bool,

    /// How many completions to generate for each prompt.
    pub n: Option<usize>,

    /// User ID to associate with the request.
    pub user: String,

    /// The maximum number of tokens to generate.
    pub max_tokens: Option<usize>,
}

impl Base {
    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = user.into();
        self
    }
}
