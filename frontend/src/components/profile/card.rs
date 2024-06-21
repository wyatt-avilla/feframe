use chrono::{Datelike, Local, TimeZone, Utc};
use chrono_tz::US::Pacific;
use stylist::yew::styled_component;
use yew::prelude::*;

fn age() -> i32 {
    let now = Local::now();
    let birthday = Local.with_ymd_and_hms(2003, 2, 24, 0, 0, 0).unwrap();
    let age = now.year() - birthday.year();

    if now.month() < birthday.month()
        || (now.month() == birthday.month() && now.day() < birthday.day())
    {
        age - 1
    } else {
        age
    }
}

#[styled_component]
pub fn Card() -> Html {
    let cake_svg = html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="#2A2A37" width="24px" height="24px" viewBox="0 0 24 24">
            <path d="M12 6c1.11 0 2-.9 2-2 0-.38-.1-.73-.29-1.03L12 0l-1.71 2.97c-.19.3-.29.65-.29 1.03 0 1.1.9 2 2 2zm4.6 9.99l-1.07-1.07-1.08 1.07c-1.3 1.3-3.58 1.31-4.89 0l-1.07-1.07-1.09 1.07C6.75 16.64 5.88 17 4.96 17c-.73 0-1.4-.23-1.96-.61V21c0 .55.45 1 1 1h16c.55 0 1-.45 1-1v-4.61c-.56.38-1.23.61-1.96.61-.92 0-1.79-.36-2.44-1.01zM18 9h-5V7h-2v2H6c-1.66 0-3 1.34-3 3v1.54c0 1.08.88 1.96 1.96 1.96.52 0 1.02-.2 1.38-.57l2.14-2.13 2.13 2.13c.74.74 2.03.74 2.77 0l2.14-2.13 2.13 2.13c.37.37.86.57 1.38.57 1.08 0 1.96-.88 1.96-1.96V12C21 10.34 19.66 9 18 9z"/>
        </svg>
    };

    let clock_svg = html! {
        <svg width="24px" height="24px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M12 7V12L14.5 10.5M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="#2A2A37" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
    };

    html! {
        <div>
            <div class={css!(r#"
                box-shadow: 0 0 5px 1px rgba(0, 0, 0, 0.7);
                height: 400px;
                width: 300px;
                border-radius: 10px;

                display: flex;
                flex-direction: column;

                padding: 30px;
                box-sizing: border-box;

                background-color: #766b90;
            "#)} id="profile-card">
                <div class={css!(r#"
                    flex: 1;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: top;
                    gap: 10px;
                "#)}>
                    <img src="/assets/me.jpg" alt="Profile Image" class={css!(r#"
                        width: 150px;
                        height: 150px;
                        border-radius: 50%;
                        margin-bottom: 10px;
                        border: 2px solid #2A2A37;
                    "#)} />
                    <div class={css!(r#"
                        font-size: 30px;
                        font-weight: 900;
                        color: #DCD7BA;
                        "#)}>
                        { "Wyatt" }
                    </div>
                    <div class={css!(r#"
                        font-size: 18px;
                        font-weight: 600;
                        color: #DCD7BA;
                        "#)}>
                        { "he/him" }
                    </div>
                    <div class={css!(r#"
                        width: 80%;
                        height: 1px;
                        background-color: #2A2A37;
                        margin: 15px 0;
                    "#)}>
                    </div>
                    <div class={css!(r#"
                        font-size: 20px;
                        font-weight: 400;
                        color: #DCD7BA;
                        text-align: center;
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        gap: 10px;
                    "#)}>
                        <div class={css!(r#"
                            display: flex;
                            align-items: center;
                            gap: 5px;
                        "#)}>
                            { cake_svg }
                            { age() }
                        </div>
                        <div class={css!(r#"
                            display: flex;
                            align-items: center;
                            gap: 5px;
                        "#)}>
                            { clock_svg }
                            { Utc::now().with_timezone(&Pacific).format("%H:%M %p") }
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
