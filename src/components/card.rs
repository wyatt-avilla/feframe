use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component]
pub fn Card() -> Html {
    let title = "My title name".to_string();

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
                <button class={css!(r#"
                        background: none;
                        border: none;
                        cursor: pointer;
                        box-shadow: 0 0 5px 1px rgba(0, 0, 0, 0.7);
                    "#)}>
                    <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M12 2C6.48 2 2 6.48 2 12C2 17.52 6.48 22 12 22C17.52 22 22 17.52 22 12C22 6.48 17.52 2 12 2ZM12 20C7.59 20 4 16.41 4 12C4 7.59 7.59 4 12 4C16.41 4 20 7.59 20 12C20 16.41 16.41 20 12 20Z" fill="currentColor"/>
                        <path d="M11 11H13V16H11V11ZM11 7H13V9H11V7Z" fill="currentColor"/>
                    </svg>
                </button>
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
                <Scroller />
            </div>
        </div>
    </div>
    }
}

#[styled_component]
pub fn Scroller() -> Html {
    let rows: Vec<String> = (1..15).map(|i| format!("Item {i}")).collect();

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
            { for rows.iter().map(|row| html! { <Row value={row.clone()} /> }) }
        </div>
    }
}

#[styled_component]
pub fn Row(props: &Props) -> Html {
    let Props { value } = props;

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

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: String,
}
