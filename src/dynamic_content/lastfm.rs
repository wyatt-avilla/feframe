use super::ApiRefresh;
use cached::proc_macro::once;
use url::Url;

#[derive(Clone)]
pub struct Song {
    pub title: String,
    pub artist_name: String,
    pub album_name: String,
    pub album_image: Url,
    pub url: Url,
}

impl ApiRefresh for Song {
    type Content = Song;

    async fn fetch_newest(n: u32) -> Result<std::vec::Vec<Song>, Box<dyn std::error::Error>> {
        fetch_newest_songs(n).await
    }
}

// 20 min
#[once(result = true, time = 1200)]
async fn fetch_newest_songs(n: u32) -> Result<std::vec::Vec<Song>, Box<dyn std::error::Error>> {
    let key = std::env::var("LASTFM_KEY")?;
    let username = std::env::var("LASTFM_USERNAME")?;

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
                album_image: Url::parse(track["image"].as_array()?.get(1)?.get("#text")?.as_str()?)
                    .ok()?,
                url: Url::parse(track["url"].as_str()?).ok()?,
            })
        })
        .take(n as usize)
        .collect())
}
