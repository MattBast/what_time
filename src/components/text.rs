use leptos::prelude::*;

#[component]
pub fn Introtext(children: Children) -> impl IntoView {
    view! {
        <div class="w-full flex justify-center">
            <div class="relative">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn IntroTitle(children: Children) -> impl IntoView {
    view! {
        <h1 class="text-5xl text-center font-bold tracking-tight text-base-content leading-none sm:text-6xl">
            {children()}
        </h1>
    }
}

#[component]
pub fn IntroSubtitle(children: Children) -> impl IntoView {
    view! {
        <p class="text-base text-center text-base-content leading-normal mt-8">
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
