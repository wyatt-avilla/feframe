use stylist::yew::styled_component;
use yew::prelude::*;

use types::Commit;

#[styled_component]
pub fn Row(props: &Commit) -> Html {
    let Commit {
        message,
        url,
        repository_name,
        repository_link,
    } = props;

    let repo_svg = html! {
        <svg fill="#89929b" width="16px" height="16px" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" d="M3 2.75A2.75 2.75 0 015.75 0h14.5a.75.75 0 01.75.75v20.5a.75.75 0 01-.75.75h-6a.75.75 0 010-1.5h5.25v-4H6A1.5 1.5 0 004.5 18v.75c0 .716.43 1.334 1.05 1.605a.75.75 0 01-.6 1.374A3.25 3.25 0 013 18.75v-16zM19.5 1.5V15H6c-.546 0-1.059.146-1.5.401V2.75c0-.69.56-1.25 1.25-1.25H19.5z"/>
            <path d="M7 18.25a.25.25 0 01.25-.25h5a.25.25 0 01.25.25v5.01a.25.25 0 01-.397.201l-2.206-1.604a.25.25 0 00-.294 0L7.397 23.46a.25.25 0 01-.397-.2v-5.01z"/>
        </svg>
    };

    html! {
        <div class={css!(r#"
            width: 100%;
            display: flex;
            flex-direction: column;
            padding: 10px;
            border-bottom: 1px solid #8D96A0;
            box-sizing: border-box;
            background-color: none;
            &:hover {
                border-radius: 5px;
                background-color: #2A3D52;
            }
        "#)}>
            <div class={css!(r#"
                display: flex;
                justify-content: space-between;
                align-items: center;
            "#)}>
                <a href={url.to_string().clone()} class={css!(r#"
                    color: #ecf2f8;
                    text-decoration: none;
                "#)}>
                    { message }
                </a>
                <div class={css!(r#"
                    display: flex;
                    align-items: top-center;
                "#)}>
                    <a href={repository_link.to_string().clone()} class={css!(r#"
                        color: #77bdfb;
                        text-decoration: none;
                        margin-right: 5px;
                        margin-left: 15px;
                        overflow: hidden;
                        white-space: nowrap;
                    "#)}>
                        { repository_name }
                    </a>
                    { repo_svg.clone() }
                </div>
            </div>
        </div>
    }
}
