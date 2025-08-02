use crate::app::{FUTURE_INCREMENTS, PAST_INCREMENTS, ZONE};
use crate::buttons::SideButton;
use crate::timecard::Timecard;
use crate::timezone::TimeIncrement;
use crate::timezone::{new_future_increments, new_past_increments};
use crate::timezone_select::TimezoneSelect;
use crate::url_parse::url_query_to_time_increments;
use chrono_tz::Tz;
use leptos::html::Div;
use leptos::prelude::*;
use leptos_router::hooks::query_signal;
use leptos_use::{use_element_bounding, UseElementBoundingReturn};

#[component]
pub fn Carousel() -> impl IntoView {
    // Watch the url query to decide whether to show the carousel or not.
    let (url_query, _set_url_query) = query_signal::<String>(ZONE);

    view! {
        // Only show CarouselInner if there are timezones
        <Show
            when=move || !url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty()
            fallback=|| view! { <div></div> }
        >
            <div
                class="flex justify-between w-full transition"
                class=(["mt-16", "sm:mt-20"], move || url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty())
            >
                <CarouselInner/>
            </div>
        </Show>

        // A select element that allows the user to add timezones to the carousel
        <TimezoneSelect/>
    }
}

#[component]
pub fn CarouselInner() -> impl IntoView {
    let (url_query, _set_url_query) = query_signal::<String>(ZONE);

    let (get_timezones, set_timezones) = signal(Vec::new());

    // Listen for the `zone` url query to change and when it does, re-render the timezones.
    Effect::new(move || {
        // Trigger these actions when the url "zone" query changes.
        let query = url_query.get().unwrap_or_default();

        // Add the timezones from url to the carousel.
        set_timezones.set(url_query_to_time_increments(query.clone()));
    });

    // Load more time increments when the right spinner is visible.
    let leftSpinnerRef = NodeRef::<Div>::new();
    let rightSpinnerRef = NodeRef::<Div>::new();

    // Element visibility only works on the client side. Adding this config
    // ensures that it never runs on the server.
    #[cfg(feature = "hydrate")]
    {
        use leptos_use::use_element_visibility;

        const ADD_INCREMENTS: i32 = 24;

        // When the left spinner is visible, add more past time increments.
        let leftSpinnerVisible = use_element_visibility(leftSpinnerRef);

        let (past, set_past) = query_signal::<i32>(PAST_INCREMENTS);
        let (future, set_future) = query_signal::<i32>(FUTURE_INCREMENTS);

        Effect::new(move || {
            if leftSpinnerVisible.get() {
                // Store the current scroll position before adding new content
                let scroll_container = leftSpinnerRef
                    .get_untracked()
                    .unwrap()
                    .parent_element()
                    .unwrap();
                let current_scroll_left = scroll_container.scroll_left();

                // Update the url query to instruct the app to add 24 time increments
                // to all presented timezones.
                let current_past = past.get_untracked().unwrap_or_default();
                set_past.set(Some(current_past + ADD_INCREMENTS));

                // After the DOM updates, adjust the scroll position
                // Each timecard is 160px wide (w-40 = 10rem = 160px) plus gap
                // Estimate ~200px per card including gaps
                let new_content_width = ADD_INCREMENTS * 200; // Approximate width of new cards

                // Use request_animation_frame to wait for the DOM to be updated.
                request_animation_frame(move || {
                    // Then adjust the scroll position so the user is scrolled back
                    // to the cards they saw before the new past increments were added.
                    scroll_container.set_scroll_left(current_scroll_left + new_content_width);
                });
            }
        });

        // When the right spinner is visible, add more future time increments.
        let rightSpinnerVisible = use_element_visibility(rightSpinnerRef);

        Effect::new(move || {
            if rightSpinnerVisible.get() {
                // Update the url query to instruct the app to add 24 time increments
                // to all presented timezones.
                let current_future = future.get_untracked().unwrap_or_default();
                set_future.set(Some(current_future + ADD_INCREMENTS));
            }
        });
    }

    view! {
        <div class="relative w-full">
            // Create a line of timezone increments for every timezone present.
            <div class="flex overflow-scroll no-scrollbar mx-8 sm:mx-10 snap-x snap-mandatory scroll-smooth w-auto">

                <SideButton node_ref=leftSpinnerRef/>

                <div class="flex-1">
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

                <SideButton node_ref=rightSpinnerRef/>
            </div>

        </div>
    }
}

#[component]
pub fn TimezoneLine(timezone: Tz) -> impl IntoView {
    let (past_increments, _) = query_signal::<i32>(PAST_INCREMENTS);
    let (future_increments, _) = query_signal::<i32>(FUTURE_INCREMENTS);

    // Create a vector of [TimeIncrement] for the Timezone.
    let (increments, set_increments) = signal(Vec::new());

    // Create a center detector element
    let center_detector_ref = NodeRef::<Div>::new();

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

    // Use element visibility to detect when the center detector is visible
    let UseElementBoundingReturn { right, left, .. } = use_element_bounding(center_detector_ref);

    view! {
        <div class="flex gap-5 py-4 sm:gap-8 justify-center content-center h-48">

            // Track which time increment is in the centre of the screen
            <div
                node_ref=center_detector_ref
                class="
                    absolute left-1/2 -translate-x-1/2 -inset-x-4 -inset-y-6
                    bg-zinc-100 sm:rounded-2xl dark:bg-zinc-800/50
                    opacity-0 scale-95 hover:opacity-100 hover:scale-100 transition
                    z-0 pointer-events-none w-50
                " // remove pointer-events-none if we want hover events. Including it though makes scrolling difficult if the mouse is over it.
            />

            // Creates a card for every time increment present.
            <For
                each=move || increments.get()
                key=|increment| increment.datetime.clone()
                let(hour)
            >

                <Timecard hour centre_left=left centre_right=right/>

            </For>

        </div>
    }
}
