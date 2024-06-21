use stylist::yew::styled_component;
use yew::prelude::*;

use types::Book;

#[styled_component]
pub fn Row(props: &Book) -> Html {
    let Book {
        title,
        author,
        rating,
        title_url,
        author_url,
        cover_url,
    } = props;

    html! {
        <div class={css!(r#"
            width: 100%;
            display: flex;
            flex-direction: column;
            padding: 10px;
            border-bottom: 1px solid #EBE8D5;
            box-sizing: border-box;
            background-color: none;
            &:hover {
                border-radius: 5px;
                background-color: #DCD6CC;
            }
        "#)}>
            <div class={css!(r#"
                display: flex;
                justify-content: space-between;
                align-items: center;
            "#)}>
                <img src={ cover_url.clone() } alt="Poster" class={css!(r#"
                    border-radius: 5px;
                    width: 35px;
                    height: auto;
                "#)} />
                    <div class={css!(r#"
                        display: flex;
                        flex-direction: column;
                        flex-grow: 1;
                        margin-left: 25px;
                        margin-right: 25px;
                    "#)}>
                    <a href={ title_url.clone() } class={css!(r#"
                        color: #00635D;
                        text-decoration: none;
                        margin-bottom: 8px;
                    "#)}>
                        { title }
                    </a>
                    <a href={ author_url.clone() } class={css!(r#"
                        color: #999999;
                        text-decoration: none;
                    "#)}>
                        { author }
                    </a>
                </div>
                <a class={css!(r#"
                    display: flex;
                    align-items: center;
                    color: #F5A623;
                "#)}>
                    { rating }
                </a>
            </div>
        </div>
    }
}
