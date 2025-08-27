use crate::components::{DarkModeToggle, Logo, Nav};
use crate::url_parse::url_query_to_time_increments;
use crate::ZONE;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::query_signal;

#[component]
pub fn Wrapper() -> impl IntoView {
    // Watch the url query to decide whether to shrink the logo or not.
    let (url_query, _set_url_query) = query_signal::<String>(ZONE);

    view! {
        <div class="font-sans flex flex-col justify-center min-h-screen px-4 sm:px-8 lg:px-12">

            <div class="fixed top-5 left-0 right-0">
                <div class="mx-auto w-full max-w-7xl">
                    <div class="relative px-4 sm:px-8">
                        <div class="mx-auto max-w-2xl lg:max-w-5xl">
                            <div class="relative flex gap-4">
                                <div class="flex flex-1">
                                    // {!url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty().then(|| view! {
                                        <Logo small=true/>
                                    // })}
                                </div>
                                <div class="flex flex-1 justify-end md:justify-center">
                                    <Nav/>
                                </div>
                                <div class="flex justify-end md:flex-1">
                                    <DarkModeToggle/>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex justify-start w-full transition-all duration-700 ease-in-out delay-800">
                {move || url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty().then(|| view! {
                    <Logo/>
                })}
            </div>

            <Outlet/>

        </div>
    }
}
