use crate::url_parse::url_query_to_time_increments;
use crate::ZONE;
use leptos::prelude::*;
use leptos_router::hooks::query_signal;

/// The outline of a circle with a "glow" sitting behind it. This is provided
/// as an SVG and inverts its colours when the browser is in dark mode.
#[component]
pub fn Logo() -> impl IntoView {
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
