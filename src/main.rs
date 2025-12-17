#![allow(non_snake_case)]
#![deny(clippy::unwrap_used)]

// modules
pub mod components;
pub mod pages;
pub mod timezone;
pub mod url_parse;

// crates
use crate::pages::*;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::path;

/// Defines the name of the "zone" url parameter
pub const ZONE: &str = "zone";
/// Defines the name of the "future_increments" url parameter
pub const FUTURE_INCREMENTS: &str = "future_increments";
/// Defines the name of the "past_increments" url parameter
pub const PAST_INCREMENTS: &str = "past_increments";
/// Defines the name of the "current_time" url parameter
pub const CURRENT_TIME: &str = "current_time";

fn main() {
    // 🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨
    // Make the logging level configurable
    // 🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨🚨
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(App)
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="What Time - Compare timezones, fast"/>
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Router>
            <main>
                <Routes fallback=|| "Page not found.">
                    <ParentRoute path=path!("") view=Wrapper>
                        <Route path=path!("/") view=Home/>
                        <Route path=path!("/compare") view=Compare/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}
