use anyhow::Result;
use vial_shared::EncryptedPayload;

pub async fn get_secret(id: String) -> Result<EncryptedPayload> {
    let client = reqwest::Client::new();

    let url = format!("https://rustypickle.onrender.com/api/secrets/{id}");

    let response = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(response)
}
