use crate::buttons::SideButton;
use crate::timecard::Timecard;
use crate::timezone::{add_24_future_increments, add_24_past_increments};
use crate::timezone_select::TimezoneSelect;
use crate::url_parse::url_query_to_time_increments;
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
    let timezones = ArcRwSignal::new(Vec::new());

    let (get_timezones, set_timezones) = timezones.split();

    // Listen for the `zone` url query to change and when it does, re-render the timezones.
    let (url_query, _set_url_query) = query_signal::<String>("zone");
    Effect::new(move || {
        // Trigger these actions when the url "zone" query changes.
        let query = url_query.get().unwrap_or_default();

        // Add the timezones from url to the carousel.
        timezones.set(url_query_to_time_increments(query.clone()));
    });

    let set_timezones_2 = set_timezones.clone();

    // Some reminder code to add increments to the url parameters
    // let (q, set_q) = query_signal::<i32>("past_increments");
    // let (q, set_q) = query_signal::<i32>("future_increments");

    view! {
        // When clicked, this button adds 24 past future time increments to all timezones
        <SideButton
            on:click=move |_| {

                set_timezones.update(|tz_list| {
                    for timezone in tz_list {
                        timezone.update(|i| {
                            add_24_past_increments(i);
                        });
                    }
                });

            }
        >
            "+ 24 Hours"
        </SideButton>

        // Creates a card for every time increment present.
        <div class="overflow-scroll mx-8 sm:mx-10 snap-x snap-mandatory scroll-smooth">

            <For
                each=move || get_timezones.get()
                key=|timezone| timezone.get_untracked().first().unwrap().timezone.clone()
                children=move|increments| {

                    view! {
                        <div class="flex gap-5 py-4 sm:gap-8">

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
            />

        </div>

        // When clicked, this button adds 24 future future time increments to all timezones
        <SideButton
            on:click=move |_| {

                set_timezones_2.update(|tz_list| {
                    for timezone in tz_list {
                        timezone.update(|i| {
                            add_24_future_increments(i);
                        });
                    }
                });

            }
        >
            "+ 24 Hours"
        </SideButton>
    }
}
