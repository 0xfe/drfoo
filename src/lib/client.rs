use derive_more::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, From, Into, FromStr, Display)]
pub struct ApiKey(String);

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

impl<T: Into<String>> From<T> for Completion {
    fn from(prompt: T) -> Self {
        Self {
            prompt: vec![prompt.into()],
            ..Default::default()
        }
    }
}

impl Completion {
    pub fn with_base(mut self, base: Base) -> Self {
        self.base = base;
        self
    }

    pub fn with_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt.push(prompt.into());
        self
    }

    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }
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
    /// ID of the model to use
    #[serde(default)]
    pub model: ChatModel,

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

    /// The underlying HTTP client.
    client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: ApiKey) -> Self {
        Self {
            base_url: "https://api.openai.com/v1".into(),
            client: reqwest::Client::builder()
                .default_headers(
                    [
                        (
                            reqwest::header::AUTHORIZATION,
                            reqwest::header::HeaderValue::from_str(
                                &format!("Bearer {}", api_key)[..],
                            )
                            .unwrap(),
                        ),
                        (
                            reqwest::header::CONTENT_TYPE,
                            reqwest::header::HeaderValue::from_static("application/json"),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                )
                .build()
                .unwrap(),
        }
    }

    pub async fn do_completion(&self, completion: impl Into<Completion>) -> anyhow::Result<String> {
        let completion = completion.into();
        debug!("Sending completion request: {:#?}", &completion);
        debug!(
            "Sending: {}",
            serde_json::to_string_pretty(&completion).unwrap()
        );
        let body = self
            .client
            .post(format!("{}/completions", self.base_url))
            .json(&completion)
            .send()
            .await?
            .text()
            .await?;

        Ok(body)
    }
}
