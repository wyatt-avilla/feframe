use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub icon_svg: Html,
}

#[styled_component]
pub fn Button(props: &ButtonProps) -> Html {
    let ButtonProps { onclick, icon_svg } = props;

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
