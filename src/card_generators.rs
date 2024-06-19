use crate::components::{ButtonProps, Card, ScrollerProps};
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RowProps {
    pub value: String,
}

#[styled_component]
pub fn Row(props: &RowProps) -> Html {
    let RowProps { value } = props;

    html! {
        <div class={css!(r#"
            width: 100%;
            display: flex;
            justify-content: space-between;
            padding: 10px;
            border-bottom: 1px solid white;
            margin-bottom: 5px;
            box-sizing: border-box;
        "#)}>
            <span>{ value }</span>
        </div>
    }
}

pub fn placeholder_card_generator() -> Html {
    let icon_svg = html! {
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M12 2C6.48 2 2 6.48 2 12C2 17.52 6.48 22 12 22C17.52 22 22 17.52 22 12C22 6.48 17.52 2 12 2ZM12 20C7.59 20 4 16.41 4 12C4 7.59 7.59 4 12 4C16.41 4 20 7.59 20 12C20 16.41 16.41 20 12 20Z" fill="currentColor"/>
            <path d="M11 11H13V16H11V11ZM11 7H13V9H11V7Z" fill="currentColor"/>
        </svg>
    };

    let button_props = ButtonProps {
        onclick: Callback::noop(),
        icon_svg,
    };

    let rows: Vec<String> = (1..15).map(|i| format!("Item {i}")).collect();
    let scroller_props = ScrollerProps {
        rows: rows
            .iter()
            .map(|row| html! { <Row value={ row.clone() } /> })
            .collect(),
    };

    html! {
        < Card title={"my string literal"} button={ button_props } scroller={ scroller_props } />
    }
}
