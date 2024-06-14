use reqwest::{self, header};
use url::Url;

pub struct Commit {
    pub message: String,
    pub url: Url,
    pub repository_name: String,
    pub repository_link: Url,
}

pub async fn get_recent_commits(
    n: u32,
) -> Result<std::vec::Vec<Commit>, Box<dyn std::error::Error>> {
    let username = std::env::var("GH_USERNAME")?;

    let url = format!("https://api.github.com/users/{}/events", username);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header(header::USER_AGENT, "pulse")
        .send()
        .await?
        .text()
        .await?;

    let json: serde_json::Value = serde_json::from_str(&response)
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?;

    let events = match json.as_array() {
        Some(events) => events.to_vec(),
        None => return Ok(Vec::new()),
    };

    let commits: Vec<_> = events
        .iter()
        .filter(|&event| event["type"] == "PushEvent")
        .cloned()
        .collect();

    Ok(commits
        .iter()
        .filter_map(|commit| {
            Some(Commit {
                message: commit["payload"]["commits"][0]["message"]
                    .as_str()?
                    .to_string(),
                url: Url::parse(commit["payload"]["commits"][0]["url"].as_str()?).ok()?,
                repository_name: commit["repo"]["name"].as_str()?.to_string(),
                repository_link: Url::parse(commit["repo"]["url"].as_str()?).ok()?,
            })
        })
        .take(n as usize)
        .collect())
}
