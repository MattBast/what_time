use icondata::{BiPlusRegular, ChCross, ChPlus, Icon};
use leptos::html::{div, label};
use leptos::prelude::*;
use leptos_icons::Icon;

pub const DRAWER_SWITCH_ID: &str = "drawer-switch";

const BTN_CLASS: &str = "btn btn-neutral";

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

#[component]
pub fn AddTimezoneButtonFab(drawer_query: Memo<Option<bool>>) -> impl IntoView {
    div().attr("class", "fab").child(
        label()
            .attr("for", DRAWER_SWITCH_ID)
            .attr("id", "add_timezone_button")
            .attr("aria-label", "Add City")
            .attr("class", "btn btn-xl btn-circle btn-neutral drawer-button")
            .attr("role", "button")
            .child({
                move || {
                    view! {
                        <Icon icon=get_fab_icon(drawer_query.get()) />
                    }
                }
            }),
    )
}

fn get_fab_icon(drawer_open: Option<bool>) -> Icon {
    if drawer_open.unwrap_or(false) {
        ChCross
    } else {
        ChPlus
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_timezone_drawer_target_matches_drawer_switch_id() {
        assert_eq!(add_timezone_drawer_target(), DRAWER_SWITCH_ID);
        assert_eq!(add_timezone_drawer_target(), "drawer-switch");
    }

    #[test]
    fn returns_plus_when_drawer_is_closed() {
        assert_eq!(get_fab_icon(Some(false)), ChPlus);
    }

    #[test]
    fn returns_cross_when_drawer_is_open() {
        assert_eq!(get_fab_icon(Some(true)), ChCross);
    }

    #[test]
    fn returns_plus_when_drawer_state_is_none() {
        assert_eq!(get_fab_icon(None), ChPlus);
    }
}
