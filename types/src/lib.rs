use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Clone, Properties, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    pub message: String,
    pub url: String,
    pub repository_name: String,
    pub repository_link: String,
}

#[derive(Clone, Properties, PartialEq, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub rating: String,
    pub release_year: String,
    pub url: String,
    pub poster_url: String,
}

#[derive(Clone, Properties, PartialEq, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub title_url: String,
    pub author_url: String,
    pub rating: String,
}

#[derive(Clone, Properties, PartialEq, Serialize, Deserialize)]
pub struct Song {
    pub title: String,
    pub artist_name: String,
    pub album_name: String,
    pub album_image: String,
    pub url: String,
}
