use leptos::{prelude::ServerFnError, server};
use shared::Downloads;

#[cfg(feature = "ssr")]
pub mod reddit;

#[server]
pub async fn reddit_downloader(input: String) -> Result<Downloads, ServerFnError> {
    use crate::reddit::get_reddit_url;
    use log::error;

    match get_reddit_url(&input).await {
        Ok(data) => Ok(data),
        Err(e) => {
            error!("Failed to download reddit data. Reason: {e}");
            Err(ServerFnError::new(e))
        }
    }
}
