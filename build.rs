use dotenv_codegen::dotenv;
use url::Url;

fn main() {
    let _ = Url::parse(dotenv!("GOODREADS_SHELF")).expect("Goodreads shelf link is invalid");
}
