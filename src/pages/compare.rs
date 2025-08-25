use crate::components::{IntroSubtitle, IntroTitle, Introtext};
use crate::components::{Timecard, TimezoneSelect};
use crate::timezone::TimeIncrement;
use crate::url_parse::url_query_to_time_increments;
use crate::{CURRENT_TIME, ZONE};
use leptos::prelude::*;
use leptos_router::hooks::query_signal;

#[component]
pub fn Compare() -> impl IntoView {
    // Watch the url query to decide whether to show the carousel or not.
    let (url_query, _set_url_query) = query_signal::<String>(ZONE);

    view! {
        <Introtext>
            <IntroTitle>"Compare Timezones"</IntroTitle>
            <IntroSubtitle>"Want to know the difference between two or more timezones? Add some timezones below to see the difference."</IntroSubtitle>
        </Introtext>

        // Only show CompareInner if there are timezones
        <Show
            when=move || !url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty()
            fallback=|| view! { <div></div> }
        >
            <div
                class="flex justify-between w-full transition"
                class=(["mt-16", "sm:mt-20"], move || url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty())
            >
                <CompareInner/>
            </div>
        </Show>

        // A select element that allows the user to add timezones to the carousel
        <TimezoneSelect/>
    }
}

#[component]
pub fn CompareInner() -> impl IntoView {
    // Watch the url to decide which timezones to include and what times to compare.
    let (url_query, _set_url_query) = query_signal::<String>(ZONE);
    let (current_time, _set_current_time) = query_signal::<i64>(CURRENT_TIME);

    let (get_timezones, set_timezones) = signal(Vec::new());

    // Listen for the `zone` url query to change and when it does, re-render the timezones.
    Effect::new(move || {
        // Trigger these actions when the url "zone" query changes.
        let query = url_query.get().unwrap_or_default();

        // Add the timezones from url to the carousel.
        set_timezones.set(url_query_to_time_increments(query.clone()));
    });

    view! {
        <div class="relative w-full">
            <div class="flex justify-center">
                <For
                    each=move || get_timezones.get()
                    key=|timezone| timezone.clone()
                    children=move|timezone| {
                        let hour = match current_time.get() {
                            Some(timestamp) => TimeIncrement::from_timestamp(timestamp),
                            None => TimeIncrement::now(timezone),
                        };

                        view! {
                            <Timecard hour/>
                        }
                    }
                />
            </div>
        </div>
    }
}
