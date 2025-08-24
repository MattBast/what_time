#![allow(non_snake_case)]

// modules
pub mod app;
pub mod buttons;
pub mod carousel;
pub mod timecard;
pub mod timezone;
pub mod timezone_select;
pub mod url_parse;

// crates
use crate::app::Home;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::{path, StaticSegment};

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
        <Stylesheet id="leptos" href="/style/output.css"/>
        // <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home/>
            </Routes>
        </Router>
    }
}
#[component]
pub fn TheRouter() -> impl IntoView {
    view! {
        <Routes fallback=|| "404">
            <Route path=path!("/") view=Home/>
        </Routes>

        // <Router>
        //     <Title text="What Time"/>
        //     <main>
        //         <Routes fallback=|| "Page not found.">
        //             <ParentRoute path=path!("/") view=Home>
        //                 <Route path=path!("") view=|| view! {
        //                     ""
        //                 }/>
        //             </ParentRoute>
        //         </Routes>
        //     </main>
        // </Router>
    }
}
