use cached::proc_macro::once;
use scraper::{Html, Selector};
use types::Movie;
use url::Url;

// 1 day
#[once(result = true, time = 86400)]
async fn fetch_newest_movies(
    username: &str,
    n: u32,
) -> Result<std::vec::Vec<Movie>, Box<dyn std::error::Error>> {
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

    Ok(html
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

            let link = row
                .select(&div_selector)
                .next()?
                .value()
                .attr("data-target-link")?;

            Some(Movie {
                title,
                rating,
                url: Url::parse(format! {"https://letterboxd.com{link}"}.as_str()).ok()?,
            })
        })
        .take(n as usize)
        .collect())
}
