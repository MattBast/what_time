use crate::timezone::TimeIncrement;
use crate::CURRENT_TIME;
use chrono::DateTime;
use chrono_tz::Tz;
use leptos::prelude::*;
use leptos_router::hooks::query_signal;

#[component]
pub fn TimePicker() -> impl IntoView {
    // Watch the url query to decide whether to show the carousel or not.
    let (current_time, set_current_time) = query_signal::<i64>(CURRENT_TIME);
    let (input_date, set_input_date) = signal(String::new());
    let (input_time, set_input_time) = signal(String::new());

    // Listen for the `current_time` url query to change and when it does, change the value of the input.
    Effect::new(move || {
        // Trigger these actions when the url "current_time" query changes.
        let ti = match current_time.get() {
            Some(timestamp) => TimeIncrement::from_timestamp(timestamp, Tz::UCT),
            None => TimeIncrement::now(Tz::UCT),
        };

        set_input_date.set(ti.input_date());
        set_input_time.set(ti.input_time());
    });

    view! {
        <div class="flex w-full gap-5 justify-end content-end">
            <input
                class="px-4 py-2 border border-zinc-300 dark:border-zinc-600 rounded-md bg-white dark:bg-zinc-800 text-zinc-900 dark:text-zinc-100 focus:outline-none focus:ring-2 focus:ring-teal-500 focus:border-teal-500  dark:[color-scheme:dark] cursor-pointer"
                type="date"
                name="date-picker"
                prop:value=input_date
                on:input:target=move |ev| {
                    let date = ev.target().value();
                    set_input_date.set(date);

                    let dt_str = format!("{} {}:00.000 +0000", input_date.get_untracked(), input_time.get_untracked());
                    let dt = DateTime::parse_from_str(&dt_str, "%Y-%m-%d %H:%M:%S%.3f %z").unwrap();
                    set_current_time.set(Some(dt.timestamp()));
                }
            />

            <input
                class="px-4 py-2 border border-zinc-300 dark:border-zinc-600 rounded-md bg-white dark:bg-zinc-800 text-zinc-900 dark:text-zinc-100 focus:outline-none focus:ring-2 focus:ring-teal-500 focus:border-teal-500  dark:[color-scheme:dark] cursor-pointer"
                type="time"
                name="time-picker"
                prop:value=input_time
                on:input:target=move |ev| {
                    let time = ev.target().value();
                    set_input_time.set(time);

                    let dt_str = format!("{} {}:00.000 +0000", input_date.get_untracked(), input_time.get_untracked());
                    let dt = DateTime::parse_from_str(&dt_str, "%Y-%m-%d %H:%M:%S%.3f %z").unwrap();
                    set_current_time.set(Some(dt.timestamp()));
                }
            />
        </div>
    }
}
