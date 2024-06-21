use stylist::yew::styled_component;
use yew::prelude::*;

use types::Movie;

#[styled_component]
pub fn Row(props: &Movie) -> Html {
    let Movie {
        title,
        rating,
        release_year,
        url,
        poster_url,
    } = props;

    html! {
        <div class={css!(r#"
            width: 100%;
            display: flex;
            flex-direction: column;
            padding: 10px;
            border-bottom: 1px solid #445566;
            box-sizing: border-box;
            background-color: none;
            &:hover {
                border-radius: 5px;
                background-color: #2C3440;
            }
        "#)}>
            <div class={css!(r#"
                display: flex;
                justify-content: space-between;
                align-items: center;
            "#)}>
                <img src={ poster_url.clone() } alt="Poster" class={css!(r#"
                    border-radius: 5px;
                    width: 70px;
                    height: auto;
                "#)} />
                    <div class={css!(r#"
                        display: flex;
                        flex-direction: column;
                        flex-grow: 1;
                        margin-left: 25px;
                        margin-right: 25px;
                    "#)}>
                    <a href={url.clone()} class={css!(r#"
                        color: #FFFFFF;
                        text-decoration: none;
                        margin-bottom: 8px;
                    "#)}>
                        { title }
                    </a>
                    <a class={css!(r#"
                        color: #667788;
                        text-decoration: none;
                    "#)}>
                        { release_year }
                    </a>
                </div>
                <a class={css!(r#"
                    display: flex;
                    align-items: center;
                    color: #00C030;
                "#)}>
                    { rating }
                </a>
            </div>
        </div>
    }
}
