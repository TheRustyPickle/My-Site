use anyhow::Result;
use frankenstein::AsyncTelegramApi;
use frankenstein::client_reqwest::Bot;
use frankenstein::methods::SendMessageParams;
use frankenstein::types::ChatId;

pub async fn send_message(text: String) -> Result<()> {
    let bot_token = std::env::var("BOT_TOKEN")?;
    let send_to = std::env::var("SEND_TO")?;

    let client = Bot::new(&bot_token);

    let message = SendMessageParams::builder()
        .chat_id(ChatId::String(send_to))
        .text(format!("New message dropped:\n\n{text}"))
        .build();

    client.send_message(&message).await?;

    Ok(())
}
