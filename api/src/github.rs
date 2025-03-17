use anyhow::{anyhow, Result};
use log::error;
use octocrab::Octocrab;
use reqwest::Client;
use shared::models::{ReleaseAsset, ReleaseInfo, RepoReleasesSummary};

pub async fn get_release_data(username: String, repo: String) -> Result<RepoReleasesSummary> {
    let client = Client::builder().user_agent("MyRustApp/1.0").build()?;

    let resp = client
        .get("https://api.github.com/repos/rust-lang/rust/releases")
        .send()
        .await?
        .text()
        .await?;

    log::info!("{resp}");
    let octocrab = Octocrab::builder().build()?;

    let mut releases = Vec::new();
    let mut page = octocrab
        .repos(&username, &repo)
        .releases()
        .list()
        .send()
        .await
        .map_err(|e| {
            error!("Failed to get repository info for {username}/{repo}. Error: {e}");
            anyhow!("Failed to get release data from the repository. Does the repository exist?")
        })?;

    while let Some(next_page) = octocrab.get_page(&page.next).await.map_err(|e| {
        error!("Failed to get repository info for {username}/{repo} during pagination. Error: {e}");
        anyhow!("Failed to get release data from the repository. Reason: {e}")
    })? {
        releases.extend(page.items);
        page = next_page;
    }

    releases.extend(page.items);

    let mut all_releases = Vec::new();
    let mut total_downloads = 0;
    let mut most_downloaded_release: Option<ReleaseInfo> = None;

    for release in releases {
        let tag_name = release.tag_name;

        let assets: Vec<ReleaseAsset> = release
            .assets
            .iter()
            .map(|asset| ReleaseAsset {
                name: asset.name.clone(),
                download_count: asset.download_count,
            })
            .collect();

        let release_downloads: i64 = assets.iter().map(|a| a.download_count).sum();
        total_downloads += release_downloads;

        let release_info = ReleaseInfo {
            url: release.html_url.to_string(),
            tag: tag_name.clone(),
            assets,
            total_downloads: release_downloads,
        };

        if most_downloaded_release
            .as_ref()
            .is_none_or(|r| r.total_downloads < release_downloads)
        {
            most_downloaded_release = Some(release_info.clone());
        }

        all_releases.push(release_info);
    }

    let release_summary = RepoReleasesSummary {
        releases: all_releases,
        total_downloads,
        most_downloaded_release,
    };

    Ok(release_summary)
}
