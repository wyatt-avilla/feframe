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

    let play_svg = html! {
            <svg width="16px" height="16px" viewBox="-1 0 12 12" version="1.1" xmlns="http://www.w3.org/2000/svg">
            <g id="Page-1" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd">
                <g id="Dribbble-Light-Preview" transform="translate(-65.000000, -3803.000000)" fill="#000000">
                    <g id="icons" transform="translate(56.000000, 160.000000)">
                        <path d="M18.074,3650.7335 L12.308,3654.6315 C10.903,3655.5815 9,3654.5835 9,3652.8985 L9,3645.1015 C9,3643.4155 10.903,3642.4185 12.308,3643.3685 L18.074,3647.2665 C19.306,3648.0995 19.306,3649.9005 18.074,3650.7335" id="play-[#1000]">
                        </path>
                    </g>
                </g>
            </g>
        </svg>
    };

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
                    { play_svg.clone() }
                </a>
            </div>
        </div>
    }
}
