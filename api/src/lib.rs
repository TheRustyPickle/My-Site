use leptos::prelude::ServerFnError;
use leptos::server;
use shared::models::{Downloads, RepoReleasesSummary};

#[cfg(feature = "ssr")]
pub mod reddit;

#[cfg(feature = "ssr")]
pub mod github;

#[cfg(feature = "ssr")]
pub mod message;

#[server]
pub async fn reddit_downloader(post_id: String) -> Result<Downloads, ServerFnError> {
    use crate::reddit::get_reddit_url;
    use log::error;

    match get_reddit_url(&post_id).await {
        Ok(data) => Ok(data),
        Err(e) => {
            error!("Failed to download reddit data. Reason: {e}");
            Err(ServerFnError::new(e))
        }
    }
}

#[server]
pub async fn github_checker(
    username: String,
    repo_link: String,
) -> Result<RepoReleasesSummary, ServerFnError> {
    use crate::github::get_release_data;
    use log::error;

    match get_release_data(username, repo_link).await {
        Ok(data) => Ok(data),
        Err(e) => {
            error!("Failed to get repo release summary. Reason: {e}");
            Err(ServerFnError::new(e))
        }
    }
}

#[server]
pub async fn message_drop(text: String) -> Result<(), ServerFnError> {
    use crate::message::send_message;
    use log::error;

    match send_message(text).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to send message. Reason: {e}");
            Err(ServerFnError::new(e))
        }
    }
}
