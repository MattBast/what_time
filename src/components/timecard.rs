use leptos::prelude::*;

#[component]
pub fn Timecard(children: Children) -> impl IntoView {
    view! {
        <div
            class="rounded-2xl relative snap-center z-1 transition"
        >
            {children()}
        </div>
    }
}
