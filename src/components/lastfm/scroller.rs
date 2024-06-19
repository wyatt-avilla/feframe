use super::row::Row;
use crate::dynamic_content::Song;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component]
pub fn Scroller() -> Html {
    let fake_songs: Vec<_> = (0..15)
        .map(|i| Song {
            title: format!("title {i}"),
            artist_name: format!("super long artist name {i}"),
            album_name: format!("album {i}"),
            url: url::Url::parse("https://github.com/wyatt-avilla/feframe").unwrap(),
            album_image: url::Url::parse(
                "https://lastfm.freetls.fastly.net/i/u/64s/3a0aa3e03cbd5467297f397e67a80cd4.jpg",
            )
            .unwrap(),
        })
        .map(|song| html! { <Row ..song.clone() /> })
        .collect();

    html! {
        <div class={css!(r#"
            width: 100%;
            height: 100%;
            border-radius: 10px;

            background: #FFFFFF;

            padding: 15px;
            box-sizing: border-box;

            box-shadow: 0 0 5px 1px rgba(0, 0, 0, 0.7);
            color: white;

            display: flex;
            flex-direction: column;

            overflow-y: auto;
            overflow-x: hidden;
            box-sizing: border-box;

            /* Hide scrollbar for Webkit browsers (Chrome, Safari) */
            &::-webkit-scrollbar {
                display: none;
            }

            /* Hide scrollbar for Firefox */
            scrollbar-width: none;
            -ms-overflow-style: none; /* Internet Explorer 10+ */
        "#)}>
            { fake_songs }
        </div>
    }
}
