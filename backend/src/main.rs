use actix_web::{web, App, HttpServer, Responder};
use config::{ENDPOINT, ENV};

mod fetching;

async fn github() -> impl Responder {
    web::Json(
        fetching::github::fetch_newest(ENV.username.github, 10)
            .await
            .unwrap(),
    )
}

async fn lastfm() -> impl Responder {
    web::Json(
        fetching::lastfm::fetch_newest(ENV.username.lastfm, ENV.key.lastfm, 10)
            .await
            .unwrap(),
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, Server!");
    HttpServer::new(|| {
        App::new()
            .route(ENDPOINT.github, web::get().to(github))
            .route(ENDPOINT.lastfm, web::get().to(lastfm))
            .service(actix_files::Files::new("/", "../frontend/dist").index_file("index.html"))
    })
    .bind(ENDPOINT.base)?
    .run()
    .await
}
