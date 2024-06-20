use super::row::Row;
use stylist::yew::styled_component;
use types::Commit;
use yew::prelude::*;

#[styled_component]
pub fn Scroller() -> Html {
    // #[allow(clippy::redundant_closure)]
    // let commits = use_state(|| std::vec::Vec::new());
    // {
    //     let commits = commits.clone();
    //     use_effect_with((), move |()| {
    //         let commits = commits.clone();
    //         wasm_bindgen_futures::spawn_local(async move {
    //             let fetched_commits: Vec<Commit> = Commit::fetch_newest(10).await.unwrap();
    //             commits.set(fetched_commits);
    //         });
    //         || ()
    //     });
    // }

    let commits: Vec<_> = (0..15)
        .map(|i| Commit {
            message: format!("message {i}"),
            repository_name: format!("repo name {i}"),
            url: url::Url::parse("https://github.com/wyatt-avilla/feframe").unwrap(),
            repository_link: url::Url::parse("https://github.com/wyatt-avilla/feframe").unwrap(),
        })
        .collect();

    html! {
        <div class={css!(r#"
            width: 100%;
            height: 100%;
            border-radius: 10px;

            background: #21262d;

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
            { commits.iter().map(|commit| html! { <Row ..commit.clone() /> }).collect::<Vec<_>>() }
        </div>
    }
}
