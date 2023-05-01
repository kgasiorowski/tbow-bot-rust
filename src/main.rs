use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde::Serialize;

mod config;

#[derive(Serialize)]
struct Payload{
    content: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string(&Payload {
        content: String::from("This is a test message - written in rust!!")
    })?;
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    let res = reqwest::Client::new()
        .post(config::DISCORD_WEBHOOK_URL)
        .headers(headers)
        .body(json)
        .send()
        .await?;

    let status = res.status();
    println!("Status: {}", status);

    Ok(())
}