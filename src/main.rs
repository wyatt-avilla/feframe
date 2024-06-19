mod card_generators;
mod components;
mod dynamic_content;

use card_generators::placeholder_card_generator;
use dynamic_content::{ApiRefresh, Book, Commit, Movie, Song};
use stylist::yew::{styled_component, Global};
use yew::prelude::*;

#[allow(dead_code)]
async fn use_content() {
    let commit = &Commit::fetch_newest(1).await.unwrap()[0];
    let book = &Book::fetch_newest(1).await.unwrap()[0];
    let song = &Song::fetch_newest(1).await.unwrap()[0];
    let movie = &Movie::fetch_newest(1).await.unwrap()[0];

    println!(
        "{}, {}, {}, {}",
        commit.repository_name, book.title, song.title, movie.title
    );
}

#[styled_component]
pub fn App() -> Html {
    html! {
        <div>
            // Global Styles can be applied with <Global /> component.
            <Global css={css!(r#"
                    html, body {
                        font-family: sans-serif;

                        padding: 0;
                        margin: 0;

                        display: flex;
                        justify-content: center;
                        align-items: center;
                        min-height: 100vh;
                        flex-direction: column;

                        background-color: rgb(237, 244, 255);
                    }
                "#)} />
            { placeholder_card_generator() }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
