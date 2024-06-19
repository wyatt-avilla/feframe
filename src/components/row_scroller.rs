use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ScrollerProps {
    pub rows: Vec<Html>,
}

#[styled_component]
pub fn Scroller(props: &ScrollerProps) -> Html {
    let ScrollerProps { rows } = props;

    html! {
        <div class={css!(r#"
            width: 100%;
            height: 100%;
            border-radius: 10px;

            background: black;

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
            { rows }
        </div>
    }
}
