use cached::proc_macro::once;
use futures::future::join_all;
use reqwest::{
    self,
    header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT},
};
use types::Commit;

// 15 min
#[once(result = true, time = 900, sync_writes = true)]
pub async fn fetch_newest(
    username: &str,
    n: u32,
) -> Result<std::vec::Vec<Commit>, Box<dyn std::error::Error>> {
    log::info!("Fetching data from github api...");
    let url = format!("https://api.github.com/users/{username}/events");

    let client = reqwest::Client::builder()
        .default_headers(HeaderMap::from_iter([
            (USER_AGENT, HeaderValue::from_static("feframe")),
            (
                HeaderName::from_static("x-github-api-version"),
                HeaderValue::from_static("2022-11-28"),
            ),
        ]))
        .build()?;

    let response = client.get(&url).send().await?.text().await?;

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

    let futures = push_events.into_iter().filter_map(|event| {
        let sha = event["payload"]["head"].as_str()?.to_string();
        let repository_name = event["repo"]["name"].as_str()?.to_string();
        let repository_link = format!("https://github.com/{repository_name}");
        let commit_url = format!("https://api.github.com/repos/{repository_name}/commits/{sha}");
        let client = client.clone();

        Some(async move {
            let commit_response = client
                .get(&commit_url)
                .send()
                .await
                .ok()?
                .json::<serde_json::Value>()
                .await
                .ok()?;

            let message = commit_response["commit"]["message"]
                .as_str()
                .unwrap_or("No message")
                .to_string();

            Some(Commit {
                message,
                url: format!("{repository_link}/commit/{sha}"),
                repository_name,
                repository_link,
            })
        })
    });

    Ok(join_all(futures)
        .await
        .into_iter()
        .flatten()
        .take(n as usize)
        .collect())
}
