use crate::components::Button;
use crate::timezone::TimeIncrement;
use crate::CURRENT_TIME;
use chrono::prelude::*;
use chrono::{DateTime, Timelike};
use chrono_tz::Tz;
use leptos::prelude::*;
use leptos_router::hooks::query_signal;

#[component]
pub fn TimePicker() -> impl IntoView {
    // Watch the url query to decide whether to show the carousel or not.
    let (current_time, set_current_time) = query_signal::<i64>(CURRENT_TIME);

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

            <DateInput current_time set_current_time></DateInput>
            <TimeInput current_time set_current_time></TimeInput>
        </div>
    }
}

#[component]
pub fn DateInput(
    current_time: Memo<Option<i64>>,
    set_current_time: SignalSetter<Option<i64>>,
) -> impl IntoView {
    let (input_date, set_input_date) = signal(String::new());

    view! {
        <input
            class="px-4 py-2 border border-zinc-300 dark:border-zinc-600 rounded-md bg-white dark:bg-zinc-800 text-zinc-900 dark:text-zinc-100 focus:outline-none focus:ring-2 focus:ring-teal-500 focus:border-teal-500  dark:[color-scheme:dark] cursor-pointer"
            type="date"
            name="date-picker"
            prop:value=input_date
            on:input:target=move |ev| {
                let date = ev.target().value();
                set_input_date.set(date);

                let last_time = get_current_time(current_time.get_untracked());

                let dt_str = format!("{} 00:00:00.000 +0000", input_date.get_untracked());
                if let Ok(dt) = DateTime::parse_from_str(&dt_str, "%Y-%m-%d %H:%M:%S%.3f %z") {
                    set_current_time.set(Some(last_time.with_day(dt.day()).unwrap().with_month(dt.month()).unwrap().with_year(dt.year()).unwrap().timestamp()))
                } else {
                    println!("Date picker failed")
                };
            }
        />
    }
}

#[component]
pub fn TimeInput(
    current_time: Memo<Option<i64>>,
    set_current_time: SignalSetter<Option<i64>>,
) -> impl IntoView {
    let (input_time, set_input_time) = signal(String::new());

    view! {
        <input
            class="px-4 py-2 border border-zinc-300 dark:border-zinc-600 rounded-md bg-white dark:bg-zinc-800 text-zinc-900 dark:text-zinc-100 focus:outline-none focus:ring-2 focus:ring-teal-500 focus:border-teal-500  dark:[color-scheme:dark] cursor-pointer"
            type="time"
            name="time-picker"
            prop:value=input_time
            on:input:target=move |ev| {
                let time = ev.target().value();
                set_input_time.set(time);

                let last_time = get_current_time(current_time.get_untracked());

                let dt_str = format!("1-1-1 {}:00.000 +0000", input_time.get_untracked());
                if let Ok(dt) = DateTime::parse_from_str(&dt_str, "%Y-%m-%d %H:%M:%S%.3f %z") {
                    set_current_time.set(Some(last_time.with_hour(dt.hour()).unwrap().with_minute(dt.minute()).unwrap().timestamp()))
                } else {
                    println!("Time picker failed")
                };
            }
        />
    }
}

fn get_current_time(current_time: Option<i64>) -> DateTime<Utc> {
    match current_time {
        Some(timestamp) => DateTime::from_timestamp(timestamp, 0).unwrap_or_default(),
        None => Utc::now(),
    }
}
