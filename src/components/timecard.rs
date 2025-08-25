use crate::timezone::TimeIncrement;
// use crate::CURRENT_TIME;
use leptos::html::Div;
use leptos::prelude::*;
// use leptos_router::hooks::query_signal;
// use leptos_use::{use_element_bounding, UseElementBoundingReturn};

#[component]
pub fn Timecard(
    hour: TimeIncrement,
    // centre_left: Signal<f64>,
    // centre_right: Signal<f64>,
    // #[prop(optional)] is_highlighted: bool
) -> impl IntoView {
    let now = hour.now.clone();
    let time = ArcRwSignal::new(hour);
    let time_one = time.clone();
    let time_two = time.clone();

    let is_highlighted = RwSignal::new(false);

    let card_ref = NodeRef::<Div>::new();

    // // Use element visibility to detect when the center detector is visible
    // let UseElementBoundingReturn { right, left, .. } = use_element_bounding(card_ref);

    // // Use to set the presented time in the url
    // let (_current_time, set_current_time) = query_signal::<i64>(CURRENT_TIME);

    // Effect::new(move || {
    //     if left.get() > centre_left.get() && right.get() < centre_right.get() {
    //         is_highlighted.set(true);

    //         // Wait for the user to stop scrolling before changing the time in the url
    //         request_animation_frame(move || set_current_time.set(Some(hour.timestamp())));
    //     } else {
    //         is_highlighted.set(false);
    //     }
    // });

    view! {
        <div
            node_ref=card_ref
            class=("scale-105", move || is_highlighted.get())
            class=("scale-75", move || !is_highlighted.get())
            class="rounded-2xl border border-zinc-100 dark:border-zinc-700/40 w-40 content-center relative snap-center z-1 transition"
        >
            // add a pinging dot if this is the current time
            {now.then(||view! {
                <span class="absolute top-2 right-2 flex size-3">
                    <span class="absolute inline-flex h-full w-full animate-ping rounded-full bg-teal-400 opacity-75"></span>
                    <span class="relative inline-flex size-3 rounded-full bg-teal-500"></span>
                </span>
            })}

            <div class="p-6 w-40 flex-none">

                // {move || is_highlighted.get().then(||view! {
                    <p class="my-2 text-sm text-zinc-600 dark:text-zinc-400">
                        {time_one.get_untracked().display_header()}
                    </p>
                // })}

                <h2 class="text-2xl font-semibold text-zinc-900 dark:text-zinc-100">
                    {time.get_untracked().display_time()}
                </h2>

                // {move || is_highlighted.get().then(||view! {
                    <p class="text-sm text-zinc-600 dark:text-zinc-400">
                        {time_two.get_untracked().display_date()}
                    </p>
                // })}

            </div>
        </div>
    }
}
