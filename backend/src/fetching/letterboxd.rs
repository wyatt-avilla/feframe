use cached::proc_macro::once;
use scraper::{Html, Selector};
use types::Movie;

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
    let img_sel = Selector::parse("img.image").unwrap();
    let rating_sel = Selector::parse("p.poster-viewingdata span.rating").unwrap();

    let base = html.select(&container_sel).next();

    let movie_iter = base
        .into_iter()
        .flat_map(move |container| {
            let griditem_sel = &griditem_sel.clone();
            container
                .select(&poster_grid_sel.clone())
                .flat_map(|pg| pg.select(griditem_sel))
                .collect::<Vec<_>>()
        })
        .filter_map(move |li| {
            let title = li.select(&img_sel).next()?.value().attr("alt")?.to_string();

            let rating = li
                .select(&rating_sel)
                .next()
                .map(|r| r.text().collect::<String>().trim().to_string())
                .filter(|s| !s.is_empty())?;

            let rc = li.select(&react_sel).next()?;
            let attrs = rc.value();

            let link = attrs.attr("data-target-link")?;
            let slug = attrs
                .attr("data-item-slug")
                .or_else(|| attrs.attr("data-film-slug"))?;

            Some(Movie {
                title,
                rating,
                release_year: String::new(),
                url: format!("https://letterboxd.com{link}"),
                poster_url: slug.to_string(),
            })
        })
        .take(n as usize);

    // no async closures :(
    let mut movies = Vec::new();
    for mut movie in movie_iter {
        let slug = movie.poster_url;

        let poster_json = &reqwest::get(format!(
            "https://letterboxd.com/film/{slug}/poster/std/150/"
        ))
        .await
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?
        .json::<serde_json::Value>()
        .await
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?;

        let movie_json = &reqwest::get(format!("https://letterboxd.com/film/{slug}/json/"))
            .await
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?
            .json::<serde_json::Value>()
            .await
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?;

        movie.poster_url = poster_json
            .get("url")
            .ok_or("Missing poster url")?
            .to_string()
            .trim_matches('"')
            .to_string();

        movie.release_year = movie_json
            .get("releaseYear")
            .ok_or("Missing release year")?
            .to_string();

        movies.push(movie);
    }

    Ok(movies)
}
