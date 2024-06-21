use super::row::Row;
use config::ENDPOINT;
use stylist::yew::styled_component;
use types::Song;
use yew::prelude::*;

#[styled_component]
pub fn Scroller() -> Html {
    #[allow(clippy::redundant_closure)]
    let songs = use_state(|| std::vec::Vec::new());
    {
        let songs = songs.clone();
        use_effect_with((), move |()| {
            let songs = songs.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = reqwest::get(format!("http://{}{}", ENDPOINT.base, ENDPOINT.lastfm))
                    .await
                    .unwrap();
                let fetched_songs: Vec<Song> = response.json().await.unwrap();
                songs.set(fetched_songs);
            });
            || ()
        });
    }

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
            { songs.iter().map(|commit| html! { <Row ..commit.clone() /> }).collect::<Vec<_>>() }
        </div>
    }
}
