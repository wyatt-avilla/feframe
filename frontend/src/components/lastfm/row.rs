use stylist::yew::styled_component;
use yew::prelude::*;

use types::Song;

#[styled_component]
pub fn Row(props: &Song) -> Html {
    let Song {
        title,
        artist_name,
        album_name: _,
        album_image,
        url,
    } = props;

    html! {
        <div class={css!(r#"
            width: 100%;
            display: flex;
            flex-direction: column;
            padding: 10px;
            border-bottom: 1px solid #8D96A0;
            box-sizing: border-box;
            background-color: none;
            &:hover {
                border-radius: 5px;
                background-color: #EDDFDE;
            }
        "#)}>
            <div class={css!(r#"
                display: flex;
                justify-content: space-between;
                align-items: center;
            "#)}>
                <img src={album_image.to_string()} alt="Album Image" class={css!(r#"
                    border-radius: 5px;
                    width: 30px;
                    height: auto;
                "#)} />
                    <div class={css!(r#"
                        display: flex;
                        flex-direction: column;
                        flex-grow: 1;
                        margin-left: 25px;
                        margin-right: 25px;
                    "#)}>
                    <a class={css!(r#"
                        color: #222222;
                        text-decoration: none;
                    "#)}>
                        { format!("{title}") }
                    </a>
                    <a class={css!(r#"
                        color: #666666;
                        text-decoration: none;
                    "#)}>
                        { format!("{artist_name}") }
                    </a>
                </div>
                <a href={url.clone()} class={css!(r#"
                    display: flex;
                    align-items: center;
                "#)}>
                    <img src="/assets/play.svg" alt="Play Button Icon" />
                </a>
            </div>
        </div>
    }
}
