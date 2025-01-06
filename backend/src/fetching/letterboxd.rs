use cached::proc_macro::once;
use regex::Regex;
use scraper::{Html, Selector};
use types::Movie;

fn parse_image(html: &Html) -> Result<String, Box<dyn std::error::Error>> {
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

fn parse_release_year(html: &Html) -> Result<String, Box<dyn std::error::Error>> {
    let frame_selector = Selector::parse("span.frame[title]").unwrap();

    let date_re = Regex::new(r"\((\d{4})\)").unwrap();

    let title = html
        .select(&frame_selector)
        .next()
        .ok_or("Frame not found in HTML")?
        .attr("title")
        .ok_or("Title not found in frame HTML")?;

    Ok(date_re
        .captures(title)
        .and_then(|caps| caps.get(1))
        .map(|year| year.as_str().to_owned())
        .ok_or("Couldn't parse date")?)
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

            Some(Movie {
                title,
                rating,
                release_year: String::new(),
                url: format! {"https://letterboxd.com{link}"},
                poster_url: slug.to_string(), // icky, just store the slug in here for now xd
            })
        })
        .take(n as usize);

    // no async closures :(
    let mut movies = Vec::new();
    for mut movie in movie_iter {
        let html = Html::parse_document(
            &reqwest::get(format!(
                "https://letterboxd.com/ajax/poster/film/{}/std/70x105/",
                movie.poster_url // aka slug
            ))
            .await
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?
            .text()
            .await
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?,
        );

        movie.poster_url = parse_image(&html)?;
        movie.release_year = parse_release_year(&html)?;

        movies.push(movie);
    }

    Ok(movies)
}
