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
        .header(header::USER_AGENT, "feframe")
        .send()
        .await?
        .text()
        .await?;

    let json: serde_json::Value = serde_json::from_str(&response)
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?;

    let json_array = match json.as_array() {
        Some(json_array) => json_array.clone(),
        None => return Ok(Vec::new()),
    };

    let push_events: Vec<_> = json_array
        .iter()
        .filter(|&event| event["type"] == "PushEvent")
        .cloned()
        .collect();

    Ok(push_events
        .iter()
        .filter_map(|event| {
            let commit = &event["payload"]["commits"][0];
            let repository_name = event["repo"]["name"].as_str()?.to_string();
            let repository_link =
                Url::parse(format!("https://github.com/{repository_name}/").as_str()).ok()?;

            Some(Commit {
                message: commit["message"].as_str()?.to_string(),
                url: Url::parse(
                    format!("{repository_link}/commit/{}", commit["sha"].as_str()?).as_str(),
                )
                .ok()?,
                repository_name,
                repository_link,
            })
        })
        .take(n as usize)
        .collect())
}
