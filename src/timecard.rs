use crate::timezone::TimeIncrement;
use leptos::prelude::*;

#[component]
pub fn Timecard(hour: TimeIncrement) -> impl IntoView {
    let now = hour.now.clone();
    let time = ArcRwSignal::new(hour);

    view! {
        <div class="rounded-2xl border border-zinc-100 dark:border-zinc-700/40 w-40 relative snap-center">
            // add a pinging dot if this is the current time
            {now.then(||view! {
                <span class="absolute top-2 right-2 flex size-3">
                    <span class="absolute inline-flex h-full w-full animate-ping rounded-full bg-teal-400 opacity-75"></span>
                    <span class="relative inline-flex size-3 rounded-full bg-teal-500"></span>
                </span>
            })}

            <div class="p-6 w-40 flex-none">
                <p class="my-2 text-sm text-zinc-600 dark:text-zinc-400">
                    {time.get_untracked().display_header()}
                </p>
                <h2 class="text-2xl font-semibold text-zinc-900 dark:text-zinc-100">
                    {time.get_untracked().display_time()}
                </h2>
                <p class="text-sm text-zinc-600 dark:text-zinc-400">
                    {time.get_untracked().display_date()}
                </p>
            </div>
        </div>
    }
}
