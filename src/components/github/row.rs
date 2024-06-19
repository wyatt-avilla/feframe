use stylist::yew::styled_component;
use yew::prelude::*;

use crate::dynamic_content::Commit;

#[styled_component]
pub fn Row(props: &Commit) -> Html {
    let Commit {
        message,
        url,
        repository_name,
        repository_link,
    } = props;

    html! {
        <div class={css!(r#"
            width: 100%;
            display: flex;
            flex-direction: column;
            padding: 10px;
            border-bottom: 1px solid white;
            margin-bottom: 5px;
            box-sizing: border-box;
            background-color: #282c34;
            color: #abb2bf;
        "#)}>
            <div class={css!(r#"
                display: flex;
                justify-content: space-between;
                align-items: center;
            "#)}>
                <a href={url.to_string().clone()} class={css!(r#"
                    color: #61dafb;
                    text-decoration: none;
                "#)}>
                    { message }
                </a>
                <a href={repository_link.to_string().clone()} class={css!(r#"
                    color: #61dafb;
                    text-decoration: none;
                "#)}>
                    { repository_name }
                </a>
            </div>
        </div>
    }
}
