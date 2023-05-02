use derive_more::*;

use crate::{Chat, ChatResponse, Completion, CompletionResponse};

#[derive(Debug, Clone, From, Into, FromStr, Display)]
pub struct ApiKey(String);

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

    pub async fn do_completion(
        &self,
        completion: &Completion,
    ) -> anyhow::Result<CompletionResponse> {
        debug!("Sending completion request: {:#?}", completion);
        debug!(
            "Sending: {}",
            serde_json::to_string_pretty(&completion).unwrap()
        );
        let body = self
            .client
            .post(format!("{}/completions", self.base_url))
            .json(completion)
            .send()
            .await?
            .text()
            .await?;

        debug!("Received: {}", body);
        Ok(serde_json::from_str(&body)?)
    }

    pub async fn do_chat(&self, chat: &Chat) -> anyhow::Result<ChatResponse> {
        debug!("Sending chat request: {:#?}", chat);
        debug!("Sending: {}", serde_json::to_string_pretty(&chat).unwrap());
        let body = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .json(chat)
            .send()
            .await?
            .text()
            .await?;

        debug!("Received: {}", body);
        Ok(serde_json::from_str(&body)?)
    }
}
