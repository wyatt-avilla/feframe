use super::button::Button;
use super::scroller::Scroller;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component]
pub fn Card() -> Html {
    let title = "Recently Watched".to_string();
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

                background-color: #14181c;
            "#)} id="yew-sample-content">
            <div class={css!(r#"
                    flex: 1;
                    display: flex;
                    align-items: center;
                    justify-content: left;
                "#)}>
                <Button />
                <h2 class={css!(r#"
                    margin-left: 20px;
                    color: #FFFFFF;
                "#)}>{ title }</h2>
            </div>
            <div class={css!(r#"
                    flex: 3;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    width: 100%;
                    overflow: hidden;
                    border-radius: 10px;
                "#)}>
                <Scroller />
            </div>
        </div>
    </div>
    }
}
