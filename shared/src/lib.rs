use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Downloads {
    pub download_type: DlType,
    pub data: Vec<DownloadData>,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum DlType {
    Image,
    Video,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DownloadData {
    pub metadata: DownloadMetadata,
    pub content: Vec<u8>,
}

#[derive(Serialize, Default, Deserialize, Clone, Eq, PartialEq)]
pub struct VideoSize {
    pub height: u64,
    pub width: u64,
    pub highest_quality: bool,
}

pub fn extract_reddit_id(url: &str) -> Option<&str> {
    if !url.contains("reddit.com/r/") {
        return None;
    }

    let parts: Vec<&str> = url.split('/').collect();
    let index = parts.iter().position(|&p| p == "comments")?;
    parts.get(index + 1).copied()
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DownloadMetadata {
    pub file_name: String,
    pub extension: String,
    pub sizing: VideoSize,
}
