use dynamic_content::{ApiRefresh, Book, Commit, Movie, Song};

mod dynamic_content;

#[allow(dead_code)]
async fn use_content() {
    let commit = &Commit::fetch_newest(1).await.unwrap()[0];
    let book = &Book::fetch_newest(1).await.unwrap()[0];
    let song = &Song::fetch_newest(1).await.unwrap()[0];
    let movie = &Movie::fetch_newest(1).await.unwrap()[0];

    println!(
        "{}, {}, {}, {}",
        commit.repository_name, book.title, song.title, movie.title
    );
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    println!("Hello, world!");
}
