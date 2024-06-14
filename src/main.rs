mod dynamic_content {
    pub mod lastfm;
}

use dynamic_content::lastfm::get_recently_listened;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    println!("Hello, world!");

    match get_recently_listened(5).await {
        Ok(songvec) => {
            for song in songvec {
                println!(
                    "{} by {} on {}",
                    song.title, song.artist_name, song.album_name
                );
            }
        }
        Err(e) => eprintln!("function failed with: {}", e),
    }
}
