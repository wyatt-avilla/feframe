use serde::{Deserialize, Serialize};
use url::Url;
use yew::Properties;

#[derive(Clone, Properties, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    pub message: String,
    pub url: String,
    pub repository_name: String,
    pub repository_link: String,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Movie {
    pub title: String,
    pub rating: String,
    pub url: Url,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub title_url: Url,
    pub author_url: Url,
}

#[derive(Clone, Properties, PartialEq, Serialize, Deserialize)]
pub struct Song {
    pub title: String,
    pub artist_name: String,
    pub album_name: String,
    pub album_image: String,
    pub url: String,
}
