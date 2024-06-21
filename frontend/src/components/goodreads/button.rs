use config::SOCIAL;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component]
pub fn Button() -> Html {
    html! {
        <a href={ SOCIAL.goodreads } class={css!(r#"
                background: none;
                border: none;
                cursor: pointer;
                border-radius: 50%;
            "#)}>
            <img src="/assets/goodreads.svg" alt="Goodreads Icon" />
        </a>
    }
}
