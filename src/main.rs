use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_url = "https://notify-api.line.me/api/notify";
    let access_token = env::var("LINE_ACCESS_TOKEN").expect("Line access token not set");

    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
    );

    let args: Vec<String> = env::args().collect();
    let message = format!("{}", args[1],);

    let response = client
        .post(api_url)
        .headers(headers)
        .form(&[("message", message)])
        .send()
        .await?;

    if response.status().is_success() {
        println!("Message sent successfully!");
    } else {
        println!("Failed to send message. Status code: {}", response.status());
    }

    Ok(())
}
