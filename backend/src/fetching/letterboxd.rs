use cached::proc_macro::once;
use scraper::{Html, Selector};
use types::Movie;

async fn fetch_image(slug: &str) -> Result<String, Box<dyn std::error::Error>> {
    let width = 70;
    let height = 105;
    let url = format!("https://letterboxd.com/ajax/poster/film/{slug}/std/{width}x{height}/");

    let html = Html::parse_document(
        &reqwest::get(&url)
            .await
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?
            .text()
            .await
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?,
    );

    let img_selector =
        Selector::parse("div.react-component.poster.film-poster img.image[src]").unwrap();

    Ok(html
        .select(&img_selector)
        .next()
        .ok_or("Image source not found in HTML")?
        .attr("src")
        .ok_or("Image source attribute not found in HTML")?
        .to_string())
}

// 1 day
#[once(result = true, time = 86400, sync_writes = true)]
pub async fn fetch_newest(
    username: &str,
    n: u32,
) -> Result<std::vec::Vec<Movie>, Box<dyn std::error::Error>> {
    println!("parsing letterboxd profile html...");
    let url = format!("https://letterboxd.com/{username}/films/by/rated-date/");
    let html = Html::parse_document(
        &reqwest::get(&url)
            .await
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?
            .text()
            .await
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?,
    );

    let row_selector = Selector::parse("li.poster-container").unwrap();

    let div_selector = Selector::parse("div.really-lazy-load").unwrap();
    let rating_selector = Selector::parse("span.rating").unwrap();
    let img_selector = Selector::parse("img.image").unwrap();

    let movie_iter = html
        .select(&row_selector)
        .filter_map(|row| {
            let title = row
                .select(&img_selector)
                .next()?
                .value()
                .attr("alt")?
                .to_string();

            let rating = row
                .select(&rating_selector)
                .next()
                .map(|r| r.inner_html())
                .filter(|r| !r.is_empty())?;

            let div_val = row.select(&div_selector).next()?.value();

            let link = div_val.attr("data-target-link")?;

            let slug = div_val.attr("data-film-slug")?;

            let year = div_val.attr("data-film-release-year")?;

            Some(Movie {
                title,
                rating,
                release_year: year.to_string(),
                url: format! {"https://letterboxd.com{link}"},
                poster_url: slug.to_string(),
            })
        })
        .take(n as usize);

    // no async closures :(
    let mut movies = Vec::new();
    for mut movie in movie_iter {
        let slug = movie.poster_url.clone();
        movie.poster_url = fetch_image(&slug).await?;
        movies.push(movie);
    }

    Ok(movies)
}
