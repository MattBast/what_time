use crate::carousel::Carousel;
use crate::url_parse::url_query_to_time_increments;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::hooks::query_signal;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <link rel="stylesheet" id="leptos" href="/pkg/leptos_tailwind.css"/>
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
                <MetaTags/>
            </head>
            <body class="bg-zinc-50 dark:bg-black">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Title text="What Time"/>
            <main>
                <Routes fallback=|| "Page not found.">
                    <ParentRoute path=path!("/") view=Home>
                        <Route path=path!("") view=|| view! {
                            ""
                        }/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="font-sans flex flex-col justify-center min-h-screen px-4 sm:px-8 lg:px-12">

            <Logo/>

            <Introtext/>

            <Carousel/>

            // This does nothing until the routes change to put a component in it.
            <Outlet/>

        </div>
    }
}

#[component]
fn Introtext() -> impl IntoView {
    // Watch the url query to decide whether to show the text or not.
    let (url_query, _set_url_query) = query_signal::<String>("zone");

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
    let (url_query, _set_url_query) = query_signal::<String>("zone");

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
