use crate::components::{
    add_timezones_to_selected_from_url, filter_timezones_when_search_term_changes,
    TimezoneSelectOption,
};
use crate::url_parse::{add_timezone_to_url_query, remove_timezone_from_url_query};
use chrono_tz::TZ_VARIANTS;
use leptos::prelude::*;

#[component]
pub fn TimezoneDrawerContent(
    timezones_query: Memo<Option<String>>,
    set_timezones_query: SignalSetter<Option<String>>,
) -> impl IntoView {
    // Get a list of all the available timezones to present in the dropdown.
    let (tz_variants, set_tz_variants) = ArcRwSignal::new(TZ_VARIANTS.to_vec()).split();

    // A list of the timezones that have been selected.
    let (selected_tz_variants, set_selected_tz_variants) = ArcRwSignal::new(Vec::new()).split();

    // Get or set the value typed into the search input field.
    let (search_term, set_search_term) = signal(String::new());

    // Watch the url query to decide which timezones to present in the dropdown.
    Effect::new({
        let set_tz_variants = set_tz_variants.clone();
        move || {
            add_timezones_to_selected_from_url(
                &timezones_query,
                &set_tz_variants,
                &set_selected_tz_variants,
            )
        }
    });

    // Listen for the `search_term` to be changed
    Effect::new({
        let set_tz_variants = set_tz_variants.clone();
        move || {
            filter_timezones_when_search_term_changes(
                &search_term,
                &timezones_query,
                &set_tz_variants,
            )
        }
    });

    view! {
        <div class="menu bg-base-200 min-h-full w-80 block sm:hidden">
            // A search input where the user can type and search for a timezone.
            <input
                type="text"
                placeholder="Search and add timezones..."
                class="input w-full mb-2"
                id="timezone_drawer_search"
                prop:value=search_term
                // When the contents of the input is changed, update the `search_term`.
                // This tells the component to update the list if timezones in the
                // dropdown.
                on:input=move |ev| set_search_term.set(event_target_value(&ev))
            />

            // The list of timezones presented as a drawer
            <ul id="drawer_timezones">
                <For
                    each=move || selected_tz_variants.get()
                    key=|tz| tz.to_string().clone()
                    children=move|tz| {

                        view! {
                            <TimezoneSelectOption
                                tz
                                selected=true
                                // When clicked, the timezone is removed from the url query.
                                // There is logic elsewhere in the app to listen to the
                                // query and remove the timezone from the carousel.
                                on:click=move |_| {
                                    let current_timezones = remove_timezone_from_url_query(timezones_query.get_untracked(), tz);
                                    set_timezones_query.set(Some(current_timezones));

                                    // Empty the search term
                                    set_search_term.set(String::new());
                                }
                            />
                        }
                    }
                />


                // Add an option per timezone not already showing
                <For
                    each=move || tz_variants.get()
                    key=|tz| tz.to_string().clone()
                    children=move|tz| {

                        view! {
                            <TimezoneSelectOption
                                tz
                                selected=false
                                // When clicked, the timezone is added to the url query.
                                // There is logic elsewhere in the app to listen to the
                                // query and update the carousel with the added timezone.
                                on:click=move |_| {
                                    let new_url = add_timezone_to_url_query(timezones_query.get_untracked(), tz);
                                    set_timezones_query.set(Some(new_url));

                                    // Empty the search term
                                    set_search_term.set(String::new());
                                }
                            />
                        }
                    }
                />

            </ul>
        </div>

    }
}
