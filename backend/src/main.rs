use actix_web::{web, App, HttpServer, Responder};
use cached::proc_macro::once;
use rand::prelude::Rng;

#[once(result = true, time = 10)]
fn get_cached_data() -> Result<String, Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let x: i32 = rng.gen_range(1..10);
    Ok(format!("Rand:{x}"))
}

async fn api_handler() -> impl Responder {
    web::Json(get_cached_data().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, Server!");
    HttpServer::new(|| {
        App::new()
            .route("/api/data", web::get().to(api_handler))
            .service(actix_files::Files::new("/", "../dist").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
