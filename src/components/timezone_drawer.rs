use crate::components::{
    add_cities_to_selected_from_url, filter_cities_when_search_term_changes, CitySelectOption,
};
use crate::timezone::CITIES;
use crate::url_parse::{add_city_to_url_query, remove_city_from_url_query};
use leptos::prelude::*;

#[component]
pub fn TimezoneDrawerContent(
    timezones_query: Memo<Option<String>>,
    set_timezones_query: SignalSetter<Option<String>>,
    search_input_ref: NodeRef<leptos::html::Input>,
) -> impl IntoView {
    // Get a list of all the available cities to present in the dropdown.
    let (city_variants, set_city_variants) = ArcRwSignal::new(CITIES.clone()).split();

    // A list of the cities that have been selected.
    let (selected_cities, set_selected_cities) = ArcRwSignal::new(Vec::new()).split();

    // Get or set the value typed into the search input field.
    let (search_term, set_search_term) = signal(String::new());

    // Watch the url query to decide which cities to present in the dropdown.
    Effect::new({
        let set_city_variants = set_city_variants.clone();
        move || {
            add_cities_to_selected_from_url(
                &timezones_query,
                &set_city_variants,
                &set_selected_cities,
            )
        }
    });

    // Listen for the `search_term` to be changed
    Effect::new({
        let set_city_variants = set_city_variants.clone();
        move || {
            filter_cities_when_search_term_changes(
                &search_term,
                &timezones_query,
                &set_city_variants,
            )
        }
    });

    view! {
        <div class="menu bg-base-200 min-h-full w-80">
            // A search input where the user can type and search for a city.
            <input
                node_ref=search_input_ref
                type="text"
                placeholder="Search and add cities..."
                class="input w-full mb-2"
                id="timezone_drawer_search"
                prop:value=search_term
                // When the contents of the input is changed, update the `search_term`.
                on:input=move |ev| set_search_term.set(event_target_value(&ev))
            />

            // The list of cities presented as a drawer
            <ul id="drawer_timezones">
                <For
                    each=move || selected_cities.get()
                    key=|city| city.slug.clone()
                    children=move|city| {
                        let c = city.clone();
                        view! {
                            <CitySelectOption
                                city=c
                                selected=true
                                on:click=move |_| {
                                    let current_cities = remove_city_from_url_query(timezones_query.get_untracked(), &city.slug);
                                    set_timezones_query.set(Some(current_cities));

                                    // Empty the search term
                                    set_search_term.set(String::new());
                                }
                            />
                        }
                    }
                />

                // Add an option per city not already showing
                <For
                    // Limit unselected drawer options to first 100 to ensure high UI performance
                    each=move || {
                        let vars = city_variants.get();
                        vars.into_iter().take(100).collect::<Vec<_>>()
                    }
                    key=|city| city.slug.clone()
                    children=move|city| {
                        let c = city.clone();
                        view! {
                            <CitySelectOption
                                city=c
                                selected=false
                                on:click=move |_| {
                                    let new_url = add_city_to_url_query(timezones_query.get_untracked(), &city.slug);
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
