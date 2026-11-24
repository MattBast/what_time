use crate::components::{BackgroundBlur, TimezoneCard};
use crate::timezone::{sort_cities, City};
use crate::url_parse::url_query_to_cities;
use leptos::prelude::*;

#[component]
pub fn Compare(
    cities_query: Memo<Option<String>>,
    time_query: ReadSignal<Option<i64>>,
) -> impl IntoView {
    let (get_cities, set_cities) = signal(Vec::new());

    // Listen for the `zone` url query to change and when it does, re-render the cities.
    Effect::new(move || {
        // Trigger these actions when the url "zone" query changes.
        let query = cities_query.get().unwrap_or_default();
        set_cities.set(sorted_cities_from_query(query));
    });

    view! {
        <BackgroundBlur>
            <div class="flex justify-center w-full overflow-x-auto px-2 pt-4">
                <div class="flex flex-wrap justify-center gap-4">
                    <For
                        each=move || get_cities.get()
                        key=|city| city.slug.clone()
                        children=move |city| {
                            let c = city.clone();
                            view! {
                                <TimezoneCard
                                    city=c
                                    time_query
                                />
                            }
                        }
                    />
                </div>
            </div>
        </BackgroundBlur>
    }
}

/// Parse the URL query and return cities sorted west to east.
pub(crate) fn sorted_cities_from_query(query: String) -> Vec<City> {
    let mut cities = url_query_to_cities(query);
    sort_cities(&mut cities);
    cities
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{
        timezone_card_header, timezone_card_local_date, timezone_card_local_time,
    };
    use chrono_tz::Tz;

    const SAMPLE_TIMESTAMP: i64 = 1765987708;

    #[test]
    fn test_sorted_cities_from_query_with_one_city() {
        let cities = sorted_cities_from_query("london".to_string());
        assert_eq!(cities.len(), 1);
        assert_eq!(cities[0].name, "London");
    }

    #[test]
    fn test_sorted_cities_from_query_with_two_cities() {
        let cities = sorted_cities_from_query("kolkata,shanghai".to_string());
        assert_eq!(cities.len(), 2);
        assert_eq!(cities[0].name, "Kolkata");
        assert_eq!(cities[1].name, "Shanghai");
    }

    #[test]
    fn test_sorted_cities_from_query_empty_string() {
        let cities = sorted_cities_from_query(String::new());
        assert!(cities.is_empty());
    }

    #[test]
    fn test_sorted_cities_from_query_ignores_invalid_segments() {
        let cities = sorted_cities_from_query("Not_A_City,london,AlsoBad".to_string());
        assert_eq!(cities.len(), 1);
        assert_eq!(cities[0].name, "London");
    }

    #[test]
    fn test_sorted_cities_from_query_sorts_west_to_east() {
        let cities = sorted_cities_from_query("paris,new-york-city,london".to_string());
        assert_eq!(cities.len(), 3);
        assert_eq!(cities[0].name, "New York City");
        assert_eq!(cities[1].name, "London");
        assert_eq!(cities[2].name, "Paris");
    }

    #[test]
    fn test_each_sorted_city_has_expected_card_content() {
        let timestamp = Some(SAMPLE_TIMESTAMP);
        let cities = sorted_cities_from_query("paris,london".to_string());

        assert_eq!(cities.len(), 2);
        assert_eq!(cities[0].name, "London");
        assert_eq!(cities[1].name, "Paris");

        assert_eq!(
            timezone_card_header(timestamp, &cities[0]),
            "🇬🇧 London (GMT)"
        );
        assert_eq!(
            timezone_card_local_time(timestamp, Tz::Europe__London),
            "16:08"
        );
        assert_eq!(
            timezone_card_local_date(timestamp, Tz::Europe__London),
            "2025-12-17"
        );

        assert_eq!(
            timezone_card_header(timestamp, &cities[1]),
            "🇫🇷 Paris (CET)"
        );
        assert_eq!(
            timezone_card_local_time(timestamp, Tz::Europe__Paris),
            "17:08"
        );
        assert_eq!(
            timezone_card_local_date(timestamp, Tz::Europe__Paris),
            "2025-12-17"
        );
    }

    #[test]
    fn test_abidjan_card_content_after_sorting_from_url() {
        let timestamp = Some(1766076397);
        let cities = sorted_cities_from_query("abidjan".to_string());

        assert_eq!(cities.len(), 1);
        assert_eq!(cities[0].name, "Abidjan");
        assert_eq!(
            timezone_card_header(timestamp, &cities[0]),
            "🇨🇮 Abidjan (GMT)"
        );
        assert_eq!(
            timezone_card_local_time(timestamp, Tz::Africa__Abidjan),
            "16:46"
        );
        assert_eq!(
            timezone_card_local_date(timestamp, Tz::Africa__Abidjan),
            "2025-12-18"
        );
    }
}
