use crate::buttons::SideButton;
use crate::timecard::Timecard;
use crate::timezone::TimeIncrement;
use crate::timezone::{new_future_increments, new_past_increments};
use crate::timezone_select::TimezoneSelect;
use crate::url_parse::url_query_to_time_increments;
use chrono_tz::Tz;
use leptos::prelude::*;
use leptos_router::hooks::query_signal;

#[component]
pub fn Carousel() -> impl IntoView {
    // Watch the url query to decide whether to show the carousel or not.
    let (url_query, _set_url_query) = query_signal::<String>("zone");

    view! {
        // Only show CarouselInner if there are timezones
        <Show
            when=move || !url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty()
            fallback=|| view! { <div></div> }
        >
            <div class="flex justify-between mt-16 sm:mt-20 w-full">
                <CarouselInner/>
            </div>
        </Show>

        // A select element that allows the user to add timezones to the carousel
        <TimezoneSelect/>
    }
}

#[component]
pub fn CarouselInner() -> impl IntoView {
    let (url_query, _set_url_query) = query_signal::<String>("zone");

    let (get_timezones, set_timezones) = signal(Vec::new());

    // Listen for the `zone` url query to change and when it does, re-render the timezones.
    Effect::new(move || {
        // Trigger these actions when the url "zone" query changes.
        let query = url_query.get().unwrap_or_default();

        // Add the timezones from url to the carousel.
        set_timezones.set(url_query_to_time_increments(query.clone()));
    });

    view! {
        <SideButton
            on:click=move |_| {

                // Update the url query to instruct the app to add 24 time increments
                // to all presented timezones.
                let (i, set_i) = query_signal::<i32>("past_increments");
                let current_i = i.get_untracked().unwrap_or_default();
                set_i.set(Some(current_i + 24));

            }
        >
            "+ 24 Hours"
        </SideButton>

        // Create a line of tiezone increments for every timezone present.
        <div class="overflow-scroll mx-8 sm:mx-10 snap-x snap-mandatory scroll-smooth">

            <For
                each=move || get_timezones.get()
                key=|timezone| timezone.clone()
                children=move|timezone| {

                    view! {
                        <TimezoneLine timezone/>
                    }
                }
            />

        </div>

        <SideButton
            on:click=move |_| {

                // Update the url query to instruct the app to add 24 time increments
                // to all presented timezones.s
                let (i, set_i) = query_signal::<i32>("future_increments");
                let current_i = i.get_untracked().unwrap_or_default();
                set_i.set(Some(current_i + 24));

            }
        >
            "+ 24 Hours"
        </SideButton>
    }
}

#[component]
pub fn TimezoneLine(timezone: Tz) -> impl IntoView {
    let (past_increments, _) = query_signal::<i32>("past_increments");
    let (future_increments, _) = query_signal::<i32>("future_increments");

    // Create a vector of [TimeIncrement] for the Timezone.
    let (increments, set_increments) = signal(Vec::new());

    // Add or remove time increments when the url query changes.
    Effect::new(move || {
        // Watch the future and past increment url queries
        let current_past_increments = past_increments.get().unwrap_or_default();
        let current_future_increments = future_increments.get().unwrap_or_default();

        // Create a vector [TimeIncrement] for the length specified in the url.
        let mut new_past_increments = new_past_increments(current_past_increments, &timezone);
        let mut new_future_increments = new_future_increments(current_future_increments, &timezone);
        let mut now = vec![TimeIncrement::now(timezone)];

        new_past_increments.append(&mut now);
        new_past_increments.append(&mut new_future_increments);

        // Set the increments to instruct the page to change.
        set_increments.set(new_past_increments);
    });

    view! {
        <div class="flex gap-5 py-4 sm:gap-8">

            // Creates a card for every time increment present.
            <For
                each=move || increments.get()
                key=|increment| increment.datetime.clone()
                let(hour)
            >

                <Timecard hour/>

            </For>

        </div>
    }
}
