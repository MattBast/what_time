use crate::components::{
    add_cities_to_selected_from_url, filter_cities_when_search_term_changes, CitySelectOption,
};
use crate::timezone::{City, CITIES};
use crate::CURRENT_TIME;
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_query_map};

#[component]
pub fn TimezoneDrawerContent(
    timezones_query: Memo<Option<String>>,
    set_timezones_query: SignalSetter<Option<String>>,
    search_input_ref: NodeRef<leptos::html::Input>,
) -> impl IntoView {
    // `set_timezones_query` is kept as a prop for API compatibility but navigation
    // is now handled by route-based URLs instead of query-string mutations.
    let _ = set_timezones_query;

    let navigate = use_navigate();
    let query_map = use_query_map();

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
                    each={
                        let selected_cities = selected_cities.clone();
                        move || selected_cities.get()
                    }
                    key=|city| city.slug.clone()
                    children={
                        let selected_cities = selected_cities.clone();
                        let navigate = navigate.clone();
                        move |city| {
                            let navigate = navigate.clone();
                            let selected_cities = selected_cities.clone();
                            let c = city.clone();
                            view! {
                                <CitySelectOption
                                    city=c
                                    selected=true
                                    on:click={
                                        let navigate = navigate.clone();
                                        let selected_cities = selected_cities.clone();
                                        move |_| {
                                            let current_time = query_map
                                                .with_untracked(|m| m.get(CURRENT_TIME));
                                            let route = remove_city_from_route(
                                                &selected_cities.get_untracked(),
                                                &city.slug,
                                                current_time,
                                            );
                                            navigate(&route, Default::default());
                                            set_search_term.set(String::new());
                                        }
                                    }
                                />
                            }
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
                    children={
                        let selected_cities = selected_cities.clone();
                        let navigate = navigate.clone();
                        move |city| {
                            let navigate = navigate.clone();
                            let selected_cities = selected_cities.clone();
                            let c = city.clone();
                            view! {
                                <CitySelectOption
                                    city=c
                                    selected=false
                                    on:click={
                                        let navigate = navigate.clone();
                                        let selected_cities = selected_cities.clone();
                                        move |_| {
                                            let current_time = query_map
                                                .with_untracked(|m| m.get(CURRENT_TIME));
                                            let route = add_city_to_route(
                                                &selected_cities.get_untracked(),
                                                &city.slug,
                                                current_time,
                                            );
                                            navigate(&route, Default::default());
                                            set_search_term.set(String::new());
                                        }
                                    }
                                />
                            }
                        }
                    }
                />
            </ul>
        </div>
    }
}

/// Build a `/compare/…` route path from an ordered list of city slugs.
/// Returns `"/"` when the slice is empty so callers can navigate back to the home page.
pub(crate) fn route_path_for_slugs(slugs: &[String]) -> String {
    if slugs.is_empty() {
        "/".to_string()
    } else {
        format!("/compare/{}", slugs.join("/"))
    }
}

/// Append `?current_time=<value>` to `path` when a value is present.
pub(crate) fn route_path_with_time(path: &str, current_time: Option<String>) -> String {
    match current_time {
        Some(t) if !t.is_empty() => format!("{path}?{CURRENT_TIME}={t}"),
        _ => path.to_string(),
    }
}

/// Build the route obtained by adding `new_slug` after the currently-selected cities,
/// preserving the `current_time` query parameter when provided.
pub(crate) fn add_city_to_route(
    selected: &[City],
    new_slug: &str,
    current_time: Option<String>,
) -> String {
    let mut slugs: Vec<String> = selected.iter().map(|c| c.slug.clone()).collect();
    slugs.push(new_slug.to_string());
    route_path_with_time(&route_path_for_slugs(&slugs), current_time)
}

/// Build the route obtained by removing `slug_to_remove` from the currently-selected cities,
/// preserving the `current_time` query parameter when provided.
/// Returns `"/"` when removing the last city.
pub(crate) fn remove_city_from_route(
    selected: &[City],
    slug_to_remove: &str,
    current_time: Option<String>,
) -> String {
    let slugs: Vec<String> = selected
        .iter()
        .filter(|c| c.slug != slug_to_remove)
        .map(|c| c.slug.clone())
        .collect();
    let path = route_path_for_slugs(&slugs);
    // Don't forward current_time when navigating all the way back to the home page.
    if path == "/" {
        path
    } else {
        route_path_with_time(&path, current_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn london() -> City {
        CITIES.iter().find(|c| c.name == "London").unwrap().clone()
    }

    fn paris() -> City {
        CITIES.iter().find(|c| c.name == "Paris").unwrap().clone()
    }

    fn tokyo() -> City {
        CITIES.iter().find(|c| c.name == "Tokyo").unwrap().clone()
    }

    // --- route_path_for_slugs ---

    #[test]
    fn test_route_path_for_no_slugs_returns_home() {
        assert_eq!(route_path_for_slugs(&[]), "/");
    }

    #[test]
    fn test_route_path_for_one_slug() {
        let slug = london().slug;
        assert_eq!(
            route_path_for_slugs(&[slug.clone()]),
            format!("/compare/{slug}")
        );
    }

    #[test]
    fn test_route_path_for_multiple_slugs() {
        let (l, p, t) = (london().slug, paris().slug, tokyo().slug);
        let slugs = vec![l.clone(), p.clone(), t.clone()];
        assert_eq!(
            route_path_for_slugs(&slugs),
            format!("/compare/{l}/{p}/{t}")
        );
    }

    // --- route_path_with_time ---

    #[test]
    fn test_route_path_with_time_appends_query_when_present() {
        assert_eq!(
            route_path_with_time("/compare/london", Some("12345".to_string())),
            "/compare/london?current_time=12345"
        );
    }

    #[test]
    fn test_route_path_with_time_is_unchanged_when_absent() {
        assert_eq!(
            route_path_with_time("/compare/london", None),
            "/compare/london"
        );
    }

    #[test]
    fn test_route_path_with_time_is_unchanged_when_empty_string() {
        assert_eq!(
            route_path_with_time("/compare/london", Some(String::new())),
            "/compare/london"
        );
    }

    // --- add_city_to_route ---

    #[test]
    fn test_add_city_to_route_from_empty_selection() {
        let slug = london().slug;
        assert_eq!(
            add_city_to_route(&[], &slug, None),
            format!("/compare/{slug}")
        );
    }

    #[test]
    fn test_add_city_to_route_appends_after_existing_city() {
        let (london, paris) = (london(), paris());
        assert_eq!(
            add_city_to_route(&[london.clone()], &paris.slug, None),
            format!("/compare/{}/{}", london.slug, paris.slug)
        );
    }

    #[test]
    fn test_add_city_to_route_preserves_current_time() {
        let (london, paris) = (london(), paris());
        assert_eq!(
            add_city_to_route(&[london.clone()], &paris.slug, Some("99999".to_string())),
            format!("/compare/{}/{}", london.slug, paris.slug) + "?current_time=99999"
        );
    }

    #[test]
    fn test_add_city_to_route_preserves_order() {
        let (london, paris, tokyo) = (london(), paris(), tokyo());
        assert_eq!(
            add_city_to_route(&[london.clone(), paris.clone()], &tokyo.slug, None),
            format!("/compare/{}/{}/{}", london.slug, paris.slug, tokyo.slug)
        );
    }

    // --- remove_city_from_route ---

    #[test]
    fn test_remove_last_city_returns_home_without_time() {
        let london = london();
        assert_eq!(
            remove_city_from_route(&[london.clone()], &london.slug, Some("12345".to_string())),
            "/"
        );
    }

    #[test]
    fn test_remove_city_leaves_remaining_cities() {
        let (london, paris) = (london(), paris());
        assert_eq!(
            remove_city_from_route(&[london.clone(), paris.clone()], &paris.slug, None),
            format!("/compare/{}", london.slug)
        );
    }

    #[test]
    fn test_remove_city_preserves_current_time() {
        let (london, paris) = (london(), paris());
        assert_eq!(
            remove_city_from_route(
                &[london.clone(), paris.clone()],
                &paris.slug,
                Some("99999".to_string()),
            ),
            format!("/compare/{}", london.slug) + "?current_time=99999"
        );
    }

    #[test]
    fn test_remove_first_city_leaves_rest() {
        let (london, paris, tokyo) = (london(), paris(), tokyo());
        assert_eq!(
            remove_city_from_route(
                &[london.clone(), paris.clone(), tokyo.clone()],
                &london.slug,
                None,
            ),
            format!("/compare/{}/{}", paris.slug, tokyo.slug)
        );
    }

    #[test]
    fn test_remove_city_not_in_selection_is_unchanged() {
        let (london, tokyo) = (london(), tokyo());
        assert_eq!(
            remove_city_from_route(&[london.clone()], &tokyo.slug, None),
            format!("/compare/{}", london.slug)
        );
    }
}
