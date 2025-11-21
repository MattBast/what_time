use leptos::prelude::*;

#[component]
pub fn Introtext(children: Children) -> impl IntoView {
    view! {
        <div class="w-full max-w-7xl flex justify-center">
            <div class="relative">
                {children()}
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
