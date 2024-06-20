use url::Url;
use yew::Properties;

#[derive(Clone, Properties, PartialEq)]
pub struct Commit {
    pub message: String,
    pub url: Url,
    pub repository_name: String,
    pub repository_link: Url,
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

#[derive(Clone, Properties, PartialEq)]
pub struct Song {
    pub title: String,
    pub artist_name: String,
    pub album_name: String,
    pub album_image: Url,
    pub url: Url,
}
