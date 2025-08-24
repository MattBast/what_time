use crate::pages::Carousel;
use crate::url_parse::url_query_to_time_increments;
use crate::ZONE;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::hooks::query_signal;
use leptos_use::{use_color_mode, ColorMode, UseColorModeReturn};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="What Time"/>
        <main>
            <div class="font-sans flex flex-col justify-center min-h-screen px-4 sm:px-8 lg:px-12">

                <DarkModeToggle/>

                <Logo/>

                <Introtext/>

                <Carousel/>

                // This does nothing until the routes change to put a component in it.
                // <Outlet/>

            </div>
        </main>
    }
}

#[component]
fn Introtext() -> impl IntoView {
    // Watch the url query to decide whether to show the text or not.
    let (url_query, _set_url_query) = query_signal::<String>(ZONE);

    view! {
        <div
            class="transition-all duration-700 ease-in-out origin-top-left overflow-hidden"
            // Hide the page title and description when there are timezones on the page.
            class=(["scale-0", "opacity-0", "max-h-0", "translate-x-4", "-translate-y-2"], move || !url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty())
            class=(["scale-100", "opacity-100", "max-h-96", "mt-9", "translate-x-0", "translate-y-0"], move || url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty())
        >
            <div class="w-full max-w-7xl">
                <div class="relative">
                    <h1 class="text-4xl font-bold tracking-tight text-zinc-800 sm:text-5xl dark:text-zinc-100">
                        "Time Carousel"
                    </h1>

                    <p class="mt-6 text-base text-zinc-600 dark:text-zinc-400">
                        "Want to know the difference between two or more timezones? Add some timezones below to compare them hour by hour."
                    </p>
                </div>
            </div>
        </div>
    }
}

/// The outline of a circle with a "glow" sitting behind it. This is provided
/// as an SVG and inverts its colours when the browser is in dark mode.
#[component]
fn Logo() -> impl IntoView {
    // Watch the url query to decide whether to shrink the logo or not.
    let (url_query, _set_url_query) = query_signal::<String>(ZONE);

    view! {
        <div
            class="flex w-full transition-all duration-700 ease-in-out delay-800"
            class=("justify-center", move || !url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty())
            class=("justify-start", move || url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty())
        >
            <div
                class="transition-all duration-700 ease-in-out origin-center delay-800"
                class=(["w-10", "h-10"], move || !url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty())
                class=(["w-24", "h-24"], move || url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty())
            >
                <svg
                    // Set the stroke (outline) colour of the circles from here. Inverts the
                    // colours when the browser is in dark mode.
                    class="stroke-zinc-800 dark:stroke-zinc-100"
                    xmlns="http://www.w3.org/2000/svg"
                    version="1.1"
                    xmlns:xlink="http://www.w3.org/1999/xlink"
                    xmlns:svgjs="http://svgjs.dev/svgjs"
                    viewBox="200 100 600 600"
                    opacity="1"
                >
                    <defs>
                        <filter id="nnneon-filter" x="-100%" y="-100%" width="400%" height="400%" filterUnits="objectBoundingBox" primitiveUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
                            <feGaussianBlur stdDeviation="17 8" x="0%" y="0%" width="100%" height="100%" in="SourceGraphic" edgeMode="none" result="blur">
                            </feGaussianBlur>
                        </filter>
                        <filter id="nnneon-filter2" x="-100%" y="-100%" width="400%" height="400%" filterUnits="objectBoundingBox" primitiveUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
                            <feGaussianBlur stdDeviation="10 17" x="0%" y="0%" width="100%" height="100%" in="SourceGraphic" edgeMode="none" result="blur">
                            </feGaussianBlur>
                        </filter>
                    </defs>
                    <g
                        stroke-width="16"
                        // stroke="#27272a"
                        fill="none"
                    >
                        <circle r="150" cx="400" cy="400" filter="url(#nnneon-filter)">
                        </circle>
                        <circle r="150" cx="412" cy="400" filter="url(#nnneon-filter2)" opacity="0.25">
                        </circle>
                        <circle r="150" cx="388" cy="400" filter="url(#nnneon-filter2)" opacity="0.25">
                        </circle>
                        <circle r="150" cx="400" cy="400">
                        </circle>
                    </g>
                </svg>
            </div>
        </div>
    }
}

/// Allow the user to toggle between light and dark mode
#[component]
fn DarkModeToggle() -> impl IntoView {
    let UseColorModeReturn {
        mode, // Signal<ColorMode::dark | ColorMode::light>
        set_mode,
        ..
    } = use_color_mode();

    log!("Hello toggle");

    view! {
        <div class="fixed top-5 right-5">
            <button
                type="button"
                aria-label="Switch to dark theme"
                class="group cursor-pointer rounded-full bg-white/90 px-3 py-2 shadow-lg ring-1 shadow-zinc-800/5 ring-zinc-900/5 backdrop-blur-sm transition dark:bg-zinc-800/90 dark:ring-white/10 dark:hover:ring-white/20"
                on:click=move |_| {
                    let new_mode = match mode.get_untracked() {
                        ColorMode::Dark => ColorMode::Light,
                        ColorMode::Light => ColorMode::Dark,
                        _ => ColorMode::Dark,
                    };

                    log!("Setting mode to: {:?}", new_mode);
                    set_mode.set(new_mode);
                }
            >
                <LightModeSvg/>
                <DarkModeSvg/>
            </button>
        </div>
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
            class="h-6 w-6 fill-zinc-100 stroke-zinc-500 transition group-hover:fill-zinc-200 group-hover:stroke-zinc-700 dark:hidden [@media(prefers-color-scheme:dark)]:fill-teal-50 [@media(prefers-color-scheme:dark)]:stroke-teal-500 [@media(prefers-color-scheme:dark)]:group-hover:fill-teal-50 [@media(prefers-color-scheme:dark)]:group-hover:stroke-teal-600"
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
            class="hidden h-6 w-6 fill-zinc-700 stroke-zinc-500 transition not-[@media_(prefers-color-scheme:dark)]:fill-teal-400/10 not-[@media_(prefers-color-scheme:dark)]:stroke-teal-500 dark:block [@media(prefers-color-scheme:dark)]:group-hover:stroke-zinc-400"
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
