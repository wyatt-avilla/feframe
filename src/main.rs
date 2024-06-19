mod components;
mod dynamic_content;

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
                <div class={css!(r#"
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    flex-direction: row;
                    gap: 150px;
                    
                    @media screen and (max-width: 1500px) {
                        flex-direction: column;
                    }
                "#)}>
                    <components::github::Card />
                    <components::lastfm::Card />
                </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
