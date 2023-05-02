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

pub trait BaseExt {
    fn with_max_tokens(self, max_tokens: usize) -> Self;
    fn with_temperature(self, temperature: f64) -> Self;
    fn with_user(self, user: impl Into<String>) -> Self;
}

impl BaseExt for Base {
    fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = user.into();
        self
    }
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
