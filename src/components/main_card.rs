use super::row_scroller::{Scroller, ScrollerProps};
use super::social_button::{Button, ButtonProps};
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {
    pub title: String,
    pub button_props: ButtonProps,
    pub scroller_props: ScrollerProps,
}

#[styled_component]
pub fn Card(props: &CardProps) -> Html {
    let CardProps {
        title,
        button_props,
        scroller_props,
    } = props;

    html! {
    <div>
        <div class={css!(r#"
                box-shadow: 0 0 5px 1px rgba(0, 0, 0, 0.7);
                height: 500px;
                width: 500px;
                border-radius: 10px;

                display: flex;
                flex-direction: column;

                padding: 15px;
                box-sizing: border-box;

                background-color: white;
            "#)} id="yew-sample-content">
            <div class={css!(r#"
                    flex: 1;
                    display: flex;
                    align-items: center;
                    justify-content: left;
                "#)}>
                <Button ..button_props.clone() />
                <h2 class={css!(r#"
                    margin-left: 20px;
                "#)}>{ title }</h2>
            </div>
            <div class={css!(r#"
                    flex: 3;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    width: 100%;
                    overflow: hidden;
                "#)}>
                <Scroller ..scroller_props.clone() />
            </div>
        </div>
    </div>
    }
}
