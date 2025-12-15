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
            class="
                rounded-2xl p-8 content-center relative snap-center z-1 transition
            "
        >
            // add a pinging dot if this is the current time
            {ping.then(||view! {
                <span class="absolute top-2 right-2 flex size-3">
                    <span class="absolute inline-flex h-full w-full animate-ping rounded-full bg-teal-400 opacity-75"></span>
                    <span class="relative inline-flex size-3 rounded-full bg-teal-500"></span>
                </span>
            })}

            <div class="p-6 flex-none text-center">
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
