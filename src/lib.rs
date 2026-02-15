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
/// Defines the name of the "future_increments" url parameter
pub const FUTURE_INCREMENTS: &str = "future_increments";
/// Defines the name of the "past_increments" url parameter
pub const PAST_INCREMENTS: &str = "past_increments";
/// Defines the name of the "current_time" url parameter
pub const CURRENT_TIME: &str = "current_time";

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="What Time - Compare timezones, fast"/>
        <Router>
            <main>
                <Routes fallback=|| "Page not found.">
                    <ParentRoute path=path!("") view=Wrapper>
                        <Route path=path!("/") view=Home/>
                        <Route path=path!("/what_time") view=Home/> // <-- Needed for GitHub pages which uses the base path `/what_time`
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}
