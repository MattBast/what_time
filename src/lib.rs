// modules
pub mod components;
pub mod pages;
pub mod timezone;
pub mod url_parse;

// crates
use crate::pages::*;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::path;

/// Defines the name of the "zone" url parameter
pub const ZONE: &str = "zone";
/// Defines the name of the "current_time" url parameter
pub const CURRENT_TIME: &str = "current_time";

/// Document title shown in the browser tab and search results.
const APP_TITLE: &str = "What Time - Compare timezones, fast";

/// Short summary for search engines (`<meta name="description">`).
const META_DESCRIPTION: &str =
    "Compare times across timezones. Pick zones, set a shared moment, and share the link.";

/// Production URL for Open Graph link previews (`og:url`).
const SITE_URL: &str = "https://mattbast.github.io/what_time/";

/// Absolute URL for social preview images (`og:image`, `twitter:image`).
const OG_IMAGE_URL: &str = "https://mattbast.github.io/what_time/og-image.png";

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text=APP_TITLE/>
        <Meta name="description" content=META_DESCRIPTION/>

        <Meta property="og:title" content=APP_TITLE/>
        <Meta property="og:description" content=META_DESCRIPTION/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:url" content=SITE_URL/>
        <Meta property="og:image" content=OG_IMAGE_URL/>
        <Meta property="og:image:width" content="1200"/>
        <Meta property="og:image:height" content="630"/>

        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=APP_TITLE/>
        <Meta name="twitter:description" content=META_DESCRIPTION/>
        <Meta name="twitter:image" content=OG_IMAGE_URL/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.">
                    <ParentRoute path=path!("") view=Wrapper>
                        <Route path=path!("/") view=Home/>
                        <Route path=path!("/what_time") view=Home/> // <-- Needed for GitHub pages which uses the base path `/what_time`
                        <Route path=path!("/compare/:city1/:city2") view=CompareCityPair/>
                        <Route path=path!("/what_time/compare/:city1/:city2") view=CompareCityPair/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}
