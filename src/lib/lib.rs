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
