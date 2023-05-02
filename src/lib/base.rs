use serde::{Deserialize, Serialize};

/// OpenAI Chat Completion API
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Base {
    /// What sampling temperature to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability mass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,

    /// If set, the server will stream back partial progress.
    pub stream: bool,

    /// How many completions to generate for each prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<usize>,

    /// User ID to associate with the request.
    pub user: String,

    /// The maximum number of tokens to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<usize>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Usage {
    /// The number of prompt tokens used for this request
    pub prompt_tokens: i64,

    /// The number of completion tokens used for this request
    pub completion_tokens: i64,

    /// The number of tokens the model could have used based on the prompt
    pub total_tokens: i64,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    /// The ID of the response
    pub id: String,

    pub object: String,

    pub created: i64,

    pub usage: Usage,
}

pub trait BaseExt {
    fn with_max_tokens(self, max_tokens: usize) -> Self;
    fn with_temperature(self, temperature: f64) -> Self;
    fn with_user(self, user: impl Into<String>) -> Self;
}

#[macro_export]
macro_rules! base_ext {
    ($l:ident) => {
        /// Add BaseExt methods to $l
        use super::BaseExt;
        impl BaseExt for $l {
            fn with_max_tokens(mut self, max_tokens: usize) -> Self {
                self.base.max_tokens = Some(max_tokens);
                self
            }

            fn with_temperature(mut self, temperature: f64) -> Self {
                self.base.temperature = Some(temperature);
                self
            }

            fn with_user(mut self, user: impl Into<String>) -> Self {
                self.base.user = user.into();
                self
            }
        }
    };
}
