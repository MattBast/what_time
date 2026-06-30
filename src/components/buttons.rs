use chrono::Utc;
use icondata::{BiPlusRegular, BiTimeRegular};
use leptos::prelude::*;
use leptos_icons::Icon;

pub const DRAWER_SWITCH_ID: &str = "drawer-switch";

const BTN_CLASS: &str = "btn btn-neutral";

/// Value written to the `current_time` URL query when the user clicks "Now".
pub(crate) fn current_time_query_value() -> Option<i64> {
    Some(Utc::now().timestamp())
}

/// Drawer checkbox id targeted by `AddTimezoneButton` (must match `TimezoneDrawer` in home).
pub(crate) fn add_timezone_drawer_target() -> &'static str {
    DRAWER_SWITCH_ID
}

#[component]
pub fn Button(
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] aria_label: Option<&'static str>,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static>>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            type="button"
            id=id
            aria-label=aria_label
            class=format!("{BTN_CLASS} gap-2")
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        >
            {children()}
        </button>
    }
}

/// Outline button that toggles the timezone drawer (DaisyUI `drawer-button` + `for` target).
#[component]
pub fn DrawerButton(
    #[prop(into)] drawer_for: String,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] aria_label: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    view! {
        <label
            for=drawer_for
            id=id
            aria-label=aria_label
            class=format!("{BTN_CLASS} drawer-button gap-2")
            role="button"
        >
            {children()}
        </label>
    }
}

#[component]
pub fn NowButton(set_time_query: SignalSetter<Option<i64>>) -> impl IntoView {
    view! {
        <Button
            id="now_button"
            aria_label="Set time to now"
            on_click=Box::new(move || set_time_query.set(current_time_query_value()))
        >
            <Icon icon=BiTimeRegular />
            "Now"
        </Button>
    }
}

#[component]
pub fn AddTimezoneButton() -> impl IntoView {
    view! {
        <DrawerButton
            drawer_for=add_timezone_drawer_target().to_string()
            id="add_timezone_button"
            aria_label="Add City"
        >
            <Icon icon=BiPlusRegular />
            "Add City"
        </DrawerButton>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_current_time_query_value_is_within_current_second() {
        let before = Utc::now().timestamp();
        let value = current_time_query_value().expect("now should produce a timestamp");
        let after = Utc::now().timestamp();

        assert!(
            (before..=after).contains(&value),
            "timestamp {value} should be between {before} and {after}"
        );
    }

    #[test]
    fn test_add_timezone_drawer_target_matches_drawer_switch_id() {
        assert_eq!(add_timezone_drawer_target(), DRAWER_SWITCH_ID);
        assert_eq!(add_timezone_drawer_target(), "drawer-switch");
    }
}

use crate::timezone::City;

#[component]
pub fn ViewToggle(
    is_compare: bool,
    cities: Memo<Vec<City>>,
    time_query: Memo<Option<i64>>,
    cities_query: Memo<Option<String>>,
) -> impl IntoView {
    let query_string = move || {
        let mut parts = Vec::new();
        if let Some(t) = time_query.get() {
            parts.push(format!("current_time={t}"));
        }
        if let Some(z) = cities_query.get() {
            parts.push(format!("zone={z}"));
        }
        if parts.is_empty() {
            String::new()
        } else {
            format!("?{}", parts.join("&"))
        }
    };

    let compare_href = move || {
        let slugs: Vec<String> = cities.get().iter().map(|c| c.slug.clone()).collect();
        if slugs.is_empty() {
            format!("/{}", query_string())
        } else {
            format!("/compare/{}{}", slugs.join("/"), query_string())
        }
    };

    let overlap_href = move || {
        let slugs: Vec<String> = cities.get().iter().map(|c| c.slug.clone()).collect();
        if slugs.is_empty() {
            format!("/overlap{}", query_string())
        } else {
            format!("/overlap/{}{}", slugs.join("/"), query_string())
        }
    };

    view! {
        <div class="flex justify-center w-full mb-6">
            <div class="bg-base-200/60 backdrop-blur-md p-1 rounded-xl flex gap-1 border border-base-300">
                <a
                    href=compare_href
                    class=move || {
                        let active = is_compare;
                        let base = "px-4 py-1.5 rounded-lg text-sm font-medium transition-all duration-200 select-none";
                        if active {
                            format!("{base} bg-neutral text-neutral-content shadow-sm")
                        } else {
                            format!("{base} text-base-content/70 hover:text-base-content hover:bg-base-300/50")
                        }
                    }
                >
                    "Compare"
                </a>
                <a
                    href=overlap_href
                    class=move || {
                        let active = !is_compare;
                        let base = "px-4 py-1.5 rounded-lg text-sm font-medium transition-all duration-200 select-none";
                        if active {
                            format!("{base} bg-neutral text-neutral-content shadow-sm")
                        } else {
                            format!("{base} text-base-content/70 hover:text-base-content hover:bg-base-300/50")
                        }
                    }
                >
                    "Overlap"
                </a>
            </div>
        </div>
    }
}

