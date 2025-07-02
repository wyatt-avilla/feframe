use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    Responder, get, http,
    web::{self, ServiceConfig},
};
use config::{ENDPOINT, ENV};
use shuttle_actix_web::ShuttleActixWeb;

mod fetching;

#[get("/github")]
async fn github() -> impl Responder {
    web::Json(
        fetching::github::fetch_newest(ENV.username.github, 10)
            .await
            .unwrap(),
    )
}

#[get("/lastfm")]
async fn lastfm() -> impl Responder {
    web::Json(
        fetching::lastfm::fetch_newest(ENV.username.lastfm, ENV.key.lastfm, 10)
            .await
            .unwrap(),
    )
}

#[get("/goodreads")]
async fn goodreads() -> impl Responder {
    web::Json(
        fetching::goodreads::fetch_newest(ENV.link.goodreads, 10)
            .await
            .unwrap(),
    )
}

#[get("/letterboxd")]
async fn letterboxd() -> impl Responder {
    web::Json(
        fetching::letterboxd::fetch_newest(ENV.username.letterboxd, 4)
            .await
            .unwrap(),
    )
}

#[allow(clippy::unused_async)]
#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        let cors = Cors::default()
            .allowed_origin(ENDPOINT.base)
            .allowed_origin("https://wyatt.wtf") // required by cloudflare worker
            .allowed_origin("https://www.wyatt.wtf")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::CONTENT_TYPE])
            .max_age(3600);

        cfg.service(
            web::scope("/api")
                .wrap(cors)
                .service(github)
                .service(lastfm)
                .service(goodreads)
                .service(letterboxd),
        );
        cfg.service(Files::new("/", "frontend/dist").index_file("index.html"));
    };

    Ok(config.into())
}
