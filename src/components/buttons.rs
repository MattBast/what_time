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
