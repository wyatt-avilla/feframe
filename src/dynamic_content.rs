mod github;
mod goodreads;
mod lastfm;
mod letterboxd;

pub use github::Commit;
pub use goodreads::Book;
pub use lastfm::Song;
pub use letterboxd::Movie;

pub trait ApiRefresh {
    type Content;

    async fn fetch_newest(
        n: u32,
    ) -> Result<std::vec::Vec<Self::Content>, Box<dyn std::error::Error>>;
}
