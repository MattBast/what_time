use leptos::prelude::*;

/// The outline of a circle with a "glow" sitting behind it. This is provided
/// as an SVG and inverts its colours when the browser is in dark mode.
#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <div class="w-10 h-10">
            <svg
                // Set the stroke (outline) colour of the circles from here. Inverts the
                // colours when the browser is in dark mode.
                class="stroke-current"
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
    }
}
