use crate::timezone::TimeIncrement;
use crate::CURRENT_TIME;
use leptos::prelude::*;
use leptos_router::hooks::query_signal;

#[component]
pub fn TimePicker() -> impl IntoView {
    // Watch the url query to decide whether to show the carousel or not.
    let (current_time, _set_current_time) = query_signal::<i64>(CURRENT_TIME);
    let (input_date, set_input_date) = signal(String::new());
    let (input_time, set_input_time) = signal(String::new());

    // Listen for the `current_time` url query to change and when it does, change the value of the input.
    Effect::new(move || {
        // Trigger these actions when the url "current_time" query changes.
        let timestamp = current_time.get().unwrap_or_default();

        let ti = TimeIncrement::from_timestamp(timestamp);

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
            />

            <input
                class="px-4 py-2 border border-zinc-300 dark:border-zinc-600 rounded-md bg-white dark:bg-zinc-800 text-zinc-900 dark:text-zinc-100 focus:outline-none focus:ring-2 focus:ring-teal-500 focus:border-teal-500  dark:[color-scheme:dark] cursor-pointer"
                type="time"
                name="time-picker"
                prop:value=input_time
            />
        </div>
    }
}
