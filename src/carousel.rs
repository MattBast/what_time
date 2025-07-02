use crate::buttons::SideButton;
use crate::timecard::Timecard;
use crate::timezone::{add_24_future_increments, add_24_past_increments};
use crate::timezone_select::TimezoneSelect;
use crate::url_parse::{remove_timezone, url_query_to_time_increments};
use chrono_tz::TZ_VARIANTS;
use leptos::prelude::*;
use leptos_router::hooks::query_signal;

#[component]
pub fn Carousel() -> impl IntoView {
    // Get a list of all the available timezones.
    let (tz_variants, set_tz_variants) =
        ArcRwSignal::new(TZ_VARIANTS.iter().map(|tz| tz.clone()).collect()).split();

    let timezones = ArcRwSignal::new(vec![
        // ArcRwSignal::new(vec![TimeIncrement::now(Tz::Europe__London)]),
        // ArcRwSignal::new(vec![TimeIncrement::now(Tz::Europe__Paris)]),
        // ArcRwSignal::new(vec![TimeIncrement::now(Tz::Asia__Kolkata)]),
        // ArcRwSignal::new(vec![TimeIncrement::now(Tz::Asia__Kathmandu)]),
        // ArcRwSignal::new(vec![TimeIncrement::now(Tz::America__New_York)]),
        // ArcRwSignal::new(vec![TimeIncrement::now(Tz::Asia__Tokyo)]),
    ]);

    let (get_timezones, set_timezones) = timezones.split();
    let set_timezones_2 = set_timezones.clone();

    let set_tz_variants_clone = set_tz_variants.clone();

    // Listen for the `zone` url wuery to change and when it does, re-render the timezones.
    let (url_query, set_url_query) = query_signal::<String>("zone");
    Effect::new(move || {
        // Trigger these actions when the url "zone" query changes.
        let query = url_query.get().unwrap_or_default();

        // Add the timezones from url to the carousel.
        timezones.set(url_query_to_time_increments(query.clone()));

        // Remove the timezones in the url from the dropdown options.
        set_tz_variants_clone.update(|variants| remove_timezone(query, variants));
    });

    // Some reminder code to add increments to the url parameters
    // let (q, set_q) = query_signal::<i32>("past_increments");
    // let (q, set_q) = query_signal::<i32>("future_increments");

    view! {

        <div class="flex justify-between mt-16 sm:mt-20 w-full">

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

        </div>

        // A select element that allows the user to add timezones to the carousel
        <TimezoneSelect url_query set_url_query tz_variants set_tz_variants/>
    }
}
