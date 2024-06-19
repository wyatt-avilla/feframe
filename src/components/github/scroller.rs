use super::row::Row;
use crate::dynamic_content::Commit;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component]
pub fn Scroller() -> Html {
    let fake_commits: Vec<_> = (0..15)
        .map(|i| Commit {
            message: format!(
                "super duper long commit message that shoulddd take multiple lines {i}"
            ),
            url: url::Url::parse("https://github.com/wyatt-avilla/feframe").unwrap(),
            repository_name: format!("huge repo name xd {i}"),
            repository_link: url::Url::parse("https://github.com/wyatt-avilla/feframe").unwrap(),
        })
        .map(|commit| html! { <Row ..commit.clone() /> })
        .collect();

    html! {
        <div class={css!(r#"
            width: 100%;
            height: 100%;
            border-radius: 10px;

            background: #21262d;

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
            { fake_commits }
        </div>
    }
}
