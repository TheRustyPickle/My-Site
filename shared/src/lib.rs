pub mod models;

pub fn extract_reddit_id(url: &str) -> Option<&str> {
    if !url.contains("reddit.com/r/") {
        return None;
    }

    let parts: Vec<&str> = url.split('/').collect();
    let index = parts.iter().position(|&p| p == "comments")?;
    parts.get(index + 1).copied()
}

pub fn extract_github_info(url: &str) -> Option<(String, String)> {
    let prefix = "github.com/";

    let start = url.find(prefix)? + prefix.len();

    let path = &url[start..];

    let mut parts = path.split('/');

    let username = parts.next()?.to_string();
    let repository = parts.next()?.to_string();

    if username.is_empty() || repository.is_empty() {
        return None;
    }

    Some((username, repository))
}
