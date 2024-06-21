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
                            <img src="/assets/cake.svg" alt="Cake Icon" />
                            { age() }
                        </div>
                        <div class={css!(r#"
                            display: flex;
                            align-items: center;
                            gap: 5px;
                        "#)}>
                            <img src="/assets/clock.svg" alt="Clock Icon" />
                            { Utc::now().with_timezone(&Pacific).format("%l:%M %p") }
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
