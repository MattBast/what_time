use crate::timezone::{tz_to_city, tz_to_country, tz_to_emoji};
use crate::url_parse::{
    add_timezone_to_url_query, remove_timezone, remove_timezone_from_url_query,
    url_query_to_timezones,
};
use crate::ZONE;
use chrono_tz::{Tz, TZ_VARIANTS};
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::hooks::query_signal;

#[component]
pub fn TimezoneSelect() -> impl IntoView {
    // Watch the url.
    let (url_query, set_url_query) = query_signal::<String>(ZONE);

    // Get a list of all the available timezones to present in the dropdown.
    let (tz_variants, set_tz_variants) = ArcRwSignal::new(TZ_VARIANTS.to_vec()).split();
    let set_tz_variants_clone = set_tz_variants.clone();

    // A list of the timezones that have been selected.
    let (selected_tz_variants, set_selected_tz_variants) = ArcRwSignal::new(Vec::new()).split();

    // Get or set the value typed into the search input field.
    let (search_term, set_search_term) = signal(String::new());

    // Use this variable to decide if the dropdoiwn should be showing or not.
    let (show_dropdown, set_show_dropdown) = signal(false);

    // Watch the url query to decide which timezones to present in the dropdown.
    Effect::new(move || {
        // Trigger these actions when the url "zone" query changes.
        let query = url_query.get().unwrap_or_default();

        // Remove the timezones in the url from the dropdown options.
        set_tz_variants_clone.update(|variants| remove_timezone(query.clone(), variants));

        // And add them to the selected timezones list
        set_selected_tz_variants.set(url_query_to_timezones(query));
    });

    // Listen for the `search_term` to be changed
    Effect::new(move || {
        // Get the latest value typed into the select input field.
        // Trigger filter functionality whenever it changes.
        let search_term_str = search_term.get();

        // Get all the timezones present in the url query
        let query = url_query.get().unwrap_or_default();
        let timezones_from_url = url_query_to_timezones(query);

        if !search_term_str.is_empty() {
            // Filter the timezones to include only the ones who contain
            // the substring typed into the search input field. Also don't
            // present timezones that are present in the url.
            set_tz_variants.set(get_tz_in_search_term(search_term_str, timezones_from_url));
        } else {
            // If nothing is typed into the search field, present all timezones
            // that are not already in the url query.
            set_tz_variants.set(get_tz_not_in_url_query(timezones_from_url));
        }
    });

    view! {
        <div class="relative w-full">
            // A search input where the user can type and search for a timezone.
            <input
                type="text"
                placeholder="Search and add timezones..."
                class="input w-full"
                id="timezone_select"
                prop:value=search_term
                // When the contents of the input is changed, update the `search_term`.
                // This tells the component to update the list if timezones in the
                // dropdown.
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    set_search_term.set(value);
                    set_show_dropdown.set(true);
                }
                on:focus=move |_| set_show_dropdown.set(true)
                on:click=move |_| set_show_dropdown.set(true)
            />

            // Dropdown containing either all the timezones or a filtered subset of the timezones.
            <ul
                class="absolute z-50 w-full mt-1 bg-white dark:bg-zinc-800 border border-zinc-300 dark:border-zinc-600 rounded-md shadow-lg max-h-60 overflow-y-auto"
                class:hidden=move || !show_dropdown.get()
            >
                <For
                    each=move || selected_tz_variants.get()
                    key=|tz| tz.to_string().clone()
                    children=move|tz| {

                        view! {
                            <TimezoneSelectOption
                                emoji=tz_to_emoji(&tz)
                                city=tz_to_city(&tz)
                                tz_country=tz_to_country(&tz)
                                selected=true
                                // When clicked, the timezone is removed from the url query.
                                // There is logic elsewhere in the app to listen to the
                                // query and remove the timezone from the carousel.
                                on:click=move |_| {
                                    let current_timezones = remove_timezone_from_url_query(url_query.get_untracked(), tz);
                                    set_url_query.set(Some(current_timezones));

                                    // Empty the search term and hide the dropdown.
                                    set_search_term.set(String::new());
                                    set_show_dropdown.set(false);
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
                                emoji=tz_to_emoji(&tz)
                                city=tz_to_city(&tz)
                                tz_country=tz_to_country(&tz)
                                selected=false
                                // When clicked, the timezone is added to the url query.
                                // There is logic elsewhere in the app to listen to the
                                // query and update the carousel with the added timezone.
                                on:click=move |_| {
                                    let new_url = add_timezone_to_url_query(url_query.get_untracked(), tz);
                                    set_url_query.set(Some(new_url));

                                    // Empty the search term and hide the dropdown.
                                    set_search_term.set(String::new());
                                    set_show_dropdown.set(false);
                                }
                            />
                        }
                    }
                />

            </ul>
        </div>

        // Click outside to close dropdown
        <Show when=move || show_dropdown.get()>
            <div
                class="fixed inset-0 z-40"
                on:click=move |_| set_show_dropdown.set(false)
            ></div>
        </Show>
    }
}

#[component]
pub fn TimezoneSelectOption(
    emoji: String,
    city: String,
    tz_country: String,
    selected: bool,
) -> impl IntoView {
    view! {
        <li
            class="group w-full text-left px-4 py-2 border-none cursor-pointer text-zinc-900 dark:text-zinc-100"
            class=(["bg-teal-100", "dark:bg-teal-700", "hover:bg-red-100", "dark:hover:bg-red-700"], selected)
            class=(["bg-transparent", "hover:bg-zinc-100", "dark:hover:bg-zinc-700"], !selected)
        >
            <div class="flex justify-between items-center">
                <div>
                    <span class="font-medium">{emoji}</span>
                    <span class="font-medium">" "</span>
                    <span class="font-medium">{city}</span>
                </div>

                {if selected {
                    view! {
                        <span
                            class="
                                text-2xl font-bold text-teal-500 dark:text-teal-400
                                group-hover:text-red-500 group-hover:dark:text-red-400
                            "
                        >
                            <span class="group-hover:hidden">
                                <Icon icon=icondata::BiCheckRegular />
                            </span>
                            <span class="hidden group-hover:inline">
                                <Icon icon=icondata::BiTrashRegular />
                            </span>
                        </span>
                    }.into_any()
                } else {
                    view! {
                        <span class="text-sm text-zinc-500 dark:text-zinc-400">
                            {tz_country}
                        </span>
                    }.into_any()
                }}
            </div>
        </li>
    }
}

/// Return a list of all available timezones not present in the url query.
fn get_tz_not_in_url_query(timezones_from_url: Vec<Tz>) -> Vec<Tz> {
    TZ_VARIANTS
        .iter()
        .filter(|tz| !timezones_from_url.contains(tz))
        .copied()
        .collect()
}

/// Return a list of all timezones that match a search term and are not present in the url query.
fn get_tz_in_search_term(search_term: String, timezones_from_url: Vec<Tz>) -> Vec<Tz> {
    TZ_VARIANTS
        .iter()
        .filter(|tz| {
            let city = tz_to_city(tz);
            let country = tz_to_country(tz);

            format!("{} {}", city.to_lowercase(), country.to_lowercase())
                .contains(&search_term.to_lowercase())
                && !timezones_from_url.contains(tz)
        })
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_timezones_when_url_query_is_empty() {
        let timezones = get_tz_not_in_url_query(Vec::new());
        assert_eq!(timezones, TZ_VARIANTS);
    }

    #[test]
    fn test_get_all_but_one_timezone_when_url_query_contains_one_timezone() {
        let timezones = get_tz_not_in_url_query(vec![Tz::Europe__London]);

        assert_eq!(timezones.len(), TZ_VARIANTS.len() - 1);
        assert!(!timezones.contains(&Tz::Europe__London));
    }

    #[test]
    fn test_get_all_but_two_timezones_when_url_query_contains_two_timezones() {
        let timezones = get_tz_not_in_url_query(vec![Tz::Europe__London, Tz::Europe__Amsterdam]);

        assert_eq!(timezones.len(), TZ_VARIANTS.len() - 2);
        assert!(!timezones.contains(&Tz::Europe__London));
        assert!(!timezones.contains(&Tz::Europe__Amsterdam));
    }

    #[test]
    fn test_get_timezone_when_city_is_searched_for() {
        let timezones = get_tz_in_search_term("london".into(), Vec::new());
        assert_eq!(timezones, vec![Tz::Europe__London]);
    }

    #[test]
    fn test_get_timezone_when_cased_city_is_searched_for() {
        let timezones = get_tz_in_search_term("London".into(), Vec::new());
        assert_eq!(timezones, vec![Tz::Europe__London]);
    }

    #[test]
    fn test_get_timezone_when_partial_typed_city_is_searched_for() {
        let timezones = get_tz_in_search_term("lond".into(), Vec::new());
        assert_eq!(timezones, vec![Tz::Europe__London]);
    }

    #[test]
    fn test_get_many_timezones_when_partial_typed_city_is_searched_for() {
        let timezones = get_tz_in_search_term("lon".into(), Vec::new());
        assert_eq!(
            timezones,
            vec![
                Tz::America__BlancSablon,
                Tz::America__Miquelon,
                Tz::Arctic__Longyearbyen,
                Tz::Europe__London
            ]
        );
    }

    #[test]
    fn test_get_one_timezone_when_country_is_searched_for() {
        let timezones = get_tz_in_search_term("belgium".into(), Vec::new());
        assert_eq!(timezones, vec![Tz::Europe__Brussels]);
    }

    #[test]
    fn test_get_many_timezones_when_country_is_searched_for() {
        let timezones = get_tz_in_search_term("germany".into(), Vec::new());
        assert_eq!(timezones, vec![Tz::Europe__Berlin, Tz::Europe__Busingen]);
    }
}
