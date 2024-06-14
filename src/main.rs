mod dynamic_content {
    pub mod lastfm;
}

use dynamic_content::lastfm::get_recently_listened;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    println!("Hello, world!");
}
