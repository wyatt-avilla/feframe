use cached::proc_macro::once;
use regex::Regex;
use scraper::{Html, Selector};
use std::collections::HashMap;
use types::Book;

fn clean_text(input: &str) -> String {
    let trimmed = input.trim().replace(['\n', '\r'], "");
    let re = Regex::new(r"\s{2,}").unwrap();
    re.replace_all(&trimmed, " ").to_string()
}

fn swap_name_order(full_name: &str) -> Result<String, String> {
    let (last, first) = full_name
        .split_once(',')
        .ok_or("No comma in author name")
        .map(|(before, after)| (before.trim(), after.trim()))?;

    Ok(format!("{first} {last}"))
}

// 1 day
#[once(result = true, time = 86400)]
pub async fn fetch_newest(
    shelf: &str,
    n: u32,
) -> Result<std::vec::Vec<Book>, Box<dyn std::error::Error>> {
    println!("Fetching data from goodreads api...");
    let html = Html::parse_document(
        &reqwest::get(shelf)
            .await
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?
            .text()
            .await
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?,
    );

    let ratings = HashMap::from([
        ("did not like it", 1),
        ("it was ok", 2),
        ("liked it", 3),
        ("really liked it", 4),
        ("it was amazing", 5),
    ]);

    let row_selector = Selector::parse(r"tr.bookalike.review").unwrap();

    let title_selector = Selector::parse(r"td.field.title a").unwrap();
    let author_selector = Selector::parse(r"td.field.author a").unwrap();
    let rating_selector = Selector::parse(r"td.field.rating span").unwrap();

    Ok(html
        .select(&row_selector)
        .filter_map(|row| {
            let title_element = row.select(&title_selector).next()?;
            let title_href = title_element.value().attr("href")?;

            let author_element = row.select(&author_selector).next()?;
            let author_href = author_element.value().attr("href")?;

            let rating = row.select(&rating_selector).next()?.value().attr("title")?;

            Some(Book {
                title: clean_text(&title_element.text().collect::<Vec<_>>().concat()),
                author: swap_name_order(&author_element.text().collect::<Vec<_>>().concat())
                    .ok()?,
                title_url: format!("https://www.goodreads.com/{title_href}"),
                author_url: format!("https://www.goodreads.com/{author_href}"),
                rating: ("★").repeat(*ratings.get(rating)?),
            })
        })
        .take(n as usize)
        .collect())
}
