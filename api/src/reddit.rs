use anyhow::{anyhow, Result};
use dash_mpd::fetch::DashDownloader;
use dash_mpd::{parse, MPD};
use log::info;
use reqwest::Client;
use roux::Reddit;
use shared::extract_reddit_id;
use shared::models::{DlType, DownloadData, DownloadMetadata, Downloads, VideoSize};
use std::env::var;
use std::fs;

pub async fn get_reddit_url(post_id: &str) -> Result<Downloads> {
    let username = var("USERNAME").unwrap();
    let password = var("PASSWORD").unwrap();
    let client_id = var("CLIENT_TOKEN").unwrap();
    let secret_id = var("SECRET_TOKEN").unwrap();

    let client = Reddit::new("myapp/0.1", &client_id, &secret_id)
        .username(&username)
        .password(&password)
        .login()
        .await
        .map_err(|e| anyhow!("Failed to get a reddit client. Reason: {e}"))?;

    let submission = client.get_submissions(&format!("t3_{post_id}")).await?;
    let data = submission.data.children;

    let mut url_found = Vec::new();

    for thing in data {
        let url = thing.data.url;

        if let Some(url) = url.as_ref() {
            url_found.push(url.to_string());
        }
    }

    if url_found.is_empty() {
        info!("No URL found");
        return Err(anyhow!("No downloadable found in the given reddit post"));
    }

    let mut downloaded_data = Vec::new();
    let mut dl_type = DlType::Image;

    for url in url_found {
        if let Some((post_id, extension)) = extract_image_info(&url) {
            dl_type = DlType::Image;
            let content = download_image(&url).await?;

            let metadata = DownloadMetadata {
                file_name: post_id.to_string(),
                extension,
                sizing: VideoSize::default(),
            };
            let download_data = DownloadData { metadata, content };
            downloaded_data.push(download_data);
        } else if is_video_link(&url) {
            dl_type = DlType::Video;
            let contents = download_video(url, post_id).await?;

            for (sizing, content) in contents {
                let metadata = DownloadMetadata {
                    file_name: post_id.to_string(),
                    extension: String::from("mp4"),
                    sizing,
                };
                let download_data = DownloadData { metadata, content };
                downloaded_data.push(download_data);
            }
        } else if extract_reddit_id(&url).is_some() {
            info!("{url} is referring to the post");
        } else if url.contains("/gallery/") {
            info!("{url} refers to a gallery. Not supported currently");
            return Err(anyhow!("Gallery is not supported currently"));
        }
    }

    if downloaded_data.is_empty() {
        return Err(anyhow!("No downloadable found in the given reddit post"));
    }

    let downloads = Downloads {
        download_type: dl_type,
        data: downloaded_data,
    };

    Ok(downloads)
}

fn is_video_link(url: &str) -> bool {
    url.starts_with("https://v.redd.it")
}

fn extract_image_info(url: &str) -> Option<(String, String)> {
    let valid_extensions = ["jpg", "jpeg", "png", "gif", "webp"];
    let url_parts: Vec<&str> = url.split('/').collect();

    if let Some(last_part) = url_parts.last() {
        if let Some((filename, extension)) = last_part.rsplit_once('.') {
            if valid_extensions.contains(&extension) {
                return Some((filename.to_string(), extension.to_string()));
            }
        }
    }

    None
}

pub async fn download_video(url: String, post_id: &str) -> Result<Vec<(VideoSize, Vec<u8>)>> {
    let mpd_url = format!("{url}/DASHPlaylist.mpd");
    let client = Client::builder().build()?;
    let xml = client
        .get(&mpd_url)
        .header("Accept", "application/dash+xml,video/vnd.mpeg.dash.mpd")
        .send()
        .await?
        .text()
        .await?;

    let mpd: MPD = parse(&xml)?;

    let mut qualities = Vec::new();

    let mut found_quality = false;
    for period in mpd.periods {
        if found_quality {
            break;
        }
        let adaptations = period.adaptations;

        for adaptation in adaptations {
            if found_quality {
                break;
            }
            if adaptation.contentType != Some("video".to_string()) {
                continue;
            }

            for repr in adaptation.representations {
                let Some(height) = repr.height else {
                    continue;
                };

                let Some(width) = repr.width else {
                    continue;
                };

                qualities.push(VideoSize {
                    height,
                    width,
                    highest_quality: false,
                });
            }

            found_quality = true;
        }
    }

    qualities.sort_by(|a, b| (b.height, b.width).cmp(&(a.height, a.width)));
    qualities.truncate(3);

    if let Some(first) = qualities.first_mut() {
        first.highest_quality = true;
    }

    let mut all_contents = Vec::new();

    for quality in qualities {
        let downloader = DashDownloader::new(&mpd_url)
            .prefer_video_height(quality.height)
            .prefer_video_width(quality.width)
            .verbosity(0)
            .with_muxer_preference("mp4", "ffmpeg");

        let path = downloader.best_quality().download().await?;
        info!("Downloaded {post_id}");

        let content = fs::read(&path)?;

        fs::remove_file(path)?;

        all_contents.push((quality, content));
    }

    Ok(all_contents)
}

async fn download_image(url: &str) -> Result<Vec<u8>> {
    let client = Client::builder().build()?;

    let response = client.get(url).send().await?.bytes().await?;

    Ok(response.into())
}
