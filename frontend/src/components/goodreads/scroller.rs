use super::row::Row;
use config::ENDPOINT;
use stylist::yew::styled_component;
use types::Book;
use yew::prelude::*;

#[styled_component]
pub fn Scroller() -> Html {
    #[allow(clippy::redundant_closure)]
    let books = use_state(|| std::vec::Vec::new());
    {
        let books = books.clone();
        use_effect_with((), move |()| {
            let books = books.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = reqwest::get(format!("{}{}", ENDPOINT.base, ENDPOINT.goodreads))
                    .await
                    .unwrap();
                let fetched_books: Vec<Book> = response.json().await.unwrap();
                books.set(fetched_books);
            });
            || ()
        });
    }

    html! {
        <div class={css!(r#"
            width: 100%;
            height: 100%;
            border-radius: 10px;

            background: #FFFFFF;

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
            { books.iter().map(|book| html! { <Row ..book.clone() /> }).collect::<Vec<_>>() }
        </div>
    }
}
