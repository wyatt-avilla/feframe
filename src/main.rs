mod dynamic_content;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    println!("Hello, world!");

    match dynamic_content::lastfm::get_recently_listened(5).await {
        Ok(songvec) => {
            for song in songvec {
                println!(
                    "{} by {} on {}",
                    song.title, song.artist_name, song.album_name
                );
            }
        }
        Err(e) => eprintln!("function failed with: {e}"),
    };

    match dynamic_content::github::get_recent_commits(5).await {
        Ok(commitvec) => {
            for commit in commitvec {
                println!("{} in {}", commit.message, commit.repository_name);
                println!("{}", commit.url);
                println!("{}", commit.repository_link);
            }
        }
        Err(e) => eprintln!("function failed with: {e}"),
    }

    match dynamic_content::goodreads::get_recently_read(5).await {
        Ok(bookvec) => {
            for book in bookvec {
                println!("{} by {}", book.title, book.author);
            }
        }
        Err(e) => eprintln!("function failed with: {e}"),
    }

    match dynamic_content::letterboxd::get_recently_watched(5).await {
        Ok(movievec) => {
            for movie in movievec {
                println!("{} rated {}", movie.title, movie.rating);
            }
        }
        Err(e) => eprintln!("function failed with: {e}"),
    }
}
