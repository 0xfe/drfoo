use drfoo::*;
use std::env;

#[tokio::main]
async fn main() {
    drfoo::init_logger();
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().into());

    /*
    let prompt = String::from("In 6 months, here's what's going to happen:");
    let request = Completion::default()
        .with_prompt(prompt)
        .with_max_tokens(100);

    let response = client.do_completion(&request).await.unwrap();
    println!("Response: {}", response);
    */

    let request = Chat::default()
        .with_messages([
            "You're a linux expert and systems administrator.",
            "You're going to provide a short clever answer to my next question",
            "What command would you use to find all files in the current directory modified in the last day?"])
        .with_max_tokens(200);

    let response = client.do_chat(&request).await.unwrap();
    println!("Response: {}", response);
}
