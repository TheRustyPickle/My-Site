use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReleaseAsset {
    pub name: String,
    pub download_count: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReleaseInfo {
    pub url: String,
    pub tag: String,
    pub assets: Vec<ReleaseAsset>,
    pub total_downloads: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RepoReleasesSummary {
    pub releases: Vec<ReleaseInfo>,
    pub total_downloads: i64,
    pub most_downloaded_release: Option<ReleaseInfo>,
}

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

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Default)]
pub struct VideoSize {
    pub height: u64,
    pub width: u64,
    pub highest_quality: bool,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DownloadMetadata {
    pub file_name: String,
    pub extension: String,
    pub sizing: VideoSize,
}
