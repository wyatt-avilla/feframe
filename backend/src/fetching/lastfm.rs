use cached::proc_macro::once;
use types::Song;

// 20 min
#[once(result = true, time = 1200, sync_writes = true)]
pub async fn fetch_newest(
    username: &str,
    key: &str,
    n: u32,
) -> Result<std::vec::Vec<Song>, Box<dyn std::error::Error>> {
    println!("Fetching data from lastfm api...");
    let url = format!("https://ws.audioscrobbler.com/2.0/?method=user.getrecenttracks&user={username}&api_key={key}&format=json");

    let response = reqwest::get(&url)
        .await
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?
        .text()
        .await
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?;

    let json: serde_json::Value = serde_json::from_str(&response)
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?;

    let tracks = match json["recenttracks"]["track"].as_array() {
        Some(tracks) => tracks.clone(),
        None => return Ok(Vec::new()),
    };

    Ok(tracks
        .iter()
        .filter_map(|track| {
            Some(Song {
                title: track["name"].as_str()?.to_string(),
                artist_name: track["artist"]["#text"].as_str()?.to_string(),
                album_name: track["album"]["#text"].as_str()?.to_string(),
                album_image: track["image"]
                    .as_array()?
                    .first()?
                    .get("#text")?
                    .as_str()?
                    .to_string(),
                url: (track["url"]).as_str()?.to_string(),
            })
        })
        .take(n as usize)
        .collect())
}
