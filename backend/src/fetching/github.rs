use cached::proc_macro::once;
use reqwest::{self, header};
use types::Commit;

// 15 min
#[once(result = true, time = 900)]
pub async fn fetch_newest(
    username: &str,
    n: u32,
) -> Result<std::vec::Vec<Commit>, Box<dyn std::error::Error>> {
    println!("Fetching data from github api...");
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
            let repository_link = format!("https://github.com/{repository_name}");

            Some(Commit {
                message: commit["message"].as_str()?.to_string(),
                url: format!("{repository_link}/commit/{}", commit["sha"].as_str()?),
                repository_name,
                repository_link,
            })
        })
        .take(n as usize)
        .collect())
}
