mod components;

use stylist::yew::{styled_component, Global};
use yew::prelude::*;

#[styled_component]
pub fn App() -> Html {
    html! {
        <div>
            // Global Styles can be applied with <Global /> component.
            <Global css={css!(r#"
                    html, body {
                        font-family: sans-serif;

                        padding: 50px;
                        margin: 0;

                        display: flex;
                        justify-content: center;
                        align-items: center;
                        min-height: 100vh;
                        flex-direction: column;

                        background-color: #a09cac;
                    }
                "#)} />
                <div class={css!(r#"
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    gap: 100px;
                "#)}>
                    <div class={css!(r#"
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        flex-direction: row;

                        @media screen and (max-width: 1500px) {
                            flex-direction: column;
                        }
                    "#)}>
                        <components::profile::Card />
                    </div>
                    <div class={css!(r#"
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        flex-direction: row;
                        gap: 100px;

                        @media screen and (max-width: 1500px) {
                            flex-direction: column;
                        }
                    "#)}>
                        <components::github::Card />
                        <components::lastfm::Card />
                    </div>
                    <div class={css!(r#"
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        flex-direction: row;
                        gap: 100px;

                        @media screen and (max-width: 1500px) {
                            flex-direction: column;
                        }
                    "#)}>
                        <components::goodreads::Card />
                        <components::letterboxd::Card />
                    </div>
                </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
