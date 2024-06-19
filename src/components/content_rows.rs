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
