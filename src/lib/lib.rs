/// DrFoo is an OpenAI client library for Rust. It is designed to be a thin shim
/// around the OpenAI HTTP API.
///
/// # Examples
///
/// ```no_run
/// use drfoo::*;
/// use std::env;
///
/// #[tokio::main]
/// async fn main() {
///     drfoo::init_logger();
///     let client = Client::new(env::var("OPENAI_API_KEY").unwrap().into());
///
///     let request = Chat::new()
///        .with_messages([
///           "You're a linux expert and systems administrator.",
///           "You're going to provide a short clever answer to my next question",
///           "What command would you use to find all files in the current directory modified in the last day?"])
///        .with_max_tokens(200);
///
///     let response = client.do_chat(&request).await.unwrap();
///     println!("Response: {:#?}", response);
/// }
/// ```

#[macro_use]
extern crate log;

pub mod base;
pub mod chat;
pub mod client;
pub mod completion;
pub mod model;

pub use base::*;
pub use chat::*;
pub use client::*;
pub use completion::*;
pub use model::*;

/// This method initializes [`env_logger`] from the environment, defaulting to `info` level logging.
pub fn init_logger() {
    // We use try_init here so it can by run by tests.
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .try_init();
    debug!("Logger initialized.");
}
