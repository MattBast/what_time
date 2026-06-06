use crate::components::{BackgroundBlur, TimezoneCard};
use crate::timezone::sort_timezones;
use crate::url_parse::{city_pair_url_query, url_query_to_timezones};
use crate::{CURRENT_TIME, ZONE};
use chrono_tz::Tz;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::{query_signal, use_params_map};

const EXTRA_CITIES: &str = "cities";

#[component]
pub fn Compare(
    timezones_query: Memo<Option<String>>,
    time_query: Memo<Option<i64>>,
    set_time_query: SignalSetter<Option<i64>>,
) -> impl IntoView {
    let (get_timezones, set_timezones) = signal(Vec::new());

    // Listen for the `zone` url query to change and when it does, re-render the timezones.
    Effect::new(move || {
        // Trigger these actions when the url "zone" query changes.
        let query = timezones_query.get().unwrap_or_default();
        set_timezones.set(sorted_timezones_from_query(query));
    });

    view! {
        <BackgroundBlur>
            <div class="flex justify-center w-full overflow-x-auto px-2 pt-4">
                <div class="flex flex-wrap justify-center gap-4">
                    <For
                        each=move || get_timezones.get()
                        key=|timezone| *timezone
                        children=move |timezone| {
                            view! {
                                <TimezoneCard
                                    timezone
                                    time_query
                                    set_time_query
                                />
                            }
                        }
                    />
                </div>
            </div>
        </BackgroundBlur>
    }
}

#[component]
pub fn CompareCityPair() -> impl IntoView {
    let params = use_params_map();
    let (extra_cities_query, _) = query_signal::<String>(EXTRA_CITIES);
    let (zone_query, set_zone_query) = query_signal::<String>(ZONE);
    let (time_query, set_time_query) = query_signal::<i64>(CURRENT_TIME);

    let route_timezones_query = Memo::new(move |_| {
        params.with(|params| {
            city_pair_url_query(
                params.get("city1"),
                params.get("city2"),
                extra_cities_query.get(),
            )
        })
    });

    let timezones_query = Memo::new(move |_| {
        let query = [
            route_timezones_query.get(),
            zone_query.get().unwrap_or_default(),
        ]
        .into_iter()
        .filter(|query| !query.is_empty())
        .collect::<Vec<_>>()
        .join(",");

        (!query.is_empty()).then_some(query)
    });

    let page_title = Memo::new(move |_| {
        params.with(|params| {
            let city1 = params.get("city1").unwrap_or_default();
            let city2 = params.get("city2").unwrap_or_default();
            city_pair_page_title(&city1, &city2)
        })
    });

    view! {
        <Title text=move || page_title.get()/>
        <crate::pages::HomeContent
            timezones_query
            set_timezones_query=set_zone_query
            time_query
            set_time_query
        />
    }
}

/// Parse the `zone` URL query and return timezones sorted west to east.
pub(crate) fn sorted_timezones_from_query(query: String) -> Vec<Tz> {
    let mut timezones = url_query_to_timezones(query);
    timezones.dedup();
    sort_timezones(&mut timezones);
    timezones.dedup();
    timezones
}

fn city_pair_page_title(city1: &str, city2: &str) -> String {
    format!(
        "{} Time vs {} Time | What Time",
        city_slug_to_title(city1),
        city_slug_to_title(city2)
    )
}

fn city_slug_to_title(slug: &str) -> String {
    slug.split(['-', '_'])
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{
        timezone_card_header, timezone_card_local_date, timezone_card_local_time,
    };

    const SAMPLE_TIMESTAMP: i64 = 1765987708;

    #[test]
    fn test_sorted_timezones_from_query_with_one_timezone() {
        let timezones = sorted_timezones_from_query("Europe__London".to_string());
        assert_eq!(timezones, vec![Tz::Europe__London]);
    }

    #[test]
    fn test_sorted_timezones_from_query_with_two_timezones() {
        let timezones = sorted_timezones_from_query("Asia__Calcutta,Asia__Shanghai".to_string());
        assert_eq!(timezones, vec![Tz::Asia__Calcutta, Tz::Asia__Shanghai]);
    }

    #[test]
    fn test_sorted_timezones_from_query_empty_string() {
        let timezones = sorted_timezones_from_query(String::new());
        assert!(timezones.is_empty());
    }

    #[test]
    fn test_sorted_timezones_from_query_ignores_invalid_segments() {
        let timezones =
            sorted_timezones_from_query("Not_A_Zone,Europe__London,AlsoBad".to_string());
        assert_eq!(timezones, vec![Tz::Europe__London]);
    }

    #[test]
    fn test_sorted_timezones_from_query_sorts_west_to_east() {
        let timezones = sorted_timezones_from_query(
            "Europe__Paris,America__Atikokan,Europe__London".to_string(),
        );

        assert_eq!(
            timezones,
            vec![Tz::America__Atikokan, Tz::Europe__London, Tz::Europe__Paris,]
        );
    }

    #[test]
    fn test_sorted_timezones_from_query_preserves_all_valid_zones() {
        let timezones = sorted_timezones_from_query(
            "Asia__Tokyo,Europe__London,America__Los_Angeles".to_string(),
        );

        assert_eq!(timezones.len(), 3);
        assert!(timezones.contains(&Tz::Asia__Tokyo));
        assert!(timezones.contains(&Tz::Europe__London));
        assert!(timezones.contains(&Tz::America__Los_Angeles));
        assert!(
            timezones.first() == Some(&Tz::America__Los_Angeles),
            "westernmost zone should be first"
        );
        assert!(
            timezones.last() == Some(&Tz::Asia__Tokyo),
            "easternmost zone should be last"
        );
    }

    #[test]
    fn test_sorted_timezones_from_query_removes_duplicates() {
        let timezones =
            sorted_timezones_from_query("Europe__London,Europe__Paris,Europe__London".to_string());

        assert_eq!(timezones, vec![Tz::Europe__London, Tz::Europe__Paris]);
    }

    #[test]
    fn test_city_pair_page_title_from_slugs() {
        assert_eq!(
            city_pair_page_title("london", "new-york"),
            "London Time vs New York Time | What Time"
        );
    }

    #[test]
    fn test_each_sorted_timezone_has_expected_card_content() {
        let timestamp = Some(SAMPLE_TIMESTAMP);
        let timezones = sorted_timezones_from_query("Europe__Paris,Europe__London".to_string());

        assert_eq!(timezones, vec![Tz::Europe__London, Tz::Europe__Paris]);

        assert_eq!(
            timezone_card_header(timestamp, Tz::Europe__London),
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
            timezone_card_header(timestamp, Tz::Europe__Paris),
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
        let timezones = sorted_timezones_from_query("Africa__Abidjan".to_string());

        assert_eq!(timezones, vec![Tz::Africa__Abidjan]);
        assert_eq!(
            timezone_card_header(timestamp, Tz::Africa__Abidjan),
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
