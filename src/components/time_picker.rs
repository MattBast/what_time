use crate::components::Button;
use crate::timezone::TimeIncrement;
use crate::CURRENT_TIME;
use chrono::offset::LocalResult::Single;
use chrono::prelude::*;
use chrono::DateTime;
use chrono_tz::Tz;
use leptos::prelude::*;
use leptos_router::hooks::query_signal;

#[component]
pub fn TimePicker() -> impl IntoView {
    // Watch the url query to decide whether to show the carousel or not.
    let (_current_time, set_current_time) = query_signal::<i64>(CURRENT_TIME);

    view! {
        <div class="flex w-full gap-5 justify-end content-end">
            <Button
                on:click=move |_| {
                    let dt = TimeIncrement::now(Tz::UCT);
                    set_current_time.set(Some(dt.timestamp()));
                }
            >
                "Now"
            </Button>
        </div>
    }
}

// #[component]
// pub fn DateInput(
//     current_time: Memo<Option<i64>>,
//     set_current_time: SignalSetter<Option<i64>>,
// ) -> impl IntoView {
//     let (input_date, set_input_date) = signal(String::new());

//     view! {
//         <input
//             class="px-4 py-2 border border-zinc-300 dark:border-zinc-600 rounded-md bg-white dark:bg-zinc-800 text-zinc-900 dark:text-zinc-100 focus:outline-none focus:ring-2 focus:ring-teal-500 focus:border-teal-500  dark:[color-scheme:dark] cursor-pointer"
//             type="date"
//             name="date-picker"
//             prop:value=input_date
//             on:input:target=move |ev| {
//                 let date = ev.target().value();
//                 set_input_date.set(date);

//                 let last_time = get_current_time(current_time.get_untracked());

//                 let dt_str = format!("{} 00:00:00.000 +0000", input_date.get_untracked());
//                 if let Ok(dt) = DateTime::parse_from_str(&dt_str, "%Y-%m-%d %H:%M:%S%.3f %z") {
//                     set_current_time.set(Some(last_time.with_day(dt.day()).unwrap().with_month(dt.month()).unwrap().with_year(dt.year()).unwrap().timestamp()))
//                 };
//             }
//         />
//     }
// }

#[component]
pub fn TimeInput(
    current_time: Memo<Option<i64>>,
    set_current_time: SignalSetter<Option<i64>>,
    timezone: Tz,
) -> impl IntoView {
    let last_time = get_current_time(current_time.get_untracked(), timezone);
    let (input_time, set_input_time) = signal(last_time.format("%H:%M").to_string());

    // Listen for the `current_time` url query to change and when it does, re-render the time in the inputs.
    Effect::new(move || {
        let now = get_current_time(current_time.get(), timezone);
        set_input_time.set(now.format("%H:%M").to_string());
    });

    view! {
        <input
            class="
                rounded-md text-zinc-900 dark:text-zinc-100 text-center
                focus:outline-none focus:ring-2 focus:ring-teal-500 focus:border-teal-500
                dark:[color-scheme:dark] cursor-pointer time-input-no-icon
            "
            type="time"
            name="time-picker"
            prop:value=input_time
            on:input:target=move |ev| {
                let last_time = get_current_time(current_time.get_untracked(), timezone);

                if let Ok(new_time) = NaiveTime::parse_from_str(&ev.target().value(), "%H:%M") {
                    if let Single(valid_new_time) = last_time.with_time(new_time) {
                        set_current_time.set(Some(valid_new_time.timestamp()))
                    }
                };
            }
        />
    }
}

fn get_current_time(current_time: Option<i64>, tz: Tz) -> DateTime<Tz> {
    match current_time {
        Some(timestamp) => DateTime::from_timestamp(timestamp, 0)
            .unwrap_or_default()
            .with_timezone(&tz),
        None => Utc::now().with_timezone(&tz),
    }
}

// fn time_input_event(
//     ev: Targeted<Event, HtmlInputElement>,
//     input_time: ReadSignal<String>,
//     set_input_time: WriteSignal<String>,
//     timezone: Tz,
// ) {
//     let time = ev.target().value();

//     set_input_time.set(time);

//     let last_time = get_current_time(current_time.get_untracked(), timezone);

//     let dt_str = format!("1-1-1 {}:00.000 +0000", input_time.get_untracked());
//     if let Ok(dt) = DateTime::parse_from_str(&dt_str, "%Y-%m-%d %H:%M:%S%.3f %z") {
//         set_current_time.set(Some(
//             last_time
//                 .with_hour(dt.hour())
//                 .unwrap()
//                 .with_minute(dt.minute())
//                 .unwrap()
//                 .timestamp(),
//         ))
//     };
// }
