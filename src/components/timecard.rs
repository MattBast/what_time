use leptos::prelude::*;

#[component]
pub fn Timecard(children: Children) -> impl IntoView {
    view! {
        <div
            class="rounded-2xl p-8 relative snap-center z-1 transition"
        >
            <div class="p-6 flex flex-col items-center">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn TimecardHeader(children: Children) -> impl IntoView {
    view! {
        <h5 class="text-2xl my-2 text-zinc-600 dark:text-zinc-400">
            {children()}
        </h5>
    }
}

#[component]
pub fn TimecardTime(children: Children) -> impl IntoView {
    view! {
        <h1 class="text-5xl sm:text-6xl font-semibold text-zinc-900 dark:text-zinc-100">
            {children()}
        </h1>
    }
}

#[component]
pub fn TimecardDate(children: Children) -> impl IntoView {
    view! {
        <p class="text-xs my-2 text-zinc-600 dark:text-zinc-400">
            {children()}
        </p>
    }
}
