use leptos::prelude::*;

/// Allow the user to toggle between light and dark mode
#[component]
pub fn DarkModeToggle() -> impl IntoView {
    view! {
        <label
            aria-label="Switch to dark theme"
            class="swap swap-rotate btn rounded-full"
        >
            // The input is hidden but controls the light-dark toggling
            <input type="checkbox" class="theme-controller" value="dark" />
            <LightModeSvg/>
            <DarkModeSvg/>
        </label>
    }
}

/// A sun icon that only shows then the browser is in Light Mode
#[component]
fn LightModeSvg() -> impl IntoView {
    view! {
        <svg
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
            // swap-off keeps the icon hidden while in light mode
            class="swap-off h-6 w-6 fill-base-300 stroke-current"
        >
            <path
                d="M8 12.25A4.25 4.25 0 0 1 12.25 8v0a4.25 4.25 0 0 1 4.25 4.25v0a4.25 4.25 0 0 1-4.25 4.25v0A4.25 4.25 0 0 1 8 12.25v0Z"
            >
            </path>
            <path
                d="M12.25 3v1.5M21.5 12.25H20M18.791 18.791l-1.06-1.06M18.791 5.709l-1.06 1.06M12.25 20v1.5M4.5 12.25H3M6.77 6.77 5.709 5.709M6.77 17.73l-1.061 1.061"
                fill="none"
            >
            </path>
        </svg>
    }
}

/// A moon icon that only shows then the browser is in Dark Mode
#[component]
fn DarkModeSvg() -> impl IntoView {
    view! {
        <svg
            viewBox="0 0 24 24"
            aria-hidden="true"
            // swap-on keeps the icon hidden while in dark mode
            class="swap-on h-6 w-6 fill-current-content stroke-current"
        >
            <path
                d="M17.25 16.22a6.937 6.937 0 0 1-9.47-9.47 7.451 7.451 0 1 0 9.47 9.47ZM12.75 7C17 7 17 2.75 17 2.75S17 7 21.25 7C17 7 17 11.25 17 11.25S17 7 12.75 7Z"
                stroke-width="1.5"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
            </path>
        </svg>
    }
}
