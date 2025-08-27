use leptos::prelude::*;

#[component]
pub fn Timecard(
    children: Children,
    #[prop(optional)] large: bool,
    #[prop(optional)] ping: bool,
) -> impl IntoView {
    let is_highlighted = RwSignal::new(large);

    view! {
        <div
            class=("scale-100", move || is_highlighted.get())
            class=("scale-75", move || !is_highlighted.get())
            class="rounded-2xl border border-zinc-100 dark:border-zinc-700/40 w-40 content-center relative snap-center z-1 transition"
        >
            // add a pinging dot if this is the current time
            {ping.then(||view! {
                <span class="absolute top-2 right-2 flex size-3">
                    <span class="absolute inline-flex h-full w-full animate-ping rounded-full bg-teal-400 opacity-75"></span>
                    <span class="relative inline-flex size-3 rounded-full bg-teal-500"></span>
                </span>
            })}

            <div class="p-6 w-40 flex-none">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn TimecardHeader(children: Children) -> impl IntoView {
    view! {
        <p class="my-2 text-sm text-zinc-600 dark:text-zinc-400">
            {children()}
        </p>
    }
}

#[component]
pub fn TimecardTime(children: Children) -> impl IntoView {
    view! {
        <h2 class="text-2xl font-semibold text-zinc-900 dark:text-zinc-100">
            {children()}
        </h2>
    }
}

#[component]
pub fn TimecardDate(children: Children) -> impl IntoView {
    view! {
        <p class="text-sm text-zinc-600 dark:text-zinc-400">
            {children()}
        </p>
    }
}
