use stylist::yew::styled_component;
use yew::prelude::*;

use types::Commit;

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
            border-bottom: 1px solid #8D96A0;
            box-sizing: border-box;
            background-color: none;
            &:hover {
                border-radius: 5px;
                background-color: #2A3D52;
            }
        "#)}>
            <div class={css!(r#"
                display: flex;
                justify-content: space-between;
                align-items: center;
            "#)}>
                <a href={url.to_string().clone()} class={css!(r#"
                    color: #ecf2f8;
                    text-decoration: none;
                "#)}>
                    { message }
                </a>
                <div class={css!(r#"
                    display: flex;
                    align-items: top-center;
                "#)}>
                    <a href={repository_link.to_string().clone()} class={css!(r#"
                        color: #77bdfb;
                        text-decoration: none;
                        margin-right: 5px;
                        margin-left: 15px;
                        overflow: hidden;
                        white-space: nowrap;
                    "#)}>
                        { repository_name }
                    </a>
                    <img src="/assets/repo.svg" alt="Repo Icon" />
                </div>
            </div>
        </div>
    }
}
