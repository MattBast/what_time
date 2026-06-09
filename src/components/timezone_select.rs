use crate::timezone::{City, CITIES};
use crate::url_parse::{add_city_to_url_query, remove_city_from_url_query, url_query_to_cities};
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn TimezoneSelect(
    cities_query: Memo<Option<String>>,
    set_cities_query: SignalSetter<Option<String>>,
) -> impl IntoView {
    // Get a list of all the available cities to present in the dropdown.
    let (city_variants, set_city_variants) = ArcRwSignal::new(CITIES.clone()).split();

    // A list of the cities that have been selected.
    let (selected_cities, set_selected_cities) = ArcRwSignal::new(Vec::new()).split();

    // Get or set the value typed into the search input field.
    let (search_term, set_search_term) = signal(String::new());

    // Use this variable to decide if the dropdown should be showing or not.
    let (show_dropdown, set_show_dropdown) = signal(false);

    // Watch the url query to decide which cities to present in the dropdown.
    Effect::new({
        let set_city_variants = set_city_variants.clone();
        move || {
            add_cities_to_selected_from_url(&cities_query, &set_city_variants, &set_selected_cities)
        }
    });

    // Listen for the `search_term` to be changed
    Effect::new({
        let set_city_variants = set_city_variants.clone();
        move || {
            filter_cities_when_search_term_changes(&search_term, &cities_query, &set_city_variants)
        }
    });

    view! {
        <div class="relative w-full">
            // A search input where the user can type and search for a city.
            <input
                type="text"
                placeholder="Search and add cities..."
                class="input w-full"
                id="timezone_select"
                prop:value=search_term
                // When the contents of the input is changed, update the `search_term`.
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    set_search_term.set(value);
                    set_show_dropdown.set(true);
                }
                on:focus=move |_| set_show_dropdown.set(true)
                on:click=move |_| set_show_dropdown.set(true)
            />

            // Dropdown containing either all the cities or a filtered subset of the cities.
            <ul
                class="absolute z-50 w-full mt-1 bg-base-100 rounded-md shadow-lg max-h-60 overflow-y-auto"
                class:hidden=move || !show_dropdown.get()
            >
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
                                    let current_cities = remove_city_from_url_query(cities_query.get_untracked(), &city.slug);
                                    set_cities_query.set(Some(current_cities));

                                    // Empty the search term and hide the dropdown.
                                    set_search_term.set(String::new());
                                    set_show_dropdown.set(false);
                                }
                            />
                        }
                    }
                />

                // Add an option per city not already showing
                <For
                    // Limit dropdown options to first 100 to ensure high UI performance
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
                                    let new_url = add_city_to_url_query(cities_query.get_untracked(), &city.slug);
                                    set_cities_query.set(Some(new_url));

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
pub fn CitySelectOption(city: City, selected: bool) -> impl IntoView {
    view! {
        <li
            class="group w-full text-left px-4 py-2 border-none cursor-pointer text-base-content"
            class=(["bg-success", "hover:bg-error"], selected)
            class=(["bg-transparent", "hover:bg-success"], !selected)
        >
            <div class="flex justify-between items-center">
                <div>
                    <span class="font-medium">{city.emoji}</span>
                    <span class="font-medium">" "</span>
                    <span class="font-medium">{city.name}</span>
                </div>

                {if selected {
                    view! {
                        <span
                            class="text-2xl font-bold text-success-content group-hover:text-error-content"
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
                        <span class="text-sm text-base-content">
                            {city.country}
                        </span>
                    }.into_any()
                }}
            </div>
        </li>
    }
}

/// Return a list of all available cities not present in the url query.
fn get_cities_not_in_url_query(selected: &[City]) -> Vec<City> {
    let selected_slugs: Vec<String> = selected.iter().map(|c| c.slug.clone()).collect();
    CITIES
        .iter()
        .filter(|city| !selected_slugs.contains(&city.slug))
        .cloned()
        .collect()
}

/// Return a list of all cities that match a search term and are not present in the url query.
fn get_cities_in_search_term(search_term: &str, selected: &[City]) -> Vec<City> {
    let term = search_term.to_lowercase();
    let selected_slugs: Vec<String> = selected.iter().map(|c| c.slug.clone()).collect();
    CITIES
        .iter()
        .filter(|city| {
            (city.name.to_lowercase().contains(&term)
                || city.country.to_lowercase().contains(&term))
                && !selected_slugs.contains(&city.slug)
        })
        .cloned()
        .collect()
}

/// Observe a url and update the cities present in the dropdown.
pub fn add_cities_to_selected_from_url(
    url_query: &Memo<Option<String>>,
    set_city_variants: &ArcWriteSignal<Vec<City>>,
    set_selected_cities: &ArcWriteSignal<Vec<City>>,
) {
    let query = url_query.get().unwrap_or_default();
    let selected = url_query_to_cities(query);

    set_selected_cities.set(selected.clone());

    let mut available = Vec::new();
    let selected_slugs: Vec<String> = selected.into_iter().map(|c| c.slug).collect();
    for city in CITIES.iter() {
        if !selected_slugs.contains(&city.slug) {
            available.push(city.clone());
        }
    }
    set_city_variants.set(available);
}

/// Listen for the `search_term` to be changed
pub fn filter_cities_when_search_term_changes(
    search_term: &ReadSignal<String>,
    url_query: &Memo<Option<String>>,
    set_city_variants: &ArcWriteSignal<Vec<City>>,
) {
    let search_term_str = search_term.get();
    let query = url_query.get().unwrap_or_default();
    let selected = url_query_to_cities(query);

    if !search_term_str.is_empty() {
        set_city_variants.set(get_cities_in_search_term(&search_term_str, &selected));
    } else {
        set_city_variants.set(get_cities_not_in_url_query(&selected));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dropdown_sample_cities_have_no_underscores() {
        for city in CITIES.iter().take(50) {
            assert!(
                !city.name.contains('_'),
                "city name must not contain underscores: {:?}",
                city.name
            );
        }
    }

    #[test]
    fn test_get_all_cities_when_url_query_is_empty() {
        let cities = get_cities_not_in_url_query(&[]);
        assert_eq!(cities.len(), CITIES.len());
    }

    #[test]
    fn test_get_all_but_one_city_when_url_query_contains_one_city() {
        let london = CITIES.iter().find(|c| c.name == "London").unwrap().clone();
        let cities = get_cities_not_in_url_query(&[london.clone()]);

        assert_eq!(cities.len(), CITIES.len() - 1);
        assert!(!cities.contains(&london));
    }

    #[test]
    fn test_get_city_when_city_is_searched_for() {
        let cities = get_cities_in_search_term("london", &[]);
        let london = CITIES.iter().find(|c| c.name == "London").unwrap();
        assert!(cities.contains(london));
    }

    #[test]
    fn test_get_city_when_cased_city_is_searched_for() {
        let cities = get_cities_in_search_term("London", &[]);
        let london = CITIES.iter().find(|c| c.name == "London").unwrap();
        assert!(cities.contains(london));
    }

    #[test]
    fn test_get_city_when_partial_typed_city_is_searched_for() {
        let cities = get_cities_in_search_term("lond", &[]);
        let london = CITIES.iter().find(|c| c.name == "London").unwrap();
        assert!(cities.contains(london));
    }

    #[test]
    fn test_get_one_city_when_country_is_searched_for() {
        let cities = get_cities_in_search_term("belgium", &[]);
        let brussels = CITIES.iter().find(|c| c.name == "Brussels").unwrap();
        assert!(cities.contains(brussels));
    }
}
