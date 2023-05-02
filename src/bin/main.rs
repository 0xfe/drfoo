use drfoo::*;
use std::env;

#[tokio::main]
async fn main() {
    drfoo::init_logger();
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().into());
    let response = client
        .do_completion("In 6 months, here's what's going to happen:")
        .await
        .unwrap();

    println!("Response: {}", response);
}
