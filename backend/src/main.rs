use actix_web::{web, App, HttpServer, Responder};
use chrono::Local;
use config::{ENDPOINT, ENV};
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

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

async fn goodreads() -> impl Responder {
    web::Json(
        fetching::goodreads::fetch_newest(ENV.link.goodreads, 10)
            .await
            .unwrap(),
    )
}

async fn letterboxd() -> impl Responder {
    web::Json(
        fetching::letterboxd::fetch_newest(ENV.username.letterboxd, 4)
            .await
            .unwrap(),
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    log::info!("Server opened on {}", ENDPOINT.base);

    HttpServer::new(|| {
        App::new()
            .route(ENDPOINT.github, web::get().to(github))
            .route(ENDPOINT.lastfm, web::get().to(lastfm))
            .route(ENDPOINT.goodreads, web::get().to(goodreads))
            .route(ENDPOINT.letterboxd, web::get().to(letterboxd))
            .service(actix_files::Files::new("/", "../frontend/dist").index_file("index.html"))
    })
    .bind(ENDPOINT.base)?
    .run()
    .await
}
