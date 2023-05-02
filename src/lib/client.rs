use derive_more::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, From, Into, FromStr, Display)]
pub struct ApiKey(String);

/// OpenAI Chat Completion API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Base {
    /// ID of the model to use
    pub model: String,

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
    pub user: Option<String>,

    /// The maximum number of tokens to generate.
    pub max_tokens: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Completeion {
    #[serde(flatten)]
    base: Base,

    /// The prompt(s) to generate completions for.
    pub prompt: Vec<String>,

    /// The suffix to add to the completion.
    pub suffix: Option<String>,
}

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
    #[serde(flatten)]
    base: Base,

    /// The list of messages to feed the model
    pub messages: Vec<Message>,
}

/// This is a thin shim around the OpenAI HTTP client. Requires a valid API token.
#[derive(Debug)]
pub struct Client {
    /// This base URL is used for all requests and is constructed from the
    /// provided API token.
    base_url: String,

    /// The API token used for authentication.
    api_key: ApiKey,

    /// The underlying HTTP client.
    client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: ApiKey) -> Self {
        Self {
            base_url: "https://openai.com/v1".into(),
            api_key,
            client: reqwest::Client::new(),
        }
    }
}
