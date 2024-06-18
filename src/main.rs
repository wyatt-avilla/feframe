mod dynamic_content;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    println!("Hello, world!");
}
