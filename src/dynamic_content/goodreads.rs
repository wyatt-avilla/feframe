use regex::Regex;
use scraper::{Html, Selector};
use url::{ParseError, Url};

pub struct Book {
    pub title: String,
    pub author: String,
    pub title_url: Url,
    pub author_url: Url,
}

async fn fetch_shelf_html(url: String) -> Result<Html, Box<dyn std::error::Error>> {
    let response = reqwest::get(&url)
        .await
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?
        .text()
        .await
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?;

    Ok(Html::parse_document(&response))
}

fn clean_text(input: String) -> String {
    let trimmed = input.trim().replace(['\n', '\r'], "");
    let re = Regex::new(r"\s{2,}").unwrap();
    re.replace_all(&trimmed, " ").to_string()
}

fn create_goodreads_url(path: &str) -> Result<Url, ParseError> {
    Url::parse(&format!("https://www.goodreads.com/{}", path))
}

fn swap_name_order(full_name: String) -> Result<String, String> {
    let (last, first) = full_name
        .split_once(',')
        .ok_or("No comma in author name")
        .map(|(before, after)| (before.trim(), after.trim()))?;

    Ok(format!("{} {}", first, last))
}

pub async fn get_recently_read(n: u32) -> Result<std::vec::Vec<Book>, Box<dyn std::error::Error>> {
    let html =
        fetch_shelf_html("https://www.goodreads.com/review/list/159014522?shelf=read".to_string())
            .await?;

    let row_selector = Selector::parse(r#"tr.bookalike.review"#).unwrap();

    let title_selector = Selector::parse(r#"td.field.title a"#).unwrap();
    let author_selector = Selector::parse(r#"td.field.author a"#).unwrap();

    Ok(html
        .select(&row_selector)
        .filter_map(|row| {
            let title_element = row.select(&title_selector).next()?;
            let title_href = title_element.value().attr("href")?;

            let author_element = row.select(&author_selector).next()?;
            let author_href = author_element.value().attr("href")?;

            Some(Book {
                title: clean_text(title_element.text().collect::<Vec<_>>().concat()),
                author: swap_name_order(author_element.text().collect::<Vec<_>>().concat()).ok()?,
                title_url: create_goodreads_url(title_href).ok()?,
                author_url: create_goodreads_url(author_href).ok()?,
            })
        })
        .take(n as usize)
        .collect())
}