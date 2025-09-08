use cached::proc_macro::once;
use futures::future;
use scraper::{Html, Selector};
use types::Movie;

async fn poster_url_from_slug(slug: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(reqwest::get(format!(
        "https://letterboxd.com/film/{slug}/poster/std/150/"
    ))
    .await
    .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?
    .json::<serde_json::Value>()
    .await
    .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?
    .get("url")
    .and_then(|v| v.as_str())
    .ok_or("Missing poster url")?
    .to_string()
    .trim_matches('"')
    .to_string())
}

async fn build_movie(slug: &str, rating: String) -> Result<Movie, Box<dyn std::error::Error>> {
    let poster_url_future = poster_url_from_slug(slug);

    let url = format!("https://letterboxd.com/film/{slug}");

    let json = reqwest::get(format!("{url}/json/"))
        .await?
        .json::<serde_json::Value>()
        .await?;

    let title = json
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or("Missing title")?
        .to_string();

    let release_year = json
        .get("releaseYear")
        .ok_or("Missing release year")?
        .to_string();

    Ok(Movie {
        title,
        rating,
        release_year,
        url,
        poster_url: poster_url_future.await?,
    })
}

// 1 day
#[once(result = true, time = 86400, sync_writes = true)]
pub async fn fetch_newest(
    username: &str,
    n: u32,
) -> Result<std::vec::Vec<Movie>, Box<dyn std::error::Error>> {
    log::info!("Parsing letterboxd profile html...");
    let url = format!("https://letterboxd.com/{username}/films/by/rated-date/");
    let html = Html::parse_document(
        &reqwest::get(&url)
            .await
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?
            .text()
            .await
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?,
    );

    let container_sel =
        Selector::parse("#content .cols-2.overflow section.section.col-main.overflow").unwrap();
    let poster_grid_sel = Selector::parse("div.poster-grid").unwrap();
    let griditem_sel = Selector::parse("li.griditem").unwrap();

    let react_sel = Selector::parse("div.react-component").unwrap();
    let rating_sel = Selector::parse("p.poster-viewingdata span.rating").unwrap();

    let movie_futures = html
        .select(&container_sel)
        .next()
        .into_iter()
        .flat_map(move |container| {
            let griditem_sel = &griditem_sel.clone();
            container
                .select(&poster_grid_sel.clone())
                .flat_map(|pg| pg.select(griditem_sel))
                .collect::<Vec<_>>()
        })
        .filter_map(move |li| {
            let rating = li
                .select(&rating_sel)
                .next()
                .map(|r| r.text().collect::<String>().trim().to_string())
                .filter(|s| !s.is_empty())?;

            let rc = li.select(&react_sel).next()?;
            let attrs = rc.value();

            let slug = attrs
                .attr("data-item-slug")
                .or_else(|| attrs.attr("data-film-slug"))?;

            Some((slug, rating))
        })
        .take(n as usize)
        .map(|(slug, rating)| build_movie(slug, rating))
        .collect::<Vec<_>>();

    future::try_join_all(movie_futures).await
}
