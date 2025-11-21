use crate::url_parse::url_query_to_time_increments;
use crate::ZONE;
use leptos::prelude::*;
use leptos_router::hooks::query_signal;

#[component]
pub fn Introtext(children: Children) -> impl IntoView {
    // Watch the url query to decide whether to show the text or not.
    let (url_query, _set_url_query) = query_signal::<String>(ZONE);

    view! {
        <div
            class="
                transition-all duration-700 ease-in-out origin-top-left
                bg-radial from-white dark:from-black from-20% py-24
            "
            // Hide the page title and description when there are timezones on the page.
            class=(["scale-0", "opacity-0", "max-h-0", "translate-x-4", "-translate-y-2"], move || !url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty())
            class=(["scale-100", "opacity-100", "max-h-96", "translate-x-0", "translate-y-0"], move || url_query_to_time_increments(url_query.get().unwrap_or_default()).is_empty())
        >
            <div class="w-full max-w-7xl flex justify-center">
                <div class="relative">
                    {children()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn IntroTitle(children: Children) -> impl IntoView {
    view! {
        <h1 class="text-4xl font-bold tracking-tight text-zinc-800 leading-none sm:text-5xl dark:text-zinc-100 max-w-2xl">
            {children()}
        </h1>
    }
}

#[component]
pub fn IntroSubtitle(children: Children) -> impl IntoView {
    view! {
        <p class="text-base text-zinc-600 leading-normal dark:text-zinc-400 max-w-2xl mt-8">
            {children()}
        </p>
    }
}

#[component]
pub fn InlineLi(children: Children) -> impl IntoView {
    view! {
        <li class="block px-4 sm:inline-block mb-2">
            {children()}
        </li>
    }
}
