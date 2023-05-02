use drfoo::*;
use std::env;

#[tokio::main]
async fn main() {
    drfoo::init_logger();
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().into());
    let prompt = String::from("In 6 months, here's what's going to happen:");

    let request = Completion::default()
        .with_prompt(prompt)
        .with_max_tokens(1000);

    let response = client.do_completion(request).await.unwrap();

    println!("Response: {}", response);
}
