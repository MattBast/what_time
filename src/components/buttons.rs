use icondata::BiPlusRegular;
use leptos::prelude::*;
use leptos_icons::Icon;

pub const DRAWER_SWITCH_ID: &str = "drawer-switch";

const BTN_CLASS: &str = "btn btn-neutral";

#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! {
        <button class=BTN_CLASS>
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
            drawer_for=DRAWER_SWITCH_ID
            id="add_timezone_button"
            aria_label="Add Timezone"
        >
            <Icon icon=BiPlusRegular />
            "Add Timezone"
        </DrawerButton>
    }
}
