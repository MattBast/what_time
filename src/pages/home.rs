use crate::components::{
    AddTimezoneButton, AddTimezoneButtonFab, BackgroundBlur, InlineLi, IntroSubtitle, IntroTitle,
    Introtext, TimezoneDrawerContent, DRAWER_SWITCH_ID,
};
use crate::pages::{Compare, Hours};
use crate::url_parse::{find_city_by_slug, route_cities_slugs, url_query_to_cities};
use crate::{CITIES, DRAWER};
use chrono::{Timelike, Utc};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::{query_signal, use_params_map};
use leptos_use_interval_fn::use_interval_fn;

#[component]
pub fn Home() -> impl IntoView {
    let params = use_params_map();

    // Discover which cities are in the URL route
    let timezones_query = Memo::new(move |_| {
        Some(params.with(|params| route_cities_slugs(params.get(CITIES), None)))
    });

    // Start tracking time and check it every minute.
    let (now, set_now) = signal(Some(Utc::now().timestamp()));

    // Initially tick once a second to ensure we change when the
    // operating system minute changes.
    let (interval, set_interval) = signal(1000);

    let _ = use_interval_fn(
        move || {
            let now_time = Utc::now();
            set_now.set(Some(now_time.timestamp()));

            // Once we reach the first minute, tick once a minute.
            if now_time.second() == 0 {
                set_interval.set(60_000);
            }
        },
        interval,
    );

    let page_title = Memo::new(move |_| {
        params.with(|params| {
            let cities_param = params.get(CITIES).unwrap_or_default();
            let slugs: Vec<&str> = cities_param.split('/').filter(|s| !s.is_empty()).collect();
            route_cities_page_title(&slugs)
        })
    });

    view! {
        <Title text=move || page_title.get()/>
        <HomeContent timezones_query time_query=now/>
    }
}

#[component]
pub fn HomeContent(
    timezones_query: Memo<Option<String>>,
    time_query: ReadSignal<Option<i64>>,
) -> impl IntoView {
    let (drawer_query, set_drawer_query) = query_signal::<bool>(DRAWER);

    view! {
        <TimezoneDrawer timezones_query set_drawer_query>
            <Show
                when=move || should_show_compare(timezones_query.get())
                fallback=|| view! {
                    <BackgroundBlur>
                        <div class="pt-24">
                            <WelcomeText/>
                        </div>
                    </BackgroundBlur>

                    <BackgroundBlur>
                        <div class="flex flex-wrap justify-center gap-3 py-8">
                            // Open drawer to add a new city.
                            <AddTimezoneButton/>
                        </div>
                    </BackgroundBlur>
                }
            >
                <Compare cities_query=timezones_query time_query/>
                <Hours cities_query=timezones_query/>
                <AddTimezoneButtonFab drawer_query/>
            </Show>
        </TimezoneDrawer>
    }
}

#[component]
pub fn WelcomeText() -> impl IntoView {
    view! {
        <Introtext>
            <IntroTitle>"Compare timezones, fast"</IntroTitle>
            <IntroSubtitle>
                <ul id="sub-headings" class="list-disc">
                    <InlineLi>"🙂 Pick your city."</InlineLi>
                    <InlineLi>"😀 Compare with another."</InlineLi>
                    <InlineLi>"😁 Keep adding more."</InlineLi>
                </ul>
            </IntroSubtitle>
        </Introtext>
    }
}

fn route_cities_page_title(slugs: &[&str]) -> String {
    if slugs.is_empty() {
        "What Time".to_string()
    } else {
        let titles: Vec<String> = slugs
            .iter()
            .map(|slug| format!("{} Time", city_slug_to_title(slug)))
            .collect();
        format!("{} | What Time", titles.join(" vs "))
    }
}

fn city_slug_to_title(slug: &str) -> String {
    if let Some(city) = find_city_by_slug(slug) {
        city.name.clone()
    } else {
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
}

/// Wraps page content and the timezone drawer. A control with class `drawer-button` and
/// `for` pointing at the drawer checkbox opens the drawer.
#[component]
fn TimezoneDrawer(
    timezones_query: Memo<Option<String>>,
    set_drawer_query: SignalSetter<Option<bool>>,
    children: Children,
) -> impl IntoView {
    // Used to focus the search input after the drawer opens.
    let search_input_ref = NodeRef::<leptos::html::Input>::new();

    view! {
        <div class="drawer drawer-end">
          <input
            id=DRAWER_SWITCH_ID
            type="checkbox"
            class="drawer-toggle"
            on:change=move |ev| {
                if event_target_checked(&ev) {
                    // Set URL query to show other components that the drawer is open.
                    set_drawer_query.set(Some(true));

                    // Focus on the drawer search bar.
                    set_timeout(move || {
                        if let Some(input) = search_input_ref.get() {
                            let _ = input.focus();
                        }
                    }, std::time::Duration::from_millis(150));
                } else {
                    // Set URL query to show other components that the drawer is closed.
                    set_drawer_query.set(Some(false));
                }
            }
          />
          <div class="drawer-content">
              {children()}
          </div>
          <div class="drawer-side z-[100]">
            <label for=DRAWER_SWITCH_ID aria-label="close sidebar" class="drawer-overlay"></label>
            <TimezoneDrawerContent
                timezones_query
                search_input_ref
            />
          </div>
        </div>
    }
}

/// Whether the compare view should render instead of the welcome screen.
pub(crate) fn should_show_compare(cities_query: Option<String>) -> bool {
    !url_query_to_cities(cities_query.unwrap_or_default()).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_show_compare_when_zone_query_is_empty() {
        assert!(!should_show_compare(None));
        assert!(!should_show_compare(String::new().into()));
    }

    #[test]
    fn test_should_show_compare_when_zone_query_has_city() {
        assert!(should_show_compare(Some("london".into())));
        assert!(should_show_compare(Some("Europe__London".into())));
        assert!(should_show_compare(Some("london,paris".into())));
    }

    #[test]
    fn test_should_show_compare_when_zone_query_only_has_invalid_segments() {
        assert!(!should_show_compare(Some("Not_A_City,Also_Invalid".into())));
    }

    #[test]
    fn test_should_show_compare_when_zone_query_has_valid_and_invalid_segments() {
        assert!(should_show_compare(Some("Bad_City,london".into())));
    }

    #[test]
    fn test_route_cities_page_title_from_slugs() {
        assert_eq!(
            route_cities_page_title(&["london"]),
            "London Time | What Time"
        );
        assert_eq!(
            route_cities_page_title(&["london", "new-york"]),
            "London Time vs New York City Time | What Time"
        );
        assert_eq!(
            route_cities_page_title(&["london", "new-york", "paris"]),
            "London Time vs New York City Time vs Paris Time | What Time"
        );
    }
}
