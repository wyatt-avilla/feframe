use dynamic_content::{Book, Commit, Movie, Song};

mod dynamic_content;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let url = "https://github.com/wyatt-avilla/pulse";

    let c = Commit {
        repository_name: String::new(),
        message: String::new(),
        url: url::Url::parse("").unwrap(),
        repository_link: url::Url::parse(url).unwrap(),
    };

    let b = Book {
        author: String::new(),
        title: String::new(),
        author_url: url::Url::parse(url).unwrap(),
        title_url: url::Url::parse(url).unwrap(),
    };

    let s = Song {
        title: String::new(),
        artist_name: String::new(),
        album_name: String::new(),
        album_image: url::Url::parse(url).unwrap(),
        url: url::Url::parse(url).unwrap(),
    };

    let m = Movie {
        title: String::new(),
        rating: String::new(),
        url: url::Url::parse(url).unwrap(),
    };

    println!(
        "{}, {}, {}, {}",
        c.repository_name, b.title, s.title, m.title
    );

    println!("Hello, world!");
}
