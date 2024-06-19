use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component]
pub fn Button() -> Html {
    let icon_svg = html! {
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M12 2C6.48 2 2 6.48 2 12C2 17.52 6.48 22 12 22C17.52 22 22 17.52 22 12C22 6.48 17.52 2 12 2ZM12 20C7.59 20 4 16.41 4 12C4 7.59 7.59 4 12 4C16.41 4 20 7.59 20 12C20 16.41 16.41 20 12 20Z" fill="currentColor"/>
            <path d="M11 11H13V16H11V11ZM11 7H13V9H11V7Z" fill="currentColor"/>
        </svg>
    };

    let onclick = Callback::noop();

    html! {
        <button class={css!(r#"
                background: none;
                border: none;
                cursor: pointer;
                box-shadow: 0 0 5px 1px rgba(0, 0, 0, 0.7);
            "#)} onclick={ onclick }>
                { icon_svg.clone() }
        </button>
    }
}
