use anyhow::Result;
use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct SendMessage<'a> {
    chat_id: &'a str,
    text: String,
}

pub async fn send_message(text: String) -> Result<()> {
    let bot_token = std::env::var("BOT_TOKEN")?;
    let send_to = std::env::var("SEND_TO")?;

    let url = format!("https://api.telegram.org/bot{bot_token}/sendMessage");

    let payload = SendMessage {
        chat_id: &send_to,
        text: format!("New message dropped:\n\n{text}"),
    };

    let client = Client::new();

    let resp = client
        .post(url)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;

    let _ = resp.text().await?;

    Ok(())
}
