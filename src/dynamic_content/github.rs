use super::ApiRefresh;
use crate::env::CONFIG;
use cached::proc_macro::once;
use reqwest::{self, header};
use url::Url;
use yew::prelude::Properties;

#[derive(Clone, Properties, PartialEq)]
pub struct Commit {
    pub message: String,
    pub url: Url,
    pub repository_name: String,
    pub repository_link: Url,
}

impl ApiRefresh for Commit {
    type Content = Commit;

    async fn fetch_newest(n: u32) -> Result<std::vec::Vec<Commit>, Box<dyn std::error::Error>> {
        fetch_newest_commits(n).await
    }
}

// 15 min
#[once(result = true, time = 900)]
async fn fetch_newest_commits(n: u32) -> Result<std::vec::Vec<Commit>, Box<dyn std::error::Error>> {
    let username = CONFIG.username.github;

    let url = format!("https://api.github.com/users/{username}/events");

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
        Some(events) => events.clone(),
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
